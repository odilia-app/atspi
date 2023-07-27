pub mod document;
pub mod focus;
pub mod keyboard;
pub mod mouse;
pub mod object;
pub mod terminal;
pub mod window;

// Unmarshalled event body signatures: These outline the event specific deserialized event types.
// Safety: These are evaluated at compile time.
// ----
// The signal signature "(so)" (an Accessible) is ambiguous, because it is used in:
// -  Cache : RemoveAccessible
// -  Socket: Available  *( signals the availability of the `Registry` daemon.)
//
// ATSPI- and QSPI both describe the generic events. These can be converted into
// specific signal types with TryFrom implementations. See crate::[`identify`]
//  EVENT_LISTENER_SIGNATURE is a type signature used to notify when events are registered or deregistered.
//  CACHE_ADD_SIGNATURE and *_REMOVE have very different types
const ATSPI_EVENT_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(siiva{sv})");
const QSPI_EVENT_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(siiv(so))");
const ACCESSIBLE_PAIR_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(so)");
const EVENT_LISTENER_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(ss)");
const CACHE_ADD_SIGNATURE: Signature<'_> =
	Signature::from_static_str_unchecked("((so)(so)(so)iiassusau)");

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zbus::{MessageField, MessageFieldCode};
use zbus_names::{OwnedUniqueName, UniqueName};
use zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Signature, Type, Value};

use crate::{
	accessible::Accessible,
	cache::{CacheItem, LegacyCacheItem},
	events::{
		document::DocumentEvents, focus::FocusEvents, keyboard::KeyboardEvents, mouse::MouseEvents,
		object::ObjectEvents, terminal::TerminalEvents, window::WindowEvents,
	},
	AtspiError,
};
//use atspi_macros::try_from_zbus_message;

#[must_use]
pub fn signatures_are_eq(lhs: &Signature, rhs: &Signature) -> bool {
	fn has_outer_parentheses(bytes: &[u8]) -> bool {
		bytes.starts_with(&[b'('])
			&& bytes.ends_with(&[b')'])
			&& (bytes[1..bytes.len() - 1].iter().fold(0, |count, byte| match byte {
				b'(' => count + 1,
				b')' if count > 0 => count - 1,
				_ => count,
			}) == 0)
	}

	let bytes = lhs.as_bytes();
	let lhs_sig_has_outer_parens = has_outer_parentheses(bytes);

	let bytes = rhs.as_bytes();
	let rhs_sig_has_outer_parens = has_outer_parentheses(bytes);

	match (lhs_sig_has_outer_parens, rhs_sig_has_outer_parens) {
		(true, false) => lhs.slice(1..lhs.len() - 1).as_bytes() == rhs.as_bytes(),
		(false, true) => lhs.as_bytes() == rhs.slice(1..rhs.len() - 1).as_bytes(),
		_ => lhs.as_bytes() == rhs.as_bytes(),
	}
}

/// A borrowed body for events.
#[derive(Debug, Serialize, Deserialize)]
pub struct EventBody<'a, T> {
	/// A generic "kind" type, defined by AT-SPI:
	/// usually a `&'a str`, but can be another type like [`State`].
	#[serde(rename = "type")]
	pub kind: T,
	/// Generic first detail defined by AT-SPI.
	pub detail1: i32,
	/// Generic second detail defined by AT-SPI.
	pub detail2: i32,
	/// Generic "any_data" field defined in AT-SPI.
	/// Can contain any type.
	#[serde(borrow)]
	pub any_data: Value<'a>,
	/// Map of string to an any type.
	/// This is not used for anything, but it is defined by AT-SPI.
	#[serde(borrow)]
	pub properties: HashMap<&'a str, Value<'a>>,
}

impl<T> Type for EventBody<'_, T> {
	fn signature() -> Signature<'static> {
		<(&str, i32, i32, Value, HashMap<&str, Value>)>::signature()
	}
}

