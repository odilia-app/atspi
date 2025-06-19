//! Extends the `AccessibilityConnection` with P2P capabilities.

use async_executor::Executor;
use async_lock::Mutex;
use atspi_common::{AtspiError, ObjectRef};
use atspi_proxies::{
	accessible::{AccessibleProxy, ObjectRefExt},
	application::{self, ApplicationProxy},
	proxy_ext::ProxyExt,
};
use futures_lite::{future::block_on, stream::StreamExt};
use std::sync::Arc;
use zbus::{
	conn::Builder,
	fdo::DBusProxy,
	names::{BusName, OwnedBusName, OwnedUniqueName, OwnedWellKnownName},
	proxy::CacheProperties,
	zvariant::ObjectPath,
	Address,
};

#[cfg(feature = "tracing")]
use tracing::{debug, info, warn};

use crate::AtspiResult;

const ACCESSIBLE_ROOT_OBJECT_PATH: &str = "/org/a11y/atspi/accessible/root";
const REGISTRY_WELL_KNOWN_NAME: &str = "org.a11y.atspi.Registry";

/// Represents a peer with the name, path and connection for the P2P peer.
#[derive(Clone, Debug)]
pub struct Peer {
	unique_name: OwnedUniqueName,
	well_known_name: Option<OwnedWellKnownName>,
	socket_address: Address,
	p2p_connection: zbus::Connection,
}

impl Peer {
	/// Creates a new `Peer` with the given bus name and socket path.
	///
	/// # Note
	/// This function is intended for use in building the initial list of peers.
	///
	/// If given a `UniqueName`, it will check if the peer also owns a well-known name.
	/// If given a `WellKnownName`, it will query the D-Bus for the unique name of the peer.
	///
	/// # Errors
	/// - `DBusProxy` cannot be created.
	/// - The socket address cannot be parsed.
	///
	pub(crate) async fn try_new<B, S>(
		bus_name: B,
		socket: S,
		conn: &zbus::Connection,
	) -> Result<Self, AtspiError>
	where
		B: Into<OwnedBusName>,
		S: TryInto<Address>,
	{
		let dbus_proxy = DBusProxy::new(conn).await?;
		let owned_bus_name: OwnedBusName = bus_name.into();

		// Because D-Bus does not let us query whether a unique name is the owner of a well-known name,
		// we need to query all well-known names and their owners, and then check if

		let well_known_names: Vec<OwnedWellKnownName> = dbus_proxy
			.list_names()
			.await?
			.into_iter()
			.filter_map(|name| {
				if let BusName::WellKnown(well_nown_name) = name.clone().inner() {
					Some(OwnedWellKnownName::from(well_nown_name.clone()))
				} else {
					None
				}
			})
			.collect();

		let unique_to_well_known: Vec<(OwnedUniqueName, OwnedWellKnownName)> = well_known_names
			.iter()
			.filter_map(|well_known_name| {
				block_on(dbus_proxy.get_name_owner(BusName::from(well_known_name.clone())))
					.ok()
					.map(|unique_name| (unique_name, well_known_name.clone()))
			})
			.collect();

		let (unique_name, well_known_name) = match owned_bus_name.inner() {
			BusName::Unique(name) => {
				// The argument name is the mandatory `UniqueName`, we do want check whether this peer also owns a well-known name.
				let owned_well_known_name = unique_to_well_known.iter().find_map(|(u, w)| {
					if u == name {
						Some(w.clone())
					} else {
						None
					}
				});
				let owned_unique_name = OwnedUniqueName::from(name.clone());
				(owned_unique_name, owned_well_known_name)
			}
			BusName::WellKnown(well_known_name) => {
				// If the argument name is a well-known name, we _must_ get its unique name.
				let bus_name = BusName::from(well_known_name.clone());
				let owned_unique_name = dbus_proxy.get_name_owner(bus_name).await?;
				let owned_well_known_name = OwnedWellKnownName::from(well_known_name.clone());
				(owned_unique_name, Some(owned_well_known_name))
			}
		};

		let socket_address = socket
			.try_into()
			.map_err(|_| AtspiError::ParseError("Bus address string did not parse"))?;

		let p2p_connection = Builder::address(socket_address.clone())?.p2p().build().await?;

		Ok(Peer { unique_name, well_known_name, socket_address, p2p_connection })
	}

