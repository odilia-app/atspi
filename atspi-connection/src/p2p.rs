//! Extends the `AccessibilityConnection` with P2P capabilities.
//!
//! # Considerations on using executors and P2P
//!
//! Every connection has a zbus `Executor` instance, on which tasks can be launched. Internally, zbus uses this executor for all sorts of tasks: listening for D-Bus signals,
//! keeping property caches up to date, handling timeout errors, etc.
//!
//! When zbus users use `tokio` (zbus feature "tokio" set), the `Executor` instance will be empty and all zbus tasks are run on the user's tokio runtime.
//! However, when you are using any other executor (smol, glommio, async-std, etc.), each `Connection` will spin up a thread with an `async_executor::Executor`.
//!
//! Usually an application will have a single connection, but using P2P you will have a connection with each application that supports it.
//! Consequently, on anything but tokio, applications will get an extra thread with an `async_executor` for each connection!
//! (So picking smol won't necessarily make your application small in the context of P2P.)

use async_lock::Mutex;
use atspi_common::{AtspiError, ObjectRef};
use atspi_proxies::{
	accessible::{AccessibleProxy, ObjectRefExt},
	application::{self, ApplicationProxy},
	proxy_ext::ProxyExt,
	registry::RegistryProxy,
};
use futures_lite::{future::block_on, stream::StreamExt};
use std::sync::Arc;
use zbus::{
	conn::Builder,
	fdo::DBusProxy,
	names::{
		BusName, OwnedBusName, OwnedUniqueName, OwnedWellKnownName, UniqueName, WellKnownName,
	},
	proxy::{CacheProperties, Defaults},
	zvariant::ObjectPath,
	Address,
};

#[cfg(feature = "tracing")]
use tracing::{debug, info, warn};

use crate::AtspiResult;

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
		// we need to query all well-known names and their owners, and then check if the unique name is one of them.

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

	/// Returns the socket [`Address`] of the peer.
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
			.cache_properties(CacheProperties::No)
			.build()
			.await?;

		let socket_path = application_proxy.get_application_bus_address().await?;
		Self::try_new(bus_name, socket_path.as_str(), conn).await
	}

	/// Returns a [`Proxies`][atspi_proxies::proxy_ext::Proxies] object for the given object path.\
	/// A `Proxies` object is used to obtain any of the proxies the object supports.
	///
	/// # Errors
	/// On invalid object path.
	pub async fn proxies(
		&'_ self,
		path: &ObjectPath<'_>,
	) -> AtspiResult<atspi_proxies::proxy_ext::Proxies<'_>> {
		let accessible_proxy = AccessibleProxy::builder(&self.p2p_connection)
			.path(path.to_owned())?
			.cache_properties(CacheProperties::No)
			.build()
			.await?;

		accessible_proxy.proxies().await
	}

	/// Returns an `AccessibleProxy` for the root accessible object of the peer.
	///
	/// # Errors
	/// In case of an invalid connection.
	pub async fn as_root_accessible_proxy(&self) -> AtspiResult<AccessibleProxy<'_>> {
		AccessibleProxy::builder(&self.p2p_connection)
			.cache_properties(CacheProperties::No)
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
			.cache_properties(CacheProperties::No)
			.build()
			.await
			.map_err(AtspiError::from)
	}
}

// A trait is needed to extend functionality on `BusName` for P2P address lookup.
pub(crate) trait BusNameExt {
	/// Looks up a `BusName`'s P2P address, if available.
	async fn get_p2p_address(&self, conn: &zbus::Connection) -> AtspiResult<Address>;
}

impl BusNameExt for BusName<'_> {
	async fn get_p2p_address(&self, conn: &zbus::Connection) -> AtspiResult<Address> {
		let application_proxy = application::ApplicationProxy::builder(conn)
			.destination(self)?
			.cache_properties(CacheProperties::No)
			.build()
			.await?;

		application_proxy
			.get_application_bus_address()
			.await
			.map_err(|e| {
				AtspiError::Owned(format!(
					"Failed to get application bus address for {}: {e}",
					&self
				))
			})
			.and_then(|address| {
				Address::try_from(address.as_str())
					.map_err(|_| AtspiError::ParseError("Invalid address string"))
			})
	}
}