/// Qt event body, which is not the same as other GUI frameworks.
/// Signature:  "siiv(so)"
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct EventBodyQT {
	/// kind variant, used for specifying an event tripple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	#[serde(rename = "type")]
	pub kind: String,
	/// Generic detail1 value descibed by AT-SPI.
	pub detail1: i32,
	/// Generic detail2 value descibed by AT-SPI.
	pub detail2: i32,
	/// Generic any_data value descibed by AT-SPI.
	/// This can be any type.
	pub any_data: OwnedValue,
	/// A tuple of properties.
	/// Not in use.
	pub properties: Accessible,
}

impl Default for EventBodyQT {
	fn default() -> Self {
		Self {
			kind: String::new(),
			detail1: 0,
			detail2: 0,
			any_data: Value::U8(0u8).into(),
			properties: Accessible::default(),
		}
	}
}

/// Standard event body (GTK, `egui`, etc.)
/// NOTE: Qt has its own signature: [`EventBodyQT`].
/// Signature `(siiva{sv})`,
#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq)]
pub struct EventBodyOwned {
	/// kind variant, used for specifying an event tripple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	#[serde(rename = "type")]
	pub kind: String,
	/// Generic detail1 value descibed by AT-SPI.
	pub detail1: i32,
	/// Generic detail2 value descibed by AT-SPI.
	pub detail2: i32,
	/// Generic any_data value descibed by AT-SPI.
	/// This can be any type.
	pub any_data: OwnedValue,
	/// A map of properties.
	/// Not in use.
	pub properties: HashMap<String, OwnedValue>,
}

impl From<EventBodyQT> for EventBodyOwned {
	fn from(body: EventBodyQT) -> Self {
		let accessible = Accessible { name: body.properties.name, path: body.properties.path };
		let mut props = HashMap::new();
		props.insert(accessible.name, Value::ObjectPath(accessible.path.into()).to_owned());
		Self {
			kind: body.kind,
			detail1: body.detail1,
			detail2: body.detail2,
			any_data: body.any_data,
			properties: props,
		}
	}
}

impl Default for EventBodyOwned {
	fn default() -> Self {
		Self {
			kind: String::new(),
			detail1: 0,
			detail2: 0,
			any_data: Value::U8(0u8).into(),
			properties: HashMap::new(),
		}
	}
}

/// Encapsulates the various different accessibility bus signal types.
///
/// Assumes being non exhaustive to allow for future- or custom signals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Event {
	/// See: [`DocumentEvents`].
	Document(DocumentEvents),
	/// See: [`FocusEvents`].
	Focus(FocusEvents),
	/// See: [`KeyboardEvents`].
	Keyboard(KeyboardEvents),
	/// See: [`MouseEvents`].
	Mouse(MouseEvents),
	/// See: [`ObjectEvents`].
	Object(ObjectEvents),
	/// See: [`TerminalEvents`].
	Terminal(TerminalEvents),
	/// See: [`WindowEvents`].
	Window(WindowEvents),
	/// See: [`AvailableEvent`].
	Available(AvailableEvent),
	/// See: [`CacheEvents`].
	Cache(CacheEvents),
	/// See: [`EventListenerEvents`].
	Listener(EventListenerEvents),
}

/// All events related to the `org.a11y.atspi.Cache` interface.
/// Note that these are not telling the client that an item *has been added* to a cache.
/// It is telling the client "here is a bunch of information to store it in your cache".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum CacheEvents {
	/// See: [`AddAccessibleEvent`].
	Add(AddAccessibleEvent),
	/// See: [`LegacyAddAccessibleEvent`].
	LegacyAdd(LegacyAddAccessibleEvent),
	/// See: [`RemoveAccessibleEvent`].
	Remove(RemoveAccessibleEvent),
}

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::LegacyCacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct LegacyAddAccessibleEvent {
	/// The [`Accessible`] the event applies to.
	pub item: Accessible,
	/// A cache item to add to the internal cache.
	pub node_added: LegacyCacheItem,
}
impl_event_conversions!(
	LegacyAddAccessibleEvent,
	CacheEvents,
	CacheEvents::LegacyAdd,
	Event::Cache
);
event_test_cases!(LegacyAddAccessibleEvent);
impl_from_dbus_message!(LegacyAddAccessibleEvent);
impl_to_dbus_message!(LegacyAddAccessibleEvent);

