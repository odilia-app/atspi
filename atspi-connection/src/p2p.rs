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
use std::sync::Arc;
use zbus::{
	conn::Builder,
	fdo::DBusProxy,
	names::{BusName, OwnedBusName},
	proxy::CacheProperties,
	zvariant::ObjectPath,
	Address,
};

#[cfg(feature = "tracing")]
use tracing::warn;

use crate::AtspiResult;

const ACCESSIBLE_ROOT_OBJECT_PATH: &str = "/org/a11y/atspi/accessible/root";
const REGISTRY_WELL_KNOWN_NAME: &str = "org.a11y.atspi.Registry";

/// Represents a peer with the name, path and connection for the P2P peer.
#[derive(Clone, Debug)]
pub struct Peer {
	bus_name: OwnedBusName,
	socket_address: Address,
	p2p_connection: zbus::Connection,
}

impl Peer {
	/// Creates a new `Peer` with the given bus name and socket path.
	pub(crate) async fn try_new<B, S>(bus_name: B, socket: S) -> Result<Self, AtspiError>
	where
		B: Into<OwnedBusName>,
		S: TryInto<Address>,
	{
		let socket_address = socket
			.try_into()
			.map_err(|_| AtspiError::ParseError("Bus address string did not parse"))?;
		let bus_name: OwnedBusName = bus_name.into();

		let p2p_connection = Builder::address(socket_address.clone())?.p2p().build().await?;

		Ok(Peer { bus_name, socket_address, p2p_connection })
	}

	/// Returns the bus name of the peer.
	#[must_use]
	pub fn bus_name(&self) -> &OwnedBusName {
		&self.bus_name
	}

	/// Returns the socket path of the peer.
	#[must_use]
	pub fn socket_address(&self) -> &Address {
		&self.socket_address
	}

	/// Returns the zbus connection for the peer.
	pub fn connection(&self) -> &zbus::Connection {
		&self.p2p_connection
	}

	/// Try to create a new `Peer` from a bus name.
	///
	/// # Errors
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
		Self::try_new(bus_name, socket_path.as_str()).await
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
	pub async fn as_accessible_proxy(&self, obj: &ObjectRef) -> AtspiResult<AccessibleProxy<'_>> {
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
	/// Assemble list of initial peers that support P2P connections.
	fn initial_peers(
		conn: &zbus::Connection,
	) -> impl std::future::Future<Output = AtspiResult<Arc<Mutex<Vec<Peer>>>>>;

	/// Returns a `Peer` for the given bus name.
	fn get_peer(&self, bus_name: &BusName) -> impl std::future::Future<Output = Option<Peer>>;

	/// An associated function that spawns a task to continuously update the list of `Peers`.
	fn spawn_peer_listener_task(
		conn: &zbus::Connection,
		dbus_proxy: DBusProxy<'_>,
		peers: Arc<Mutex<Vec<Peer>>>,
	);

	/// Returns an `AccessibleProxy` with a P2P connection for the given object if available,
	/// otherwise returns an `AccessibleProxy` with a bus connection.
	fn object_as_accessible(
		&'_ self,
		obj: &ObjectRef,
	) -> impl std::future::Future<Output = AtspiResult<AccessibleProxy<'_>>>;

	/// Returns an `AccessibleProxy` with a P2P connection for the given bus name if available,
	/// otherwise returns an `AccessibleProxy` with a bus connection.
	fn bus_name_as_root_accessible(
		&'_ self,
		name: &BusName,
	) -> impl std::future::Future<Output = AtspiResult<AccessibleProxy<'_>>>;

	/// Return a list of peers that are currently connected.
	fn peers(&self) -> impl std::future::Future<Output = AtspiResult<Vec<Peer>>>;
}

