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
// -  Socket: Available  *( signals the availability of the `Registry` daeomon.)
//
// ATSPI- and QSPI both describe the generic events. These can be converted into
// specific signal types with TryFrom implementations. See crate::[`identify`]
//  EVENT_LISTENER_SIGNATURE is a type signature used to notify when events are registered or deregistered.
//  CACHE_ADD_SIGNATURE and *_REMOVE have very different types
pub const ATSPI_EVENT_SIGNATURE: Signature<'_> =
	Signature::from_static_str_unchecked("(siiva{sv})");
pub const QSPI_EVENT_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(siiv(so))");
pub const ACCESSIBLE_PAIR_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(so)");
pub const EVENT_LISTENER_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(ss)");
pub const CACHE_ADD_SIGNATURE: Signature<'_> =
	Signature::from_static_str_unchecked("((so)(so)(so)iiassusau)");

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zbus_names::{OwnedUniqueName, UniqueName};
use zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Signature, Type, Value};

use crate::{
	cache::CacheItem,
	error::ObjectPathConversionError,
	events::{
		document::DocumentEvents, focus::FocusEvents, keyboard::KeyboardEvents, mouse::MouseEvents,
		object::ObjectEvents, terminal::TerminalEvents, window::WindowEvents,
	},
	AtspiError,
};
//use atspi_macros::try_from_zbus_message;

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
#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Event {
	Document(DocumentEvents),
	Focus(FocusEvents),
	Keyboard(KeyboardEvents),
	Mouse(MouseEvents),
	Object(ObjectEvents),
	Terminal(TerminalEvents),
	Window(WindowEvents),
	/// Emitted when the ` Registry` interface on `org.a11y.atspi.Registry` becomes available.
	Available(AvailableEvent),
	/// Both `CacheAdd` and `CacheRemove` signals
	Cache(CacheEvents),
	/// Emitted on registration or de-registration of event listeners.
	///
	/// (eg. "Cache:AddAccessible:")
	Listener(EventListenerEvents),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum CacheEvents {
	Add(AddAccessibleEvent),
	Remove(RemoveAccessibleEvent),
}

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::CacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct AddAccessibleEvent {
	pub item: Accessible,
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

	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
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
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
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

// TODO: Try to make borrowed versions work,
// check where the lifetimes of the borrow are tied to, see also: comment on `interface()` method
// in `DefaultEvent` impl
// then rename into Owned for this one.
/// Owned Accessible type
/// Emitted by `CacheRemove` and `Available`
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct Accessible {
	pub name: OwnedUniqueName,
	pub path: OwnedObjectPath,
}
impl TryFrom<zvariant::OwnedValue> for Accessible {
	type Error = AtspiError;
	fn try_from<'a>(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
		match &*value {
			zvariant::Value::Structure(s) => {
				if s.signature() != ACCESSIBLE_PAIR_SIGNATURE {
					return Err(zvariant::Error::SignatureMismatch(s.signature(), format!("To turn a zvariant::Value into an atspi::Accessible, it must be of type {}", ACCESSIBLE_PAIR_SIGNATURE.as_str())).into());
				}
				let fields = s.fields();
				let name_value: String =
					fields.get(0).ok_or(zvariant::Error::IncorrectType)?.try_into()?;
				let path_value: ObjectPath<'_> =
					fields.get(1).ok_or(zvariant::Error::IncorrectType)?.try_into()?;
				let name = UniqueName::try_from(name_value)?.into();
				Ok(Accessible { name, path: path_value.into() })
			}
			_ => Err(zvariant::Error::IncorrectType.into()),
		}
	}
}
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

impl From<Accessible> for zvariant::Structure<'_> {
	fn from(accessible: Accessible) -> Self {
		(accessible.name.as_str().to_string(), accessible.path).into()
	}
}
impl Default for Accessible {
	fn default() -> Self {
		Accessible {
			name: UniqueName::from_static_str(":0.0").unwrap().into(),
			path: ObjectPath::from_static_str("/org/a11y/atspi/accessible/null")
				.unwrap()
				.into(),
		}
	}
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
		Ok(Accessible {
			name: message
				.header()?
				.sender()?
				.ok_or(ObjectPathConversionError::NoIdAvailable)?
				.to_owned()
				.into(),
			path: message.path().ok_or(ObjectPathConversionError::NoIdAvailable)?.into(),
		})
	}
}