impl GenericEvent<'_> for LegacyAddAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Add";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'";
	const DBUS_MEMBER: &'static str = "AddAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";

	type Body = LegacyCacheItem;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, node_added: body })
	}

	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		self.node_added.clone()
	}
}

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::CacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct AddAccessibleEvent {
	/// The [`Accessible`] the event applies to.
	pub item: Accessible,
	/// A cache item to add to the internal cache.
	pub node_added: CacheItem,
}
impl_event_conversions!(AddAccessibleEvent, CacheEvents, CacheEvents::Add, Event::Cache);
event_test_cases!(AddAccessibleEvent);

impl GenericEvent<'_> for AddAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Add";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'";
	const DBUS_MEMBER: &'static str = "AddAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";

	type Body = CacheItem;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, node_added: body })
	}

	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		self.node_added.clone()
	}
}
impl<'a, T: GenericEvent<'a>> HasMatchRule for T {
	const MATCH_RULE_STRING: &'static str = <T as GenericEvent>::MATCH_RULE_STRING;
}
impl<'a, T: GenericEvent<'a>> HasRegistryEventString for T {
	const REGISTRY_EVENT_STRING: &'static str = <T as GenericEvent>::REGISTRY_EVENT_STRING;
}
impl_from_dbus_message!(AddAccessibleEvent);
impl_to_dbus_message!(AddAccessibleEvent);

/// `Cache::RemoveAccessible` signal event type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct RemoveAccessibleEvent {
	/// The application that emitted the signal TODO Check Me
	/// The [`Accessible`] the event applies to.
	pub item: Accessible,
	/// The node that was removed from the application tree  TODO Check Me
	pub node_removed: Accessible,
}
impl_event_conversions!(RemoveAccessibleEvent, CacheEvents, CacheEvents::Remove, Event::Cache);
event_test_cases!(RemoveAccessibleEvent);
impl GenericEvent<'_> for RemoveAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Remove";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='RemoveAccessible'";
	const DBUS_MEMBER: &'static str = "RemoveAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";

	type Body = Accessible;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, node_removed: body })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		self.node_removed.clone()
	}
}

impl_from_dbus_message!(RemoveAccessibleEvent);
impl_to_dbus_message!(RemoveAccessibleEvent);

#[cfg(test)]
pub mod accessible_deserialization_tests {
	use crate::events::Accessible;
	use zvariant::Value;

	#[test]
	fn try_into_value() {
		let acc = Accessible::default();
		let value_struct = Value::try_from(acc).expect("Unable to convert into a zvariant::Value");
		let Value::Structure(structure) = value_struct else {
			panic!("Unable to destructure a structure out of the Value.");
		};
		let vals = structure.into_fields();
		assert_eq!(vals.len(), 2);
		let Value::Str(bus_name) = vals.get(0).unwrap() else {
			panic!("Unable to destructure field value: {:?}", vals.get(0).unwrap());
		};
		assert_eq!(bus_name, ":0.0");
		let Value::ObjectPath(path) = vals.get(1).unwrap() else {
			panic!("Unable to destructure field value: {:?}", vals.get(1).unwrap());
		};
		assert_eq!(path.as_str(), "/org/a11y/atspi/accessible/null");
	}
	#[test]
	fn try_from_value() {}
}

#[cfg(test)]
pub mod accessible_tests {
	use super::Accessible;

