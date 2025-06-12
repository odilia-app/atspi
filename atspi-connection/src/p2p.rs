//! Parts for use with P2P connections.

use async_executor::Executor;
use async_lock::Mutex;
use atspi_common::{AtspiError, ObjectRef};
use atspi_proxies::{
	accessible::{AccessibleProxy, ObjectRefExt},
	application::ApplicationProxy,
	proxy_ext::ProxyExt,
};
use futures_lite::stream::StreamExt;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::Arc;
use zbus::{
	fdo::DBusProxy,
	names::{BusName, OwnedBusName},
	zvariant::ObjectPath,
};

#[cfg(feature = "tracing")]
use tracing::warn;

use crate::AtspiResult;

const UNIX_SOCKET_PATH_PREFIX: &str = "unix:path=";
const ACCESSIBLE_ROOT_OBJECT_PATH: &str = "/org/a11y/atspi/accessible/root";

/// Represents a peer with the name and path for a P2P connection.
#[derive(Clone, Debug)]
pub struct Peer {
	bus_name: OwnedBusName,
	socket_path: PathBuf,
	p2p_connection: zbus::Connection,
}

impl Peer {
	/// Creates a new `Peer` with the given bus name and socket path.
	pub(crate) async fn try_new<B, S>(bus_name: B, socket_path: S) -> Result<Self, AtspiError>
	where
		B: Into<OwnedBusName>,
		S: Into<PathBuf>,
	{
		let bus_name: OwnedBusName = bus_name.into();
		let socket_path: PathBuf = socket_path.into();

		let unix_stream = UnixStream::connect(&socket_path)?;

		let p2p_connection = zbus::connection::Builder::unix_stream(unix_stream)
			.p2p()
			.build()
			.await?;

		Ok(Peer { bus_name, socket_path, p2p_connection })
	}

	/// Returns the bus name of the peer.
	#[must_use]
	pub fn bus_name(&self) -> &OwnedBusName {
		&self.bus_name
	}

	/// Returns the socket path of the peer.
	#[must_use]
	pub fn socket_path(&self) -> &PathBuf {
		&self.socket_path
	}

	/// Returns the zbus connection for the peer.
	pub fn connection(&self) -> &zbus::Connection {
		&self.p2p_connection
	}

	/// Try to create a new `Peer` from a bus name.
	///
	/// # Errors
	/// If the appliocation (bus name) does not implement the `Application` interface,
	/// or when it does not support `get_application_bus_address`.\
	/// A non-existent bus name will also return an error.
	pub async fn try_from_bus_name(
		bus_name: BusName<'_>,
		conn: &zbus::Connection,
	) -> AtspiResult<Self> {
		// Get the application proxy for the bus name
		let application_proxy = ApplicationProxy::builder(conn)
			.destination(&bus_name)?
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await?;

		let socket_path = application_proxy.get_application_bus_address().await?;
		let socket_path = socket_path
			.strip_prefix(UNIX_SOCKET_PATH_PREFIX)
			.ok_or(AtspiError::InvalidSocketPath)?;

		Self::try_new(bus_name, socket_path).await
	}

	/// Returns a `Proxies` object, to access the proxies of the peer.
	///
	/// # Errors
	/// On invalid object path.
	pub async fn proxies(
		&'_ self,
		path: &ObjectPath<'_>,
	) -> AtspiResult<atspi_proxies::proxy_ext::Proxies<'_>> {
		let accessible_proxy = AccessibleProxy::builder(&self.p2p_connection)
			.path(path.to_owned())?
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await?;

		accessible_proxy.proxies().await
	}

	/// Returns an `AccessibleProxy` for the root accessible object of the peer.
	///
	/// # Errors
	/// In case of an anvalid connection.
	pub async fn as_root_accessible_proxy(&self) -> AtspiResult<AccessibleProxy<'_>> {
		AccessibleProxy::builder(&self.p2p_connection)
			.path(ACCESSIBLE_ROOT_OBJECT_PATH)?
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await
			.map_err(AtspiError::from)
	}

	/// Returns an `AccessibleProxy` for the accessible object of the peer.
	///
	/// # Errors
	/// In case of an invalid connection or object path.
	pub async fn as_accessible_proxy(&'_ self, obj: &ObjectRef) -> AtspiResult<AccessibleProxy> {
		AccessibleProxy::builder(&self.p2p_connection)
			.path(obj.path.clone())?
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await
			.map_err(AtspiError::from)
	}
}

/// Trait for P2P connection handling.
pub trait P2P {
	/// Retrrieve list of peers that are currently connected.
	async fn initial_peers(conn: &zbus::Connection) -> AtspiResult<Arc<Mutex<Vec<Peer>>>>;

	/// Returns a `Peer` for the given bus name.
	async fn get_peer(&self, bus_name: &BusName) -> Option<Peer>;

	/// Spawns a task to continuously update the `Peers`.
	fn spawn_peer_listener_task(
		conn: &zbus::Connection,
		dbus_proxy: DBusProxy<'_>,
		peers: Arc<Mutex<Vec<Peer>>>,
	);

	/// Returns an `AccessibleProxy` with a P2P connection for the given object if available,
	/// otherwise returns an `AccessibleProxy` with a bus connection.
	async fn object_as_accessible(&self, obj: &ObjectRef) -> AtspiResult<AccessibleProxy>;

	/// Returns an `AccessibleProxy` with a P2P connection for the given bus name if available,
	/// otherwise returns an `AccessibleProxy` with a bus connection.
	async fn bus_name_as_root_accessible(&self, name: &BusName) -> AtspiResult<AccessibleProxy>;
}

