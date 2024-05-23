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
pub const ATSPI_EVENT_SIGNATURE: Signature<'_> =
	Signature::from_static_str_unchecked("(siiva{sv})");
pub const QSPI_EVENT_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(siiv(so))");
pub const EVENT_LISTENER_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(ss)");
pub const CACHE_ADD_SIGNATURE: Signature<'_> =
	Signature::from_static_str_unchecked("((so)(so)(so)iiassusau)");

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zbus_lockstep_macros::validate;
use zbus_names::{BusName, OwnedBusName, OwnedUniqueName, UniqueName};
#[cfg(feature = "zbus")]
use zvariant::OwnedObjectPath;
use zvariant::{ObjectPath, OwnedValue, Signature, Type, Value};

use crate::{
	cache::{CacheItem, LegacyCacheItem},
	events::{
		document::DocumentEvents, focus::FocusEvents, keyboard::KeyboardEvents, mouse::MouseEvents,
		object::ObjectEvents, terminal::TerminalEvents, window::WindowEvents,
	},
	AtspiError, ObjectRef,
};

/// A borrowed body for events.
#[derive(Debug, Serialize, Deserialize)]
pub struct EventBody<'a, T> {
	/// A generic "kind" type, defined by AT-SPI:
	/// usually a `&'a str`, but can be another type like [`crate::state::State`].
	#[serde(rename = "type")]
	pub kind: T,
	/// Generic first detail defined by AT-SPI.
	pub detail1: i32,
	/// Generic second detail defined by AT-SPI.
	pub detail2: i32,
	/// Generic `any_data` field defined in AT-SPI.
	/// Can contain any type.
	#[serde(borrow)]
	pub any_data: Value<'a>,
	/// Map of string to an any type.
	/// This is not used for anything, but it is defined by AT-SPI.
	#[serde(borrow)]
	pub properties: HashMap<BusName<'a>, Value<'a>>,
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
	pub properties: HashMap<OwnedBusName, OwnedValue>,
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

impl EventTypeProperties for Event {
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
		}
	}
}

impl EventProperties for Event {
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
		}
	}
	fn sender(&self) -> BusName<'_> {
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
		}
	}
}

impl HasMatchRule for CacheEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Cache'";
}

impl HasRegistryEventString for CacheEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Cache";
}

impl HasMatchRule for EventListenerEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Registry'";
}

impl HasRegistryEventString for EventListenerEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Event";
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

impl EventTypeProperties for CacheEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.member(),
			Self::LegacyAdd(inner) => inner.member(),
			Self::Remove(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.interface(),
			Self::LegacyAdd(inner) => inner.interface(),
			Self::Remove(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.match_rule(),
			Self::LegacyAdd(inner) => inner.match_rule(),
			Self::Remove(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.registry_string(),
			Self::LegacyAdd(inner) => inner.registry_string(),
			Self::Remove(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for CacheEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Add(inner) => inner.path(),
			Self::LegacyAdd(inner) => inner.path(),
			Self::Remove(inner) => inner.path(),
		}
	}
	fn sender(&self) -> BusName<'_> {
		match self {
			Self::Add(inner) => inner.sender(),
			Self::LegacyAdd(inner) => inner.sender(),
			Self::Remove(inner) => inner.sender(),
		}
	}
}

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::LegacyCacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct LegacyAddAccessibleEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	/// A cache item to add to the internal cache.
	pub node_added: LegacyCacheItem,
}

impl_from_user_facing_event_for_interface_event_enum!(
	LegacyAddAccessibleEvent,
	CacheEvents,
	CacheEvents::LegacyAdd
);
impl_from_user_facing_type_for_event_enum!(LegacyAddAccessibleEvent, Event::Cache);
impl_try_from_event_for_user_facing_type!(
	LegacyAddAccessibleEvent,
	CacheEvents::LegacyAdd,
	Event::Cache
);
event_test_cases!(LegacyAddAccessibleEvent);
impl_from_dbus_message!(LegacyAddAccessibleEvent);
impl_event_properties!(LegacyAddAccessibleEvent);
impl_to_dbus_message!(LegacyAddAccessibleEvent);