	#[test]
	fn test_accessible_default_doesnt_panic() {
		let acc = Accessible::default();
		assert_eq!(acc.name.as_str(), ":0.0");
		assert_eq!(acc.path.as_str(), "/org/a11y/atspi/accessible/null");
	}
}
#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for Accessible {
	type Error = AtspiError;
	fn try_from(message: &zbus::Message) -> Result<Self, Self::Error> {
		let path = message.path().expect("returned path is either Some or panics");
		let owned_path = OwnedObjectPath::try_from(path)?;
		let fields = message.fields()?;
		let sender = fields.get_field(MessageFieldCode::Sender);
		let sender = sender
			.expect("We get the sender field from a valid MessageFieldCode, so it should be there");

		let MessageField::Sender(unique_name) = sender else {
			return Err(AtspiError::Conversion("Unable to convert zbus::Message to Accessible"));
		};
		let name_string = unique_name.as_str().to_owned();

		Ok(Accessible { name: name_string, path: owned_path })
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for EventBodyOwned {
	type Error = AtspiError;

	fn try_from(message: &zbus::Message) -> Result<Self, Self::Error> {
		let signature = message.body_signature()?;
		if signatures_are_eq(&signature, &QSPI_EVENT_SIGNATURE) {
			Ok(EventBodyOwned::from(message.body::<EventBodyQT>()?))
		} else if signatures_are_eq(&signature, &ATSPI_EVENT_SIGNATURE) {
			Ok(message.body::<EventBodyOwned>()?)
		} else {
			Err(AtspiError::Conversion(
				"Unable to convert from zbus::Message to EventBodyQT or EventBodyOwned",
			))
		}
	}
}

/// Signal type emitted by `EventListenerRegistered` and `EventListenerDeregistered` signals,
/// which belong to the `Registry` interface, implemented by the registry-daemon.
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct EventListeners {
	pub bus_name: OwnedUniqueName,
	pub path: String,
}
impl Default for EventListeners {
	fn default() -> Self {
		Self {
			bus_name: UniqueName::try_from(":0.0").unwrap().into(),
			path: "/org/a11y/atspi/accessible/null".to_string(),
		}
	}
}
#[test]
fn test_event_listener_default_no_panic() {
	let el = EventListeners::default();
	assert_eq!(el.bus_name.as_str(), ":0.0");
	assert_eq!(el.path.as_str(), "/org/a11y/atspi/accessible/null");
}

#[test]
fn test_event_listener_signature() {
	assert_eq_signatures!(&EventListeners::signature(), &EVENT_LISTENER_SIGNATURE);
}

/// Covers both `EventListener` events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum EventListenerEvents {
	/// See: [`EventListenerRegisteredEvent`].
	Registered(EventListenerRegisteredEvent),
	/// See: [`EventListenerDeregisteredEvent`].
	Deregistered(EventListenerDeregisteredEvent),
}

/// An event that is emitted by the regostry daemon to signal that an event has been deregistered
/// to no longer listen for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct EventListenerDeregisteredEvent {
	/// The [`Accessible`] the event applies to.
	pub item: Accessible,
	/// A list of events that have been deregistered via the registry interface.
	/// See `atspi-connection`.
	pub deregistered_event: EventListeners,
}
impl_event_conversions!(
	EventListenerDeregisteredEvent,
	EventListenerEvents,
	EventListenerEvents::Deregistered,
	Event::Listener
);
event_test_cases!(EventListenerDeregisteredEvent);
impl GenericEvent<'_> for EventListenerDeregisteredEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Registry:EventListenerDeregistered";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerDeregistered'";
	const DBUS_MEMBER: &'static str = "EventListenerDeregistered";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Registry";

	type Body = EventListeners;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, deregistered_event: body })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		self.deregistered_event.clone()
	}
}
impl_from_dbus_message!(EventListenerDeregisteredEvent);
impl_to_dbus_message!(EventListenerDeregisteredEvent);