#[test]
fn test_accessible_signature() {
	assert_eq!(Accessible::signature(), "(so)");
}

#[cfg(feature = "zbus")]
impl TryFrom<zbus::Message> for EventBodyOwned {
	type Error = AtspiError;

	fn try_from(message: zbus::Message) -> Result<Self, Self::Error> {
		let signature = message.body_signature()?;
		if signature == QSPI_EVENT_SIGNATURE {
			Ok(EventBodyOwned::from(message.body::<EventBodyQT>()?))
		} else {
			Ok(message.body::<EventBodyOwned>()?)
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
	let _el = EventListeners::default();
	assert!(true);
}

#[test]
fn test_event_listener_signature() {
	assert_eq!(EventListeners::signature(), "(ss)");
}

/// Covers both `EventListener` events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum EventListenerEvents {
	Registered(EventListenerRegisteredEvent),
	Deregistered(EventListenerDeregisteredEvent),
}

/// An event that is emitted by the regostry daemon to signal that an event has been deregistered
/// to no longer listen for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct EventListenerDeregisteredEvent {
	pub item: Accessible,
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
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
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
	pub item: Accessible,
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
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
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
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
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
		let message_signature = body_signature.as_str();
		let signal_member = msg.member().ok_or(AtspiError::MissingMember)?;
		let message_member = signal_member.as_str();

		// As we are matching against `body_signature()`, which yields the marshalled D-Bus signatures.
		// Therefore no outer parentheses.
		match message_signature {
			// Marshalled Accessible signature
			"so" => match message_member {
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
				let Some(interface) = msg.interface() else {  return Err(AtspiError::MissingInterface);  };
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
			"ss" => {
				if let Ok(ev) = EventListenerRegisteredEvent::try_from(msg) {
					return Ok(Event::Listener(EventListenerEvents::Registered(ev)));
				}
				if let Ok(ev) = EventListenerDeregisteredEvent::try_from(msg) {
					return Ok(Event::Listener(EventListenerEvents::Deregistered(ev)));
				}
				Err(AtspiError::UnknownSignal)
			}
			// Marshalled `AddAccessible` signature
			"(so)(so)(so)iiassusau" => {
				let ev = AddAccessibleEvent::try_from(msg)?;
				Ok(Event::Cache(CacheEvents::Add(ev)))
			}
			_ => Err(AtspiError::UnknownBusSignature),
		}
	}
}

/// Shared behavior of bus `Signal` events.
pub trait GenericEvent<'a> {
	const DBUS_MEMBER: &'static str;
	const DBUS_INTERFACE: &'static str;
	const MATCH_RULE_STRING: &'static str;
	const REGISTRY_EVENT_STRING: &'static str;

	/// What is the body type of this event.
	type Body: Type + Serialize + Deserialize<'a>;

	/// Build the event from the object pair (Accessible and the Body).
	///
	/// # Errors
	///
	/// When the body type, which is what the raw message looks like over `DBus`, does not match the type that is expected for the given event.
	/// It is not possible for this to error on most events, but on events whoes raw message [`Self::Body`] type contains a [`enum@zvariant::Value`], you may get errors when constructing the structure.
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
	fn sender(&self) -> UniqueName<'_>;

	/// The body of the object.
	fn body(&self) -> Self::Body;
}

pub trait HasMatchRule {
	const MATCH_RULE_STRING: &'static str;
}

pub trait HasRegistryEventString {
	const REGISTRY_EVENT_STRING: &'static str;
}

