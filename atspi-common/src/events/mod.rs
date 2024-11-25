pub mod cache;
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
// Same as "(siiva{sv})"
pub const ATSPI_EVENT_SIGNATURE: &Signature = &Signature::static_structure(&[
	&Signature::Str,
	&Signature::I32,
	&Signature::I32,
	&Signature::Variant,
	&Signature::Dict {
		key: Child::Static { child: &Signature::Str },
		value: Child::Static { child: &Signature::Variant },
	},
]);
pub const EVENT_NAME_SIGNATURE: &Signature =
	&Signature::static_structure(&[&Signature::Str, &Signature::Str]);
// Same as "(siiv(so))"
pub const QSPI_EVENT_SIGNATURE: &Signature = &Signature::static_structure(&[
	&Signature::Str,
	&Signature::I32,
	&Signature::I32,
	&Signature::Variant,
	&Signature::Structure(Fields::Static { fields: &[&Signature::Str, &Signature::ObjectPath] }),
]);
// Same as "(so)"
pub const EVENT_LISTENER_SIGNATURE: &Signature =
	&Signature::static_structure(&[&Signature::Str, &Signature::ObjectPath]);
// Same as "((so)(so)(so)iiassusau)"
pub const CACHE_ADD_SIGNATURE: &Signature = &Signature::static_structure(&[
	&Signature::Structure(Fields::Static { fields: &[&Signature::Str, &Signature::ObjectPath] }),
	&Signature::Structure(Fields::Static { fields: &[&Signature::Str, &Signature::ObjectPath] }),
	&Signature::Structure(Fields::Static { fields: &[&Signature::Str, &Signature::ObjectPath] }),
	&Signature::I32,
	&Signature::I32,
	&Signature::Array(Child::Static { child: &Signature::Str }),
	&Signature::Str,
	&Signature::U32,
	&Signature::Str,
	&Signature::Array(Child::Static { child: &Signature::U32 }),
]);

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zbus_lockstep_macros::validate;
use zbus_names::{OwnedUniqueName, UniqueName};
#[cfg(feature = "zbus")]
use zvariant::OwnedObjectPath;
use zvariant::{
	signature::{Child, Fields},
	ObjectPath, OwnedValue, Signature, Type, Value,
};

pub use crate::events::{
	cache::CacheEvents, document::DocumentEvents, focus::FocusEvents, keyboard::KeyboardEvents,
	mouse::MouseEvents, object::ObjectEvents, terminal::TerminalEvents, window::WindowEvents,
};
use crate::{AtspiError, ObjectRef};

/// Qt event body, which is not the same as other GUI frameworks.
/// Signature:  "siiv(so)"
#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
pub struct EventBodyQT {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	// #[serde(rename = "type")]
	pub kind: String,
	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,
	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,
	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	pub any_data: OwnedValue,
	/// A tuple of properties.
	/// Not in use.
	pub properties: ObjectRef,
}
impl From<EventBodyOwned> for EventBodyQT {
	fn from(ev: EventBodyOwned) -> Self {
		EventBodyQT {
			kind: ev.kind,
			detail1: ev.detail1,
			detail2: ev.detail2,
			any_data: ev.any_data,
			properties: ObjectRef::default(),
		}
	}
}

impl Default for EventBodyQT {
	fn default() -> Self {
		Self {
			kind: String::new(),
			detail1: 0,
			detail2: 0,
			any_data: 0u8.into(),
			properties: ObjectRef::default(),
		}
	}
}

/// Standard event body (GTK, `egui`, etc.)
/// NOTE: Qt has its own signature: [`EventBodyQT`].
/// Signature `(siiva{sv})`,
#[validate(signal: "PropertyChange")]
#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
pub struct EventBodyOwned {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	#[serde(rename = "type")]
	pub kind: String,
	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,
	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,
	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	pub any_data: OwnedValue,
	/// A map of properties.
	/// Not in use.
	pub properties: HashMap<OwnedUniqueName, OwnedValue>,
}

impl From<EventBodyQT> for EventBodyOwned {
	fn from(body: EventBodyQT) -> Self {
		let mut props = HashMap::new();

		let name = body.properties.name;
		let path = body.properties.path;

		// We know `path` is a `OwnedObjectPath`, so the conversion to
		// `OwnedValue` is infallible at present.
		// Should this ever change, we need to know.
		let value = Value::ObjectPath(path.into()).try_to_owned().unwrap_or_else(|err| {
			panic!("Error occurred: {err:?}");
		});

		props.insert(name, value);
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
			any_data: 0u8.into(),
			properties: HashMap::new(),
		}
	}
}

