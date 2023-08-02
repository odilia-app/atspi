#[cfg(all(not(feature = "async-std"), not(feature = "tokio")))]
compile_error!("You must specify at least one of the `async-std` or `tokio` features.");

pub use atspi_common as common;

use atspi_proxies::{
	bus::{BusProxy, StatusProxy},
	registry::RegistryProxy,
};
use common::error::AtspiError;
use common::events::{Event, GenericEvent, HasMatchRule, HasRegistryEventString};
use futures_lite::stream::{Stream, StreamExt};
use std::ops::Deref;
use zbus::{fdo::DBusProxy, Address, MatchRule, MessageStream, MessageType};

pub type AtspiResult<T> = std::result::Result<T, AtspiError>;

/// A connection to the at-spi bus
pub struct AccessibilityConnection {
	registry: RegistryProxy<'static>,
	dbus_proxy: DBusProxy<'static>,
}

impl AccessibilityConnection {
	/// Open a new connection to the bus
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub async fn open() -> zbus::Result<Self> {
		// Grab the a11y bus address from the session bus
		let a11y_bus_addr = {
			#[cfg(feature = "tracing")]
			tracing::debug!("Connecting to session bus");
			let session_bus = Box::pin(zbus::Connection::session()).await?;
			#[cfg(feature = "tracing")]
			tracing::debug!(
				name = session_bus.unique_name().map(|n| n.as_str()),
				"Connected to session bus"
			);
			let proxy = BusProxy::new(&session_bus).await?;
			#[cfg(feature = "tracing")]
			tracing::debug!("Getting a11y bus address from session bus");
			proxy.get_address().await?
		};
		#[cfg(feature = "tracing")]
		tracing::debug!(address = %a11y_bus_addr, "Got a11y bus address");
		let addr: Address = a11y_bus_addr.parse()?;
		Self::connect(addr).await
	}

	/// Returns an [`AccessibilityConnection`], a wrapper for the [`RegistryProxy`]; a handle for the registry provider
	/// on the accessibility bus.
	///
	/// You may want to call this if you have the accessibility bus address and want a connection with
	/// a convenient async event stream provisioning.
	///
	/// Without address, you will want to call  `open`, which tries to obtain the accessibility bus' address
	/// on your behalf.
	///
	/// # Errors
	///
	/// `RegistryProxy` is configured with invalid path, interface or destination
	pub async fn connect(bus_addr: Address) -> zbus::Result<Self> {
		#[cfg(feature = "tracing")]
		tracing::debug!("Connecting to a11y bus");
		let bus = Box::pin(zbus::ConnectionBuilder::address(bus_addr)?.build()).await?;
		#[cfg(feature = "tracing")]
		tracing::debug!(name = bus.unique_name().map(|n| n.as_str()), "Connected to a11y bus");

		// The Proxy holds a strong reference to a Connection, so we only need to store the proxy
		let registry = RegistryProxy::new(&bus).await?;
		let dbus_proxy = DBusProxy::new(registry.connection()).await?;

		Ok(Self { registry, dbus_proxy })
	}

	/// Stream yielding all `Event` types.
	///
	/// Monitor this stream to be notified and receive events on the a11y bus.
	///
	/// # Example
	/// Basic use:
	///
	/// ```rust
	/// use atspi_connection::AccessibilityConnection;
	/// use enumflags2::BitFlag;
	/// use atspi_connection::common::events::object::{ObjectEvents, StateChangedEvent};
	/// use zbus::{fdo::DBusProxy, MatchRule, MessageType};
	/// use atspi_connection::common::events::Event;
	/// # use futures_lite::StreamExt;
	/// # use std::error::Error;
	///
	/// # fn main() {
	/// #   assert!(tokio_test::block_on(example()).is_ok());
	/// # }
	///
	/// # async fn example() -> Result<(), Box<dyn Error>> {
	///     let atspi = AccessibilityConnection::open().await?;
	///     atspi.register_event::<ObjectEvents>().await?;
	///
	///     let mut events = atspi.event_stream();
	///     std::pin::pin!(&mut events);
	/// #   let output = std::process::Command::new("busctl")
	/// #       .arg("--user")
	/// #       .arg("call")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("/org/a11y/bus")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("GetAddress")
	/// #       .output()
	/// #       .unwrap();
	/// #    let addr_string = String::from_utf8(output.stdout).unwrap();
	/// #    let addr_str = addr_string
	/// #        .strip_prefix("s \"")
	/// #        .unwrap()
	/// #        .trim()
	/// #        .strip_suffix('"')
	/// #        .unwrap();
	/// #   let mut base_cmd = std::process::Command::new("busctl");
	/// #   let thing = base_cmd
	/// #       .arg("--address")
	/// #       .arg(addr_str)
	/// #       .arg("emit")
	/// #       .arg("/org/a11y/atspi/accessible/null")
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("StateChanged")
	/// #       .arg("siiva{sv}")
	/// #       .arg("")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .arg("i")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .output()
	/// #       .unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         // Handle Object events
	///        if let Ok(event) = StateChangedEvent::try_from(ev) {
	/// #        break;
	///          // do something else here
	///        } else { continue }
	///     }
	/// #    Ok(())
	/// # }
	/// ```
	pub fn event_stream(&self) -> impl Stream<Item = Result<Event, AtspiError>> {
		MessageStream::from(self.registry.connection()).filter_map(|res| {
			let msg = match res {
				Ok(m) => m,
				Err(e) => return Some(Err(e.into())),
			};
			match msg.message_type() {
				MessageType::Signal => Some(Event::try_from(&*msg)),
				_ => None,
			}
		})
	}