impl BusProperties for LegacyAddAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Add";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'";
	const DBUS_MEMBER: &'static str = "AddAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";

	type Body = LegacyCacheItem;

	#[cfg(feature = "zbus")]
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let body = msg.body();
		let node_added: Self::Body = body.deserialize::<Self::Body>()?;
		let item = msg.try_into()?;

		Ok(Self { item, node_added })
	}

	fn body(&self) -> Self::Body {
		self.node_added.clone()
	}
}

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::CacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct AddAccessibleEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	/// A cache item to add to the internal cache.
	pub node_added: CacheItem,
}

impl_from_user_facing_event_for_interface_event_enum!(
	AddAccessibleEvent,
	CacheEvents,
	CacheEvents::Add
);
impl_from_user_facing_type_for_event_enum!(AddAccessibleEvent, Event::Cache);
impl_try_from_event_for_user_facing_type!(AddAccessibleEvent, CacheEvents::Add, Event::Cache);
event_test_cases!(AddAccessibleEvent);

impl BusProperties for AddAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Add";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'";
	const DBUS_MEMBER: &'static str = "AddAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";

	type Body = CacheItem;

	#[cfg(feature = "zbus")]
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let body = msg.body();
		let node_added: Self::Body = body.deserialize::<Self::Body>()?;
		let item = msg.try_into()?;

		Ok(Self { item, node_added })
	}

	fn body(&self) -> Self::Body {
		self.node_added.clone()
	}
}
impl<T: BusProperties> HasMatchRule for T {
	const MATCH_RULE_STRING: &'static str = <T as BusProperties>::MATCH_RULE_STRING;
}
impl<T: BusProperties> HasRegistryEventString for T {
	const REGISTRY_EVENT_STRING: &'static str = <T as BusProperties>::REGISTRY_EVENT_STRING;
}
impl_from_dbus_message!(AddAccessibleEvent);
impl_event_properties!(AddAccessibleEvent);
impl_to_dbus_message!(AddAccessibleEvent);

/// `Cache::RemoveAccessible` signal event type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct RemoveAccessibleEvent {
	/// The application that emitted the signal TODO Check Me
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	/// The node that was removed from the application tree  TODO Check Me
	pub node_removed: ObjectRef,
}

impl_from_user_facing_event_for_interface_event_enum!(
	RemoveAccessibleEvent,
	CacheEvents,
	CacheEvents::Remove
);
impl_from_user_facing_type_for_event_enum!(RemoveAccessibleEvent, Event::Cache);
impl_try_from_event_for_user_facing_type!(RemoveAccessibleEvent, CacheEvents::Remove, Event::Cache);
event_test_cases!(RemoveAccessibleEvent);

impl BusProperties for RemoveAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Remove";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='RemoveAccessible'";
	const DBUS_MEMBER: &'static str = "RemoveAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";

	type Body = ObjectRef;

	#[cfg(feature = "zbus")]
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let body = msg.body();
		let node_removed: Self::Body = body.deserialize::<Self::Body>()?;
		let item = msg.try_into()?;

		Ok(Self { item, node_removed })
	}

	fn body(&self) -> Self::Body {
		self.node_removed.clone()
	}
}

