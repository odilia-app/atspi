//! This module contains the events that are emitted by the registry daemon.
//! The events are [`EventListenerRegisteredEvent`] and [`EventListenerDeregisteredEvent`].

use crate::events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString};
use crate::object_ref::NonNullObjectRef;
#[cfg(feature = "zbus")]
use crate::{error::AtspiError, events::MessageConversion, EventProperties};
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};
use zbus_lockstep_macros::validate;
use zbus_names::{OwnedUniqueName, UniqueName};
use zvariant::Type;

/// An event that is emitted by the registry daemon, to inform that an event has been deregistered
/// to no longer listen for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct EventListenerDeregisteredEvent<'a> {
	/// The [`crate::NonNullObjectRef`] the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
	/// A list of events that have been deregistered via the registry interface.
	/// See `atspi-connection`.
	pub deregistered_event: EventListeners,
}

impl_event_type_properties_for_event!(EventListenerDeregisteredEvent<'_>);
event_test_cases!(EventListenerDeregisteredEvent, Explicit);

// The registry string cannot be found in upstrream at-spi2-core.
impl_member_interface_registry_string_and_match_rule_for_event!(
	EventListenerDeregisteredEvent<'_>,
	"EventListenerDeregistered",
	"org.a11y.atspi.Registry",
	"registry:event-listener-deregistered",
	"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerDeregistered'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for EventListenerDeregisteredEvent<'a> {
	type Body<'msg>
		= EventListeners
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'a>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		let deregistered_event = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item, deregistered_event })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.deregistered_event.clone()
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: EventListenerDeregisteredEvent<'_>, body: EventListeners);
impl_from_dbus_message!(EventListenerDeregisteredEvent<'_>, Explicit);
impl_event_properties!(EventListenerDeregisteredEvent<'_>);
impl_to_dbus_message!(EventListenerDeregisteredEvent<'_>);

/// An event that is emitted by the regostry daemon to signal that an event has been registered to listen for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct EventListenerRegisteredEvent<'a> {
	/// The [`crate::NonNullObjectRef`] the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,

	/// A list of events that have been registered via the registry interface.
	/// See `atspi-connection`.
	pub registered_event: EventListeners,
}

impl_event_type_properties_for_event!(EventListenerRegisteredEvent<'_>);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for EventListenerRegisteredEvent<'a> {
	type Body<'msg>
		= EventListeners
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'a>,
		dbus_body: DbusBody,
	) -> Result<Self, AtspiError> {
		let registered_event = dbus_body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item, registered_event })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.registered_event.clone()
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: EventListenerRegisteredEvent<'_>, body: EventListeners);
impl_from_dbus_message!(EventListenerRegisteredEvent<'_>, Explicit);
impl_event_properties!(EventListenerRegisteredEvent<'_>);
impl_to_dbus_message!(EventListenerRegisteredEvent<'_>);

event_test_cases!(EventListenerRegisteredEvent, Explicit);

impl_test_event!(
	EventListenerRegisteredEvent<'_> { registered_event },
	EventListenerDeregisteredEvent<'_> { deregistered_event }
);

// The registry string cannot be found in upstrream at-spi2-core.
impl_member_interface_registry_string_and_match_rule_for_event!(
	EventListenerRegisteredEvent<'_>,
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

	// TODO: `path` should be a `zvariant::ObjectPath` but that requires changing the signature with an attribute
	// and `Serialize`/`Deserialize` impls.
	pub path: String,
}

impl Default for EventListeners {
	fn default() -> Self {
		Self {
			bus_name: UniqueName::from_static_str_unchecked(":0.0").into(),
			path: String::from("/org/a11y/atspi/null"),
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
		assert_eq!(el.path.as_str(), "/org/a11y/atspi/null");
	}
}

pub mod socket {
	//! This module contains the event that is emitted by the registry daemon's `Socket` interface.

	#[cfg(feature = "zbus")]
	use crate::events::MessageConversion;
	use crate::events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString};
	use crate::object_ref::TEST_NON_NULL_OBJECT_REF;
	#[cfg(feature = "zbus")]
	use crate::AtspiError;
	#[cfg(feature = "zbus")]
	use crate::EventProperties;
	use crate::NonNullObjectRef;
	use serde::{Deserialize, Serialize};
	#[cfg(feature = "zbus")]
	use zbus::message::{Body as DbusBody, Header};

	/// An event that is emitted when the registry daemon has started.
	///
	/// The accessibility registry emits this signal early during startup,
	/// when it has registered with the `DBus` daemon and is available for
	/// calls from applications.
	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
	pub struct AvailableEvent<'a> {
		/// The emitting [`crate::NonNullObjectRef`].
		#[serde(borrow)]
		pub item: NonNullObjectRef<'a>,

		/// The [`crate::NonNullObjectRef`] for the Registry's root object.
		#[serde(borrow)]
		pub socket: NonNullObjectRef<'a>,
	}

	impl_event_type_properties_for_event!(AvailableEvent<'_>);

	event_test_cases!(AvailableEvent, Explicit);

	// We cannot register at Registry for this event, as it is emitted by the Registry itself at early startup.
	// So, we do not have a registry event string for this event.
	// The `Available` event is unconditionally emitted:
	// [at-spi2-core/registryd/registry.c:1437](https://github.com/GNOME/at-spi2-core/blob/019d1a4013216d7d01040cf4eb3b8647bffc0dc9/registryd/registry.c#L1437)
	impl_member_interface_registry_string_and_match_rule_for_event!(
		AvailableEvent<'_>,
		"Available",
		"org.a11y.atspi.Socket",
		"",
		"type='signal',interface='org.a11y.atspi.Socket',member='Available'"
	);

	#[cfg(feature = "zbus")]
	impl<'a> MessageConversion<'a> for AvailableEvent<'a> {
		type Body<'msg>
			= NonNullObjectRef<'msg>
		where
			Self: 'msg;

		fn from_message_unchecked_parts(
			item: NonNullObjectRef<'_>,
			body: DbusBody,
		) -> Result<Self, AtspiError> {
			let socket = body.deserialize_unchecked::<Self::Body<'_>>()?;
			Ok(Self { item: item.into_owned(), socket: socket.into_owned() })
		}

		fn from_message_unchecked(
			msg: &zbus::Message,
			header: &Header,
		) -> Result<Self, AtspiError> {
			let item: NonNullObjectRef<'_> = header.try_into()?;
			let body = msg.body();
			Self::from_message_unchecked_parts(item.into_owned(), body)
		}

		fn body(&self) -> Self::Body<'_> {
			self.socket.clone()
		}
	}

	impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: AvailableEvent<'_>, body: NonNullObjectRef<'a>);
	impl_from_dbus_message!(AvailableEvent<'_>, Explicit);
	impl_event_properties!(AvailableEvent<'_>);
	impl_to_dbus_message!(AvailableEvent<'_>);

	// `AvailableEvent` has a `socket` field, which is a `NonNullObjectRef` which does not implement `Defeult`.
	// So it is best to manually implement test event.
	// instead of `impl_test_event!(AvailableEvent<'_> { socket });`
	impl<'o> AvailableEvent<'o> {
		#[doc(hidden)]
		#[must_use]
		pub fn new_test_event(origin: &crate::NonNullObjectRef<'o>) -> Self {
			let socket = TEST_NON_NULL_OBJECT_REF;
			Self { item: origin.clone(), socket }
		}
	}
}