	/// Registers an events as defined in [`atspi-types::events`]. This function registers a single event, like so:
	/// ```rust
	/// use atspi_connection::common::events::object::StateChangedEvent;
	/// # tokio_test::block_on(async {
	/// let connection = atspi_connection::AccessibilityConnection::open().await.unwrap();
	/// connection.register_event::<StateChangedEvent>().await.unwrap();
	/// # })
	/// ```
	///
	/// # Errors
	///
	/// This function may return an error if a [`zbus::Error`] is caused by all the various calls to [`zbus::fdo::DBusProxy`] and [`zbus::MatchRule::try_from`].
	pub async fn add_match_rule<T: HasMatchRule>(&self) -> Result<(), AtspiError> {
		let match_rule = MatchRule::try_from(<T as HasMatchRule>::MATCH_RULE_STRING)?;
		self.dbus_proxy.add_match_rule(match_rule).await?;
		Ok(())
	}

	/// Deregisters an events as defined in [`atspi-types::events`]. This function registers a single event, like so:
	/// ```rust
	/// use atspi_connection::common::events::object::StateChangedEvent;
	/// # tokio_test::block_on(async {
	/// let connection = atspi_connection::AccessibilityConnection::open().await.unwrap();
	/// connection.add_match_rule::<StateChangedEvent>().await.unwrap();
	/// connection.remove_match_rule::<StateChangedEvent>().await.unwrap();
	/// # })
	/// ```
	///
	/// # Errors
	///
	/// This function may return an error if a [`zbus::Error`] is caused by all the various calls to [`zbus::fdo::DBusProxy`] and [`zbus::MatchRule::try_from`].
	pub async fn remove_match_rule<T: HasMatchRule>(&self) -> Result<(), AtspiError> {
		let match_rule = MatchRule::try_from(<T as HasMatchRule>::MATCH_RULE_STRING)?;
		self.dbus_proxy.add_match_rule(match_rule).await?;
		Ok(())
	}

	/// Add a registry event.
	/// This tells accessible applications which events should be forwarded to the accessibility bus.
	/// This is called by [`Self::register_event`].
	///
	/// ```rust
	/// use atspi_connection::common::events::object::StateChangedEvent;
	/// # tokio_test::block_on(async {
	/// let connection = atspi_connection::AccessibilityConnection::open().await.unwrap();
	/// connection.add_registry_event::<StateChangedEvent>().await.unwrap();
	/// connection.remove_registry_event::<StateChangedEvent>().await.unwrap();
	/// # })
	/// ```
	///
	/// # Errors
	///
	/// May cause an error if the `DBus` method [`atspi_proxies::registry::RegistryProxy::register_event`] fails.
	pub async fn add_registry_event<T: HasRegistryEventString>(&self) -> Result<(), AtspiError> {
		self.registry
			.register_event(<T as HasRegistryEventString>::REGISTRY_EVENT_STRING)
			.await?;
		Ok(())
	}

	/// Remove a registry event.
	/// This tells accessible applications which events should be forwarded to the accessibility bus.
	/// This is called by [`Self::deregister_event`].
	/// It may be called like so:
	///
	/// ```rust
	/// use atspi_connection::common::events::object::StateChangedEvent;
	/// # tokio_test::block_on(async {
	/// let connection = atspi_connection::AccessibilityConnection::open().await.unwrap();
	/// connection.add_registry_event::<StateChangedEvent>().await.unwrap();
	/// connection.remove_registry_event::<StateChangedEvent>().await.unwrap();
	/// # })
	/// ```
	///
	/// # Errors
	///
	/// May cause an error if the `DBus` method [`RegistryProxy::deregister_event`] fails.
	pub async fn remove_registry_event<T: HasRegistryEventString>(&self) -> Result<(), AtspiError> {
		self.registry
			.deregister_event(<T as HasRegistryEventString>::REGISTRY_EVENT_STRING)
			.await?;
		Ok(())
	}

