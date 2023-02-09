use crate::{
	bus::BusProxy,
	events::{Event, HasMatchRule},
	registry::RegistryProxy,
	AtspiError,
};
use enumflags2::{BitFlag, BitFlags};
use futures_lite::stream::{Stream, StreamExt};
use serde::Serialize;
use std::ops::Deref;
use zbus::{fdo::DBusProxy, zvariant::Signature, Address, MatchRule, MessageStream, MessageType};

// Event body signatures: These outline the event specific deserialized event types.
// Safety: These are evaluated at compile time.
// ----
// The signal signature "(so)" (an Accessible) is ambiguous, because it is used in:
// -  Cache : RemoveAccessible
// -  Socket: Available  *( signals the availability of the `Registry` daeomon.)
//
// ATSPI- and QSPI both describe the generic events. These can be converted into
// specific signal types with TryFrom implementations. See crate::[`identify`]
//  EVENT_LISTENER is a type signature used to notify when events are registered or deregistered.
//  CACHE_ADD and *_REMOVE have very different types
pub const ATSPI_EVENT: Signature<'_> = Signature::from_static_str_unchecked("siiva{sv}");
pub const QSPI_EVENT: Signature<'_> = Signature::from_static_str_unchecked("siiv(so)");
pub const ACCESSIBLE: Signature<'_> = Signature::from_static_str_unchecked("(so)");
pub const EVENT_LISTENER: Signature<'_> = Signature::from_static_str_unchecked("(ss)");
pub const CACHE_ADD: Signature<'_> =
	Signature::from_static_str_unchecked("((so)(so)(so)iiassusau)");

/// A connection to the at-spi bus
pub struct Connection {
	registry: RegistryProxy<'static>,
}

impl Connection {
	/// Open a new connection to the bus
	#[tracing::instrument]
	pub async fn open() -> zbus::Result<Self> {
		// Grab the a11y bus address from the session bus
		let a11y_bus_addr = {
			tracing::debug!("Connecting to session bus");
			let session_bus = zbus::Connection::session().await?;
			tracing::debug!(
				name = session_bus.unique_name().map(|n| n.as_str()),
				"Connected to session bus"
			);
			let proxy = BusProxy::new(&session_bus).await?;
			tracing::debug!("Getting a11y bus address from session bus");
			proxy.get_address().await?
		};
		tracing::debug!(address = %a11y_bus_addr, "Got a11y bus address");
		let addr: Address = a11y_bus_addr.parse()?;
		Self::connect(addr).await
	}

	/// Returns a  [`Connection`], a wrapper for the [`RegistryProxy`]; a handle for the registry provider
	/// on the accessibility bus.
	///
	/// You may want to call this if you have the accessibility bus address and want a connection with
	/// a convenient async event stream provisioning.
	///
	/// Without address, you will want to call  `open`, which tries to obtain the accessibility bus' address
	/// on your behalf.
	///
	/// ## Errors
	/// * `RegistryProxy` is configured with invalid path, interface or destination defaults.
	pub async fn connect(bus_addr: Address) -> zbus::Result<Self> {
		tracing::debug!("Connecting to a11y bus");
		let bus = zbus::ConnectionBuilder::address(bus_addr)?.build().await?;
		tracing::debug!(name = bus.unique_name().map(|n| n.as_str()), "Connected to a11y bus");
		// The Proxy holds a strong reference to a Connection, so we only need to store the proxy
		let registry = RegistryProxy::new(&bus).await?;

		Ok(Self { registry })
	}

	/// Stream yielding all `Event` types.
	///
	/// Monitor this stream to be notified and receive events on the a11y bus.
	///
	/// # Example
	/// Basic use:
	///
	/// ```
	/// use atspi::events::{
	///   EventInterfaces,
	///		HasMatchRules,
	/// };
	/// use enumflags2::BitFlag;
	/// use atspi::identify::object::ObjectEvents;
	/// use atspi::signify::Signified;
	/// use atspi::zbus::{fdo::DBusProxy, MatchRule, MessageType};
	/// use atspi::Event;
	/// # use futures_lite::StreamExt;
	/// # use std::error::Error;
	///
	/// # fn main() {
	/// #   assert!(futures_lite::future::block_on(example()).is_ok());
	/// # }
	///
	/// # async fn example() -> Result<(), Box<dyn Error>> {
	///     let atspi = atspi::Connection::open().await?;
	///     atspi.register_events(ObjectEvents::match_rules().unwrap()).await?;
	///
	///     let events = atspi.event_stream();
	///     futures_lite::pin!(events);
	/// #   
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
	///         // Handle Objject events
	///        break;
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
				MessageType::Signal => Some(Event::try_from(msg)),
				_ => None,
			}
		})
	}

	// TODO: do this without instantiating a DBus proxy evwry time.
	/// Registers an events as defined in [`crate::events::names`]. This function registers a single event, like so:
	/// ```rust
	/// use atspi::{
	///		identify::object::StateChangedEvent,
	///		events::HasMatchRule,
	///	};
	/// # tokio_test::block_on(async {
	/// let connection = atspi::Connection::open().await.unwrap();
	/// connection.register_event(StateChangedEvent::match_rule().unwrap()).await.unwrap();
	/// # })
	/// ```
	///
	/// # Errors
	///
	/// This function may return an error if it is unable to serialize the variant of the enum that has been passed (should never happen), or
	/// a [`zbus::Error`] is caused by all the various calls to [`zbus::fdo::DBusProxy`] and [`zbus::MatchRule`].
	pub async fn register_event(&self, match_rule: MatchRule<'_>) -> Result<(), AtspiError> {
		let dbus_proxy = DBusProxy::new(self.registry.connection()).await?;
		dbus_proxy.add_match_rule(match_rule).await?;
		Ok(())
	}

	/// Register multiple events in one swoop!
	/// Very useful for registering events of one interface together.
	/// This can be done like so:
	///
	/// ```rust
	/// use atspi::{
	///		identify::object::ObjectEvents,
	///		events::HasMatchRules,
	///	};
	/// # tokio_test::block_on(async {
	/// let connection = atspi::Connection::open().await.unwrap();
	/// connection.register_events(ObjectEvents::match_rules().unwrap()).await.unwrap();
	/// # })
	/// ```
	///
	/// # Errors
	/// For failure conditions, see [`Self::register_event`].
	pub async fn register_events<'a, I>(&self, events: I) -> Result<(), AtspiError>
	where
		I: IntoIterator<Item = MatchRule<'a>>,
	{
		for event in events {
			self.register_event(event).await?;
		}
		Ok(())
	}
}

impl Deref for Connection {
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
///     use futures_lite::future::block_on;
///
///     let result =  block_on( atspi::set_session_accessibility(true) );
///     assert!(result.is_ok());
/// ```
///  ## Errors
/// * when no connection with the session bus can be established,
/// * if creation of a [`crate::bus::StatusProxy`] fails
/// * if the `IsEnabled` property cannot be read
/// * the `IsEnabled` property cannot be set.
pub async fn set_session_accessibility(status: bool) -> std::result::Result<(), AtspiError> {
	// Get a connection to the session bus.
	let session = zbus::Connection::session().await?;

	// Aqcuire a `StatusProxy` for the session bus.
	let status_proxy = crate::bus::StatusProxy::new(&session).await?;

	if status_proxy.is_enabled().await? != status {
		status_proxy.set_is_enabled(status).await?;
	}
	Ok(())
}
