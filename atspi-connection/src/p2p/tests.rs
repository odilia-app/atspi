use super::*;

enum Capability {
	Address(String),
	Failed,
	Silent,
}

struct TestApplication {
	capability: Capability,
	probes: Arc<std::sync::atomic::AtomicUsize>,
	delay: Duration,
}

#[zbus::interface(name = "org.a11y.atspi.Application")]
impl TestApplication {
	async fn get_application_bus_address(&self) -> zbus::fdo::Result<String> {
		self.probes.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
		async_io::Timer::after(self.delay).await;
		match &self.capability {
			Capability::Address(address) => Ok(address.clone()),
			Capability::Failed => Err(zbus::fdo::Error::Failed("probe failed".into())),
			Capability::Silent => std::future::pending().await,
		}
	}
}

struct TestAccessible(&'static str);

#[zbus::interface(name = "org.a11y.atspi.Accessible")]
impl TestAccessible {
	fn get_role_name(&self) -> &'static str {
		self.0
	}
}

async fn application(
	capability: Capability,
) -> (zbus::Connection, Arc<std::sync::atomic::AtomicUsize>) {
	application_delayed(capability, Duration::ZERO).await
}

async fn application_delayed(
	capability: Capability,
	delay: Duration,
) -> (zbus::Connection, Arc<std::sync::atomic::AtomicUsize>) {
	let probes = Arc::new(std::sync::atomic::AtomicUsize::new(0));
	let connection = Builder::session()
		.unwrap()
		.serve_at(
			ACCESSIBLE_ROOT_PATH,
			TestApplication { capability, delay, probes: Arc::clone(&probes) },
		)
		.unwrap()
		.serve_at(ACCESSIBLE_ROOT_PATH, TestAccessible("shared"))
		.unwrap()
		.build()
		.await
		.unwrap();
	(connection, probes)
}

async fn wrapper() -> crate::AccessibilityConnection {
	crate::AccessibilityConnection::from_address(
		std::env::var("DBUS_SESSION_BUS_ADDRESS").unwrap().parse().unwrap(),
	)
	.await
	.unwrap()
}

#[cfg(unix)]
struct PeerSocket {
	listener: async_io::Async<std::os::unix::net::UnixListener>,
	directory: std::path::PathBuf,
}

#[cfg(unix)]
impl PeerSocket {
	fn new() -> Self {
		let directory = std::env::temp_dir().join(format!("atspi-peer-{}", zbus::Guid::generate()));
		std::fs::create_dir(&directory).unwrap();
		let listener = std::os::unix::net::UnixListener::bind(directory.join("peer")).unwrap();
		Self { listener: async_io::Async::new(listener).unwrap(), directory }
	}

	fn address(&self) -> String {
		format!("unix:path={}", self.directory.join("peer").display())
	}

	async fn accept(&self) -> std::os::unix::net::UnixStream {
		self.listener
			.read_with(std::os::unix::net::UnixListener::accept)
			.await
			.unwrap()
			.0
	}
}

#[cfg(unix)]
impl Drop for PeerSocket {
	fn drop(&mut self) {
		std::fs::remove_file(self.directory.join("peer")).unwrap();
		std::fs::remove_dir(&self.directory).unwrap();
	}
}

#[cfg(unix)]
async fn expect_closed(socket: std::os::unix::net::UnixStream) {
	use std::io::Read;
	let socket = async_io::Async::new(socket).unwrap();
	bounded(async {
		let mut buffer = [0; 1024];
		while socket.read_with(|mut socket| socket.read(&mut buffer)).await.unwrap() != 0 {}
	})
	.await;
}

#[cfg(unix)]
#[test]
fn invalidation_and_clear_cancel_silent_handshakes_after_caller_drop() {
	tokio_test::block_on(bounded(async {
		for clear in [false, true] {
			let socket = PeerSocket::new();
			let (server, _) = application(Capability::Address(socket.address())).await;
			let client = zbus::Connection::session().await.unwrap();
			let peers = Peers::default();
			let name = BusName::Unique(server.unique_name().unwrap().as_ref());
			let mut lookup = Box::pin(peers.get(&client, &name));
			assert!(futures_lite::future::poll_once(&mut lookup).await.is_none());
			let accepted = socket.accept().await;
			drop(lookup);
			let start = Instant::now();
			if clear {
				peers.clear();
			} else {
				peers.invalidate_owner(server.unique_name().unwrap());
			}
			expect_closed(accepted).await;
			assert!(
				start.elapsed() < Duration::from_millis(500),
				"invalidation must cancel I/O, not wait for deadline"
			);
			assert!(peers.snapshot().is_empty());
		}
	}));
}