impl_from_dbus_message!(RemoveAccessibleEvent);
impl_event_properties!(RemoveAccessibleEvent);
impl_to_dbus_message!(RemoveAccessibleEvent);

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
	fn try_from(message: &zbus::Message) -> Result<Self, Self::Error> {
		let header = message.header();
		let path = header.path().expect("returned path is either `Some` or panics");
		let owned_path: OwnedObjectPath = path.as_ref().into();

		let sender: UniqueName<'_> = header.sender().expect("No sender in header").as_ref();
		let bus_name: BusName<'_> = sender.into();
		let name = OwnedBusName::from(bus_name);

		Ok(ObjectRef { name, path: owned_path })
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for EventBodyOwned {
	type Error = AtspiError;

	fn try_from(message: &zbus::Message) -> Result<Self, Self::Error> {
		let body = message.body();
		let signature = body.signature().ok_or_else(|| AtspiError::MissingSignature)?;

		if signature == QSPI_EVENT_SIGNATURE {
			let qt_body = body.deserialize::<EventBodyQT>()?;
			Ok(EventBodyOwned::from(qt_body))
		} else if signature == ATSPI_EVENT_SIGNATURE {
			Ok(body.deserialize::<EventBodyOwned>()?)
		} else {
			Err(AtspiError::Conversion(
				"Unable to convert from zbus::Message to EventBodyQT or EventBodyOwned",
			))
		}
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
	fn sender(&self) -> BusName<'_> {
		match self {
			Self::Registered(inner) => inner.sender(),
			Self::Deregistered(inner) => inner.sender(),
		}
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
event_test_cases!(EventListenerDeregisteredEvent);
impl BusProperties for EventListenerDeregisteredEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Registry:EventListenerDeregistered";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerDeregistered'";
	const DBUS_MEMBER: &'static str = "EventListenerDeregistered";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Registry";

	type Body = EventListeners;

	#[cfg(feature = "zbus")]
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body: Self::Body = msg.body().deserialize::<Self::Body>()?;
		Ok(Self { item, deregistered_event: body })
	}

	fn body(&self) -> Self::Body {
		self.deregistered_event.clone()
	}
}
impl_from_dbus_message!(EventListenerDeregisteredEvent);
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
event_test_cases!(EventListenerRegisteredEvent);

impl BusProperties for EventListenerRegisteredEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Registry:EventListenerRegistered";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerRegistered'";
	const DBUS_MEMBER: &'static str = "EventListenerRegistered";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Registry";

	type Body = EventListeners;

	#[cfg(feature = "zbus")]
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body: Self::Body = msg.body().deserialize::<Self::Body>()?;
		Ok(Self { item, registered_event: body })
	}

	fn body(&self) -> Self::Body {
		self.registered_event.clone()
	}
}
impl_from_dbus_message!(EventListenerRegisteredEvent);
impl_event_properties!(EventListenerRegisteredEvent);
impl_to_dbus_message!(EventListenerRegisteredEvent);

/// An event that is emitted when the registry daemon has started.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Eq, Hash)]
pub struct AvailableEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	pub socket: ObjectRef,
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

