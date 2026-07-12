//! Lazy peer-to-peer discovery for [`crate::AccessibilityConnection`].

use atspi_common::{object_ref::ObjectRefOwned, AtspiError, ACCESSIBLE_ROOT_PATH};
use atspi_proxies::{accessible::AccessibleProxy, application::ApplicationProxy};
use event_listener::Event;
use futures_lite::stream::StreamExt;
#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::{
	collections::HashMap,
	future::Future,
	pin::Pin,
	sync::{Arc, Mutex},
	time::{Duration, Instant},
};
use zbus::{
	conn::Builder,
	fdo::DBusProxy,
	names::{BusName, OwnedUniqueName, OwnedWellKnownName, UniqueName, WellKnownName},
	proxy::CacheProperties,
	zvariant::ObjectPath,
	Address,
};

use crate::AtspiResult;

const NEGATIVE_CAPACITY: usize = 1_024;

type DiscoveryFuture = Pin<Box<dyn Future<Output = DiscoveryResult> + Send + 'static>>;
type DiscoveryFn = dyn Fn(zbus::Connection, OwnedUniqueName) -> DiscoveryFuture + Send + Sync;
type TaskFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
type SpawnFn = dyn Fn(&zbus::Connection, TaskFuture) + Send + Sync;
type Clock = dyn Fn() -> Instant + Send + Sync;
#[cfg(test)]
type AfterPublishFn = dyn Fn(&Arc<Mutex<DiscoveryState>>, &OwnedUniqueName) + Send + Sync;

/// A direct connection to one accessible application.
#[derive(Clone, Debug)]
pub struct Peer {
	unique_name: OwnedUniqueName,
	well_known_name: Option<OwnedWellKnownName>,
	socket_address: Address,
	p2p_connection: zbus::Connection,
}

impl Peer {
	fn for_alias(&self, alias: Option<OwnedWellKnownName>) -> Self {
		let mut peer = self.clone();
		peer.well_known_name = alias;
		peer
	}

	/// Returns the peer's canonical unique bus name.
	#[must_use]
	pub fn unique_name(&self) -> &OwnedUniqueName {
		&self.unique_name
	}

	/// Returns the well-known name requested by this lookup, if any.
	///
	/// Ready transport is stored canonically by unique owner. Consequently,
	/// unique-name lookups and [`P2P::peers`] snapshots return `None`, while a
	/// well-known lookup returns that lookup's requested alias.
	#[must_use]
	pub fn well_known_name(&self) -> Option<&OwnedWellKnownName> {
		self.well_known_name.as_ref()
	}

	/// Returns the advertised peer address.
	#[must_use]
	pub fn socket_address(&self) -> &Address {
		&self.socket_address
	}

	/// Returns the direct connection.
	pub fn connection(&self) -> &zbus::Connection {
		&self.p2p_connection
	}

	/// Creates a peer for `bus_name` immediately.
	///
	/// Prefer [`P2P::get_peer`], which coalesces concurrent attempts and caches
	/// normal P2P unavailability.
	///
	/// # Errors
	/// Returns an error if name resolution, address retrieval, address parsing,
	/// or connection creation fails.
	pub async fn try_from_bus_name(
		bus_name: BusName<'_>,
		conn: &zbus::Connection,
	) -> AtspiResult<Self> {
		let (owner, alias) = resolve_owner(conn, &bus_name).await?;
		match discover(conn.clone(), owner).await {
			DiscoveryResult::Ready(peer) => Ok(peer.for_alias(alias)),
			DiscoveryResult::Unsupported => Err(AtspiError::Owned(
				"application does not support peer-to-peer connections".into(),
			)),
			DiscoveryResult::Transient(message) => Err(AtspiError::Owned(message)),
		}
	}

	/// Returns proxy helpers for `path` over this peer connection.
	///
	/// # Errors
	/// Returns an error if the accessible proxy cannot be built or queried.
	pub async fn proxies(
		&'_ self,
		path: &ObjectPath<'_>,
	) -> AtspiResult<atspi_proxies::proxy_ext::Proxies<'_>> {
		use atspi_proxies::proxy_ext::ProxyExt;
		let proxy = AccessibleProxy::builder(&self.p2p_connection)
			.path(path.to_owned())?
			.destination(&self.unique_name)?
			.cache_properties(CacheProperties::No)
			.build()
			.await?;
		proxy.proxies().await
	}

	/// Returns the peer's root accessible proxy.
	///
	/// # Errors
	/// Returns an error if the proxy cannot be built.
	pub async fn as_root_accessible_proxy(&self) -> AtspiResult<AccessibleProxy<'_>> {
		AccessibleProxy::builder(&self.p2p_connection)
			.path(ACCESSIBLE_ROOT_PATH)?
			.destination(&self.unique_name)?
			.cache_properties(CacheProperties::No)
			.build()
			.await
			.map_err(Into::into)
	}

	/// Returns an accessible proxy for `obj` over this peer connection.
	///
	/// # Errors
	/// Returns an error if the object has no path or the proxy cannot be built.
	pub async fn as_accessible_proxy(
		&self,
		obj: &ObjectRefOwned,
	) -> AtspiResult<AccessibleProxy<'_>> {
		AccessibleProxy::builder(&self.p2p_connection)
			.path(obj.path())?
			.destination(&self.unique_name)?
			.cache_properties(CacheProperties::No)
			.build()
			.await
			.map_err(Into::into)
	}
}

#[derive(Clone, Debug)]
enum FlightOutcome {
	Ready { generation: u64, peer: Peer },
	Absent,
}

#[derive(Debug)]
struct Flight {
	outcome: Mutex<Option<FlightOutcome>>,
	event: Event,
}

impl Flight {
	fn new() -> Self {
		Self { outcome: Mutex::new(None), event: Event::new() }
	}

	fn complete(&self, outcome: FlightOutcome) {
		*self.outcome.lock().expect("peer flight lock poisoned") = Some(outcome);
		self.event.notify(usize::MAX);
	}

	async fn wait(&self) -> FlightOutcome {
		loop {
			let listener = self.event.listen();
			if let Some(outcome) = self.outcome.lock().expect("peer flight lock poisoned").clone() {
				return outcome;
			}
			listener.await;
		}
	}
}