#[cfg(unix)]
#[test]
fn direct_discovery_serves_a_real_accessible_method() {
	tokio_test::block_on(bounded(async {
		let socket = PeerSocket::new();
		let (server, probes) = application(Capability::Address(socket.address())).await;
		let wrapper = wrapper().await;
		let name = BusName::Unique(server.unique_name().unwrap().as_ref());
		let serve = async {
			let stream = socket.accept().await;
			stream.set_nonblocking(true).unwrap();
			// zbus expects std or Tokio UnixStream after workspace feature unification.
			#[allow(clippy::useless_conversion)]
			let builder = Builder::unix_stream(stream.try_into().unwrap());
			builder
				.server(zbus::Guid::generate())
				.unwrap()
				.p2p()
				.serve_at(ACCESSIBLE_ROOT_PATH, TestAccessible("direct"))
				.unwrap()
				.build()
				.await
				.unwrap()
		};
		// Poll both ends of the real authentication handshake.
		let (direct, proxy) =
			futures_lite::future::zip(serve, wrapper.bus_name_as_root_accessible(&name)).await;
		assert_eq!(proxy.unwrap().get_role_name().await.unwrap(), "direct");
		assert_eq!(wrapper.peers().len(), 1);
		assert_eq!(probes.load(std::sync::atomic::Ordering::SeqCst), 1);
		direct.close().await.unwrap();
	}));
}

#[test]
fn listener_drains_churn_without_probing_untargeted_clients() {
	tokio_test::block_on(bounded(async {
		let wrapper = wrapper().await;
		let (silent, probes) = application(Capability::Silent).await;
		// Exceed the ownership stream's queue capacity with a silent client present.
		for index in 0..80 {
			let alias = format!("org.example.Churn{index}");
			silent.request_name(alias.as_str()).await.unwrap();
			silent.release_name(alias.as_str()).await.unwrap();
		}
		let marker = "org.example.DrainMarker";
		silent.request_name(marker).await.unwrap();
		wait_until(|| wrapper.peers.state.lock().unwrap().aliases.contains_key(marker)).await;
		assert!(DBusProxy::new(wrapper.connection())
			.await
			.unwrap()
			.list_names()
			.await
			.is_ok());
		assert_eq!(probes.load(std::sync::atomic::Ordering::SeqCst), 0);
	}));
}

#[test]
fn unavailable_capabilities_use_shared_bus_and_cache_failures() {
	tokio_test::block_on(bounded(async {
		let wrapper = wrapper().await;
		for capability in [
			Capability::Address(String::new()),
			Capability::Address("not a dbus address".into()),
			Capability::Address("unix:path=/nonexistent/atspi-peer".into()),
			Capability::Failed,
		] {
			let unsupported =
				matches!(&capability, Capability::Address(address) if address.is_empty());
			let (server, probes) = application(capability).await;
			let name = BusName::Unique(server.unique_name().unwrap().as_ref());
			for _ in 0..2 {
				let proxy = wrapper.bus_name_as_root_accessible(&name).await.unwrap();
				assert_eq!(proxy.get_role_name().await.unwrap(), "shared");
			}
			assert_eq!(probes.load(std::sync::atomic::Ordering::SeqCst), 1);
			let owner = server.unique_name().unwrap();
			{
				let state = wrapper.peers.state.lock().unwrap();
				assert!(if unsupported {
					matches!(state.owners.get(owner.as_str()), Some(OwnerState::Unsupported { .. }))
				} else {
					matches!(state.owners.get(owner.as_str()), Some(OwnerState::Transient { .. }))
				});
			}
			if unsupported {
				wrapper.peers.invalidate_owner(owner);
				assert_eq!(
					wrapper
						.bus_name_as_root_accessible(&name)
						.await
						.unwrap()
						.get_role_name()
						.await
						.unwrap(),
					"shared"
				);
				assert_eq!(probes.load(std::sync::atomic::Ordering::SeqCst), 2);
			}
		}
		assert!(wrapper.peers().is_empty());
	}));
}

#[cfg(unix)]
#[test]
fn total_discovery_deadline_closes_silent_socket_and_routes_shared() {
	tokio_test::block_on(bounded(async {
		let socket = PeerSocket::new();
		let (server, probes) =
			application_delayed(Capability::Address(socket.address()), Duration::from_millis(650))
				.await;
		let wrapper = wrapper().await;
		let name = BusName::Unique(server.unique_name().unwrap().as_ref());
		let start = Instant::now();
		let mut lookup = Box::pin(wrapper.bus_name_as_root_accessible(&name));
		assert!(futures_lite::future::poll_once(&mut lookup).await.is_none());
		let accepted = socket.accept().await;
		assert_eq!(lookup.await.unwrap().get_role_name().await.unwrap(), "shared");
		assert!(
			start.elapsed() < Duration::from_millis(1400),
			"probe must not reset direct-build budget"
		);
		expect_closed(accepted).await;
		assert!(wrapper.get_peer(&name).await.unwrap().is_none());
		assert_eq!(probes.load(std::sync::atomic::Ordering::SeqCst), 1);
	}));
}