/// Safety: This implementation of [`Clone`] *can panic!* Although the chance is extremely remote.
///
/// If:
/// 1. the `any_data` or `properties` field contain an [`std::os::fd::OwnedFd`] type, and
/// 2. the maximum number of open files for the process is exceeded.
///
/// Then, and only then, will this function panic.
/// None of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
/// Events on the AT-SPI bus *could, theoretically* send a file descriptor, but nothing in the
/// specification allows that.
///
/// See [`zvariant::Value::try_clone`] for more information.
impl Clone for EventBodyOwned {
	fn clone(&self) -> Self {
		let cloned_any_data = self.any_data.try_clone().unwrap_or_else(|err| {
			panic!("Failure cloning 'any_data' field: {err:?}");
		});

		let cloned_properties = {
			let mut map = HashMap::new();
			for (key, value) in &self.properties {
				let cloned_value = value.try_clone().unwrap_or_else(|err| {
					panic!("Failure cloning 'props' field: {err:?}");
				});
				map.insert(key.clone(), cloned_value);
			}
			map
		};

		Self {
			kind: self.kind.clone(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: cloned_any_data,
			properties: cloned_properties,
		}
	}
}

/// Encapsulates the various different accessibility bus signal types.
///
/// Assumes being non exhaustive to allow for future- or custom signals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Event<'a> {
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
	Fuck(&'a str),
}

impl EventTypeProperties for Event<'_> {
	fn member(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.member(),
			Self::Focus(inner) => inner.member(),
			Self::Keyboard(inner) => inner.member(),
			Self::Mouse(inner) => inner.member(),
			Self::Object(inner) => inner.member(),
			Self::Terminal(inner) => inner.member(),
			Self::Window(inner) => inner.member(),
			Self::Available(inner) => inner.member(),
			Self::Cache(inner) => inner.member(),
			Self::Listener(inner) => inner.member(),
			Self::Fuck(_) => panic!("FUCK!"),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.interface(),
			Self::Focus(inner) => inner.interface(),
			Self::Keyboard(inner) => inner.interface(),
			Self::Mouse(inner) => inner.interface(),
			Self::Object(inner) => inner.interface(),
			Self::Terminal(inner) => inner.interface(),
			Self::Window(inner) => inner.interface(),
			Self::Available(inner) => inner.interface(),
			Self::Cache(inner) => inner.interface(),
			Self::Listener(inner) => inner.interface(),
			Self::Fuck(_) => panic!("FUCK!"),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.match_rule(),
			Self::Focus(inner) => inner.match_rule(),
			Self::Keyboard(inner) => inner.match_rule(),
			Self::Mouse(inner) => inner.match_rule(),
			Self::Object(inner) => inner.match_rule(),
			Self::Terminal(inner) => inner.match_rule(),
			Self::Window(inner) => inner.match_rule(),
			Self::Available(inner) => inner.match_rule(),
			Self::Cache(inner) => inner.match_rule(),
			Self::Listener(inner) => inner.match_rule(),
			Self::Fuck(_) => panic!("FUCK!"),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.registry_string(),
			Self::Focus(inner) => inner.registry_string(),
			Self::Keyboard(inner) => inner.registry_string(),
			Self::Mouse(inner) => inner.registry_string(),
			Self::Object(inner) => inner.registry_string(),
			Self::Terminal(inner) => inner.registry_string(),
			Self::Window(inner) => inner.registry_string(),
			Self::Available(inner) => inner.registry_string(),
			Self::Cache(inner) => inner.registry_string(),
			Self::Listener(inner) => inner.registry_string(),
			Self::Fuck(_) => panic!("FUCK!"),
		}
	}
}

impl EventProperties for Event<'_> {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Document(inner) => inner.path(),
			Self::Focus(inner) => inner.path(),
			Self::Keyboard(inner) => inner.path(),
			Self::Mouse(inner) => inner.path(),
			Self::Object(inner) => inner.path(),
			Self::Terminal(inner) => inner.path(),
			Self::Window(inner) => inner.path(),
			Self::Available(inner) => inner.path(),
			Self::Cache(inner) => inner.path(),
			Self::Listener(inner) => inner.path(),
			Self::Fuck(_) => panic!("FUCK!"),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Document(inner) => inner.sender(),
			Self::Focus(inner) => inner.sender(),
			Self::Keyboard(inner) => inner.sender(),
			Self::Mouse(inner) => inner.sender(),
			Self::Object(inner) => inner.sender(),
			Self::Terminal(inner) => inner.sender(),
			Self::Window(inner) => inner.sender(),
			Self::Available(inner) => inner.sender(),
			Self::Cache(inner) => inner.sender(),
			Self::Listener(inner) => inner.sender(),
			Self::Fuck(_) => panic!("FUCK!"),
		}
	}
}