#[derive(Debug)]
struct ListenerReady {
	result: Mutex<Option<AtspiResult<()>>>,
	event: Event,
}

impl ListenerReady {
	fn new() -> Self {
		Self { result: Mutex::new(None), event: Event::new() }
	}

	fn complete(&self, result: AtspiResult<()>) {
		*self.result.lock().expect("listener readiness lock poisoned") = Some(result);
		self.event.notify(usize::MAX);
	}

	async fn wait(&self) -> AtspiResult<()> {
		loop {
			let listener = self.event.listen();
			if let Some(result) =
				self.result.lock().expect("listener readiness lock poisoned").take()
			{
				return result;
			}
			listener.await;
		}
	}
}

#[derive(Debug)]
enum OwnerState {
	Discovering { generation: u64, flight: Arc<Flight> },
	Ready { generation: u64, peer: Peer },
	Unsupported { generation: u64, last_attempt: Instant },
	Transient { generation: u64, failures: u32, retry_at: Instant, last_attempt: Instant },
}

impl OwnerState {
	fn generation(&self) -> u64 {
		match self {
			Self::Discovering { generation, .. }
			| Self::Ready { generation, .. }
			| Self::Unsupported { generation, .. }
			| Self::Transient { generation, .. } => *generation,
		}
	}

	fn negative_last_attempt(&self) -> Option<Instant> {
		match self {
			Self::Unsupported { last_attempt, .. } | Self::Transient { last_attempt, .. } => {
				Some(*last_attempt)
			}
			_ => None,
		}
	}
}

#[derive(Clone, Debug)]
struct Alias {
	owner: OwnedUniqueName,
	generation: u64,
}

#[derive(Debug)]
struct DiscoveryState {
	owners: HashMap<OwnedUniqueName, OwnerState>,
	aliases: HashMap<OwnedWellKnownName, Alias>,
	next_generation: u64,
}

impl Default for DiscoveryState {
	fn default() -> Self {
		Self { owners: HashMap::new(), aliases: HashMap::new(), next_generation: 1 }
	}
}

impl DiscoveryState {
	fn allocate_generation(&mut self) -> Result<u64, Vec<Arc<Flight>>> {
		let generation = self.next_generation;
		if generation == 0 {
			return Err(self.clear());
		}
		match generation.checked_add(1) {
			Some(next) => self.next_generation = next,
			None => self.next_generation = 0,
		}
		Ok(generation)
	}

	fn clear(&mut self) -> Vec<Arc<Flight>> {
		let flights = self
			.owners
			.values()
			.filter_map(|state| match state {
				OwnerState::Discovering { flight, .. } => Some(Arc::clone(flight)),
				_ => None,
			})
			.collect();
		self.owners.clear();
		self.aliases.clear();
		flights
	}

	fn invalidate_owner(&mut self, owner: &UniqueName<'_>) -> Vec<Arc<Flight>> {
		self.aliases.retain(|_, alias| alias.owner.as_str() != owner.as_str());
		self.owners
			.remove(owner.as_str())
			.and_then(|state| match state {
				OwnerState::Discovering { flight, .. } => Some(vec![flight]),
				_ => None,
			})
			.unwrap_or_default()
	}

	fn update_alias(
		&mut self,
		name: &WellKnownName<'_>,
		old_owner: Option<&UniqueName<'_>>,
		new_owner: Option<&UniqueName<'_>>,
	) -> Vec<Arc<Flight>> {
		let mut flights = Vec::new();
		let owned_name = OwnedWellKnownName::from(name.clone());
		let current_owner = self.aliases.get(&owned_name).map(|alias| alias.owner.clone());
		if let Some(old_owner) = old_owner {
			if current_owner
				.as_ref()
				.is_some_and(|current| current.as_str() != old_owner.as_str())
			{
				// A duplicate transfer already applied, or an out-of-order stale signal.
				return flights;
			}
			flights.extend(self.invalidate_owner(old_owner));
		} else if let Some(current_owner) = current_owner {
			if new_owner.is_some_and(|new_owner| current_owner.as_str() == new_owner.as_str()) {
				return flights;
			}
			flights.extend(self.invalidate_owner(&current_owner.as_ref()));
		}
		self.aliases.remove(&owned_name);
		if let Some(new_owner) = new_owner {
			let generation = match self.allocate_generation() {
				Ok(generation) => generation,
				Err(cleared) => {
					flights.extend(cleared);
					return flights;
				}
			};
			self.aliases.insert(
				owned_name,
				Alias { owner: OwnedUniqueName::from(new_owner.clone()), generation },
			);
		}
		flights
	}

	fn enforce_negative_capacity(&mut self, now: Instant) {
		self.owners.retain(
			|_, state| !matches!(state, OwnerState::Transient { retry_at, .. } if *retry_at <= now),
		);
		while self
			.owners
			.values()
			.filter(|state| state.negative_last_attempt().is_some())
			.count() >= NEGATIVE_CAPACITY
		{
			let candidate = self
				.owners
				.iter()
				.filter_map(|(owner, state)| {
					state.negative_last_attempt().map(|attempt| (owner.clone(), attempt))
				})
				.min_by(|(left_owner, left_time), (right_owner, right_time)| {
					left_time.cmp(right_time).then_with(|| left_owner.cmp(right_owner))
				})
				.map(|(owner, _)| owner);
			let Some(candidate) = candidate else { break };
			self.owners.remove(&candidate);
		}
	}
}

enum DiscoveryResult {
	Ready(Peer),
	Unsupported,
	Transient(String),
}

enum LookupAction {
	Ready(Peer),
	Absent,
	Wait(Arc<Flight>),
	Spawn { generation: u64, flight: Arc<Flight>, previous_failures: u32 },
}

#[derive(Clone)]
pub(crate) struct Peers {
	state: Arc<Mutex<DiscoveryState>>,
	clock: Arc<Clock>,
	discover: Arc<DiscoveryFn>,
	spawn: Arc<SpawnFn>,
	#[cfg(test)]
	listener_started: Arc<AtomicBool>,
	#[cfg(test)]
	after_publish: Arc<AfterPublishFn>,
}