#[derive(Clone, Debug)]
pub(crate) struct Peers {
	peers: Arc<Mutex<Vec<Peer>>>,
}

impl Peers {
	/// Returns a `Peers` containing the initial peers that support P2P connections.
	///
	/// # Note
	/// Intended for internal use with `AccessibilityConnection::new()`.
	///
	/// # Errors
	/// This function can return an error in the following cases:
	/// - the `AccessibleProxy` to the registry cannot be created.
	/// - the registry returns an error when querying for children.
	/// - for any child, the `AccessibleProxy` cannot be created or the `ApplicationProxy` cannot be created.
	pub(crate) async fn initialize_peers(conn: &zbus::Connection) -> AtspiResult<Self> {
		let registry_well_known_name = RegistryProxy::DESTINATION
			.as_ref()
			.expect("RegistryProxy `default_destination` is not set");
		let reg_accessible = AccessibleProxy::builder(conn)
			.destination(registry_well_known_name)?
			.cache_properties(CacheProperties::No)
			.build()
			.await?;

		let accessible_applications = reg_accessible.get_children().await?;
		let mut peers = Vec::with_capacity(accessible_applications.len());

		for app in accessible_applications {
			let accessible_proxy = app.as_accessible_proxy(conn).await?;
			let proxies = accessible_proxy.proxies().await?;
			let application_proxy = proxies.application().await?;

			// Get the application bus address
			// aka: Does the application support P2P connections?
			if let Ok(address) = application_proxy.get_application_bus_address().await {
				let bus_name = BusName::from(app.name);
				let peer = Peer::try_new(bus_name, address.as_str(), conn).await?;
				peers.push(peer);
			}
		}

		Ok(Peers { peers: Arc::new(Mutex::new(peers)) })
	}