impl HasInterfaceName for EventListenerEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Registry";
}

impl HasMatchRule for EventListenerEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Registry'";
}

impl HasRegistryEventString for EventListenerEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Event";
}

#[cfg(feature = "zbus")]
impl<T> MessageConversion for T
where
	ObjectRef: Into<T>,
	// this bound is not actually used for anything, but I do not want to implement this trait for
	// just any type that has an infallible conversion from an ObjectRef
	T: BusProperties,
{
	type Body = EventBodyOwned;
	fn from_message_unchecked_parts(obj_ref: ObjectRef, _: Self::Body) -> Result<Self, AtspiError> {
		Ok(obj_ref.into())
	}
	fn from_message_unchecked(msg: zbus::Message) -> Result<Self, AtspiError> {
		let item: ObjectRef = (&msg).try_into()?;
		Ok(item.into())
	}
	fn body(&self) -> Self::Body {
		EventBodyOwned::default()
	}
}

impl<T: BusProperties> HasMatchRule for T {
	const MATCH_RULE_STRING: &'static str = <T as BusProperties>::MATCH_RULE_STRING;
}
impl<T: BusProperties> HasRegistryEventString for T {
	const REGISTRY_EVENT_STRING: &'static str = <T as BusProperties>::REGISTRY_EVENT_STRING;
}
impl<T: BusProperties> HasInterfaceName for T {
	const DBUS_INTERFACE: &'static str = <T as BusProperties>::DBUS_INTERFACE;
}

#[cfg(test)]
mod accessible_deserialization_tests {
	use crate::events::ObjectRef;
	use zvariant::Value;

	#[test]
	fn try_into_value() {
		let acc = ObjectRef::default();
		let value_struct = Value::from(acc);
		let Value::Structure(structure) = value_struct else {
			panic!("Unable to destructure a structure out of the Value.");
		};
		let vals = structure.into_fields();
		assert_eq!(vals.len(), 2);
		let Value::Str(bus_name) = vals.first().unwrap() else {
			panic!("Unable to destructure field value: {:?}", vals.first().unwrap());
		};
		assert_eq!(bus_name, ":0.0");
		let Value::ObjectPath(path) = vals.last().unwrap() else {
			panic!("Unable to destructure field value: {:?}", vals.get(1).unwrap());
		};
		assert_eq!(path.as_str(), "/org/a11y/atspi/accessible/null");
	}
	#[test]
	fn try_from_value() {}
}

#[cfg(test)]
mod accessible_tests {
	use super::ObjectRef;

	#[test]
	fn test_accessible_default_doesnt_panic() {
		let acc = ObjectRef::default();
		assert_eq!(acc.name.as_str(), ":0.0");
		assert_eq!(acc.path.as_str(), "/org/a11y/atspi/accessible/null");
	}
}
#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for ObjectRef {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		let path = msg.path().expect("returned path is either `Some` or panics");
		let owned_path: OwnedObjectPath = path.clone().into();

		let sender: UniqueName<'_> = msg.sender().expect("No sender in header");
		let name: OwnedUniqueName = sender.to_owned().into();

		Ok(ObjectRef { name, path: owned_path })
	}
}

/// Signal type emitted by `EventListenerRegistered` and `EventListenerDeregistered` signals,
/// which belong to the `Registry` interface, implemented by the registry-daemon.
#[validate(signal: "EventListenerRegistered")]
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