impl std::fmt::Debug for Peers {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter.debug_struct("Peers").finish_non_exhaustive()
	}
}

impl Default for Peers {
	fn default() -> Self {
		Self {
			state: Arc::new(Mutex::new(DiscoveryState::default())),
			clock: Arc::new(Instant::now),
			discover: Arc::new(|connection, owner| Box::pin(discover(connection, owner))),
			spawn: Arc::new(|connection, task| {
				connection.executor().spawn(task, "P2PDiscoveryTask").detach();
			}),
			#[cfg(test)]
			listener_started: Arc::new(AtomicBool::new(false)),
			#[cfg(test)]
			after_publish: Arc::new(|_, _| {}),
		}
	}
}

struct DiscoveryCleanup {
	state: Arc<Mutex<DiscoveryState>>,
	owner: OwnedUniqueName,
	generation: u64,
	flight: Arc<Flight>,
	armed: bool,
}

impl DiscoveryCleanup {
	fn disarm(&mut self) {
		self.armed = false;
	}
}

impl Drop for DiscoveryCleanup {
	fn drop(&mut self) {
		if !self.armed {
			return;
		}
		let removed = {
			let mut state = self.state.lock().expect("peer state lock poisoned");
			let matches = matches!(
				state.owners.get(&self.owner),
				Some(OwnerState::Discovering { generation, flight })
					if *generation == self.generation && Arc::ptr_eq(flight, &self.flight)
			);
			if matches {
				state.owners.remove(&self.owner);
			}
			matches
		};
		if removed {
			self.flight.complete(FlightOutcome::Absent);
		}
	}
}

impl Peers {
	fn snapshot(&self) -> Vec<Peer> {
		let mut peers: Vec<_> = self
			.state
			.lock()
			.expect("peer state lock poisoned")
			.owners
			.values()
			.filter_map(|state| match state {
				OwnerState::Ready { peer, .. } => Some(peer.clone()),
				_ => None,
			})
			.collect();
		peers.sort_by(|left, right| left.unique_name.cmp(&right.unique_name));
		peers
	}

	async fn resolve_owner(
		&self,
		connection: &zbus::Connection,
		name: &BusName<'_>,
	) -> AtspiResult<(OwnedUniqueName, Option<OwnedWellKnownName>)> {
		match name {
			BusName::Unique(owner) => Ok((OwnedUniqueName::from(owner.clone()), None)),
			BusName::WellKnown(name) => {
				let owned_name = OwnedWellKnownName::from(name.clone());
				if let Some(alias) = self
					.state
					.lock()
					.expect("peer state lock poisoned")
					.aliases
					.get(&owned_name)
					.cloned()
				{
					debug_assert_ne!(alias.generation, 0);
					return Ok((alias.owner, Some(owned_name)));
				}
				let proxy = DBusProxy::new(connection).await?;
				let owner = proxy.get_name_owner(BusName::WellKnown(name.clone())).await?;
				// Only NameOwnerChanged mutates aliases. Caching this method reply could
				// overwrite a transfer signal processed while the call was in flight.
				Ok(Self::resolved_alias(owned_name, owner))
			}
		}
	}

	fn resolved_alias(
		name: OwnedWellKnownName,
		owner: OwnedUniqueName,
	) -> (OwnedUniqueName, Option<OwnedWellKnownName>) {
		// Method replies are lookup-local. Only NameOwnerChanged may mutate aliases.
		(owner, Some(name))
	}

	async fn get(
		&self,
		connection: &zbus::Connection,
		name: &BusName<'_>,
	) -> AtspiResult<(OwnedUniqueName, Option<Peer>)> {
		let (owner, alias) = self.resolve_owner(connection, name).await?;
		let now = (self.clock)();
		let action = {
			let mut state = self.state.lock().expect("peer state lock poisoned");
			match state.owners.get(&owner) {
				Some(OwnerState::Ready { peer, .. }) => {
					LookupAction::Ready(peer.for_alias(alias.clone()))
				}
				Some(OwnerState::Unsupported { .. }) => LookupAction::Absent,
				Some(OwnerState::Transient { retry_at, .. }) if *retry_at > now => {
					LookupAction::Absent
				}
				Some(OwnerState::Discovering { flight, .. }) => {
					LookupAction::Wait(Arc::clone(flight))
				}
				current => {
					let previous_failures = match current {
						Some(OwnerState::Transient { failures, .. }) => *failures,
						_ => 0,
					};
					let generation = state.allocate_generation().map_err(|flights| {
						complete_absent(flights);
						AtspiError::Owned("peer generation token exhausted".into())
					})?;
					let flight = Arc::new(Flight::new());
					state.owners.insert(
						owner.clone(),
						OwnerState::Discovering { generation, flight: Arc::clone(&flight) },
					);
					LookupAction::Spawn { generation, flight, previous_failures }
				}
			}
		};

		let flight = match action {
			LookupAction::Ready(peer) => return Ok((owner, Some(peer))),
			LookupAction::Absent => return Ok((owner, None)),
			LookupAction::Wait(flight) => flight,
			LookupAction::Spawn { generation, flight, previous_failures } => {
				self.spawn_discovery(
					connection,
					owner.clone(),
					generation,
					previous_failures,
					Arc::clone(&flight),
				);
				flight
			}
		};

		match flight.wait().await {
			FlightOutcome::Ready { generation, peer } => {
				let valid = matches!(
					self.state.lock().expect("peer state lock poisoned").owners.get(&owner),
					Some(OwnerState::Ready { generation: current, .. }) if *current == generation
				);
				Ok((owner, valid.then(|| peer.for_alias(alias))))
			}
			FlightOutcome::Absent => Ok((owner, None)),
		}
	}