	/// Returns the bus name of the peer.
	#[must_use]
	pub fn unique_name(&self) -> &OwnedUniqueName {
		&self.unique_name
	}

	/// Returns the well-known bus name of the peer, if it has one.
	#[must_use]
	pub fn well_known_name(&self) -> Option<&OwnedWellKnownName> {
		self.well_known_name.as_ref()
	}

	/// Returns the socket [`Address`][zbus::Address] of the peer.
	#[must_use]
	pub fn socket_address(&self) -> &Address {
		&self.socket_address
	}

	/// Returns the p2p [`Connection`][zbus::Connection] of the peer.
	pub fn connection(&self) -> &zbus::Connection {
		&self.p2p_connection
	}

	/// Try to create a new `Peer` from a bus name.
	///
	/// # Errors
	/// Returns an error if the application proxy cannot be created or when it does not support `get_application_bus_address`.\
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
		Self::try_new(bus_name, socket_path.as_str(), conn).await
	}

	/// Returns a [`Proxies`][atspi_proxies::proxy_ext::Proxies] object for the given object path.\
	/// A [`Proxies`] object is used to obtain any of the proxies the object supports.
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

	/// Returns an [`AccessibleProxy`] for the accessible object of the peer.
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
	/// Returns a `Peer` for the given bus name.
	fn get_peer(&self, bus_name: &BusName) -> impl std::future::Future<Output = Option<Peer>>;

	/// Returns an `AccessibleProxy` with a P2P connection for the given object if available,
	/// otherwise returns an `AccessibleProxy` with a bus connection.
	fn object_as_accessible(
		&'_ self,
		obj: &ObjectRef,
	) -> impl std::future::Future<Output = AtspiResult<AccessibleProxy<'_>>>;

	/// Returns an `AccessibleProxy` to the root accessible object with a P2P connection for the given bus name _if available_.\
	/// If the P2P connection is not available, it returns an `AccessibleProxy` with a bus connection.
	fn bus_name_as_root_accessible(
		&'_ self,
		name: &BusName,
	) -> impl std::future::Future<Output = AtspiResult<AccessibleProxy<'_>>>;

	/// Return a list of peers that are currently connected.
	fn peers(&self) -> impl std::future::Future<Output = Vec<Peer>>;
}