/// An event that is emitted by the regostry daemon to signal that an event has been registered to listen for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct EventListenerRegisteredEvent {
	/// The [`Accessible`] the event applies to.
	pub item: Accessible,
	/// A list of events that have been registered via the registry interface.
	/// See `atspi-connection`.
	pub registered_event: EventListeners,
}
impl_event_conversions!(
	EventListenerRegisteredEvent,
	EventListenerEvents,
	EventListenerEvents::Registered,
	Event::Listener
);
event_test_cases!(EventListenerRegisteredEvent);
impl GenericEvent<'_> for EventListenerRegisteredEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Registry:EventListenerRegistered";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerRegistered'";
	const DBUS_MEMBER: &'static str = "EventListenerRegistered";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Registry";

	type Body = EventListeners;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, registered_event: body })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		self.registered_event.clone()
	}
}
impl_from_dbus_message!(EventListenerRegisteredEvent);
impl_to_dbus_message!(EventListenerRegisteredEvent);

/// An event that is emitted when the registry daemon has started.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Eq, Hash)]
pub struct AvailableEvent {
	/// The [`Accessible`] the event applies to.
	pub item: Accessible,
	pub socket: Accessible,
}
impl From<AvailableEvent> for Event {
	fn from(ev: AvailableEvent) -> Event {
		Event::Available(ev)
	}
}
impl TryFrom<Event> for AvailableEvent {
	type Error = AtspiError;
	fn try_from(generic_event: Event) -> Result<AvailableEvent, Self::Error> {
		if let Event::Available(specific_event) = generic_event {
			Ok(specific_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}
event_test_cases!(AvailableEvent);
impl GenericEvent<'_> for AvailableEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Socket:Available";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Socket',member='Available'";
	const DBUS_MEMBER: &'static str = "Available";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Socket";

	type Body = Accessible;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, socket: body })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		self.socket.clone()
	}
}
impl_from_dbus_message!(AvailableEvent);
impl_to_dbus_message!(AvailableEvent);

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for Event {
	type Error = AtspiError;

	fn try_from(msg: &zbus::Message) -> Result<Event, AtspiError> {
		let body_signature = msg.body_signature()?;
		let body_signature = body_signature.as_str();
		let signal_member = msg.member().ok_or(AtspiError::MissingMember)?;
		let member_str = signal_member.as_str();

		// As we are matching against `body_signature()`, which yields the marshalled D-Bus signatures.
		// Therefore no outer parentheses.
		match body_signature {
			// Marshalled Accessible signature
			"so" | "(so)" => match member_str {
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
			"siiva{sv}" | "siiv(so)" | "(siiva{sv})" | "(siiv(so))" => {
				let Some(interface) = msg.interface() else {
					return Err(AtspiError::MissingInterface);
				};
				match interface.as_str() {
					"org.a11y.atspi.Event.Document" => {
						Ok(Event::Document(DocumentEvents::try_from(msg)?))
					}
					"org.a11y.atspi.Event.Focus" => Ok(Event::Focus(FocusEvents::try_from(msg)?)),
					"org.a11y.atspi.Event.Keyboard" => {
						Ok(Event::Keyboard(KeyboardEvents::try_from(msg)?))
					}
					"org.a11y.atspi.Event.Mouse" => Ok(Event::Mouse(MouseEvents::try_from(msg)?)),
					"org.a11y.atspi.Event.Object" => {
						Ok(Event::Object(ObjectEvents::try_from(msg)?))
					}
					"org.a11y.atspi.Event.Terminal" => {
						Ok(Event::Terminal(TerminalEvents::try_from(msg)?))
					}
					"org.a11y.atspi.Event.Window" => {
						Ok(Event::Window(WindowEvents::try_from(msg)?))
					}
					_ => Err(AtspiError::UnknownInterface),
				}
			}
			"ss" | "(ss)" => {
				if let Ok(ev) = EventListenerRegisteredEvent::try_from(msg) {
					return Ok(Event::Listener(EventListenerEvents::Registered(ev)));
				}
				if let Ok(ev) = EventListenerDeregisteredEvent::try_from(msg) {
					return Ok(Event::Listener(EventListenerEvents::Deregistered(ev)));
				}
				Err(AtspiError::UnknownSignal)
			}
			// Marshalled `AddAccessible` signature
			"(so)(so)(so)iiassusau" | "((so)(so)(so)iiassusau)" => {
				let ev = AddAccessibleEvent::try_from(msg)?;
				Ok(Event::Cache(CacheEvents::Add(ev)))
			}
			// LegacyCacheAdd signature
			"(so)(so)(so)a(so)assusau" | "((so)(so)(so)a(so)assusau)" => {
				let ev = LegacyAddAccessibleEvent::try_from(msg)?;
				Ok(Event::Cache(CacheEvents::LegacyAdd(ev)))
			}
			sig => Err(AtspiError::UnknownBusSignature(sig.to_string())),
		}
	}
}

/// Shared behavior of bus `Signal` events.
pub trait GenericEvent<'a> {
	/// The DBus member for the event.
	/// For example, for an [`object::TextChangedEvent`] this should be "TextChanged"
	const DBUS_MEMBER: &'static str;
	/// The DBus interface name for this event.
	/// For example, for any event within [`object`], this should be "org.a11y.atspi.Event.Object".
	const DBUS_INTERFACE: &'static str;
	/// A static match rule string for DBus.
	/// This should usually be a string that looks like this: "type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'";
	/// This should be deprecated in favour of composing the string from [`Self::DBUS_MEMBER`] and [`Self::DBUS_INTERFACE`].
	const MATCH_RULE_STRING: &'static str;
	/// A registry event string for registering for event receiving via the `RegistryProxy`.
	/// This should be deprecated in favour of composing the string from [`Self::DBUS_MEMBER`] and [`Self::DBUS_INTERFACE`].
	const REGISTRY_EVENT_STRING: &'static str;

	/// What is the body type of this event.
	type Body: Type + Serialize + Deserialize<'a>;

	/// Build the event from the object pair (Accessible and the Body).
	///
	/// # Errors
	///
	/// When the body type, which is what the raw message looks like over `DBus`, does not match the type that is expected for the given event.
	/// It is not possible for this to error on most events, but on events whose raw message [`Self::Body`] type contains a [`enum@zvariant::Value`], you may get errors when constructing the structure.
	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError>
	where
		Self: Sized;

	/// Path of the signalling object.
	fn path(&self) -> ObjectPath<'_>;

	/// Sender of the signal.
	///
	/// ### Errors
	/// - when deserializing the header failed, or
	/// - When `zbus::get_field!` finds that 'sender' is an invalid field.
	fn sender(&self) -> String;

	/// The body of the object.
	fn body(&self) -> Self::Body;
}