#[cfg(test)]
#[test]
fn test_event_listener_default_no_panic() {
	let el = EventListeners::default();
	assert_eq!(el.bus_name.as_str(), ":0.0");
	assert_eq!(el.path.as_str(), "/org/a11y/atspi/accessible/null");
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

impl EventTypeProperties for EventListenerEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.member(),
			Self::Deregistered(inner) => inner.member(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.match_rule(),
			Self::Deregistered(inner) => inner.match_rule(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.interface(),
			Self::Deregistered(inner) => inner.interface(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.registry_string(),
			Self::Deregistered(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for EventListenerEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Registered(inner) => inner.path(),
			Self::Deregistered(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Registered(inner) => inner.sender(),
			Self::Deregistered(inner) => inner.sender(),
		}
	}
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for EventListenerEvents {
	fn try_from_message_interface_checked(msg: zbus::Message) -> Result<Self, AtspiError> {
		let member = msg.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			EventListenerRegisteredEvent::DBUS_MEMBER => Ok(EventListenerEvents::Registered(
				EventListenerRegisteredEvent::from_message_unchecked(msg)?,
			)),
			EventListenerDeregisteredEvent::DBUS_MEMBER => Ok(EventListenerEvents::Deregistered(
				EventListenerDeregisteredEvent::from_message_unchecked(msg)?,
			)),
			_ => Err(AtspiError::MemberMatch(format!(
				"No member {} in {}",
				member.as_str(),
				Self::DBUS_INTERFACE
			))),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<zbus::Message> for EventListenerEvents {
	type Error = AtspiError;
	fn try_from(msg: zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

/// An event that is emitted by the registry daemon, to inform that an event has been deregistered
/// to no longer listen for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct EventListenerDeregisteredEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	/// A list of events that have been deregistered via the registry interface.
	/// See `atspi-connection`.
	pub deregistered_event: EventListeners,
}

impl_from_user_facing_event_for_interface_event_enum!(
	EventListenerDeregisteredEvent,
	EventListenerEvents,
	EventListenerEvents::Deregistered
);
impl_from_user_facing_type_for_event_enum!(EventListenerDeregisteredEvent, Event::Listener);
impl_try_from_event_for_user_facing_type!(
	EventListenerDeregisteredEvent,
	EventListenerEvents::Deregistered,
	Event::Listener
);
event_test_cases!(EventListenerDeregisteredEvent, Explicit);
impl BusProperties for EventListenerDeregisteredEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Registry:EventListenerDeregistered";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerDeregistered'";
	const DBUS_MEMBER: &'static str = "EventListenerDeregistered";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Registry";
}

#[cfg(feature = "zbus")]
impl MessageConversion for EventListenerDeregisteredEvent {
	type Body = EventListeners;

	fn from_message_unchecked_parts(
		item: ObjectRef,
		deregistered_event: Self::Body,
	) -> Result<Self, AtspiError> {
		Ok(Self { item, deregistered_event })
	}
	fn from_message_unchecked(msg: zbus::Message) -> Result<Self, AtspiError> {
		let item = (&msg).try_into()?;
		let body = msg.body().deserialize()?;
		Self::from_message_unchecked_parts(item, body)
	}
	fn body(&self) -> Self::Body {
		self.deregistered_event.clone()
	}
}
impl_from_dbus_message!(EventListenerDeregisteredEvent, Explicit);
impl_event_properties!(EventListenerDeregisteredEvent);
impl_to_dbus_message!(EventListenerDeregisteredEvent);

/// An event that is emitted by the regostry daemon to signal that an event has been registered to listen for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct EventListenerRegisteredEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	/// A list of events that have been registered via the registry interface.
	/// See `atspi-connection`.
	pub registered_event: EventListeners,
}

impl_from_user_facing_event_for_interface_event_enum!(
	EventListenerRegisteredEvent,
	EventListenerEvents,
	EventListenerEvents::Registered
);
impl_from_user_facing_type_for_event_enum!(EventListenerRegisteredEvent, Event::Listener);
impl_try_from_event_for_user_facing_type!(
	EventListenerRegisteredEvent,
	EventListenerEvents::Registered,
	Event::Listener
);
event_test_cases!(EventListenerRegisteredEvent, Explicit);
impl BusProperties for EventListenerRegisteredEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Registry:EventListenerRegistered";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerRegistered'";
	const DBUS_MEMBER: &'static str = "EventListenerRegistered";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Registry";
}

#[cfg(feature = "zbus")]
impl MessageConversion for EventListenerRegisteredEvent {
	type Body = EventListeners;

	fn from_message_unchecked_parts(
		item: ObjectRef,
		registered_event: Self::Body,
	) -> Result<Self, AtspiError> {
		Ok(Self { item, registered_event })
	}
	fn from_message_unchecked(msg: zbus::Message) -> Result<Self, AtspiError> {
		let item = (&msg).try_into()?;
		let body = msg.body().deserialize()?;
		Self::from_message_unchecked_parts(item, body)
	}
	fn body(&self) -> Self::Body {
		self.registered_event.clone()
	}
}
impl_from_dbus_message!(EventListenerRegisteredEvent, Explicit);
impl_event_properties!(EventListenerRegisteredEvent);
impl_to_dbus_message!(EventListenerRegisteredEvent);

/// An event that is emitted when the registry daemon has started.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Eq, Hash)]
pub struct AvailableEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	pub socket: ObjectRef,
}
impl<'a> From<AvailableEvent> for Event<'a> {
	fn from(ev: AvailableEvent) -> Event<'a> {
		Event::Available(ev)
	}
}
impl<'a> TryFrom<Event<'a>> for AvailableEvent {
	type Error = AtspiError;
	fn try_from(generic_event: Event<'a>) -> Result<AvailableEvent, Self::Error> {
		if let Event::Available(specific_event) = generic_event {
			Ok(specific_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}
event_test_cases!(AvailableEvent, Explicit);
impl BusProperties for AvailableEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Socket:Available";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Socket',member='Available'";
	const DBUS_MEMBER: &'static str = "Available";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Socket";
}

#[cfg(feature = "zbus")]
impl MessageConversion for AvailableEvent {
	type Body = ObjectRef;

	fn from_message_unchecked_parts(
		item: ObjectRef,
		socket: Self::Body,
	) -> Result<Self, AtspiError> {
		Ok(Self { item, socket })
	}
	fn from_message_unchecked(msg: zbus::Message) -> Result<Self, AtspiError> {
		let item = (&msg).try_into()?;
		let body = msg.body().deserialize()?;
		Self::from_message_unchecked_parts(item, body)
	}
	fn body(&self) -> Self::Body {
		self.socket.clone()
	}
}
impl_from_dbus_message!(AvailableEvent, Explicit);
impl_event_properties!(AvailableEvent);
impl_to_dbus_message!(AvailableEvent);

#[cfg(feature = "zbus")]
impl<'a> TryFrom<zbus::Message> for Event<'a> {
	type Error = AtspiError;

	fn try_from(msg: zbus::Message) -> Result<Event<'a>, AtspiError> {
		let interface = msg.interface().ok_or(AtspiError::MissingInterface)?;
		let interface_str = interface.as_str();

		match interface_str {
			<AvailableEvent as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(AvailableEvent::try_from(msg)?.into())
			}
			<ObjectEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Object(ObjectEvents::try_from_message_interface_checked(msg)?))
			}
			<DocumentEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Document(DocumentEvents::try_from_message_interface_checked(msg)?))
			}
			<WindowEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Window(WindowEvents::try_from_message_interface_checked(msg)?))
			}
			<TerminalEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Terminal(TerminalEvents::try_from_message_interface_checked(msg)?))
			}
			<MouseEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Mouse(MouseEvents::try_from_message_interface_checked(msg)?))
			}
			<FocusEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Focus(FocusEvents::try_from_message_interface_checked(msg)?))
			}
			<KeyboardEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Keyboard(KeyboardEvents::try_from_message_interface_checked(msg)?))
			}
			<CacheEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Cache(CacheEvents::try_from_message_interface_checked(msg)?))
			}
			<EventListenerEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Listener(EventListenerEvents::try_from_message_interface_checked(msg)?))
			}
			_ => Err(AtspiError::InterfaceMatch(format!(
				"No events found with interface {interface_str}"
			))),
		}
	}
}