impl P2P for crate::AccessibilityConnection {
	async fn initial_peers(conn: &zbus::Connection) -> AtspiResult<Arc<Mutex<Vec<Peer>>>> {
		let peers = Arc::new(Mutex::new(Vec::new()));

		let reg_accessible = AccessibleProxy::builder(conn)
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await?;

		let children = reg_accessible.get_children().await?;

		for child in children {
			let application_proxy = child
				.as_accessible_proxy(conn)
				.await?
				.proxies()
				.await?
				.application()
				.await;

			let Ok(application_proxy) = application_proxy else {
				#[cfg(feature = "tracing")]
				tracing::warn!(
					"Skipping child that does not appear to implement the `Application` interface: {:?}",
					child
				);
				continue; // Skip if the child is not an ApplicationProxy
			};

			if let Ok(address) = application_proxy.get_application_bus_address().await {
				let Some(address) = address.strip_prefix(UNIX_SOCKET_PATH_PREFIX) else {
					#[cfg(feature = "tracing")]
					tracing::warn!("Skipping peer with non-matching Unix socket path prefix: {address}, expected: {UNIX_SOCKET_PATH_PREFIX}" );
					continue; // Skip if the address is not a valid Unix socket path
				};

				let bus_name = BusName::from(child.name);

				let peer = Peer::try_new(bus_name, address).await?;
				peers.lock().await.push(peer);
			}
		}

		Ok(peers)
	}

	fn spawn_peer_listener_task(
		conn: &zbus::Connection,
		dbus_proxy: DBusProxy<'_>,
		peers: Arc<Mutex<Vec<Peer>>>,
	) {
		let executor = Executor::new();

		executor
			.spawn(async move {
				let mut name_acquired_stream = match dbus_proxy.receive_name_acquired().await {
					Ok(stream) => stream,
					Err(err) => {
						#[cfg(feature = "tracing")]
						warn!("Failed to get DBusProxy `NameAcquired` stream: {}", err);
						return;
					}
				};
				let mut name_lost_stream = match dbus_proxy.receive_name_lost().await {
					Ok(stream) => stream,
					Err(err) => {
						#[cfg(feature = "tracing")]
						warn!("Failed to get DBusProxy `NameLost` stream: {}", err);
						return;
					}
				};

				loop {
					// Handle both separately
					if let Some(name_acquired) = name_acquired_stream.next().await {
						let Ok(args) = name_acquired.args() else {
							#[cfg(feature = "tracing")]
							tracing::warn!("Received name acquired event without bus name");
							continue;
						};

						let bus_name = args.name().clone();

						let peer = Peer::try_from_bus_name(bus_name, conn).await;
						match peer {
							Ok(peer) => {
								let mut peers_lock = peers.lock().await;
								peers_lock.push(peer);
							}
							Err(e) => {
								#[cfg(feature = "tracing")]
								tracing::warn!("Failed to create peer from bus name: {}", e);
							}
						}
					}

					if let Some(name_lost) = name_lost_stream.next().await {
						let Ok(args) = name_lost.args() else {
							#[cfg(feature = "tracing")]
							tracing::warn!("Received name lost event without bus name");
							continue;
						};

						let bus_name = args.name().clone();
						let mut peers_lock = peers.lock().await;
						peers_lock.retain(|peer| peer.bus_name != bus_name);
					}
				}
			})
			.detach();
	}

	/// Returns an `AccessibleProxy` with a P2P connection for the given object if available,
	/// otherwise returns an `AccessibleProxy` with a bus connection.
	async fn object_as_accessible(&self, obj: &ObjectRef) -> AtspiResult<AccessibleProxy> {
		// Look up peer by bus name
		let lookup = self
			.peers
			.lock()
			.await
			.iter()
			.find(|peer| peer.bus_name == BusName::from(obj.name.clone()))
			.cloned();

		if let Some(peer) = lookup {
			// If a peer is found, create an AccessibleProxy with a P2P connection
			AccessibleProxy::builder(peer.connection())
				.path(obj.path.clone())?
				.cache_properties(zbus::proxy::CacheProperties::No)
				.build()
				.await
				.map_err(Into::into)
		} else {
			// If no peer is found, fall back to the bus connection
			let conn = self.connection();
			AccessibleProxy::builder(conn)
				.path(obj.path.clone())?
				.cache_properties(zbus::proxy::CacheProperties::No)
				.build()
				.await
				.map_err(Into::into)
		}
	}

	/// Returns an `AccessibleProxy` with a P2P connection for the given bus name if available,
	/// otherwise returns an `AccessibleProxy` with a bus connection.
	async fn bus_name_as_root_accessible(
		&'_ self,
		name: &BusName<'_>,
	) -> AtspiResult<AccessibleProxy> {
		// Look up peer by bus name
		let lookup = self
			.peers
			.lock()
			.await
			.iter()
			.find(|peer| &peer.bus_name == name)
			.cloned();

		if let Some(peer) = lookup {
			// If a peer is found, create an AccessibleProxy with a P2P connection
			AccessibleProxy::builder(peer.connection())
				.path(ACCESSIBLE_ROOT_OBJECT_PATH)?
				.cache_properties(zbus::proxy::CacheProperties::No)
				.build()
				.await
				.map_err(Into::into)
		} else {
			// If no peer is found, fall back to the bus connection
			let conn = self.connection();
			AccessibleProxy::builder(conn)
				.path(ACCESSIBLE_ROOT_OBJECT_PATH)?
				.cache_properties(zbus::proxy::CacheProperties::No)
				.build()
				.await
				.map_err(Into::into)
		}
	}

	async fn get_peer(&self, bus_name: &BusName<'_>) -> Option<Peer> {
		let peers = self.peers.lock().await;

		peers.iter().find(|peer| &peer.bus_name == bus_name).cloned()
	}
}