/// A specific trait *only* to define match rules.
pub trait HasMatchRule {
	/// A static match rule string for DBus.
	/// This should usually be a string that looks like this: "type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'";
	/// This should be deprecated in favour of composing the string from [`Self::DBUS_MEMBER`] and [`Self::DBUS_INTERFACE`].
	const MATCH_RULE_STRING: &'static str;
}

/// A specific trait *only* to define registry event matches.
pub trait HasRegistryEventString {
	/// A registry event string for registering for event receiving via the `RegistryProxy`.
	/// This should be deprecated in favour of composing the string from [`Self::DBUS_MEMBER`] and [`Self::DBUS_INTERFACE`].
	const REGISTRY_EVENT_STRING: &'static str;
}

#[cfg(test)]
mod tests {
	use atspi_common::events::{
		AddAccessibleEvent, CacheEvents, Event, EventBodyOwned, EventBodyQT, RemoveAccessibleEvent,
		ATSPI_EVENT_SIGNATURE, CACHE_ADD_SIGNATURE, QSPI_EVENT_SIGNATURE,
	};
	use atspi_common::{
		accessible::ACCESSIBLE_PAIR_SIGNATURE, Accessible, CacheItem, InterfaceSet, Role, StateSet,
	};
	use atspi_connection::AccessibilityConnection;
	use std::{collections::HashMap, time::Duration};
	use tokio_stream::StreamExt;
	use zbus::MessageBuilder;
	use zvariant::{ObjectPath, OwnedObjectPath, Signature, Type};