/// Describes properties of a specific event _type_.
///
/// - `DBus` member name
/// - `DBus` interface name
///
/// Together, the member and interface name can describe a specific event _type_.
/// Likewise, the path and sender bus name collectively make up an [`ObjectRef`], which is a way to uniquely identify an individual accessible item available to `atspi`.
/// The latter is available via the [`EventProperties`] trait.
///
/// This can also be generalized, for example this is implemented for [`Event`] by dispatching to the matching variants.
/// NOTE: to use `EventProperties` on wrapper types, like `Event`, you must enable the "enum-dispatch" feature.
///
/// This trait *is* object-safe.
pub trait EventTypeProperties {
	fn member(&self) -> &'static str;
	fn interface(&self) -> &'static str;
	fn match_rule(&self) -> &'static str;
	fn registry_string(&self) -> &'static str;
}

impl<T: BusProperties> EventTypeProperties for T {
	fn member(&self) -> &'static str {
		<T>::DBUS_MEMBER
	}
	fn interface(&self) -> &'static str {
		<T>::DBUS_INTERFACE
	}
	fn match_rule(&self) -> &'static str {
		<T>::MATCH_RULE_STRING
	}
	fn registry_string(&self) -> &'static str {
		<T>::REGISTRY_EVENT_STRING
	}
}

assert_obj_safe!(EventTypeProperties);

/// `EventProperties` allows access to the internals of an event, specifically:
///
/// - The `DBUs` name which sent the event.
/// - The `ObjectPath`, a unique id for a given application.
/// - Collectively, this is called an [`ObjectRef`].
///
/// This trait *is* object-safe.
pub trait EventProperties {
	fn sender(&self) -> UniqueName<'_>;
	fn path(&self) -> ObjectPath<'_>;
	fn object_ref(&self) -> ObjectRef {
		ObjectRef { name: self.sender().into(), path: self.path().into() }
	}
}

assert_obj_safe!(EventProperties);