impl BusProperties for AvailableEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Socket:Available";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Socket',member='Available'";
	const DBUS_MEMBER: &'static str = "Available";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Socket";

	type Body = ObjectRef;

	#[cfg(feature = "zbus")]
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = msg.body();
		let socket: Self::Body = body.deserialize::<Self::Body>()?;
		Ok(Self { item, socket })
	}
	fn body(&self) -> Self::Body {
		self.socket.clone()
	}
}
impl_from_dbus_message!(AvailableEvent);
impl_event_properties!(AvailableEvent);
impl_to_dbus_message!(AvailableEvent);

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for Event {
	type Error = AtspiError;

	fn try_from(msg: &zbus::Message) -> Result<Event, AtspiError> {
		let body = msg.body();
		let header = msg.header();

		let body_signature = body.signature().ok_or(AtspiError::MissingSignature)?;
		let body_signature_str = body_signature.as_str();

		let member = header.member().ok_or(AtspiError::MissingMember)?;
		let member_str = member.as_str();

		let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
		let interface_str = interface.as_str();

		// The `body_signature` is a marshalled D-Bus signatures, this means that outer STRUCT
		// parentheses are not included in the signature.
		// However, `Cache` signals are often emitted with outer parentheses, so we also need to
		// match against the same signature, but with outer parentheses.
		match (interface_str, member_str, body_signature_str) {
			("org.a11y.atspi.Socket", "Available", "so") => {
				Ok(AvailableEvent::try_from(msg)?.into())
			}
			("org.a11y.atspi.Event.Object", _, "siiva{sv}" | "siiv(so)") => {
				Ok(Event::Object(ObjectEvents::try_from(msg)?))
			}
			("org.a11y.atspi.Event.Document", _, "siiva{sv}" | "siiv(so)") => {
				Ok(Event::Document(DocumentEvents::try_from(msg)?))
			}
			("org.a11y.atspi.Event.Window", _, "siiva{sv}" | "siiv(so)") => {
				Ok(Event::Window(WindowEvents::try_from(msg)?))
			}
			("org.a11y.atspi.Event.Terminal", _, "siiva{sv}" | "siiv(so)") => {
				Ok(Event::Terminal(TerminalEvents::try_from(msg)?))
			}
			("org.a11y.atspi.Event.Mouse", _, "siiva{sv}" | "siiv(so)") => {
				Ok(Event::Mouse(MouseEvents::try_from(msg)?))
			}
			("org.a11y.atspi.Event.Focus", _, "siiva{sv}" | "siiv(so)") => {
				Ok(Event::Focus(FocusEvents::try_from(msg)?))
			}
			("org.a11y.atspi.Event.Keyboard", _, "siiva{sv}" | "siiv(so)") => {
				Ok(Event::Keyboard(KeyboardEvents::try_from(msg)?))
			}
			("org.a11y.atspi.Registry", "EventListenerRegistered", "ss") => {
				Ok(EventListenerRegisteredEvent::try_from(msg)?.into())
			}
			("org.a11y.atspi.Registry", "EventListenerDeregistered", "ss") => {
				Ok(EventListenerDeregisteredEvent::try_from(msg)?.into())
			}
			(
				"org.a11y.atspi.Cache",
				"AddAccessible",
				"(so)(so)(so)iiassusau" | "((so)(so)(so)iiassusau)",
			) => Ok(AddAccessibleEvent::try_from(msg)?.into()),
			(
				"org.a11y.atspi.Cache",
				"AddAccessible",
				"(so)(so)(so)a(so)assusau" | "((so)(so)(so)a(so)assusau)",
			) => Ok(LegacyAddAccessibleEvent::try_from(msg)?.into()),
			("org.a11y.atspi.Cache", "RemoveAccessible", "so" | "(so)") => {
				Ok(RemoveAccessibleEvent::try_from(msg)?.into())
			}
			(_iface, _method, sig) => Err(AtspiError::UnknownBusSignature(sig.to_string())),
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
	fn sender(&self) -> BusName<'_>;
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

	/// What is the body type of this event.
	type Body: Type + Serialize + for<'a> Deserialize<'a>;

	/// Build the event from the `zbus::Message`.
	///
	/// When called on a `&zbus::Message` from a message stream, you may want to make
	/// sure event and message match. There is a helper to check for the match:
	///
	/// ```ignore
	///    if !msg.matches_event::<EventType>()? {
	///        return Err(AtspiError::EventMismatch);
	///    }
	///    let event = EventType::try_from_message(msg)?;
	/// ```
	///
	/// These checks are already performed in the `Event::try_from` implementation, therefore the check is omited in the implementations of `BusProperties`.
	///
	/// # Errors
	///
	/// When the body type, which is what the raw message looks like over `DBus`, does not match the type that is expected for the given event.
	/// It is not possible for this to error on most events, but on events whose raw message [`Self::Body`] type contains a [`enum@zvariant::Value`], you may get errors when constructing the structure.
	#[cfg(feature = "zbus")]
	fn try_from_message(msg: &zbus::Message) -> std::result::Result<Self, AtspiError>
	where
		Self: Sized;

	/// The body of the object.
	fn body(&self) -> Self::Body;
}

/// A specific trait *only* to define match rules.
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

#[cfg(test)]
mod tests {
	use super::{EventBodyOwned, EventBodyQT, QSPI_EVENT_SIGNATURE};
	use std::collections::HashMap;
	use zvariant::{ObjectPath, Type};

	#[test]
	fn check_event_body_qt_signature() {
		assert_eq!(&<EventBodyQT as Type>::signature(), &QSPI_EVENT_SIGNATURE);
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