	use super::signatures_are_eq;

	#[test]
	fn check_event_body_qt_signature() {
		assert_eq_signatures!(&<EventBodyQT as Type>::signature(), &QSPI_EVENT_SIGNATURE);
	}

	#[test]
	fn check_event_body_signature() {
		assert_eq_signatures!(&<EventBodyOwned as Type>::signature(), &ATSPI_EVENT_SIGNATURE);
	}

	#[test]
	fn test_event_body_qt_to_event_body_owned_conversion() {
		let event_body: EventBodyOwned = EventBodyQT::default().into();

		let accessible = crate::Accessible::default();
		let name = accessible.name;
		let path = accessible.path;
		let props = HashMap::from([(name, ObjectPath::try_from(path).unwrap().into())]);
		assert_eq!(event_body.properties, props);
	}

	// `assert_eq_signatures!` and `signatures_are_eq` are helpers to deal with the difference
	// in `Signatures` as consequence of marshalling. While `zvariant` is very lenient with respect
	// to outer parentheses, these helpers only take one marshalling step into account.
	#[test]
	fn test_signatures_are_equal_macro_and_fn() {
		let with_parentheses = &Signature::from_static_str_unchecked("(ii)");
		let without_parentheses = &Signature::from_static_str_unchecked("ii");
		assert_eq_signatures!(with_parentheses, without_parentheses);
		assert!(signatures_are_eq(with_parentheses, without_parentheses));
		// test against themselves
		assert!(signatures_are_eq(with_parentheses, with_parentheses));
		assert!(signatures_are_eq(without_parentheses, without_parentheses));
		assert!(signatures_are_eq(with_parentheses, with_parentheses));
		assert!(signatures_are_eq(without_parentheses, without_parentheses));
		let with_parentheses = &Signature::from_static_str_unchecked("(ii)(ii)");
		let without_parentheses = &Signature::from_static_str_unchecked("((ii)(ii))");
		assert_eq_signatures!(with_parentheses, without_parentheses);
		assert!(signatures_are_eq(with_parentheses, without_parentheses));
		// test against themselves
		assert!(signatures_are_eq(with_parentheses, with_parentheses));
		assert!(signatures_are_eq(without_parentheses, without_parentheses));
		assert_eq_signatures!(with_parentheses, with_parentheses);
		assert_eq_signatures!(without_parentheses, without_parentheses);
		// test false cases with unbalanced parentheses
		let with_parentheses = &Signature::from_static_str_unchecked("(ii)(ii)");
		let without_parentheses = &Signature::from_static_str_unchecked("((ii)(ii)");
		assert!(!signatures_are_eq(with_parentheses, without_parentheses));
		// test case with more than oune extra outer parentheses
		let with_parentheses = &Signature::from_static_str_unchecked("((ii)(ii))");
		let without_parentheses = &Signature::from_static_str_unchecked("((((ii)(ii))))");
		assert!(!signatures_are_eq(with_parentheses, without_parentheses));
	}