impl crate::AccessibilityConnection {
	/// Returns an `Arc<Mutex<Vec<Peer>>>` containing the initial peers that support P2P connections.
	///
	/// # Note
	/// Intended for internal use with `AccessibilityConnection::new()`.
	///
	/// # Errors
	/// This function can return an error in the following cases:
	/// - the `AccessibleProxy` to the registry cannot be created.
	/// - the registry returns an error when querying for children.
	/// - for any child, the `AccessibleProxy` cannot be created or the `ApplicationProxy` cannot be created.
	pub(crate) async fn initial_peers(
		conn: &zbus::Connection,
	) -> AtspiResult<Arc<Mutex<Vec<Peer>>>> {
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
				let peer = Peer::try_new(bus_name, address.as_str(), conn).await?;
				peers.push(peer);
			}
		}

		Ok(Arc::new(Mutex::new(peers)))
	}

	/// Spawns a task to listen for  peer mutations.
	///
	/// # Async executor
	/// This function uses the `async_executor::Executor` to spawn a task that listens for `NameAcquired` and `NameLost` signals on the `DBus`.
	///
	/// # Note
	/// This function is called internally by `AccessibilityConnection::new()`.
	// TODO: Address the clippy warning about too many lines.
	#[allow(clippy::too_many_lines)]
	pub(crate) fn spawn_peer_listener_task(
		conn: &zbus::Connection,
		dbus_proxy: DBusProxy<'_>,
		peers: Arc<Mutex<Vec<Peer>>>,
	) {
		let executor = Executor::new();

		executor
			.spawn(async move {
				let mut peer_mutations = match dbus_proxy.receive_name_owner_changed().await {
					Ok(stream) => stream,
					Err(_err) => {
						#[cfg(feature = "tracing")]
						warn!("Failed to get DBusProxy `NameOwnerChanged` stream: {}", _err);
						return;
					}
				};

				while let Some(name_owner_event) = peer_mutations.next().await {
					let Ok(mutation) = name_owner_event.args() else {
						#[cfg(feature = "tracing")]
						warn!("Received name owner changed event without args, skipping.");
						continue;
					};
					let name = mutation.name().clone();
					let new = mutation.new_owner().clone();
					let old = mutation.old_owner().clone();

					// `NameOwnerChanged` table (U = Unique, W = Well-Known):
					// | Name | Old Owner | New Owner | Operation |
					// |------|-----------|-----------|----------|
					// |   U  |   None    |  Some(U)  |  Add     |
					// |   U  |   Some(U) |    None   |  Remove  |
					// |   W  |   None    |  Some(U)  |  Add     |
					// |   W  |   Some(U) |    None   |  Remove  |
					// |   W  |   Some(U) |  Some(U)  |  Replace |

					match name {
						BusName::Unique(unique_name) => {
							// `zvariant:Optional` has deref target `Option`.
							match (&*old, &*new) {
								// Application appeared on the bus.
								(None, Some(new_owner)) => {
									debug_assert!(new_owner == &unique_name, "When a name appears on the bus, the new owner must be the unique name itself.");

									let bus_name = BusName::Unique(unique_name.clone());
									let Ok(address) = get_address(bus_name, conn).await else {
										// Most likely the application does not support p2p connections.
										#[cfg(feature = "tracing")]
										info!("Failed to get address for unique name: {}", unique_name);
										continue;
									};

									let Ok(p2p_connection_builder) = Builder::address(address.clone()) else {
										// Most likely the application does not support p2p connections.
										#[cfg(feature = "tracing")]
										info!("Failed to create p2p connection for unique name: {}", unique_name);
										continue;
									};

									let p2p_connection = match p2p_connection_builder.p2p().build().await {
										Ok(conn) => conn,
										Err(_err) => {
											#[cfg(feature = "tracing")]
											warn!("Failed to create p2p connection for unique name {}: {}", unique_name, _err);
											continue;
										}
									};

									let unique_name = OwnedUniqueName::from(unique_name);
									let peer = Peer {
										unique_name,
										well_known_name: None,
										socket_address: address,
										p2p_connection,
									};

									let mut peers_lock = peers.lock().await;
									// Add the new peer to the list.
									peers_lock.push(peer);
								}
								// Unique name left the bus.
								(Some(old), None) => {
									debug_assert!(old == &unique_name, "When a unique name is removed from the bus, the old owner must be the unique name itself.");
									let unique_name = OwnedUniqueName::from(unique_name);
									let mut peers_lock = peers.lock().await;
									peers_lock.retain(|p| *p.unique_name() != unique_name);
								}
								// Unknown combination.
								(Some(_), Some(_)) => {
									#[cfg(feature = "tracing")]
									debug!("Received `NameOwnerChanged` event with both old and new owner for unique name: {}", unique_name);
								}
								// Unknown combination.
								(None, None) => {
									#[cfg(feature = "tracing")]
									debug!("Received `NameOwnerChanged` event with no old or new owner for unique name: {}", unique_name);
								}
							}
						}
						BusName::WellKnown(well_known_name) => {
							match (&*old, &*new) {
								// Unknown mutatuion. Well-known names should always have at least a new or old owner.
								(None, None) => {
									#[cfg(feature = "tracing")]
									debug!("Received `NameOwnerChanged` event with no old or new owner for well-known name: {}", well_known_name);
								}
								// Well-known name appeared on the bus.
								(None, Some(new_owner_unique_name)) => {
									let bus_name = BusName::WellKnown(well_known_name.clone());
									let Ok(address) = get_address(bus_name, conn).await else {
										// The application may not support p2p connections.
										#[cfg(feature = "tracing")]
										info!("Failed to get address for well-known name: {}", well_known_name);
										continue;
									};

									let Ok(p2p_connection_builder) = Builder::address(address.clone()) else {
										#[cfg(feature = "tracing")]
										info!("Failed to create p2p connection for well-known name: {}", well_known_name);
										continue;
									};
									let Ok(p2p_connection) = p2p_connection_builder.p2p().build().await else {
										#[cfg(feature = "tracing")]
										warn!("Failed to create p2p connection for well-known name {}: {}", well_known_name, err);
										continue;
									};

									let well_known_name = OwnedWellKnownName::from(well_known_name);
									let peer = Peer {
										unique_name: OwnedUniqueName::from(new_owner_unique_name.to_owned()),
										well_known_name: Some(well_known_name),
										socket_address: address,
										p2p_connection,
									};

									let mut peers_lock = peers.lock().await;
									// Add the new peer to the list.
									peers_lock.push(peer);
								}
								// Well-known name appeared on the bus.
								(Some(old_owner_unique_name), None) => {
									// We need to find the peer by its well-known name and old owner unique name.
									let well_known_name = OwnedWellKnownName::from(well_known_name);
									let old_owner_unique_name = OwnedUniqueName::from(old_owner_unique_name.to_owned());
									let mut peers_lock = peers.lock().await;
									peers_lock.retain(|p| {
										// Retain only the peers that do not match the well-known name and old owner unique name.
										!(p.well_known_name() == Some(&well_known_name)
											&& p.unique_name() == &old_owner_unique_name)
									});
								},
								// Well-known name received a new owner on the bus.
								(Some(old_owner_unique_name), Some(new_owner_unique_name)) => {
									// We need to find the peer by its well-known name and old owner unique name.
									let well_known_name = OwnedWellKnownName::from(well_known_name);
									let old_owner_unique_name = OwnedUniqueName::from(old_owner_unique_name.to_owned());
									let new_owner_unique_name = OwnedUniqueName::from(new_owner_unique_name.to_owned());

									let bus_name = BusName::Unique(new_owner_unique_name.as_ref());
									let Ok(new_address) = get_address(bus_name, conn).await else {
										// The application may not support p2p connections.
										#[cfg(feature = "tracing")]
										info!("Failed to get address for well-known name {}: {}", well_known_name, new_owner_unique_name);
										continue;
									};

									let Ok(p2p_connection_builder) = Builder::address(new_address.clone()) else {
										#[cfg(feature = "tracing")]
										info!("Failed to create p2p connection for well-known name {}: {}", well_known_name, new_owner_unique_name);
										continue;
									};

									let Ok(new_p2p_connection) = p2p_connection_builder.p2p().build().await else {
										#[cfg(feature = "tracing")]
										warn!("Failed to create p2p connection for well-known name {}: {}", well_known_name, new_owner_unique_name);
										continue;
									};

									let mut peers_lock = peers.lock().await;
									if let Some(peer) = peers_lock.iter_mut().find(|p| {
										p.well_known_name() == Some(&well_known_name)
											&& p.unique_name() == &old_owner_unique_name
									}) {
										// Update the peer's unique name and connection.
										peer.unique_name = new_owner_unique_name;
										peer.socket_address = new_address;
										peer.p2p_connection = new_p2p_connection;
									}
								}
							}
						}
					}
				}

				// Handle the case where the stream is closed
				#[cfg(feature = "tracing")]
				warn!("Peer listener task stopped, no more events will be processed.");
				peers.lock().await.clear();
			})
			.detach();
	}
}