impl P2P for crate::AccessibilityConnection {
	async fn initial_peers(conn: &zbus::Connection) -> AtspiResult<Arc<Mutex<Vec<Peer>>>> {
		let reg_accessible = AccessibleProxy::builder(conn)
			.path(ACCESSIBLE_ROOT_OBJECT_PATH)?
			.destination(REGISTRY_WELL_KNOWN_NAME)?
			.cache_properties(CacheProperties::No)
			.build()
			.await?;

		let children = reg_accessible.get_children().await?;
		let mut peers = Vec::with_capacity(children.len());

		for child in children {
			let accessible_proxy = child.as_accessible_proxy(conn).await?;
			let proxies = accessible_proxy.proxies().await?;
			let application_proxy = proxies.application().await?;

			// Get the application bus address
			if let Ok(address) = application_proxy.get_application_bus_address().await {
				let bus_name = BusName::from(child.name);

				// We depend on the implementation of `ObjectRef` having `OwnedUniqueName` as the
				// bus name, so we can later safely compare it to a `UniqueName`.
				// Cost of the assertion is at startup, so it should not affect performance.
				assert!(
					matches!(bus_name, BusName::Unique(_)),
					"Expected bus name to be a unique name, ObjectRef implementation changed: {bus_name}"
				);

				let peer = Peer::try_new(bus_name, address.as_str()).await?;
				peers.push(peer);
			}
		}

		Ok(Arc::new(Mutex::new(peers)))
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
					Err(_err) => {
						#[cfg(feature = "tracing")]
						warn!("Failed to get DBusProxy `NameAcquired` stream: {}", _err);
						return;
					}
				};
				let mut name_lost_stream = match dbus_proxy.receive_name_lost().await {
					Ok(stream) => stream,
					Err(_err) => {
						#[cfg(feature = "tracing")]
						warn!("Failed to get DBusProxy `NameLost` stream: {}", _err);
						return;
					}
				};

				loop {
					// Handle `NameAcquired` and `NameLost` streams separately
					match name_acquired_stream.next().await {
						Some(name_acquired) => {
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
								Err(_err) => {
									#[cfg(feature = "tracing")]
									tracing::warn!("Failed to create peer from bus name: {}", _err);
								}
							}
						}
						None => {
							// If the stream is terminated, break the loop
							#[cfg(feature = "tracing")]
							tracing::debug!("NameAcquired stream ended");
							break;
						}
					}

					match name_lost_stream.next().await {
						Some(name_lost) => {
							let Ok(args) = name_lost.args() else {
								#[cfg(feature = "tracing")]
								tracing::warn!("Received name lost event without bus name");
								continue;
							};

							let bus_name = args.name().clone();
							let mut peers_lock = peers.lock().await;
							peers_lock.retain(|peer| peer.bus_name != bus_name);
						}
						None => {
							#[cfg(feature = "tracing")]
							tracing::warn!(
							"NameAcquired or NameLost stream terminated, stopping listener task"
							);
							break;
						}
					}
				}
			})
			.detach();
	}

	/// Returns an `AccessibleProxy` with a P2P connection for the given object if available,
	/// otherwise returns an `AccessibleProxy` with a bus connection.
	async fn object_as_accessible(&self, obj: &ObjectRef) -> AtspiResult<AccessibleProxy<'_>> {
		// Look up peer by bus name
		let lookup = self
			.peers
			.lock()
			.await
			.iter()
			// If the stored bus name is created from an `ObjectRef`, which carries an `OwnedUserName`,
			// We don't need to take RHS Well-KnownName into consideration.
			.find(|peer| {
				let BusName::Unique(lhs) = &*peer.bus_name else { return false };
				*lhs == obj.name
			})
			.cloned();

		if let Some(peer) = lookup {
			// If a peer is found, create an `AccessibleProxy` with a P2P connection
			AccessibleProxy::builder(peer.connection())
				.path(obj.path.clone())?
				.cache_properties(CacheProperties::No)
				.build()
				.await
				.map_err(Into::into)
		} else {
			// If no peer is found, fall back to the bus connection
			let conn = self.connection();
			AccessibleProxy::builder(conn)
				.path(obj.path.clone())?
				.cache_properties(CacheProperties::No)
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
	) -> AtspiResult<AccessibleProxy<'_>> {
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
				.cache_properties(CacheProperties::No)
				.build()
				.await
				.map_err(Into::into)
		}
	}

	async fn get_peer(&self, bus_name: &BusName<'_>) -> Option<Peer> {
		let peers = self.peers.lock().await;

		peers.iter().find(|peer| &peer.bus_name == bus_name).cloned()
	}

	/// Get the list of peers
	///
	/// # Errors
	/// This will return an error if the peers cannot be retrieved from behind the mutex.
	async fn peers(&self) -> AtspiResult<Vec<Peer>> {
		let peers = self.peers.lock().await;
		Ok(peers.clone())
	}
}