	#[tokio::test]
	async fn test_recv_remove_accessible() {
		let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();

		atspi.register_event::<RemoveAccessibleEvent>().await.unwrap();

		let events = tokio_stream::StreamExt::timeout(atspi.event_stream(), Duration::from_secs(1));
		tokio::pin!(events);

		let msg: zbus::Message = {
			let path = "/org/a11y/atspi/accessible/null";
			let iface = "org.a11y.atspi.Cache";
			let member = "RemoveAccessible";

			let unique_bus_name = atspi.connection().unique_name().unwrap();
			let remove_body = Accessible {
				name: ":69.420".into(),
				path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/remove").unwrap(),
			};

			MessageBuilder::signal(path, iface, member)
				.expect("Could not create signal")
				.sender(unique_bus_name.clone())
				.expect("Could not set sender to {unique_bus_name:?}")
				.build(&remove_body)
				.unwrap()
		};

		assert_eq_signatures!(&msg.body_signature().unwrap(), &ACCESSIBLE_PAIR_SIGNATURE);
		atspi.connection().send_message(msg).await.unwrap();

		loop {
			let to = events.try_next().await;
			assert!(to.is_ok(), "Stream timed out");
			let opt = to.unwrap();

			match opt {
				Some(res) => {
					match res {
						Ok(event) => match event {
							Event::Cache(CacheEvents::Remove(event)) => {
								let removed_accessible = event.node_removed;
								assert_eq!(
									removed_accessible.path.as_str(),
									"/org/a11y/atspi/accessible/remove"
								);
								break;
							}
							_ => continue,
						},
						// Stream yields a Some(Err(Error)) when a message is received
						Err(e) => panic!("Error: conversion to Event failed {e:?}"),
					}
				}
				// Stream yields a None when the stream is closed
				None => panic!("Stream closed"),
			}
		}
	}

	#[tokio::test]
	async fn test_recv_add_accessible() {
		let atspi = AccessibilityConnection::open().await.unwrap();
		atspi.register_event::<AddAccessibleEvent>().await.unwrap();

		let events = tokio_stream::StreamExt::timeout(atspi.event_stream(), Duration::from_secs(1));
		tokio::pin!(events);

		let msg: zbus::Message = {
			let path = "/org/a11y/atspi/accessible/null";
			let iface = "org.a11y.atspi.Cache";
			let member = "AddAccessible";

			let unique_bus_name = atspi.connection().unique_name().unwrap();

			let add_body = CacheItem {
				object: Accessible {
					name: ":1.1".to_string(),
					path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/object").unwrap(),
				},
				app: Accessible {
					name: ":1.1".to_string(),
					path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/application")
						.unwrap(),
				},
				parent: Accessible {
					name: ":1.1".to_string(),
					path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/parent").unwrap(),
				},
				index: 0,
				children: 0,
				ifaces: InterfaceSet::empty(),
				short_name: String::new(),
				role: Role::Application,
				name: "Hi".to_string(),
				states: StateSet::empty(),
			};

			MessageBuilder::signal(path, iface, member)
				.expect("Could not create signal")
				.sender(unique_bus_name.clone())
				.expect("Could not set sender to {unique_bus_name:?}")
				.build(&add_body)
				.unwrap()
		};

		assert_eq_signatures!(
			&msg.body_signature()
				.expect("marshalled AddAccessible body signature != expected"),
			&CACHE_ADD_SIGNATURE
		);
		atspi
			.connection()
			.send_message(msg)
			.await
			.expect("Message sending unsuccesful");

		loop {
			let to = events.try_next().await;
			assert!(to.is_ok(), "Stream timed out");
			let opt = to.unwrap();

			match opt {
				Some(res) => {
					// This result comes from inner event-stream, Stream yields a Result<Event, AtspiError>
					match res {
						Ok(event) => match event {
							Event::Cache(CacheEvents::Add(AddAccessibleEvent {
								item: _,
								node_added: cache_item,
							})) => {
								assert_eq!(
									cache_item.object.path.as_str(),
									"/org/a11y/atspi/accessible/object"
								);
								assert_eq!(
									cache_item.app.path.as_str(),
									"/org/a11y/atspi/accessible/application"
								);
								assert_eq!(
									cache_item.parent.path.as_str(),
									"/org/a11y/atspi/accessible/parent"
								);
								break;
							}
							_any_other_event => continue,
						},
						Err(e) => panic!("Error: conversion to Event failed {e:?}"),
					}
				}
				// Stream yields a None when the stream is closed
				None => panic!("Stream closed"),
			}
		}
	}
}
