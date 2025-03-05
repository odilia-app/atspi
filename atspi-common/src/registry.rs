//! This module contains the events that are emitted by the registry daemon.
//! The events are [`EventListenerRegisteredEvent`] and [`EventListenerDeregisteredEvent`].

use zbus_lockstep_macros::validate;
use zbus_names::{OwnedUniqueName, UniqueName};
use zvariant::{ObjectPath, Type};

#[cfg(feature = "zbus")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

use crate::{
	error::AtspiError,
	events::{
		BusProperties, Event, EventProperties, EventTypeProperties, HasInterfaceName, HasMatchRule,
		HasRegistryEventString, MessageConversion,
	},
	ObjectRef,
};

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
impl MessageConversion<'_> for EventListenerDeregisteredEvent {
	type Body<'a> = EventListeners;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let deregistered_event = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item, deregistered_event })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.deregistered_event.clone()
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: EventListenerDeregisteredEvent, body: EventListeners);
impl_from_dbus_message!(EventListenerDeregisteredEvent, Explicit);
impl_event_properties!(EventListenerDeregisteredEvent);
impl_to_dbus_message!(EventListenerDeregisteredEvent);

/// An event that is emitted by the regostry daemon to signal that an event has been registered to listen for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct EventListenerRegisteredEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: crate::ObjectRef,
	/// A list of events that have been registered via the registry interface.
	/// See `atspi-connection`.
	pub registered_event: EventListeners,
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for EventListenerRegisteredEvent {
	type Body<'a> = EventListeners;

	fn from_message_unchecked_parts(
		item: ObjectRef,
		registered_event: DbusBody,
	) -> Result<Self, AtspiError> {
		let registered_event = registered_event.deserialize_unchecked()?;
		Ok(Self { item, registered_event })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.registered_event.clone()
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: EventListenerRegisteredEvent, body: EventListeners);
impl_from_dbus_message!(EventListenerRegisteredEvent, Explicit);
impl_event_properties!(EventListenerRegisteredEvent);
impl_to_dbus_message!(EventListenerRegisteredEvent);

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

/// The events that can be emitted by the registry daemon.
/// This enum is used to wrap the events that are emitted by the registry daemon.
/// The events are [`EventListenerRegisteredEvent`] and [`EventListenerDeregisteredEvent`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum EventListenerEvents {
	/// See: [`EventListenerRegisteredEvent`].
	Registered(EventListenerRegisteredEvent),
	/// See: [`EventListenerDeregisteredEvent`].
	Deregistered(EventListenerDeregisteredEvent),
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
impl crate::events::EventWrapperMessageConversion for EventListenerEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &zbus::message::Header,
	) -> Result<Self, crate::AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			EventListenerRegisteredEvent::DBUS_MEMBER => Ok(EventListenerEvents::Registered(
				EventListenerRegisteredEvent::from_message_unchecked(msg, hdr)?,
			)),
			EventListenerDeregisteredEvent::DBUS_MEMBER => Ok(EventListenerEvents::Deregistered(
				EventListenerDeregisteredEvent::from_message_unchecked(msg, hdr)?,
			)),
			_ => Err(AtspiError::MemberMatch(format!(
				"No member {} in {}",
				member.as_str(),
				Self::DBUS_INTERFACE
			))),
		}
	}
}

impl_tryfrommessage_for_event_wrapper!(EventListenerEvents);

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for EventListenerEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		<Self as crate::events::TryFromMessage>::try_from_message(msg)
	}
}

#[cfg(test)]
mod event_listener_tests {
	use super::*;

	#[test]
	fn test_event_listener_default_no_panic() {
		let el = EventListeners::default();
		assert_eq!(el.bus_name.as_str(), ":0.0");
		assert_eq!(el.path.as_str(), "/org/a11y/atspi/accessible/null");
	}
}

pub mod socket {
	//! This module contains the event that is emitted by the registry daemon's `Socket` interface.

	use crate::{
		events::MessageConversion, AtspiError, BusProperties, Event, EventProperties, ObjectRef,
	};
	use zbus::message::Body as DbusBody;

	#[cfg(feature = "zbus")]
	use serde::{Deserialize, Serialize};
	#[cfg(feature = "zbus")]
	use zbus::message::Header;

	/// An event that is emitted when the registry daemon has started.
	///
	/// The accessibility registry emits this signal early during startup,
	/// when it has registered with the DBus daemon and is available for
	/// calls from applications.
	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Eq, Hash)]
	pub struct AvailableEvent {
		/// The emitting [`ObjectRef`].
		pub item: ObjectRef,

		/// The [`ObjectRef`] for the Registry's root object.
		pub socket: ObjectRef,
	}

	impl From<AvailableEvent> for Event {
		fn from(ev: AvailableEvent) -> Event {
			Event::Available(ev)
		}
	}

	#[cfg(feature = "zbus")]
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
	impl MessageConversion<'_> for AvailableEvent {
		type Body<'a> = ObjectRef;

		fn from_message_unchecked_parts(
			item: ObjectRef,
			body: DbusBody,
		) -> Result<Self, AtspiError> {
			let socket = body.deserialize_unchecked::<Self::Body<'_>>()?;
			Ok(Self { item, socket })
		}

		fn from_message_unchecked(
			msg: &zbus::Message,
			header: &Header,
		) -> Result<Self, AtspiError> {
			let item = header.try_into()?;
			let body = msg.body();
			Self::from_message_unchecked_parts(item, body)
		}

		fn body(&self) -> Self::Body<'_> {
			self.socket.clone()
		}
	}

	impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: AvailableEvent, body: ObjectRef);
	impl_from_dbus_message!(AvailableEvent, Explicit);
	impl_event_properties!(AvailableEvent);
	impl_to_dbus_message!(AvailableEvent);
}