/// Describes the `DBus`-related information about a given struct.
///
/// - `DBus` member name
/// - `DBus` interface name
/// - `DBus` match string: used to tell `DBus` you are interested in a particular signal
/// - accessibility registry event string: used to tell the accessibility registry that you are interested in a particular event
///
/// This trait *is not* object-safe.
/// For a similar, but object-safe trait, see [`EventProperties`].
pub trait BusProperties {
	/// The `DBus` member for the event.
	/// For example, for an [`object::TextChangedEvent`] this should be `"TextChanged"`
	const DBUS_MEMBER: &'static str;
	/// The `DBus` interface name for this event.
	/// For example, for any event within [`object`], this should be "org.a11y.atspi.Event.Object".
	const DBUS_INTERFACE: &'static str;
	/// A static match rule string for `DBus`.
	/// This should usually be a string that looks like this: `"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'"`;
	/// This should be deprecated in favour of composing the string from [`Self::DBUS_MEMBER`] and [`Self::DBUS_INTERFACE`].
	const MATCH_RULE_STRING: &'static str;
	/// A registry event string for registering for event receiving via the `RegistryProxy`.
	/// This should be deprecated in favour of composing the string from [`Self::DBUS_MEMBER`] and [`Self::DBUS_INTERFACE`].
	const REGISTRY_EVENT_STRING: &'static str;
}

#[cfg(feature = "zbus")]
pub trait MessageConversion: BusProperties {
	/// What is the body type of this event.
	type Body: Type + Serialize + for<'a> Deserialize<'a>;

	/// Build an event from a [`zbus::Message`] reference.
	/// This function will not check for any of the following error conditions:
	///
	/// - That the message has an interface: [`type@AtspiError::MissingInterface`]
	/// - That the message interface matches the one for the event: [`type@AtspiError::InterfaceMatch`]
	/// - That the message has an member: [`type@AtspiError::MissingMember`]
	/// - That the message member matches the one for the event: [`type@AtspiError::MemberMatch`]
	/// - That the message signature matches the one for the event: [`type@AtspiError::SignatureMatch`]
	///
	/// Therefore, this should only be used when one has checked the above conditions.
	/// These must be checked manually.
	/// Alternatively, there is the [`MessageConversionExt::try_from_message`] that will check these
	/// conditions for you.
	///
	/// This type also implements `TryFrom<&zbus::Message>`; consider using this if you are not an
	/// internal developer.
	///
	/// # Errors
	///
	/// It is possible to get a [`type@AtspiError::Zvariant`] error if you do not check the proper
	/// conditions before calling this.
	fn from_message_unchecked(msg: zbus::Message) -> Result<Self, AtspiError>
	where
		Self: Sized;

	/// Build an event from an [`ObjectRef`] and [`Self::Body`].
	/// This function will not check for any of the following error conditions:
	///
	/// - That the message has an interface: [`type@AtspiError::MissingInterface`]
	/// - That the message interface matches the one for the event: [`type@AtspiError::InterfaceMatch`]
	/// - That the message has an member: [`type@AtspiError::MissingMember`]
	/// - That the message member matches the one for the event: [`type@AtspiError::MemberMatch`]
	///
	/// Therefore, this should only be used when one has checked the above conditions.
	///
	/// # Errors
	///
	/// Some [`Self::Body`] types may fallibly convert data fields contained in the body.
	/// If this happens, then the function will return an error.
	fn from_message_unchecked_parts(
		obj_ref: ObjectRef,
		body: Self::Body,
	) -> Result<Self, AtspiError>
	where
		Self: Sized;

	/// The body of the object.
	fn body(&self) -> Self::Body;
}

#[cfg(feature = "zbus")]
impl<T> MessageConversionExt<crate::LegacyCacheItem> for T
where
	T: MessageConversion<Body = crate::LegacyCacheItem>,
{
	fn try_from_message(msg: zbus::Message) -> Result<Self, AtspiError> {
		<T as MessageConversionExt<crate::LegacyCacheItem>>::validate_interface(&msg)?;
		<T as MessageConversionExt<crate::LegacyCacheItem>>::validate_member(&msg)?;
		<T as MessageConversionExt<crate::LegacyCacheItem>>::validate_body(&msg)?;
		<T as MessageConversion>::from_message_unchecked(msg)
	}
}

#[cfg(feature = "zbus")]
impl<T> MessageConversionExt<EventListeners> for T
where
	T: MessageConversion<Body = EventListeners>,
{
	fn try_from_message(msg: zbus::Message) -> Result<Self, AtspiError> {
		<T as MessageConversionExt<EventListeners>>::validate_interface(&msg)?;
		<T as MessageConversionExt<EventListeners>>::validate_member(&msg)?;
		<T as MessageConversionExt<EventListeners>>::validate_body(&msg)?;
		<T as MessageConversion>::from_message_unchecked(msg)
	}
}