	fn spawn_discovery(
		&self,
		connection: &zbus::Connection,
		owner: OwnedUniqueName,
		generation: u64,
		previous_failures: u32,
		flight: Arc<Flight>,
	) {
		let task_connection = connection.clone();
		let state = Arc::clone(&self.state);
		let clock = Arc::clone(&self.clock);
		let discover = Arc::clone(&self.discover);
		let spawn = Arc::clone(&self.spawn);
		#[cfg(test)]
		let after_publish = Arc::clone(&self.after_publish);
		spawn(
			connection,
			Box::pin(async move {
				let mut cleanup = DiscoveryCleanup {
					state: Arc::clone(&state),
					owner: owner.clone(),
					generation,
					flight: Arc::clone(&flight),
					armed: true,
				};
				let result = discover(task_connection, owner.clone()).await;
				let now = clock();
				let outcome = {
					let mut state = state.lock().expect("peer state lock poisoned");
					let valid = state
						.owners
						.get(&owner)
						.is_some_and(|entry| entry.generation() == generation);
					if valid {
						match result {
							DiscoveryResult::Ready(peer) => {
								state.owners.insert(
									owner.clone(),
									OwnerState::Ready { generation, peer: peer.clone() },
								);
								FlightOutcome::Ready { generation, peer }
							}
							DiscoveryResult::Unsupported => {
								state.enforce_negative_capacity(now);
								state.owners.insert(
									owner.clone(),
									OwnerState::Unsupported { generation, last_attempt: now },
								);
								FlightOutcome::Absent
							}
							DiscoveryResult::Transient(message) => {
								#[cfg(feature = "tracing")]
								tracing::debug!(%owner, %message, "P2P discovery temporarily unavailable");
								#[cfg(not(feature = "tracing"))]
								let _ = message;
								let failures = previous_failures.saturating_add(1);
								let retry_at = now + retry_delay(failures);
								state.enforce_negative_capacity(now);
								state.owners.insert(
									owner.clone(),
									OwnerState::Transient {
										generation,
										failures,
										retry_at,
										last_attempt: now,
									},
								);
								FlightOutcome::Absent
							}
						}
					} else {
						FlightOutcome::Absent
					}
				};
				#[cfg(test)]
				after_publish(&state, &owner);
				flight.complete(outcome);
				cleanup.disarm();
			}),
		);
	}

	pub(crate) async fn spawn_listener(&self, connection: &zbus::Connection) -> AtspiResult<()> {
		let peers = self.clone();
		let connection = connection.clone();
		let executor = connection.executor().clone();
		let ready = Arc::new(ListenerReady::new());
		let task_ready = Arc::clone(&ready);
		executor
			.spawn(
				async move {
					let proxy = match DBusProxy::new(&connection).await {
						Ok(proxy) => proxy,
						Err(error) => {
							peers.clear();
							task_ready.complete(Err(error.into()));
							return;
						}
					};
					let mut stream = match proxy.receive_name_owner_changed().await {
						Ok(stream) => stream,
						Err(error) => {
							peers.clear();
							task_ready.complete(Err(error.into()));
							return;
						}
					};
					#[cfg(test)]
					peers.listener_started.store(true, AtomicOrdering::SeqCst);
					task_ready.complete(Ok(()));
					while let Some(signal) = stream.next().await {
						let Ok(args) = signal.args() else { continue };
						match args.name() {
							BusName::Unique(name) => {
								if args.new_owner().is_none() {
									peers.invalidate_owner(name);
								}
							}
							BusName::WellKnown(name) => peers.update_alias(
								name,
								args.old_owner().as_ref(),
								args.new_owner().as_ref(),
							),
						}
					}
					peers.clear();
				},
				"P2PListenerTask",
			)
			.detach();
		ready.wait().await
	}

	#[cfg(test)]
	pub(crate) fn listener_started(&self) -> bool {
		self.listener_started.load(AtomicOrdering::SeqCst)
	}

	fn invalidate_owner(&self, owner: &UniqueName<'_>) {
		let flights = self
			.state
			.lock()
			.expect("peer state lock poisoned")
			.invalidate_owner(owner);
		complete_absent(flights);
	}

	fn update_alias(
		&self,
		name: &WellKnownName<'_>,
		old_owner: Option<&UniqueName<'_>>,
		new_owner: Option<&UniqueName<'_>>,
	) {
		let flights = self
			.state
			.lock()
			.expect("peer state lock poisoned")
			.update_alias(name, old_owner, new_owner);
		complete_absent(flights);
	}

	fn clear(&self) {
		let flights = self.state.lock().expect("peer state lock poisoned").clear();
		complete_absent(flights);
	}
}

fn complete_absent(flights: Vec<Arc<Flight>>) {
	for flight in flights {
		flight.complete(FlightOutcome::Absent);
	}
}

fn retry_delay(failures: u32) -> Duration {
	let seconds = match failures {
		1..=5 => 1_u64 << (failures - 1),
		_ => 30,
	};
	Duration::from_secs(seconds)
}

async fn resolve_owner(
	connection: &zbus::Connection,
	name: &BusName<'_>,
) -> AtspiResult<(OwnedUniqueName, Option<OwnedWellKnownName>)> {
	match name {
		BusName::Unique(owner) => Ok((OwnedUniqueName::from(owner.clone()), None)),
		BusName::WellKnown(name) => {
			let owner = DBusProxy::new(connection)
				.await?
				.get_name_owner(BusName::WellKnown(name.clone()))
				.await?;
			Ok((owner, Some(OwnedWellKnownName::from(name.clone()))))
		}
	}
}

async fn discover(connection: zbus::Connection, owner: OwnedUniqueName) -> DiscoveryResult {
	let builder = match ApplicationProxy::builder(&connection).destination(&owner) {
		Ok(builder) => builder,
		Err(error) => return DiscoveryResult::Transient(error.to_string()),
	};
	let proxy = match builder.cache_properties(CacheProperties::No).build().await {
		Ok(proxy) => proxy,
		Err(error) => return DiscoveryResult::Transient(error.to_string()),
	};
	let advertised = match proxy.get_application_bus_address().await {
		Ok(address) if address.is_empty() => return DiscoveryResult::Unsupported,
		Ok(address) => address,
		Err(error) if is_unknown_method(&error) => return DiscoveryResult::Unsupported,
		Err(error) => return DiscoveryResult::Transient(error.to_string()),
	};
	let address = match Address::try_from(advertised.as_str()) {
		Ok(address) => address,
		Err(error) => return DiscoveryResult::Transient(error.to_string()),
	};
	let p2p_connection = match Builder::address(address.clone()) {
		Ok(builder) => match builder.p2p().build().await {
			Ok(connection) => connection,
			Err(error) => return DiscoveryResult::Transient(error.to_string()),
		},
		Err(error) => return DiscoveryResult::Transient(error.to_string()),
	};
	DiscoveryResult::Ready(Peer {
		unique_name: owner,
		well_known_name: None,
		socket_address: address,
		p2p_connection,
	})
}