	/// Returns a [`Peer`] by its bus name.
	///
	/// # Note
	/// Bus names are initialized from `ObjectRef` names, which are `OwnedUniqueName`s.
	/// This means that the bus name should be a unique name, not a well-known name.
	///
	async fn find_peer(&self, bus_name: &BusName<'_>) -> Option<Peer> {
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

	/// Returns the inner `Arc<Mutex<Vec<Peer>>>`.
	fn inner(&self) -> Arc<Mutex<Vec<Peer>>> {
		Arc::clone(&self.peers)
	}

	/// Inserts a new `Peer` into the list of peers.
	async fn insert_unique(
		&self,
		unique_name: &zbus::names::UniqueName<'_>,
		conn: &zbus::Connection,
	) -> AtspiResult<()> {
		let bus_name = BusName::Unique(unique_name.as_ref());
		let address = bus_name.get_p2p_address(conn).await?;
		let p2p_connection = Builder::address(address.clone())?.p2p().build().await?;

		let unique_name = OwnedUniqueName::from(unique_name.clone());

		let peer =
			Peer { unique_name, well_known_name: None, socket_address: address, p2p_connection };

		self.peers.lock().await.push(peer);
		Ok(())
	}

	/// Removes a `Peer` from the list of peers by its unique name.
	async fn remove_unique(&self, unique_name: &zbus::names::UniqueName<'_>) {
		let mut peers = self.peers.lock().await;
		peers.retain(|peer| peer.unique_name() != unique_name);
	}

	/// Inserts a new `Peer` with a well-known name into the list of peers.
	async fn insert_well_known(
		&self,
		well_known_name: &WellKnownName<'_>,
		name_owner: &UniqueName<'_>,
		conn: &zbus::Connection,
	) -> AtspiResult<()> {
		let bus_name = BusName::WellKnown(well_known_name.clone());
		let address = bus_name.get_p2p_address(conn).await?;
		let p2p_connection = Builder::address(address.clone())?.p2p().build().await?;

		let well_known_name = OwnedWellKnownName::from(well_known_name.clone());
		let unique_name = OwnedUniqueName::from(name_owner.clone());

		let peer = Peer {
			unique_name,
			well_known_name: Some(well_known_name),
			socket_address: address,
			p2p_connection,
		};

		self.peers.lock().await.push(peer);
		Ok(())
	}

	/// Removes a `Peer` with a well-known name from the list of peers.
	async fn remove_well_known(
		&self,
		well_known_name: &WellKnownName<'_>,
		name_owner: &UniqueName<'_>,
	) -> AtspiResult<()> {
		let mut peers = self.peers.lock().await;
		let owned_well_known_name = OwnedWellKnownName::from(well_known_name.clone());
		peers.retain(|peer| {
			(peer.well_known_name() != Some(&owned_well_known_name))
				&& peer.unique_name() == name_owner
		});
		Ok(())
	}

	/// Update a `Peer` with a new owner of it's well-known name in the list of peers.
	async fn update_well_known_owner(
		&self,
		well_known_name: &WellKnownName<'_>,
		old_name_owner: &UniqueName<'_>,
		new_name_owner: &UniqueName<'_>,
		conn: &zbus::Connection,
	) -> AtspiResult<()> {
		let socket_address = BusName::from(new_name_owner.clone()).get_p2p_address(conn).await?;
		let p2p_connection = Builder::address(socket_address.clone())?.p2p().build().await?;

		let well_known_name = Some(OwnedWellKnownName::from(well_known_name.clone()));
		let old_name_owner = OwnedUniqueName::from(old_name_owner.clone());
		let unique_name = OwnedUniqueName::from(new_name_owner.clone());

		let peer = Peer {
			unique_name,
			well_known_name: well_known_name.clone(),
			socket_address,
			p2p_connection,
		};

		let mut peers = self.peers.lock().await;
		if let Some(existing_peer) = peers.iter_mut().find(|p| {
			p.well_known_name() == well_known_name.as_ref() && p.unique_name() == &old_name_owner
		}) {
			*existing_peer = peer;
		} else {
			return Err(AtspiError::Owned(format!(
                "Owner swap failed: well-known name {well_known_name:?} with owner: {old_name_owner} not found"
            )));
		}
		Ok(())
	}

	/// Spawns a task which listens for peer mutations.
	///
	/// This task listens for `NameOwnerChanged` signals and updates the list of peers accordingly.
	///
	/// # executor
	/// The task is spawned on the executor of the `zbus::Connection`.
	///
	/// # Note
	/// This function is called internally by `AccessibilityConnection::new()`.
	pub(crate) fn spawn_peer_listener_task(&self, conn: &zbus::Connection) {
		// Clone the `Peers` and `Connection` to move them into the async task.
		// This is necessary because the async task needs to own these values.
		let peers = self.clone();
		let conn = conn.clone();
		let dbus_proxy = block_on(DBusProxy::new(&conn)).expect("Failed to create DBusProxy");

		let executor = conn.executor().clone();

		executor.spawn(async move {
			let Ok(mut name_owner_changed_stream) = dbus_proxy.receive_name_owner_changed().await.inspect_err(|#[allow(unused_variables)] err| {
				#[cfg(feature = "tracing")]
				debug!("Failed to receive `NameOwnerChanged` stream: {err}");
			}) else {
				return;
			};

			while let Some(name_owner_event) = name_owner_changed_stream.next().await {
					let Ok(args) = name_owner_event.args() else {
						#[cfg(feature = "tracing")]
						tracing::debug!("Received name owner changed event without args, skipping.");
						continue;
					};
					let name = args.name().clone();
					let new = args.new_owner().clone();
					let old = args.old_owner().clone();

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
									debug_assert_eq!(new_owner, &unique_name, "When a name appears on the bus, the new owner must be the unique name itself.");

									if let Ok(()) = peers.insert_unique(&unique_name, &conn).await.inspect_err(|#[allow(unused_variables)] err| {
										#[cfg(feature = "tracing")]
										warn!("Failed to insert unique name: {unique_name}: {err}");
									}) {
										#[cfg(feature = "tracing")]
										info!("Inserted unique name: {unique_name} into the peer list.");
									};

								}
								// Unique name left the bus.
								(Some(old), None) => {
									debug_assert!(old == &unique_name, "When a unique name is removed from the bus, the old owner must be the unique name itself.");
									peers.remove_unique(&unique_name).await;

									#[cfg(feature = "tracing")]
									info!("Peer with unique name: {unique_name} left the bus - removed from peer list.");
								}

								// Unknown combination.
								(_, _) => {
									#[cfg(feature = "tracing")]
									debug!("NameOwnerChanged` with unique name: {unique_name} has unknown argument combination ({old:?}, {new:?}).");
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
									if let Ok(()) = peers.insert_well_known(
										&well_known_name,
										new_owner_unique_name,
										&conn,
									).await.inspect_err(|#[allow(unused_variables)] err| {
										#[cfg(feature = "tracing")]
										warn!("Failed to insert well-known name: {} with owner: {} - {}", &well_known_name, &new_owner_unique_name, err);
									}) {
										#[cfg(feature = "tracing")]
										info!("Well-known name: {} with owner: {} inserted into the peer list.", &well_known_name, &new_owner_unique_name);
									}
								}

								// Well-known name left the bus.
								(Some(old_owner_unique_name), None) => {
									if let Ok(()) = peers.remove_well_known(
										&well_known_name,
										old_owner_unique_name,
									).await.inspect_err(|#[allow(unused_variables)] err| {
										#[cfg(feature = "tracing")]
										warn!("Failed to remove well-known name: {} with owner: {} - {err}", &well_known_name, &old_owner_unique_name);
									}) {
										#[cfg(feature = "tracing")]
										info!(
											"Well-known name: {} with owner: {} removed from the peer list.",
											&well_known_name,
											&old_owner_unique_name
										);
									}
								},