#[cfg(feature = "zbus")]
impl<T> MessageConversionExt<crate::CacheItem> for T
where
	T: MessageConversion<Body = crate::CacheItem>,
{
	fn try_from_message(msg: zbus::Message) -> Result<Self, AtspiError> {
		<T as MessageConversionExt<crate::CacheItem>>::validate_interface(&msg)?;
		<T as MessageConversionExt<crate::CacheItem>>::validate_member(&msg)?;
		<T as MessageConversionExt<crate::CacheItem>>::validate_body(&msg)?;
		<T as MessageConversion>::from_message_unchecked(msg)
	}
}

#[cfg(feature = "zbus")]
impl<T> MessageConversionExt<ObjectRef> for T
where
	T: MessageConversion<Body = ObjectRef>,
{
	fn try_from_message(msg: zbus::Message) -> Result<Self, AtspiError> {
		<T as MessageConversionExt<ObjectRef>>::validate_interface(&msg)?;
		<T as MessageConversionExt<ObjectRef>>::validate_member(&msg)?;
		<T as MessageConversionExt<ObjectRef>>::validate_body(&msg)?;
		<T as MessageConversion>::from_message_unchecked(msg)
	}
}

#[cfg(feature = "zbus")]
impl<T> MessageConversionExt<EventBodyOwned> for T
where
	T: MessageConversion<Body = EventBodyOwned>,
{
	fn try_from_message(msg: zbus::Message) -> Result<Self, AtspiError> {
		<T as MessageConversionExt<EventBodyOwned>>::validate_interface(&msg)?;
		<T as MessageConversionExt<EventBodyOwned>>::validate_member(&msg)?;
		let body = msg.body();
		let body_sig = body.signature();
		let data_body: EventBodyOwned = if body_sig == ATSPI_EVENT_SIGNATURE {
			body.deserialize_unchecked()?
		} else if body_sig == QSPI_EVENT_SIGNATURE {
			let qtbody: EventBodyQT = body.deserialize_unchecked()?;
			qtbody.into()
		} else {
			return Err(AtspiError::SignatureMatch(format!(
				"The message signature {} does not match the signal's body signature: {} or {}",
				body_sig,
				EventBodyOwned::SIGNATURE,
				EventBodyQT::SIGNATURE,
			)));
		};
		let item = (&msg).try_into()?;
		Self::from_message_unchecked_parts(item, data_body)
	}
}

#[cfg(feature = "zbus")]
pub trait MessageConversionExt<B>: MessageConversion<Body = B>
where
	B: Type + Serialize + for<'a> Deserialize<'a>,
{
	/// Convert a [`zbus::Message`] into this event type.
	/// Does all the validation for you.
	///
	/// # Errors
	///
	/// - The message does not have an interface: [`type@AtspiError::MissingInterface`]
	/// - The message interface does not match the one for the event: [`type@AtspiError::InterfaceMatch`]
	/// - The message does not have an member: [`type@AtspiError::MissingMember`]
	/// - The message member does not match the one for the event: [`type@AtspiError::MemberMatch`]
	/// - The message signature does not match the one for the event: [`type@AtspiError::SignatureMatch`]
	///
	/// See [`MessageConversion::from_message_unchecked`] for info on panic condition that should never
	/// happen.
	fn try_from_message(msg: zbus::Message) -> Result<Self, AtspiError>
	where
		Self: Sized;
	/// Validate the interface string via [`zbus::message::Header::interface`] against `Self`'s assignment of [`BusProperties::DBUS_INTERFACE`]
	///
	/// # Errors
	///
	/// - [`type@AtspiError::MissingInterface`] if there is no interface
	/// - [`type@AtspiError::InterfaceMatch`] if the interfaces do not match
	fn validate_interface(msg: &zbus::Message) -> Result<(), AtspiError> {
		let interface = msg.interface().ok_or(AtspiError::MissingInterface)?;
		if interface != Self::DBUS_INTERFACE {
			return Err(AtspiError::InterfaceMatch(format!(
				"The interface {} does not match the signal's interface: {}",
				interface,
				Self::DBUS_INTERFACE,
			)));
		}
		Ok(())
	}
	/// Validate the member string via [`zbus::message::Header::member`] against `Self`'s assignment of [`BusProperties::DBUS_MEMBER`]
	///
	/// # Errors
	///
	/// - [`type@AtspiError::MissingMember`] if there is no member
	/// - [`type@AtspiError::MemberMatch`] if the members do not match
	fn validate_member(msg: &zbus::Message) -> Result<(), AtspiError> {
		let member = msg.member().ok_or(AtspiError::MissingMember)?;
		if member != Self::DBUS_MEMBER {
			return Err(AtspiError::MemberMatch(format!(
				"The member {} does not match the signal's member: {}",
				// unwrap is safe here because of guard above
				member,
				Self::DBUS_MEMBER,
			)));
		}
		Ok(())
	}
	/// Validate the body signature against the [`zvariant::Signature`] of [`MessageConversion::Body`]
	///
	/// # Errors
	///
	/// - [`type@AtspiError::SignatureMatch`] if the signatures do not match
	fn validate_body(msg: &zbus::Message) -> Result<(), AtspiError> {
		let body = msg.body();
		let body_signature = body.signature();
		if body_signature != Self::Body::SIGNATURE {
			return Err(AtspiError::SignatureMatch(format!(
				"The message signature {} does not match the signal's body signature: {}",
				body_signature,
				&Self::Body::SIGNATURE.to_string(),
			)));
		}
		Ok(())
	}
}