fn is_unknown_method(error: &zbus::Error) -> bool {
	match error {
		zbus::Error::MethodError(name, _, _) => {
			name.as_str() == "org.freedesktop.DBus.Error.UnknownMethod"
		}
		zbus::Error::FDO(error) => matches!(error.as_ref(), zbus::fdo::Error::UnknownMethod(_)),
		_ => false,
	}
}

fn routing_connection<'a>(
	shared: &'a zbus::Connection,
	peer: Option<&'a Peer>,
) -> &'a zbus::Connection {
	peer.map_or(shared, Peer::connection)
}

/// P2P-aware proxy construction and lazy peer discovery.
pub trait P2P {
	/// Returns an accessible proxy over P2P when available, otherwise over the
	/// connection's shared accessibility bus.
	fn object_as_accessible(
		&'_ self,
		obj: &ObjectRefOwned,
	) -> impl Future<Output = AtspiResult<AccessibleProxy<'_>>>;

	/// Returns a root accessible proxy over P2P when available, otherwise over
	/// the connection's shared accessibility bus.
	fn bus_name_as_root_accessible(
		&'_ self,
		name: &BusName,
	) -> impl Future<Output = AtspiResult<AccessibleProxy<'_>>>;

	/// Returns a detached, unique-name-sorted snapshot of ready peers.
	///
	/// The returned vector can be retained or modified without locking or
	/// changing discovery state.
	///
	/// ```no_run
	/// # use atspi_connection::{AccessibilityConnection, P2P};
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// let connection = AccessibilityConnection::new().await?;
	/// for peer in connection.peers() {
	///     println!("{}", peer.unique_name());
	/// }
	/// # Ok(())
	/// # }
	/// ```
	fn peers(&self) -> Vec<Peer>;

	/// Finds or lazily discovers a peer.
	///
	/// `Ok(None)` means P2P is unsupported, temporarily unavailable, or the
	/// owner changed during discovery. Identity and shared-routing failures are
	/// returned as errors.
	///
	/// # Errors
	/// Returns an error when the target's canonical owner cannot be resolved or
	/// discovery fails in a way that prevents reliable routing.
	///
	/// ```no_run
	/// # use atspi_connection::{AccessibilityConnection, P2P};
	/// # use zbus::names::BusName;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// let connection = AccessibilityConnection::new().await?;
	/// let name = BusName::try_from(":1.42")?;
	/// if let Some(peer) = connection.get_peer(&name).await? {
	///     println!("direct connection to {}", peer.unique_name());
	/// }
	/// # Ok(())
	/// # }
	/// ```
	fn get_peer(
		&'_ self,
		bus_name: &BusName<'_>,
	) -> impl Future<Output = AtspiResult<Option<Peer>>>;
}

