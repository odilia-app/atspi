#[cfg(feature = "zbus")]
use crate::events::{MessageConversion, MessageConversionExt};
use crate::{
	cache::{CacheItem, LegacyCacheItem},
	error::AtspiError,
	events::{BusProperties, ObjectRef},
	Event, EventProperties,
};
use serde::{Deserialize, Serialize};
use zbus_names::UniqueName;
use zvariant::{ObjectPath, Type};

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::LegacyCacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct LegacyAddAccessibleEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	/// A cache item to add to the internal cache.
	pub node_added: LegacyCacheItem,
}

impl_from_user_facing_type_for_event_enum!(LegacyAddAccessibleEvent, Event::Cache);
event_test_cases!(LegacyAddAccessibleEvent, Explicit);
impl_from_dbus_message!(LegacyAddAccessibleEvent, Explicit);
impl_event_properties!(LegacyAddAccessibleEvent);
impl_to_dbus_message!(LegacyAddAccessibleEvent);

impl BusProperties for LegacyAddAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Add";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'";
	const DBUS_MEMBER: &'static str = "AddAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

#[cfg(feature = "zbus")]
impl MessageConversion for LegacyAddAccessibleEvent {
	type Body = LegacyCacheItem;

	fn from_message_unchecked_parts(item: ObjectRef, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, node_added: body })
	}
	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = msg.body().deserialize()?;
		Self::from_message_unchecked_parts(item, body)
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

impl_from_user_facing_type_for_event_enum!(AddAccessibleEvent, Event::Cache);
event_test_cases!(AddAccessibleEvent, Explicit);

impl BusProperties for AddAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Add";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'";
	const DBUS_MEMBER: &'static str = "AddAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

#[cfg(feature = "zbus")]
impl MessageConversion for AddAccessibleEvent {
	type Body = CacheItem;

	fn from_message_unchecked_parts(item: ObjectRef, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, node_added: body })
	}
	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = msg.body().deserialize()?;
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body {
		self.node_added.clone()
	}
}

impl_from_dbus_message!(AddAccessibleEvent, Explicit);
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

impl_from_user_facing_type_for_event_enum!(RemoveAccessibleEvent, Event::Cache);
event_test_cases!(RemoveAccessibleEvent, Explicit);

impl BusProperties for RemoveAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Remove";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='RemoveAccessible'";
	const DBUS_MEMBER: &'static str = "RemoveAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

#[cfg(feature = "zbus")]
impl MessageConversion for RemoveAccessibleEvent {
	type Body = ObjectRef;

	fn from_message_unchecked_parts(item: ObjectRef, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, node_removed: body })
	}
	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = msg.body().deserialize()?;
		Self::from_message_unchecked_parts(item, body)
	}
	fn body(&self) -> Self::Body {
		self.node_removed.clone()
	}
}

impl_from_dbus_message!(RemoveAccessibleEvent, Explicit);
impl_event_properties!(RemoveAccessibleEvent);
impl_to_dbus_message!(RemoveAccessibleEvent);
