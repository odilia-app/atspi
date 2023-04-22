pub mod document;
pub mod focus;
pub mod keyboard;
pub mod mouse;
pub mod object;
pub mod terminal;
pub mod window;

// Event body signatures: These outline the event specific deserialized event types.
// Safety: These are evaluated at compile time.
// ----
// The signal signature "(so)" (an Accessible) is ambiguous, because it is used in:
// -  Cache : RemoveAccessible
// -  Socket: Available  *( signals the availability of the `Registry` daeomon.)
//
// ATSPI- and QSPI both describe the generic events. These can be converted into
// specific signal types with TryFrom implementations. See crate::[`identify`]
//  EVENT_LISTENER_SIGNATURE is a type signature used to notify when events are registered or deregistered.
//  CACHE_ADD_SIGNATURE and *_REMOVE have very different types
pub const ATSPI_EVENT_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("siiva{sv}");
pub const QSPI_EVENT_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("siiv(so)");
pub const ACCESSIBLE_PAIR_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(so)");
pub const EVENT_LISTENER_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(ss)");
pub const CACHE_ADD_SIGNATURE: Signature<'_> =
	Signature::from_static_str_unchecked("((so)(so)(so)iiassusau)");

use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use zbus::{
	names::{InterfaceName, MemberName, OwnedUniqueName, UniqueName},
	zvariant::{self, OwnedObjectPath, OwnedValue, Signature, Type, Value},
	Message,
};

use crate::{
	cache::CacheItem,
	identify::{
		document::DocumentEvents, focus::FocusEvents, keyboard::KeyboardEvents, mouse::MouseEvents,
		object::ObjectEvents, terminal::TerminalEvents, window::WindowEvents,
	},
	AtspiError,
};
use atspi_macros::{try_from_zbus_message, GenericEvent};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventBody<'a, T> {
	#[serde(rename = "type")]
	pub kind: T,
	pub detail1: i32,
	pub detail2: i32,
	#[serde(borrow)]
	pub any_data: Value<'a>,
	#[serde(borrow)]
	pub properties: HashMap<&'a str, Value<'a>>,
}

impl<T> Type for EventBody<'_, T> {
	fn signature() -> Signature<'static> {
		<(&str, i32, i32, Value, HashMap<&str, Value>)>::signature()
	}
}

// Signature:  "siiv(so)",
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct EventBodyQT {
	#[serde(rename = "type")]
	pub kind: String,
	pub detail1: i32,
	pub detail2: i32,
	pub any_data: OwnedValue,
	pub properties: (String, OwnedObjectPath),
}

// Signature (siiva{sv}),
#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct EventBodyOwned {
	#[serde(rename = "type")]
	pub kind: String,
	pub detail1: i32,
	pub detail2: i32,
	pub any_data: OwnedValue,
	pub properties: HashMap<String, OwnedValue>,
}

impl From<EventBodyQT> for EventBodyOwned {
	fn from(body: EventBodyQT) -> Self {
		let mut props = HashMap::new();
		props.insert(
			body.properties.0,
			Value::ObjectPath(body.properties.1.into_inner()).to_owned(),
		);
		Self {
			kind: body.kind,
			detail1: body.detail1,
			detail2: body.detail2,
			any_data: body.any_data,
			properties: props,
		}
	}
}