async fn get_address(bus_name: BusName<'_>, conn: &zbus::Connection) -> AtspiResult<Address> {
	let application_proxy = application::ApplicationProxy::builder(conn)
		.destination(&bus_name)?
		.cache_properties(CacheProperties::No)
		.build()
		.await?;

	application_proxy
		.get_application_bus_address()
		.await
		.map_err(|e| {
			AtspiError::Owned(format!("Failed to get application bus address for {bus_name}: {e}"))
		})
		.and_then(|address| {
			Address::try_from(address.as_str())
				.map_err(|_| AtspiError::ParseError("Invalid address string"))
		})
}

impl P2P for crate::AccessibilityConnection {
	/// Returns a [`Peer`] by its bus name.
	///
	/// # Note
	/// Bus names are initialized from `ObjectRef` names, which are `OwnedUniqueName`s.
	/// This means that the bus name should be a unique name, not a well-known name.
	///
	/// # Examples
	/// ```rust
	/// # use tokio;
	/// # use atspi::AccessibilityConnection;
	/// # use atspi::BusName;
	/// # use atspi::AtspiResult;
	/// # use atspi-connection::p2p::Peer;
	/// #[tokio::main]
	/// async fn main() -> AtspiResult<()> {
	/// let conn = AccessibilityConnection::new()?;
	/// let bus_name = BusName::from(":1.4242");
	/// let peer: Option<Peer> = conn.get_peer(&bus_name).await;
	/// # assert!(peer.is_none(), "Peer should not be found");
	/// # Ok::<(), AtspiResult>(())
	/// }
	/// ```
	async fn get_peer(&self, bus_name: &BusName<'_>) -> Option<Peer> {
		let peers = self.peers.lock().await;

		let matched = match bus_name {
			BusName::Unique(unique_name) => {
				peers.iter().find(|peer| peer.unique_name() == unique_name)
			}
			BusName::WellKnown(well_known_name) => {
				let owned_well_known_name = OwnedWellKnownName::from(well_known_name.clone());
				peers
					.iter()
					.find(|peer| peer.well_known_name() == Some(&owned_well_known_name))
			}
		};
		matched.cloned()
	}