								// Well-known name received a new owner on the bus.
								(Some(old_owner_unique_name), Some(new_owner_unique_name)) => {
									if let Ok(()) = peers.update_well_known_owner(&well_known_name, old_owner_unique_name, new_owner_unique_name, &conn).await.inspect_err(|#[allow(unused_variables)] err| {
										#[cfg(feature = "tracing")]
										warn!("Failed to update well-known name: {} owner from: {} to: {} - {}", &well_known_name, &old_owner_unique_name, &new_owner_unique_name, err);
									}) {
										#[cfg(feature = "tracing")]
										info!("Well-known name: {} updated owner from: {} to: {}", &well_known_name, &old_owner_unique_name, &new_owner_unique_name);
									};
								}
							}
						}
					} // End of match on `name`
				} // End of while loop

				#[cfg(feature = "tracing")]
				tracing::warn!("Peer listener task stopped, clearing peers list.");
				peers.clear().await;
			}, "PeerListenerTask")
			.detach();
	}

	/// Clears the list of peers.
	///
	/// # Note
	/// This is used to reset the list of peers when the D-Bus connection is lost.
	async fn clear(&self) {
		let mut peers = self.peers.lock().await;
		peers.clear();
	}
}

/// Trait for P2P connection handling.
pub trait P2P {
	/// Returns a P2P connected `AccessibleProxy`for object, _if available_.\
	/// If the application does not support P2P, this returns an `AccessibleProxy` for the object with a bus connection.
	fn object_as_accessible(
		&'_ self,
		obj: &ObjectRef,
	) -> impl std::future::Future<Output = AtspiResult<AccessibleProxy<'_>>>;

	/// Returns a P2P connected `AccessibleProxy` to the root  accessible object for the given bus name, _if available_.\
	/// If the P2P connection is not available, it returns an `AccessibleProxy` with a bus connection.
	fn bus_name_as_root_accessible(
		&'_ self,
		name: &BusName,
	) -> impl std::future::Future<Output = AtspiResult<AccessibleProxy<'_>>>;

	/// Return a list of peers that are currently connected.
	fn peers(&self) -> impl std::future::Future<Output = Arc<Mutex<Vec<Peer>>>>;