/// Encapsulates the various different accessibility bus signal types.
///
/// Assumes being non exhaustive to allow for future- or custom signals.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
	// Exploring if having a fully destructable hierarchical  works as we'd like..
	Interfaces(EventInterfaces),
	/// Emitted when the ` Registry` interface on `org.a11y.atspi.Registry` becomes available.
	Available(AvailableEvent),
	/// Both `CacheAdd` and `CacheRemove` signals
	Cache(CacheEvents),
	/// Emitted on registry or deregristry of event listeners.,
	///
	/// (eg. "Cache:AddAccessible:")
	Listener(EventListenerEvents),
}

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub enum CacheEvents {
	Add(AddAccessibleEvent),
	Remove(RemoveAccessibleEvent),
}

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::CacheItem`]
#[derive(Debug, Clone, GenericEvent)]
#[try_from_zbus_message(body = "CacheItem")]
pub struct AddAccessibleEvent {
	pub(crate) message: Arc<Message>,
	pub(crate) body: CacheItem,
}
impl HasRegistryEventString for AddAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Add";
}
impl HasMatchRule for AddAccessibleEvent {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'";
}

impl AddAccessibleEvent {
	/// When an object in an application is added, this may evoke a `CacheAdd` event,
	/// this yields an [`crate::cache::CacheItem`]
	#[must_use]
	pub fn item(&self) -> &CacheItem {
		&self.body
	}

	/// When an object in an application is added, this may evoke a `CacheAdd` event,
	/// this yields an [`crate::cache::CacheItem`]
	/// Consumes the `CacheAdd` event.
	#[must_use]
	pub fn into_item(self) -> CacheItem {
		self.body
	}
}

#[derive(Debug, Clone, GenericEvent)]
#[try_from_zbus_message(body = "Accessible")]
pub struct RemoveAccessibleEvent {
	pub(crate) message: Arc<Message>,
	pub(crate) body: Accessible,
}
impl HasRegistryEventString for RemoveAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Remove";
}
impl HasMatchRule for RemoveAccessibleEvent {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='RemoveAccessible'";
}

impl RemoveAccessibleEvent {
	/// What `Accessible` is removed from the application state.
	/// A reference to the `Accessible`
	#[must_use]
	pub fn as_accessible(&self) -> &Accessible {
		&self.body
	}

	/// What `Accessible` is removed from the application state.
	/// Converts the event to an Accessible
	/// Consumes the cache remove event.
	#[must_use]
	pub fn into_accessible(self) -> Accessible {
		self.body
	}

	// pub fn as_iface_reusing_connection(&self, conn: &Connection) -> AccessibleProxy {
	//     let Accessible { name, path } = self.as_accessible();
	//     crate::accessible::new(&**conn, sender, path.into())
	// }
}

// TODO: Try to make borrowed versions work,
// check where the lifetimes of the borrow are tied to, see also: comment on `interface()` method
// in `DefaultEvent` impl
// then rename into Owned for this one.
/// Owned Accessible type
/// Emitted by `CacheRemove` and `Available`
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Accessible {
	pub name: OwnedUniqueName,
	pub path: OwnedObjectPath,
}

#[test]
fn test_accessible_signature() {
	assert_eq!(Accessible::signature(), "(so)");
}

/// Offers events, grouped-by Interface.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EventInterfaces {
	Document(DocumentEvents),
	Focus(FocusEvents),
	Keyboard(KeyboardEvents),
	Mouse(MouseEvents),
	Object(ObjectEvents),
	Terminal(TerminalEvents),
	Window(WindowEvents),
}

impl TryFrom<AtspiEvent> for EventInterfaces {
	type Error = AtspiError;

	fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
		let Some(interface) = ev.interface() else {  return Err(AtspiError::MissingInterface);  };
		match interface.as_str() {
			"org.a11y.atspi.Event.Document" => {
				Ok(EventInterfaces::Document(DocumentEvents::try_from(ev)?))
			}
			"org.a11y.atspi.Event.Focus" => Ok(EventInterfaces::Focus(FocusEvents::try_from(ev)?)),
			"org.a11y.atspi.Event.Keyboard" => {
				Ok(EventInterfaces::Keyboard(KeyboardEvents::try_from(ev)?))
			}
			"org.a11y.atspi.Event.Mouse" => Ok(EventInterfaces::Mouse(MouseEvents::try_from(ev)?)),
			"org.a11y.atspi.Event.Object" => {
				Ok(EventInterfaces::Object(ObjectEvents::try_from(ev)?))
			}
			"org.a11y.atspi.Event.Terminal" => {
				Ok(EventInterfaces::Terminal(TerminalEvents::try_from(ev)?))
			}
			"org.a11y.atspi.Event.Window" => {
				Ok(EventInterfaces::Window(WindowEvents::try_from(ev)?))
			}
			_ => Err(AtspiError::UnknownInterface),
		}
	}
}

#[derive(Debug, Clone)]
pub struct AtspiEvent {
	pub(crate) message: Arc<Message>,
	pub(crate) body: EventBodyOwned,
}

impl<'name> PartialEq<AtspiEvent> for MemberName<'name> {
	fn eq(&self, other: &AtspiEvent) -> bool {
		let other_member = other.member().expect("AtspiEvent without member?");
		*self == other_member
	}
}

impl<'name> PartialEq<MemberName<'name>> for AtspiEvent {
	fn eq(&self, other: &MemberName) -> bool {
		let self_member = self.member().expect("AtspiEvent w/o member?");
		self_member == *other
	}
}

//  Equality on `AtspiEvent` may be considered ambiguous.
//
// Because `AtspiEvent`'s message has eg. a `SerialNumber` in the primary header,
// only exacly same instances would be equal, _if_ `PartialEq` were derivable.
//
// This PartialEq implements the strictest kind where only same instances are considered the same.
// Other equalities should be made with PartailEq<T> for AtspiEvent and PartialEq<AtspiEvent> for T
impl PartialEq for AtspiEvent {
	fn eq(&self, other: &AtspiEvent) -> bool {
		self.message.as_bytes() == other.message().as_bytes()
	}
}

impl Eq for AtspiEvent {}

impl TryFrom<Arc<Message>> for AtspiEvent {
	type Error = AtspiError;

	fn try_from(message: Arc<Message>) -> Result<Self, Self::Error> {
		let signature = message.body_signature()?;
		let body = if signature == QSPI_EVENT_SIGNATURE {
			EventBodyOwned::from(message.body::<EventBodyQT>()?)
		} else {
			message.body::<EventBodyOwned>()?
		};
		Ok(Self { message, body })
	}
}

/// Signal type emitted by `EventListenerRegistered` and `EventListenerDeregistered` signals,
/// which belong to the `Registry` interface, implemented by the registry-daemon.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct EventListeners {
	pub bus_name: OwnedUniqueName,
	pub path: String,
}

#[test]
fn test_event_listener_signature() {
	assert_eq!(EventListeners::signature(), "(ss)");
}

/// Covers both `EventListener` events.
#[derive(Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum EventListenerEvents {
	Registered(EventListenerRegisteredEvent),
	Deregistered(EventListenerDeregisteredEvent),
}

/// An event that is emitted by the regostry daemon to signal that an event has been deregistered
/// to no longer listen for.
#[derive(Clone, Debug, GenericEvent)]
#[try_from_zbus_message(body = "EventListeners")]
pub struct EventListenerDeregisteredEvent {
	pub(crate) message: Arc<Message>,
	pub body: EventListeners,
}

/// An event that is emitted by the regostry daemon to signal that an event has been registered to listen for.
#[derive(Clone, Debug, GenericEvent)]
#[try_from_zbus_message(body = "EventListeners")]
pub struct EventListenerRegisteredEvent {
	pub(crate) message: Arc<Message>,
	pub body: EventListeners,
}

/// An event that is emitted when the registry daemon has started.
#[derive(Clone, Debug, GenericEvent)]
#[try_from_zbus_message(body = "Accessible")]
pub struct AvailableEvent {
	pub(crate) message: Arc<Message>,
	pub(crate) body: Accessible,
}

impl AvailableEvent {
	#[must_use]
	pub fn registry(&self) -> &Accessible {
		&self.body
	}
}

impl TryFrom<Arc<Message>> for Event {
	type Error = AtspiError;

	fn try_from(msg: Arc<Message>) -> Result<Event, AtspiError> {
		let body_signature = msg.body_signature()?;
		let message_signature = body_signature.as_str();
		let signal_member = msg
			.member()
			.ok_or(AtspiError::MemberMatch("signal w/o member".to_string()))?;
		let message_member = signal_member.as_str();

		match message_signature {
			// Accessible signature
			"(so)" => match message_member {
				"RemoveAccessible" => {
					let ev = RemoveAccessibleEvent::try_from(msg)?;
					Ok(Event::Cache(CacheEvents::Remove(ev)))
				}
				"Available" => {
					let ev = AvailableEvent::try_from(msg)?;
					Ok(Event::Available(ev))
				}
				_ => Err(AtspiError::UnknownSignal),
			},
			// Atspi / Qspi signature
			"siiva{sv}" | "siiv(so)" => {
				let ev = AtspiEvent::try_from(msg)?;
				let event_interfaces: EventInterfaces = ev.try_into()?;
				Ok(Event::Interfaces(event_interfaces))
			}
			"(ss)" => {
				if let Ok(ev) = EventListenerRegisteredEvent::try_from(msg.clone()) {
					return Ok(Event::Listener(EventListenerEvents::Registered(ev)));
				}
				if let Ok(ev) = EventListenerDeregisteredEvent::try_from(msg) {
					return Ok(Event::Listener(EventListenerEvents::Deregistered(ev)));
				}
				Err(AtspiError::UnknownSignal)
			}
			// CacheAdd signature
			"((so)(so)(so)iiassusau)" => {
				let ev = AddAccessibleEvent::try_from(msg)?;
				Ok(Event::Cache(CacheEvents::Add(ev)))
			}
			_ => Err(AtspiError::UnknownBusSignature),
		}
	}
}

/// Shared behavior of bus `Signal` events.
pub trait GenericEvent {
	/// Returns the `Message` of the event type.
	fn message(&self) -> &Arc<Message>;

	/// Interface that has the signal member implemented.
	fn interface(&self) -> Option<InterfaceName<'_>>;

	/// Interface member that sent the signal.
	fn member(&self) -> Option<MemberName<'_>>;

	/// Path of the signalling object.
	fn path(&self) -> Option<zvariant::ObjectPath<'_>>;

	/// Sender of the signal.
	///
	/// ### Errors
	/// - when deserializeing the header failed, or
	/// - When `zbus::get_field!` finds that 'sender' is an invalid field.
	fn sender(&self) -> Result<Option<UniqueName>, AtspiError>;
}

pub trait HasMatchRule {
	const MATCH_RULE_STRING: &'static str;
}

pub trait HasRegistryEventString {
	const REGISTRY_EVENT_STRING: &'static str;
}

impl AtspiEvent {
	/// Deserialized signal body type.
	#[must_use]
	pub fn body(&self) -> &EventBodyOwned {
		&self.body
	}

	/// Returns the atspi event string for this event type (E.G. "Object:StateChanged:Focused").
	///
	/// This should not be used for matching on events as it needlessly allocates and copies the 3
	/// components of the event type. It is meant for logging, etc.
	#[must_use]
	pub fn event_string(&self) -> String {
		let interface = self.message.interface().expect("Event should have an interface");
		let interface = interface.rsplit('.').next().expect("Interface should contain a '.'");
		let member = self.message.member().expect("Event should have a member");
		let kind = self.kind();
		format!("{interface}:{member}:{kind}")
	}

	#[must_use]
	pub fn kind(&self) -> &str {
		&self.body.kind
	}

	/// Event dependant detail.
	#[must_use]
	pub fn detail1(&self) -> i32 {
		self.body.detail1
	}

	/// Event dependant detail.
	#[must_use]
	pub fn detail2(&self) -> i32 {
		self.body.detail2
	}

	/// Event dependant generic `Value`.
	#[must_use]
	pub fn any_data(&self) -> &zvariant::OwnedValue {
		&self.body.any_data
	}

	#[must_use]
	pub fn properties(&self) -> &HashMap<String, zvariant::OwnedValue> {
		&self.body.properties
	}
}

impl GenericEvent for AtspiEvent {
	/// Bus message.
	#[must_use]
	fn message(&self) -> &Arc<Message> {
		&self.message
	}

	/// The interface that emitted the event.
	#[must_use]
	fn interface(&self) -> Option<InterfaceName<'_>> {
		self.message.interface()
	}

	/// Identifies this event interface's member name.
	#[must_use]
	fn member(&self) -> Option<MemberName<'_>> {
		self.message.member()
	}

	/// The object path to the object where the signal was emitted.
	#[must_use]
	fn path(&self) -> std::option::Option<zbus::zvariant::ObjectPath<'_>> {
		self.message.path()
	}

	/// Identifies the `sender` of the event.
	/// # Errors
	/// - when deserializeing the header failed, or
	/// - When `zbus::get_field!` finds that 'sender' is an invalid field.
	fn sender(&self) -> Result<Option<zbus::names::UniqueName>, crate::AtspiError> {
		Ok(self.message.header()?.sender()?.cloned())
	}
}

#[cfg(test)]
mod tests {
	use crate::events::{
		CacheEvents, Event, EventBodyOwned, EventBodyQT, RemoveAccessibleEvent, GenericEvent, AddAccessibleEvent, CacheItem, ACCESSIBLE_PAIR_SIGNATURE, CACHE_ADD_SIGNATURE,
	};
	use tokio::time::timeout;
	use crate::{AccessibilityConnection, InterfaceSet, StateSet, accessible::Role};
	use futures_lite::StreamExt;
	use std::{collections::HashMap,time::Duration};
	use zbus::zvariant::{ObjectPath, Value, OwnedObjectPath};
	use zbus::MessageBuilder;

	fn gen_event_body_qt() -> EventBodyQT {
		EventBodyQT {
			kind: "remove".to_string(),
			detail1: 0,
			detail2: 0,
			any_data: Value::U8(0u8).into(),
			properties: ("".to_string(), ObjectPath::try_from("/").unwrap().into()),
		}
	}

	#[test]
	fn test_event_body_qt_to_event_body_owned_conversion() {
		let event_body: EventBodyOwned = gen_event_body_qt().into();
		let props = HashMap::from([("".to_string(), ObjectPath::try_from("/").unwrap().into())]);
		assert_eq!(event_body.properties, props);
	}
	#[tokio::test]
	async fn test_recv_remove_accessible() {
		let atspi = AccessibilityConnection::open().await.unwrap();
		let mut events = atspi.event_stream();
		atspi.register_event::<RemoveAccessibleEvent>().await.unwrap();
		std::pin::pin!(&mut events);
		let to = timeout(Duration::from_secs(1), events.next());
		let unique_bus_name = atspi.connection().unique_name();
		let msg = MessageBuilder::signal(
			"/org/a11y/atspi/accessible/null",
			"org.a11y.atspi.Cache",
			"RemoveAccessible",
		).expect("Could not create signal")
			.sender(unique_bus_name.unwrap())
			.expect("Could not set sender to {unique_bus_name:?}")
			.build(&(
				((":69.420".to_string(), OwnedObjectPath::try_from("/org/a11y/atspi/accessible/remove").unwrap()),)
			),)
			.unwrap();
		assert_eq!(msg.body_signature().unwrap(), ACCESSIBLE_PAIR_SIGNATURE);
		atspi.connection().send_message(msg).await.unwrap();
		match to.await {
			Ok(Some(Ok(Event::Cache(CacheEvents::Remove(event))))) => {
				assert_eq!(event.path().unwrap(), "/org/a11y/atspi/accessible/null");
				assert_eq!(event.as_accessible().path.as_str(), "/org/a11y/atspi/accessible/remove");
				assert_eq!(event.as_accessible().name.as_str(), ":69.420");
			},
			Ok(Some(Ok(another_event))) => {
				println!("{:?}", another_event);
				panic!("The wrong event was sent");
			},
			Ok(e) => {
				println!("{:?}", e);
				panic!("Something else happened");
			}
			Err(e) => {
				panic!("An error occured: {:?}", e);
			}
		}
	}
	#[tokio::test]
	async fn test_recv_add_accessible() {
		let atspi = AccessibilityConnection::open().await.unwrap();
		let mut events = atspi.event_stream();
		atspi.register_event::<AddAccessibleEvent>().await.unwrap();
		std::pin::pin!(&mut events);
		let unique_bus_name = atspi.connection().unique_name();
		let msg = MessageBuilder::signal(
			"/org/a11y/atspi/accessible/null",
			"org.a11y.atspi.Cache",
			"AddAccessible",
		).expect("Could not create signal")
			.sender(unique_bus_name.unwrap())
			.expect("Could not set sender to {unique_bus_name:?}")
			.build(&(CacheItem {
				object: (":1.1".to_string(), OwnedObjectPath::try_from("/org/a11y/atspi/accessible/object").unwrap()),
				app: (":1.1".to_string(), OwnedObjectPath::try_from("/org/a11y/atspi/accessible/application").unwrap()),
				parent: (":1.1".to_string(), OwnedObjectPath::try_from("/org/a11y/atspi/accessible/parent").unwrap()),
				index: 0,
				children: 0,
				ifaces: InterfaceSet::empty(),
				short_name: "".to_string(),
				role: Role::Application,
				name: "Hi".to_string(),
				states: StateSet::empty(),
			},))
			.unwrap();
		assert_eq!(msg.body_signature().unwrap(), CACHE_ADD_SIGNATURE);
		atspi.connection().send_message(msg).await.unwrap();
		let to = timeout(Duration::from_secs(1), events.next());
		match to.await {
			Ok(Some(Ok(Event::Cache(CacheEvents::Add(event))))) => {
				assert_eq!(event.path().unwrap(), "/org/a11y/atspi/accessible/null");
				let cache_item = event.item();
				assert_eq!(cache_item.object.1.as_str(), "/org/a11y/atspi/accessible/object");
				assert_eq!(cache_item.parent.1.as_str(), "/org/a11y/atspi/accessible/parent");
				assert_eq!(cache_item.app.1.as_str(), "/org/a11y/atspi/accessible/application");
			},
			Ok(Some(Ok(another_event))) => {
				println!("{:?}", another_event);
				panic!("The wrong event was sent");
			},
			Ok(Some(Err(e))) => {
				println!("{:?}", e);
				panic!("An error occured destructuring the body");
			}
			Ok(e) => {
				println!("{:?}", e);
				panic!("Something else happened");
			}
			Err(e) => {
				panic!("An error occured: {:?}", e);
			}
		}
	}
}
