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
		DBusInterface, DBusMatchRule, DBusMember, Event, EventProperties, EventTypeProperties,
		MessageConversion, RegistryEventString,
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

// The registry string cannot be found in upstrream at-spi2-core.

impl_member_interface_registry_string_and_match_rule_for_event!(
	EventListenerDeregisteredEvent,
	"EventListenerDeregistered",
	"org.a11y.atspi.Registry",
	"registry:event-listener-deregistered",
	"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerDeregistered'"
);

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

// The registry string cannot be found in upstrream at-spi2-core.
impl_member_interface_registry_string_and_match_rule_for_event!(
	EventListenerRegisteredEvent,
	"EventListenerRegistered",
	"org.a11y.atspi.Registry",
	"registry:event-listener-registered",
	"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerRegistered'"
);

/// Signal type emitted by `EventListenerRegistered` and `EventListenerDeregistered` signals,
/// which belong to the `Registry` interface, implemented by the registry-daemon.
#[validate(signal: "EventListenerRegistered")]
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct EventListeners {
	pub bus_name: OwnedUniqueName,
	pub path: OwnedObjectPath,
}

impl Default for EventListeners {
	fn default() -> Self {
		Self {
			bus_name: OwnedUniqueName::from_static_str_unchecked(":0.0"),
			path: String::from("/org/a11y/atspi/accessible/null"),
		}
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

	use crate::{events::MessageConversion, AtspiError, Event, EventProperties, ObjectRef};
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

	// We cannot really register at Registry for this event, as it is emitted by the Registry itself at early startup.
	// So, we do not have a registry event string for this event.
	impl_member_interface_registry_string_and_match_rule_for_event!(
		AvailableEvent,
		"Available",
		"org.a11y.atspi.Socket",
		"",
		"type='signal',interface='org.a11y.atspi.Socket',member='Available'"
	);

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