#[test]
fn silent_capability_probe_has_a_shared_deadline_and_falls_back() {
	tokio_test::block_on(bounded(async {
		let (server, probes) = application(Capability::Silent).await;
		let wrapper = wrapper().await;
		let name = BusName::Unique(server.unique_name().unwrap().as_ref());
		let mut first = Box::pin(wrapper.get_peer(&name));
		assert!(futures_lite::future::poll_once(&mut first).await.is_none());
		wait_until(|| probes.load(std::sync::atomic::Ordering::SeqCst) == 1).await;
		drop(first);
		let proxy = wrapper.bus_name_as_root_accessible(&name).await.unwrap();
		assert_eq!(proxy.get_role_name().await.unwrap(), "shared");
		assert_eq!(probes.load(std::sync::atomic::Ordering::SeqCst), 1);
	}));
}

#[test]
fn immediate_peer_constructor_also_bounds_the_capability_probe() {
	tokio_test::block_on(bounded(async {
		let (server, _) = application(Capability::Silent).await;
		let client = zbus::Connection::session().await.unwrap();
		let result = Peer::try_from_bus_name(
			BusName::Unique(server.unique_name().unwrap().as_ref()),
			&client,
		)
		.await;
		assert!(result.unwrap_err().to_string().contains("deadline"));
	}));
}

async fn bounded<T>(future: impl Future<Output = T>) -> T {
	futures_lite::future::or(future, async {
		async_io::Timer::after(Duration::from_secs(3)).await;
		panic!("test operation exceeded three seconds");
	})
	.await
}

async fn wait_until(mut condition: impl FnMut() -> bool) {
	bounded(async {
		while !condition() {
			async_io::Timer::after(Duration::from_millis(2)).await;
		}
	})
	.await;
}

#[test]
fn alias_transfer_during_discovery_rejects_stale_shared_fallback() {
	tokio_test::block_on(bounded(async {
		let (old, probes) = application(Capability::Silent).await;
		let new = zbus::Connection::session().await.unwrap();
		let wrapper = wrapper().await;
		let alias = WellKnownName::try_from("org.example.DuringDiscovery").unwrap();
		old.request_name(alias.clone()).await.unwrap();
		wait_until(|| {
			wrapper
				.peers
				.state
				.lock()
				.unwrap()
				.aliases
				.contains_key(alias.as_str())
		})
		.await;
		let name = BusName::WellKnown(alias.clone());
		let mut lookup = Box::pin(wrapper.bus_name_as_root_accessible(&name));
		assert!(futures_lite::future::poll_once(&mut lookup).await.is_none());
		wait_until(|| probes.load(std::sync::atomic::Ordering::SeqCst) == 1).await;
		old.release_name(alias.clone()).await.unwrap();
		new.request_name(alias.clone()).await.unwrap();
		wait_until(|| {
			wrapper
				.peers
				.state
				.lock()
				.unwrap()
				.aliases
				.get(alias.as_str())
				.is_some_and(|entry| &entry.owner == new.unique_name().unwrap())
		})
		.await;
		assert!(lookup.await.is_err(), "must not route old owner on shared fallback");
		assert!(wrapper.peers().is_empty());
	}));
}

#[test]
fn late_owner_reply_is_rejected_after_transfer() {
	tokio_test::block_on(bounded(async {
		let old = zbus::Connection::session().await.unwrap();
		let new = zbus::Connection::session().await.unwrap();
		let client = zbus::Connection::session().await.unwrap();
		let alias = WellKnownName::try_from("org.example.LateOwner").unwrap();
		old.request_name(alias.clone()).await.unwrap();
		let peers = Peers {
			discover: Arc::new(|connection, owner| {
				Box::pin(async move {
					DiscoveryResult::Ready(Peer {
						unique_name: owner,
						well_known_name: None,
						socket_address: Address::try_from("unix:path=/unused").unwrap(),
						p2p_connection: connection,
					})
				})
			}),
			..Peers::default()
		};
		peers.spawn_listener(&client).await.unwrap();
		let name = BusName::WellKnown(alias.clone());
		let mut lookup = Box::pin(peers.get(&client, &name));
		assert!(futures_lite::future::poll_once(&mut lookup).await.is_none());
		// A subsequent bus reply is a barrier: the real GetNameOwner reply is
		// already queued, but its consumer has not been polled again.
		DBusProxy::new(&client).await.unwrap().list_names().await.unwrap();
		old.release_name(alias.clone()).await.unwrap();
		new.request_name(alias.clone()).await.unwrap();
		wait_until(|| {
			peers
				.state
				.lock()
				.unwrap()
				.aliases
				.get(alias.as_str())
				.is_some_and(|entry| &entry.owner == new.unique_name().unwrap())
		})
		.await;
		assert!(lookup.await.is_err(), "must not admit or route the old owner");
		assert!(!peers
			.state
			.lock()
			.unwrap()
			.owners
			.contains_key(old.unique_name().unwrap()));
	}));
}