#[cfg(test)]
mod tests {
	use atspi_common::events::{
		Accessible, AddAccessibleEvent, CacheEvents, Event, EventBodyOwned, EventBodyQT,
		RemoveAccessibleEvent, ACCESSIBLE_PAIR_SIGNATURE, ATSPI_EVENT_SIGNATURE,
		CACHE_ADD_SIGNATURE, QSPI_EVENT_SIGNATURE,
	};
	use atspi_common::{CacheItem, InterfaceSet, Role, StateSet};
	use atspi_connection::AccessibilityConnection;
	use std::{collections::HashMap, time::Duration};
	use tokio_stream::StreamExt;
	use zbus::MessageBuilder;
	use zbus_names::OwnedUniqueName;
	use zvariant::{ObjectPath, OwnedObjectPath, Type, Value};

	#[test]
	fn check_event_body_qt_signature() {
		assert_eq!(<EventBodyQT as Type>::signature(), QSPI_EVENT_SIGNATURE);
	}

	#[test]
	fn check_event_body_signature() {
		assert_eq!(<EventBodyOwned as Type>::signature(), ATSPI_EVENT_SIGNATURE);
	}

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

	fn has_outer_parentheses(bytes: &[u8]) -> bool {
		bytes.starts_with(&[b'('])
			&& bytes.ends_with(&[b')'])
			&& (bytes[1..bytes.len() - 1].iter().fold(0, |count, byte| match byte {
				b'(' => count + 1,
				b')' if count > 0 => count - 1,
				_ => count,
			}) == 0)
	}

	// This asserts that the signatures are equal, but ignores the outer parentheses as
	// the difference between marshalled and unmarshalled signatures is often just one set of outer parentheses.
	#[macro_export]
	macro_rules! assert_eq_signatures {
		($bus_signature:expr, $type_signature:expr) => {
			let lhs_sig: zvariant::Signature<'_> = $bus_signature;
			let rhs_sig: zvariant::Signature<'_> = $type_signature;

			match (
				has_outer_parentheses(lhs_sig.as_bytes()),
				has_outer_parentheses(rhs_sig.as_bytes()),
			) {
				(true, false) => {
					assert_eq!(lhs_sig.slice(1..lhs_sig.len() - 1).as_bytes(), rhs_sig.as_bytes());
				}
				(false, true) => {
					assert_eq!(lhs_sig.as_bytes(), rhs_sig.slice(1..rhs_sig.len() - 1).as_bytes());
				}
				_ => assert_eq!(lhs_sig.as_bytes(), rhs_sig.as_bytes()),
			}
		};
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
				name: OwnedUniqueName::try_from(":69.420").unwrap(),
				path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/remove").unwrap(),
			};

			MessageBuilder::signal(path, iface, member)
				.expect("Could not create signal")
				.sender(unique_bus_name.clone())
				.expect("Could not set sender to {unique_bus_name:?}")
				.build(&remove_body)
				.unwrap()
		};

		assert_eq_signatures!(msg.body_signature().unwrap(), ACCESSIBLE_PAIR_SIGNATURE);
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
							_some_other_event => continue,
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
				object: (
					":1.1".to_string(),
					OwnedObjectPath::try_from("/org/a11y/atspi/accessible/object").unwrap(),
				),
				app: (
					":1.1".to_string(),
					OwnedObjectPath::try_from("/org/a11y/atspi/accessible/application").unwrap(),
				),
				parent: (
					":1.1".to_string(),
					OwnedObjectPath::try_from("/org/a11y/atspi/accessible/parent").unwrap(),
				),
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

		assert_eq_signatures!(msg.body_signature().unwrap(), CACHE_ADD_SIGNATURE);
		atspi.connection().send_message(msg).await.unwrap();

		loop {
			let to = events.try_next().await;
			assert!(to.is_ok(), "Stream timed out");
			let opt = to.unwrap();

			match opt {
				Some(res) => {
					// This result comes from inner event-stream, Stream yields a Result<Event, AtspiError>
					match res {
						Ok(event) => match event {
							Event::Cache(CacheEvents::Add(add_accessible_event)) => {
								let cache_item = add_accessible_event.node_added;
								assert_eq!(
									cache_item.object.1.as_str(),
									"/org/a11y/atspi/accessible/object"
								);
								assert_eq!(
									cache_item.app.1.as_str(),
									"/org/a11y/atspi/accessible/application"
								);
								assert_eq!(
									cache_item.parent.1.as_str(),
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