impl P2P for crate::AccessibilityConnection {
	async fn object_as_accessible(&self, obj: &ObjectRefOwned) -> AtspiResult<AccessibleProxy<'_>> {
		if obj.is_null() {
			return Err(AtspiError::NullRef(
				"`p2p::object_as_accessible` called with null-reference ObjectRef",
			));
		}
		let owner = OwnedUniqueName::from(obj.name().ok_or(AtspiError::MissingName)?.to_owned());
		let (_, peer) = self
			.peers
			.get(self.connection(), &BusName::Unique(owner.as_ref()))
			.await?;
		let connection = routing_connection(self.connection(), peer.as_ref());
		AccessibleProxy::builder(connection)
			.destination(owner)?
			.path(obj.path())?
			.cache_properties(CacheProperties::No)
			.build()
			.await
			.map_err(Into::into)
	}

	async fn bus_name_as_root_accessible(
		&'_ self,
		name: &BusName<'_>,
	) -> AtspiResult<AccessibleProxy<'_>> {
		let (owner, peer) = self.peers.get(self.connection(), name).await?;
		let connection = routing_connection(self.connection(), peer.as_ref());
		AccessibleProxy::builder(connection)
			.path(ACCESSIBLE_ROOT_PATH)?
			.destination(owner)?
			.cache_properties(CacheProperties::No)
			.build()
			.await
			.map_err(Into::into)
	}

	fn peers(&self) -> Vec<Peer> {
		self.peers.snapshot()
	}

	async fn get_peer(&self, bus_name: &BusName<'_>) -> AtspiResult<Option<Peer>> {
		self.peers
			.get(self.connection(), bus_name)
			.await
			.map(|(_, peer)| peer)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::sync::atomic::{AtomicUsize, Ordering};

	fn owner(name: &str) -> OwnedUniqueName {
		OwnedUniqueName::try_from(name).unwrap()
	}

	async fn test_connections() -> (zbus::Connection, zbus::Connection) {
		let server = zbus::Connection::session().await.unwrap();
		let client = zbus::Connection::session().await.unwrap();
		(server, client)
	}

	fn test_peers(discover: Arc<DiscoveryFn>) -> Peers {
		Peers {
			state: Arc::new(Mutex::new(DiscoveryState::default())),
			clock: Arc::new(Instant::now),
			discover,
			spawn: Arc::new(|_, task| {
				std::thread::spawn(move || futures_lite::future::block_on(task));
			}),
			listener_started: Arc::new(AtomicBool::new(false)),
			after_publish: Arc::new(|_, _| {}),
		}
	}

	fn ready_result(connection: zbus::Connection, owner: OwnedUniqueName) -> DiscoveryResult {
		DiscoveryResult::Ready(Peer {
			unique_name: owner,
			well_known_name: None,
			socket_address: Address::try_from("unix:path=/tmp/atspi-test-peer").unwrap(),
			p2p_connection: connection,
		})
	}

	#[test]
	fn retry_policy_is_bounded_and_saturating() {
		assert_eq!(retry_delay(1), Duration::from_secs(1));
		assert_eq!(retry_delay(2), Duration::from_secs(2));
		assert_eq!(retry_delay(3), Duration::from_secs(4));
		assert_eq!(retry_delay(4), Duration::from_secs(8));
		assert_eq!(retry_delay(5), Duration::from_secs(16));
		assert_eq!(retry_delay(6), Duration::from_secs(30));
		assert_eq!(retry_delay(u32::MAX), Duration::from_secs(30));
		assert_eq!(u32::MAX.saturating_add(1), u32::MAX);
	}

	#[test]
	fn listener_readiness_is_pending_until_subscription_reports_success() {
		futures_lite::future::block_on(async {
			let ready = Arc::new(ListenerReady::new());
			let mut waiting = Box::pin(ready.wait());
			assert!(futures_lite::future::poll_once(&mut waiting).await.is_none());
			ready.complete(Ok(()));
			waiting.await.unwrap();
		});
	}

	#[test]
	fn negative_capacity_prunes_expired_then_oldest_with_name_tie_break() {
		let now = Instant::now();
		let mut state = DiscoveryState::default();
		state.owners.insert(
			owner(":1.2"),
			OwnerState::Transient { generation: 1, failures: 1, retry_at: now, last_attempt: now },
		);
		for index in 0..NEGATIVE_CAPACITY {
			state.owners.insert(
				owner(&format!(":2.{index}")),
				OwnerState::Unsupported { generation: 2, last_attempt: now },
			);
		}
		state.enforce_negative_capacity(now);
		assert!(!state.owners.contains_key(&owner(":1.2")));
		assert_eq!(
			state
				.owners
				.values()
				.filter(|entry| entry.negative_last_attempt().is_some())
				.count(),
			NEGATIVE_CAPACITY - 1
		);
	}

	#[test]
	fn invalidation_is_aba_safe_and_wakes_flight() {
		let mut state = DiscoveryState::default();
		let owner = owner(":1.7");
		let first = state.allocate_generation().unwrap();
		let flight = Arc::new(Flight::new());
		state
			.owners
			.insert(owner.clone(), OwnerState::Discovering { generation: first, flight });
		let flights = state.invalidate_owner(&owner.as_ref());
		assert_eq!(flights.len(), 1);
		let second = state.allocate_generation().unwrap();
		assert_ne!(first, second);
	}

	#[test]
	fn alias_transfer_invalidates_old_owner_without_discovery() {
		let mut state = DiscoveryState::default();
		let alias = WellKnownName::try_from("org.example.App").unwrap();
		let old = owner(":1.1");
		let new = owner(":1.2");
		state.update_alias(&alias, None, Some(&old.as_ref()));
		state.update_alias(&alias, Some(&old.as_ref()), Some(&new.as_ref()));
		assert_eq!(state.aliases.get(&OwnedWellKnownName::from(alias)).unwrap().owner, new);
	}

	#[test]
	fn unsupported_and_transient_states_obey_invalidation_and_retry_boundaries() {
		let now = Instant::now();
		let owner = owner(":1.9");
		let mut state = DiscoveryState::default();
		state
			.owners
			.insert(owner.clone(), OwnerState::Unsupported { generation: 1, last_attempt: now });
		assert!(matches!(state.owners.get(&owner), Some(OwnerState::Unsupported { .. })));
		state.invalidate_owner(&owner.as_ref());
		assert!(!state.owners.contains_key(&owner));

		state.owners.insert(
			owner.clone(),
			OwnerState::Transient {
				generation: 2,
				failures: 1,
				retry_at: now + Duration::from_secs(1),
				last_attempt: now,
			},
		);
		assert!(matches!(
			state.owners.get(&owner),
			Some(OwnerState::Transient { retry_at, .. }) if *retry_at > now
		));
		state.enforce_negative_capacity(now + Duration::from_secs(1));
		assert!(!state.owners.contains_key(&owner));
	}

	#[test]
	fn duplicate_and_out_of_order_alias_signals_are_idempotent() {
		let mut state = DiscoveryState::default();
		let name = WellKnownName::try_from("org.example.App").unwrap();
		let old = owner(":1.1");
		let new = owner(":1.2");
		state.update_alias(&name, None, Some(&old.as_ref()));
		state.update_alias(&name, Some(&old.as_ref()), Some(&new.as_ref()));
		let generation = state
			.aliases
			.get(&OwnedWellKnownName::from(name.clone()))
			.unwrap()
			.generation;

		state.update_alias(&name, Some(&old.as_ref()), Some(&new.as_ref()));
		state.update_alias(&name, Some(&old.as_ref()), None);
		let alias = state.aliases.get(&OwnedWellKnownName::from(name)).unwrap();
		assert_eq!(alias.owner, new);
		assert_eq!(alias.generation, generation);
	}

	#[test]
	fn clearing_state_releases_all_waiters_and_aliases() {
		let mut state = DiscoveryState::default();
		let first = Arc::new(Flight::new());
		let second = Arc::new(Flight::new());
		state.owners.insert(
			owner(":1.1"),
			OwnerState::Discovering { generation: 1, flight: Arc::clone(&first) },
		);
		state.owners.insert(
			owner(":1.2"),
			OwnerState::Discovering { generation: 2, flight: Arc::clone(&second) },
		);
		let alias = WellKnownName::try_from("org.example.App").unwrap();
		state.update_alias(&alias, None, Some(&owner(":1.1").as_ref()));

		let flights = state.clear();
		assert_eq!(flights.len(), 2);
		complete_absent(flights);
		assert!(state.owners.is_empty());
		assert!(state.aliases.is_empty());
		assert!(matches!(futures_lite::future::block_on(first.wait()), FlightOutcome::Absent));
		assert!(matches!(futures_lite::future::block_on(second.wait()), FlightOutcome::Absent));
	}

	#[test]
	fn unique_then_alias_and_two_alias_orders_have_lookup_specific_metadata() {
		futures_lite::future::block_on(async {
			for aliases in [
				["org.example.First", "org.example.Second"],
				["org.example.Second", "org.example.First"],
			] {
				let (_server, client) = test_connections().await;
				let attempts = Arc::new(AtomicUsize::new(0));
				let discover: Arc<DiscoveryFn> = {
					let attempts = Arc::clone(&attempts);
					Arc::new(move |connection, owner| {
						attempts.fetch_add(1, Ordering::SeqCst);
						Box::pin(async move { ready_result(connection, owner) })
					})
				};
				let peers = test_peers(discover);
				let owner = owner(":1.42");
				let unique = peers
					.get(&client, &BusName::Unique(owner.as_ref()))
					.await
					.unwrap()
					.1
					.unwrap();
				assert_eq!(unique.well_known_name(), None);

				for alias_name in aliases {
					let alias = WellKnownName::try_from(alias_name).unwrap();
					peers.update_alias(&alias, None, Some(&owner.as_ref()));
					let peer = peers
						.get(&client, &BusName::WellKnown(alias.clone()))
						.await
						.unwrap()
						.1
						.unwrap();
					assert_eq!(peer.well_known_name().map(|name| name.as_str()), Some(alias_name));
				}
				assert_eq!(attempts.load(Ordering::SeqCst), 1);
				assert_eq!(peers.snapshot()[0].well_known_name(), None);
			}
		});
	}

	#[test]
	fn initiating_caller_cancellation_keeps_one_shared_attempt() {
		futures_lite::future::block_on(async {
			let (_server, client) = test_connections().await;
			let gate = Arc::new(Event::new());
			let attempts = Arc::new(AtomicUsize::new(0));
			let discover: Arc<DiscoveryFn> = {
				let gate = Arc::clone(&gate);
				let attempts = Arc::clone(&attempts);
				Arc::new(move |_, _| {
					let listener = gate.listen();
					attempts.fetch_add(1, Ordering::SeqCst);
					Box::pin(async move {
						listener.await;
						DiscoveryResult::Transient("expected test failure".into())
					})
				})
			};
			let peers = test_peers(discover);
			let owner = owner(":1.44");
			let name = BusName::Unique(owner.as_ref());
			let mut initiating = Box::pin(peers.get(&client, &name));
			assert!(futures_lite::future::poll_once(&mut initiating).await.is_none());
			drop(initiating);
			while attempts.load(Ordering::SeqCst) == 0 {
				std::thread::yield_now();
			}
			gate.notify(usize::MAX);
			let result = peers.get(&client, &BusName::Unique(owner.as_ref())).await.unwrap();
			assert!(result.1.is_none());
			assert_eq!(attempts.load(Ordering::SeqCst), 1);
		});
	}

	#[test]
	fn invalidation_wins_a_completion_race() {
		futures_lite::future::block_on(async {
			let (_server, client) = test_connections().await;
			let gate = Arc::new(Event::new());
			let discover: Arc<DiscoveryFn> = {
				let gate = Arc::clone(&gate);
				Arc::new(move |connection, owner| {
					let listener = gate.listen();
					Box::pin(async move {
						listener.await;
						ready_result(connection, owner)
					})
				})
			};
			let peers = test_peers(discover);
			let owner = owner(":1.45");
			let name = BusName::Unique(owner.as_ref());
			let mut lookup = Box::pin(peers.get(&client, &name));
			assert!(futures_lite::future::poll_once(&mut lookup).await.is_none());
			peers.invalidate_owner(&owner.as_ref());
			gate.notify(usize::MAX);
			assert!(lookup.await.unwrap().1.is_none());
			assert!(peers.snapshot().is_empty());
		});
	}

	#[test]
	fn completion_then_invalidation_removes_ready_transport() {
		futures_lite::future::block_on(async {
			let (_server, client) = test_connections().await;
			let discover: Arc<DiscoveryFn> = Arc::new(move |connection, owner| {
				Box::pin(async move { ready_result(connection, owner) })
			});
			let peers = test_peers(discover);
			let owner = owner(":1.451");
			assert!(peers
				.get(&client, &BusName::Unique(owner.as_ref()))
				.await
				.unwrap()
				.1
				.is_some());
			assert_eq!(peers.snapshot().len(), 1);
			peers.invalidate_owner(&owner.as_ref());
			assert!(peers.snapshot().is_empty());
		});
	}

	#[test]
	fn invalidation_between_ready_publication_and_notification_returns_absent() {
		futures_lite::future::block_on(async {
			let (_server, client) = test_connections().await;
			let discover: Arc<DiscoveryFn> = Arc::new(move |connection, owner| {
				Box::pin(async move { ready_result(connection, owner) })
			});
			let mut peers = test_peers(discover);
			peers.after_publish = Arc::new(|state, owner| {
				let flights = state
					.lock()
					.expect("peer state lock poisoned")
					.invalidate_owner(&owner.as_ref());
				assert!(flights.is_empty(), "ready state must already be published");
			});
			let owner = owner(":1.454");
			let result = peers.get(&client, &BusName::Unique(owner.as_ref())).await.unwrap();
			assert!(result.1.is_none());
			assert!(peers.snapshot().is_empty());
		});
	}

	#[test]
	fn stale_alias_resolution_reply_cannot_overwrite_transfer_signal() {
		let peers = Peers::default();
		let alias = WellKnownName::try_from("org.example.Race").unwrap();
		let old = owner(":1.10");
		let new = owner(":1.11");
		peers.update_alias(&alias, None, Some(&new.as_ref()));
		let resolved = Peers::resolved_alias(OwnedWellKnownName::from(alias.clone()), old.clone());
		assert_eq!(resolved.0, old);
		assert_eq!(
			peers
				.state
				.lock()
				.expect("peer state lock poisoned")
				.aliases
				.get(&OwnedWellKnownName::from(alias))
				.unwrap()
				.owner,
			new
		);
	}

	#[test]
	fn shared_bus_is_selected_when_discovery_returns_no_peer() {
		futures_lite::future::block_on(async {
			let (_server, client) = test_connections().await;
			assert!(std::ptr::eq(routing_connection(&client, None), &client));
			let DiscoveryResult::Ready(peer) = ready_result(client.clone(), owner(":1.452")) else {
				unreachable!()
			};
			assert!(std::ptr::eq(routing_connection(&client, Some(&peer)), peer.connection()));
		});
	}

	#[test]
	fn cleanup_guard_does_not_remove_replacement_generation() {
		let state = Arc::new(Mutex::new(DiscoveryState::default()));
		let owner = owner(":1.453");
		let old_flight = Arc::new(Flight::new());
		let replacement = Arc::new(Flight::new());
		state.lock().expect("peer state lock poisoned").owners.insert(
			owner.clone(),
			OwnerState::Discovering { generation: 2, flight: Arc::clone(&replacement) },
		);
		drop(DiscoveryCleanup {
			state: Arc::clone(&state),
			owner: owner.clone(),
			generation: 1,
			flight: old_flight,
			armed: true,
		});
		assert!(matches!(
			state.lock().expect("peer state lock poisoned").owners.get(&owner),
			Some(OwnerState::Discovering { generation: 2, flight }) if Arc::ptr_eq(flight, &replacement)
		));
	}

	#[test]
	fn aborted_discovery_cleans_matching_generation_and_wakes_waiter() {
		futures_lite::future::block_on(async {
			let (_server, client) = test_connections().await;
			let gate = Arc::new(Event::new());
			let attempts = Arc::new(AtomicUsize::new(0));
			let discover: Arc<DiscoveryFn> = {
				let gate = Arc::clone(&gate);
				let attempts = Arc::clone(&attempts);
				Arc::new(move |_, _| {
					let listener = gate.listen();
					attempts.fetch_add(1, Ordering::SeqCst);
					Box::pin(async move {
						listener.await;
						panic!("abort mocked discovery");
					})
				})
			};
			let peers = test_peers(discover);
			let owner = owner(":1.46");
			let name = BusName::Unique(owner.as_ref());
			let mut first = Box::pin(peers.get(&client, &name));
			assert!(futures_lite::future::poll_once(&mut first).await.is_none());
			while attempts.load(Ordering::SeqCst) == 0 {
				std::thread::yield_now();
			}
			let mut waiter = Box::pin(peers.get(&client, &name));
			assert!(futures_lite::future::poll_once(&mut waiter).await.is_none());
			gate.notify(usize::MAX);
			assert!(waiter.await.unwrap().1.is_none());
			assert!(!peers
				.state
				.lock()
				.expect("peer state lock poisoned")
				.owners
				.contains_key(&owner));
		});
	}

	#[test]
	fn actual_get_applies_backoff_and_capacity_with_injected_time() {
		futures_lite::future::block_on(async {
			let (_server, client) = test_connections().await;
			let now = Arc::new(Mutex::new(Instant::now()));
			let attempts = Arc::new(AtomicUsize::new(0));
			let discover: Arc<DiscoveryFn> = {
				let attempts = Arc::clone(&attempts);
				Arc::new(move |_, _| {
					attempts.fetch_add(1, Ordering::SeqCst);
					Box::pin(async { DiscoveryResult::Transient("temporary".into()) })
				})
			};
			let mut peers = test_peers(discover);
			peers.clock = {
				let now = Arc::clone(&now);
				Arc::new(move || *now.lock().expect("clock lock poisoned"))
			};
			let target = owner(":1.47");
			assert!(peers
				.get(&client, &BusName::Unique(target.as_ref()))
				.await
				.unwrap()
				.1
				.is_none());
			assert!(peers
				.get(&client, &BusName::Unique(target.as_ref()))
				.await
				.unwrap()
				.1
				.is_none());
			assert_eq!(attempts.load(Ordering::SeqCst), 1);
			*now.lock().expect("clock lock poisoned") += Duration::from_secs(1);
			assert!(peers
				.get(&client, &BusName::Unique(target.as_ref()))
				.await
				.unwrap()
				.1
				.is_none());
			assert_eq!(attempts.load(Ordering::SeqCst), 2);

			let current = *now.lock().expect("clock lock poisoned");
			{
				let mut state = peers.state.lock().expect("peer state lock poisoned");
				state.owners.clear();
				for index in 0..NEGATIVE_CAPACITY {
					state.owners.insert(
						owner(&format!(":2.{index}")),
						OwnerState::Unsupported { generation: 1, last_attempt: current },
					);
				}
			}
			assert!(peers
				.get(&client, &BusName::Unique(target.as_ref()))
				.await
				.unwrap()
				.1
				.is_none());
			let state = peers.state.lock().expect("peer state lock poisoned");
			assert_eq!(
				state
					.owners
					.values()
					.filter(|entry| entry.negative_last_attempt().is_some())
					.count(),
				NEGATIVE_CAPACITY
			);
			assert!(matches!(state.owners.get(&target), Some(OwnerState::Transient { .. })));
		});
	}

	#[cfg(unix)]
	#[test]
	fn non_unknown_method_probe_error_is_transient() {
		struct ErrorApplication {
			fail: bool,
		}

		#[zbus::interface(name = "org.a11y.atspi.Application")]
		impl ErrorApplication {
			fn get_application_bus_address(&self) -> zbus::fdo::Result<String> {
				if self.fail {
					Err(zbus::fdo::Error::Failed("probe failed".into()))
				} else {
					Ok(String::new())
				}
			}
		}

		futures_lite::future::block_on(async {
			let server = Builder::session()
				.unwrap()
				.serve_at(ACCESSIBLE_ROOT_PATH, ErrorApplication { fail: true })
				.unwrap()
				.build()
				.await
				.unwrap();
			let client = zbus::Connection::session().await.unwrap();
			let peers = Peers::default();
			let owner = server.unique_name().unwrap().to_owned();
			let result = peers.get(&client, &BusName::Unique(owner.as_ref())).await.unwrap();
			assert!(result.1.is_none());
			assert!(matches!(
				peers
					.state
					.lock()
					.expect("peer state lock poisoned")
					.owners
					.get(&owner),
				Some(OwnerState::Transient { .. })
			));
		});
	}
}