	/// Returns a [`Peer`] by its bus name.
	fn find_peer(&self, bus_name: &BusName<'_>) -> impl std::future::Future<Output = Option<Peer>>;
}

impl P2P for crate::AccessibilityConnection {
	/// Returns a P2P connected `AccessibleProxy` for the object, _if available_.\
	/// If the application does not support P2P, an `AccessibleProxy` with a bus connection is returned.
	///
	/// # Examples
	/// ```rust
	/// # use futures_lite::future::block_on;
	/// use atspi_proxies::accessible::AccessibleProxy;
	/// use atspi_common::ObjectRef;
	/// use atspi_connection::{P2P, Peer};
	/// use atspi_connection::AccessibilityConnection;
	///
	/// block_on(async {
	///     let conn = AccessibilityConnection::new().await.unwrap();
	///
	///     let obj_ref = ObjectRef::default();
	///     let accessible_proxy = conn.object_as_accessible(&obj_ref).await;
	///     assert!(
	///         accessible_proxy.is_ok(),
	///         "Failed to get accessible proxy: {:?}",
	///         accessible_proxy.err()
	///     );
	/// });
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

	/// Returns a P2P connected [`AccessibleProxy`] to the root accessible object for the given bus name _if available_.\
	/// If the P2P connection is not available, it returns an `AccessibleProxy` with a bus connection.
	///
	/// # Examples
	///
	/// ```rust
	/// # use futures_lite::future::block_on;
	/// use zbus::names::BusName;
	/// use atspi_proxies::accessible::AccessibleProxy;
	/// use atspi_common::ObjectRef;
	/// use atspi_connection::{AccessibilityConnection, P2P};
	///
	/// # block_on(async {
	///   let conn = AccessibilityConnection::new().await.unwrap();
	///   let bus_name = BusName::from_static_str("org.a11y.atspi.Registry").unwrap();
	///   let _accessible_proxy = conn.bus_name_as_root_accessible(&bus_name).await.unwrap();
	///   // Use the accessible proxy as needed
	/// # });
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
				.cache_properties(CacheProperties::No)
				.build()
				.await
				.map_err(Into::into)
		} else {
			// If no peer is found, fall back to the bus connection
			let conn = self.connection();
			AccessibleProxy::builder(conn)
				.cache_properties(CacheProperties::No)
				.build()
				.await
				.map_err(Into::into)
		}
	}

	/// Get a snapshot of currently connected P2P capable peers.
	///
	/// # Examples
	/// ```rust
	/// # use futures_lite::future::block_on;
	/// use atspi_connection::AccessibilityConnection;
	/// use atspi_connection::{P2P, Peer};
	///
	/// # block_on(async {
	///   let conn = AccessibilityConnection::new().await.unwrap();
	///   let peers_locked = conn.peers().await;
	///   let peers = peers_locked.lock().await;
	///   for peer in &*peers {
	///       println!("Peer: {} at {}", peer.unique_name(), peer.socket_address());
	///   }
	/// # });
	/// ```
	async fn peers(&self) -> Arc<Mutex<Vec<Peer>>> {
		self.peers.inner()
	}

	/// Returns a [`Peer`] by its bus name.
	///
	/// # Examples
	/// ```rust
	/// # use futures_lite::future::block_on;
	/// use atspi_connection::AccessibilityConnection;
	/// use zbus::names::BusName;
	/// use atspi_connection::{Peer, P2P};
	///
	/// # block_on(async {
	///   let a11y = AccessibilityConnection::new().await.unwrap();
	///   let bus_name = BusName::from_static_str(":1.42").unwrap();
	///   # let bus_peers = a11y.peers().await;
	///   # let bus_peers = &*bus_peers.lock().await;
	///   # let bus_name = bus_peers
	///   #     .last()
	///   #     .map(|p| p.unique_name().to_owned())
	///   #     .unwrap();
	///   # let bus_name = bus_name.as_ref();
	///   # let bus_name = BusName::from(bus_name);
	///   let peer = a11y.find_peer(&bus_name).await;
	///   assert!(peer.is_some(), "Failed to find peer with bus name: {}", bus_name);
	/// # });
	/// ```
	async fn find_peer(&self, bus_name: &BusName<'_>) -> Option<Peer> {
		self.peers.find_peer(bus_name).await
	}
}