/// A specific trait *only* to define an interface name.
/// This is useful for event wrappers like [`ObjectEvents`], which, while it does not have other
/// information required to implement the [`BusProperties`] trait, you can indeed attach in
/// interface name for all sub events of [`ObjectEvents`].
///
/// This trait *is not* object-safe.
pub trait HasInterfaceName {
	/// A static interface string for `DBus`.
	/// This should usually be a string that looks like this: `"org.a11y.atspi.Event.*"`;
	const DBUS_INTERFACE: &'static str;
}

/// A specific trait *only* to define match rules.
///
/// This is useful for event wrappers like [`ObjectEvents`], which, while it does not have other
/// information required to implement the [`BusProperties`] trait, you can indeed add a match rule
/// to the `DBus` connection to capture all sub events of [`ObjectEvents`].
///
/// This trait *is not* object-safe.
pub trait HasMatchRule {
	/// A static match rule string for `DBus`.
	/// This should usually be a string that looks like this: `"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'"`;
	/// This should be deprecated in favour of composing the string from [`BusProperties::DBUS_MEMBER`] and [`BusProperties::DBUS_INTERFACE`].
	const MATCH_RULE_STRING: &'static str;
}

/// A specific trait *only* to define registry event matches.
///
/// This is useful for event wrappers like [`ObjectEvents`], which, while it does not have other
/// information required to implement the [`BusProperties`] trait, you can indeed add a match rule
/// to the AT-SPI connection to subscribe to all sub events of [`ObjectEvents`].
///
/// This trait *is not* object-safe.
pub trait HasRegistryEventString {
	/// A registry event string for registering for event receiving via the `RegistryProxy`.
	/// This should be deprecated in favour of composing the string from [`BusProperties::DBUS_MEMBER`] and [`BusProperties::DBUS_INTERFACE`].
	const REGISTRY_EVENT_STRING: &'static str;
}

/// An way to convert a [`zbus::Message`] without checking its interface.
#[cfg(feature = "zbus")]
pub(crate) trait EventWrapperMessageConversion {
	/// # Errors
	/// Will fail if no matching member or body signature is found.
	fn try_from_message_interface_checked(msg: zbus::Message) -> Result<Self, AtspiError>
	where
		Self: Sized;
}

#[cfg(feature = "zbus")]
pub(crate) trait TryFromMessage {
	fn try_from_message(msg: zbus::Message) -> Result<Self, AtspiError>
	where
		Self: Sized;
}
#[cfg(feature = "zbus")]
impl<T: EventWrapperMessageConversion + HasInterfaceName> TryFromMessage for T {
	fn try_from_message(msg: zbus::Message) -> Result<T, AtspiError> {
		let interface = msg.interface().ok_or(AtspiError::MissingInterface)?;
		if interface != <T as HasInterfaceName>::DBUS_INTERFACE {
			return Err(AtspiError::InterfaceMatch(format!(
				"Interface {} does not match require interface for event: {}",
				interface,
				<T as HasInterfaceName>::DBUS_INTERFACE
			)));
		}
		<T as EventWrapperMessageConversion>::try_from_message_interface_checked(msg)
	}
}

#[cfg(test)]
mod tests {
	use super::{EventBodyOwned, EventBodyQT, QSPI_EVENT_SIGNATURE};
	use std::collections::HashMap;
	use zvariant::{ObjectPath, Type};

	#[test]
	fn check_event_body_qt_signature() {
		assert_eq!(<EventBodyQT as Type>::SIGNATURE, QSPI_EVENT_SIGNATURE);
	}

	#[test]
	fn test_event_body_qt_to_event_body_owned_conversion() {
		let event_body: EventBodyOwned = EventBodyQT::default().into();

		let accessible = crate::ObjectRef::default();
		let name = accessible.name;
		let path = accessible.path;
		let props = HashMap::from([(name, ObjectPath::from(path).into())]);
		assert_eq!(event_body.properties, props);
	}
}