	/// Returns an `AccessibleProxy` with a P2P connection for the given object if available,
	/// otherwise returns an `AccessibleProxy` with a bus connection.
	///
	/// # Examples
	/// ```rust
	/// # use tokio;
	/// # use atspi::{AtspiResult, AccessibleProxy, ObjectRef};
	/// # use atspi-connection::p2p::Peer;
	/// let conn = atspi::AccessibilityConnection::new().await?;
	/// let obj_ref = ObjectRef::default(); // Replace with a valid ObjectRef
	/// let accessible_proxy = conn.object_as_accessible(&obj_ref).await?;
	/// // Use the `accessible_proxy` as needed
	/// # Ok::<(), AtspiResult>(())
	/// ```
	///
	/// # Errors
	/// If the `AccessibleProxy` cannot be created, or if the object path is invalid.
	///
	/// # Note
	/// This function will first try to find a [`Peer`] with a P2P connection
	async fn object_as_accessible(&self, obj: &ObjectRef) -> AtspiResult<AccessibleProxy<'_>> {
		let lookup = self
			.peers
			.lock()
			.await
			.iter()
			.find(|peer| obj.name == *peer.unique_name())
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
			// If _no_ peer was found, fall back to the bus connection
			let conn = self.connection();
			AccessibleProxy::builder(conn)
				.path(obj.path.clone())?
				.cache_properties(CacheProperties::No)
				.build()
				.await
				.map_err(Into::into)
		}
	}

	/// Returns an [`AccessibleProxy`] to the root accessible object with a P2P connection for the given bus name _if available_.\
	/// If the P2P connection is not available, it returns an `AccessibleProxy` with a bus connection.
	///
	///
	/// # Note
	/// The initial peer list is populated from the `AccessibleProxy` at the root object path.
	///
	/// # Examples
	///
	/// ```no_run
	/// use atspi::AccessibilityConnection;
	/// use atspi::{AtspiResult, P2P, AccessibleProxy};
	///
	/// let conn = atspi::AccessibilityConnection::new().await?;
	/// let bus_name = atspi::BusName::from("org.a11y.atspi.Registry");
	/// let accessible_proxy = conn.bus_name_as_root_accessible(&bus_name).await?;
	/// // Use the `accessible_proxy` as needed
	///
	/// # assert
	/// # Ok::<(), AtspiResult>(())
	/// ```
	///
	/// # Errors
	/// In case of an invalid connection or object path.
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
			.find(|peer| {
				// Check if the peer's unique name matches the bus name
				match name {
					BusName::Unique(unique_name) => peer.unique_name() == unique_name,
					BusName::WellKnown(well_known_name) => {
						peer.well_known_name().is_some_and(|w| w == well_known_name)
					}
				}
			})
			.cloned();

		if let Some(peer) = lookup {
			// If a peer is found, create an AccessibleProxy with a P2P connection
			AccessibleProxy::builder(peer.connection())
				.path(ACCESSIBLE_ROOT_OBJECT_PATH)?
				.cache_properties(CacheProperties::No)
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

	/// Get a snapshot of currently connected peers.
	///
	/// # Examples
	/// ```no_run
	/// # use atspi::AccessibilityConnection;
	/// # use atspi::AtspiResult;
	/// let conn = AccessibilityConnection::new().await?;
	/// let peers = conn.peers().await;
	/// for peer in peers {
	///     println!("Peer: {} at {}", peer.bus_name(), peer.socket_address());
	/// }
	/// # Ok::<(), AtspiResult>(())
	/// ```
	async fn peers(&self) -> Vec<Peer> {
		let peers = self.peers.lock().await;
		peers.clone()
	}
}
