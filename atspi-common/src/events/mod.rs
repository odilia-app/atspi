pub mod cache;
pub mod document;
#[cfg(feature = "wrappers")]
pub mod event_wrappers;

pub use event_body::{EventBodyBorrowed, EventBodyOwned, EventBodyQtBorrowed, EventBodyQtOwned};
#[cfg(feature = "wrappers")]
pub use event_wrappers::{
	CacheEvents, DocumentEvents, Event, FocusEvents, KeyboardEvents, MouseEvents, ObjectEvents,
	TerminalEvents, WindowEvents,
};
pub mod event_body;
pub mod focus;
pub mod keyboard;
pub mod mouse;
pub mod object;
pub mod terminal;
pub mod traits;
pub mod window;
pub use traits::*;

// Same as "(siiv(so))"
// The only signature that is not found in XML descriptions
pub(crate) const QSPI_EVENT_SIGNATURE: &Signature = &Signature::static_structure(&[
	&Signature::Str,
	&Signature::I32,
	&Signature::I32,
	&Signature::Variant,
	&Signature::Structure(Fields::Static { fields: &[&Signature::Str, &Signature::ObjectPath] }),
]);

use serde::{Deserialize, Serialize};
use zbus_lockstep_macros::validate;
use zbus_names::{OwnedUniqueName, UniqueName};
#[cfg(feature = "zbus")]
use zvariant::OwnedObjectPath;
use zvariant::{signature::Fields, ObjectPath, Signature, Type};

#[cfg(feature = "zbus")]
use crate::AtspiError;
use crate::ObjectRef;

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
	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item: ObjectRef = msg.try_into()?;
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
	fn try_from(message: &zbus::Message) -> Result<Self, Self::Error> {
		let header = message.header();
		let path = header.path().expect("returned path is either `Some` or panics");
		let owned_path: OwnedObjectPath = path.clone().into();

		let sender: UniqueName<'_> = header.sender().expect("No sender in header").into();
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
	fn try_from_message_interface_checked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let header = msg.header();
		let member = header.member().ok_or(AtspiError::MissingMember)?;
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
impl TryFrom<&zbus::Message> for EventListenerEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
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
	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
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
	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
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
#[cfg(feature = "wrappers")]
impl From<AvailableEvent> for Event {
	fn from(ev: AvailableEvent) -> Event {
		Event::Available(ev)
	}
}
#[cfg(feature = "wrappers")]
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
	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
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

#[cfg(all(feature = "zbus", feature = "wrappers"))]
impl TryFrom<&zbus::Message> for Event {
	type Error = AtspiError;

	fn try_from(msg: &zbus::Message) -> Result<Event, AtspiError> {
		let header = msg.header();
		let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
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

assert_obj_safe!(EventProperties);

#[cfg(feature = "zbus")]
impl<T> MessageConversionExt<crate::LegacyCacheItem> for T
where
	T: MessageConversion<Body = crate::LegacyCacheItem>,
{
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		<T as MessageConversionExt<crate::LegacyCacheItem>>::validate_interface(msg)?;
		<T as MessageConversionExt<crate::LegacyCacheItem>>::validate_member(msg)?;
		<T as MessageConversionExt<crate::LegacyCacheItem>>::validate_body(msg)?;
		<T as MessageConversion>::from_message_unchecked(msg)
	}
}

#[cfg(feature = "zbus")]
impl<T> MessageConversionExt<EventListeners> for T
where
	T: MessageConversion<Body = EventListeners>,
{
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		<T as MessageConversionExt<EventListeners>>::validate_interface(msg)?;
		<T as MessageConversionExt<EventListeners>>::validate_member(msg)?;
		<T as MessageConversionExt<EventListeners>>::validate_body(msg)?;
		<T as MessageConversion>::from_message_unchecked(msg)
	}
}

#[cfg(feature = "zbus")]
impl<T> MessageConversionExt<crate::CacheItem> for T
where
	T: MessageConversion<Body = crate::CacheItem>,
{
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		<T as MessageConversionExt<crate::CacheItem>>::validate_interface(msg)?;
		<T as MessageConversionExt<crate::CacheItem>>::validate_member(msg)?;
		<T as MessageConversionExt<crate::CacheItem>>::validate_body(msg)?;
		<T as MessageConversion>::from_message_unchecked(msg)
	}
}

#[cfg(feature = "zbus")]
impl<T> MessageConversionExt<ObjectRef> for T
where
	T: MessageConversion<Body = ObjectRef>,
{
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		<T as MessageConversionExt<ObjectRef>>::validate_interface(msg)?;
		<T as MessageConversionExt<ObjectRef>>::validate_member(msg)?;
		<T as MessageConversionExt<ObjectRef>>::validate_body(msg)?;
		<T as MessageConversion>::from_message_unchecked(msg)
	}
}

#[cfg(feature = "zbus")]
impl<T> MessageConversionExt<EventBodyOwned> for T
where
	T: MessageConversion<Body = EventBodyOwned>,
{
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		<T as MessageConversionExt<EventBodyOwned>>::validate_interface(msg)?;
		<T as MessageConversionExt<EventBodyOwned>>::validate_member(msg)?;
		let body = msg.body();
		let body_sig = body.signature();
		let data_body: EventBodyOwned = if body_sig == EventBodyOwned::SIGNATURE {
			body.deserialize_unchecked()?
		} else if body_sig == QSPI_EVENT_SIGNATURE {
			let qtbody: EventBodyQtOwned = body.deserialize_unchecked()?;
			qtbody.into()
		} else {
			return Err(AtspiError::SignatureMatch(format!(
				"The message signature {} does not match the signal's body signature: {} or {}",
				body_sig,
				EventBodyOwned::SIGNATURE,
				EventBodyQtOwned::SIGNATURE,
			)));
		};
		let item = msg.try_into()?;
		Self::from_message_unchecked_parts(item, data_body)
	}
}

#[cfg(feature = "zbus")]
impl<T: EventWrapperMessageConversion + HasInterfaceName> TryFromMessage for T {
	fn try_from_message(msg: &zbus::Message) -> Result<T, AtspiError> {
		let header = msg.header();
		let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
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