	/// This calls [`Self::add_registry_event`] and [`Self::add_match_rule`], two components necessary to receive accessibility events.
	/// # Errors
	/// This will only fail if [`Self::add_registry_event`[ or [`Self::add_match_rule`] fails.
	pub async fn register_event<T: HasRegistryEventString + HasMatchRule>(
		&self,
	) -> Result<(), AtspiError> {
		self.add_registry_event::<T>().await?;
		self.add_match_rule::<T>().await?;
		Ok(())
	}

	/// This calls [`Self::remove_registry_event`] and [`Self::remove_match_rule`], two components necessary to receive accessibility events.
	/// # Errors
	/// This will only fail if [`Self::remove_registry_event`] or [`Self::remove_match_rule`] fails.
	pub async fn deregister_event<T: HasRegistryEventString + HasMatchRule>(
		&self,
	) -> Result<(), AtspiError> {
		self.remove_registry_event::<T>().await?;
		self.remove_match_rule::<T>().await?;
		Ok(())
	}

	/// Shorthand for a reference to the underlying [`zbus::Connection`]
	#[must_use = "The reference to the underlying zbus::Connection must be used"]
	pub fn connection(&self) -> &zbus::Connection {
		self.registry.connection()
	}

	/// Send an event over the accessibility bus.
	/// This converts the event into a [`zbus::Message`] using the [`GenericEvent`] trait.
	///
	/// # Errors
	///
	/// This will only fail if:
	/// 1. [`zbus::MessageBuilder`] fails at any point, or
	/// 2. sending the event fails for some reason.
	///
	/// Both of these conditions should never happen as long as you have a valid event.
	pub async fn send_event<T>(&self, event: T) -> Result<u32, AtspiError>
	where
		T: for<'a> GenericEvent<'a>,
	{
		let conn = self.connection();
		let new_message = zbus::MessageBuilder::signal(
			event.path(),
			<T as GenericEvent>::DBUS_INTERFACE,
			<T as GenericEvent>::DBUS_MEMBER,
		)?
		.sender(conn.unique_name().ok_or(AtspiError::MissingName)?)?
		// this re-encodes the entire body; it's not great..., but you can't replace a sender once a message a created.
		.build(&event.body())?;
		Ok(conn.send_message(new_message).await?)
	}
}

impl Deref for AccessibilityConnection {
	type Target = RegistryProxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.registry
	}
}

/// Set the `IsEnabled` property in the session bus.
///
/// Assistive Technology provider applications (ATs) should set the accessibility
/// `IsEnabled` status on the users session bus on startup as applications may monitor this property
/// to  enable their accessibility support dynamically.
///
/// See: The [freedesktop - AT-SPI2 wiki](https://www.freedesktop.org/wiki/Accessibility/AT-SPI2/)
///
///  ## Example
/// ```rust
///     let result =  tokio_test::block_on( atspi_connection::set_session_accessibility(true) );
///     assert!(result.is_ok());
/// ```
/// # Errors
///
/// 1. when no connection with the session bus can be established,
/// 2. if creation of a [`atspi_proxies::bus::StatusProxy`] fails
/// 3. if the `IsEnabled` property cannot be read
/// 4. the `IsEnabled` property cannot be set.
pub async fn set_session_accessibility(status: bool) -> std::result::Result<(), AtspiError> {
	// Get a connection to the session bus.
	let session = Box::pin(zbus::Connection::session()).await?;

	// Acquire a `StatusProxy` for the session bus.
	let status_proxy = StatusProxy::new(&session).await?;

	if status_proxy.is_enabled().await? != status {
		status_proxy.set_is_enabled(status).await?;
	}
	Ok(())
}

/// Read the `IsEnabled` accessibility status property on the session bus.
///
/// # Examples
/// ```rust
///     # tokio_test::block_on( async {
///     let status = atspi_connection::read_session_accessibility().await;
///
///     // The status is either true or false
///        assert!(status.is_ok());
///     # });
/// ```
///
/// # Errors
///
/// - If no connection with the session bus could be established.
/// - If creation of a [`atspi_proxies::bus::StatusProxy`] fails.
/// - If the `IsEnabled` property cannot be read.
pub async fn read_session_accessibility() -> AtspiResult<bool> {
	// Get a connection to the session bus.
	let session = Box::pin(zbus::Connection::session()).await?;

	// Acquire a `StatusProxy` for the session bus.
	let status_proxy = StatusProxy::new(&session).await?;

	// Read the `IsEnabled` property.
	status_proxy.is_enabled().await.map_err(Into::into)
}
