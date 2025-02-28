use crate::{
	cache::{CacheItem, LegacyCacheItem},
	events::{BusProperties, ObjectRef},
	EventProperties,
};
#[cfg(feature = "zbus")]
use crate::{
	error::AtspiError,
	events::{MessageConversion, MessageConversionExt},
};
use serde::{Deserialize, Serialize};
use zbus::message::{Body as DbusBody, Header};
use zbus_names::UniqueName;

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::LegacyCacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct LegacyAddAccessibleEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	/// A cache item to add to the internal cache.
	pub node_added: LegacyCacheItem,
}

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
impl MessageConversion<'_> for LegacyAddAccessibleEvent {
	type Body<'msg> = LegacyCacheItem;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		Ok(Self { item, node_added: body.deserialize_unchecked::<Self::Body<'_>>()? })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
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

event_test_cases!(AddAccessibleEvent, Explicit);

impl BusProperties for AddAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Add";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'";
	const DBUS_MEMBER: &'static str = "AddAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for AddAccessibleEvent {
	type Body<'msg> = CacheItem;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		Ok(Self { item, node_added: body.deserialize_unchecked::<Self::Body<'_>>()? })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.node_added.clone()
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: AddAccessibleEvent, body: CacheItem);
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

event_test_cases!(RemoveAccessibleEvent, Explicit);

impl BusProperties for RemoveAccessibleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Remove";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='RemoveAccessible'";
	const DBUS_MEMBER: &'static str = "RemoveAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for RemoveAccessibleEvent {
	type Body<'msg> = ObjectRef;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		Ok(Self { item, node_removed: body.deserialize_unchecked::<Self::Body<'_>>()? })
	}
	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}
	fn body(&self) -> Self::Body<'_> {
		self.node_removed.clone()
	}
}

#[cfg(feature = "zbus")]
impl MessageConversionExt<'_, LegacyCacheItem> for LegacyAddAccessibleEvent {
	fn try_from_message(msg: &zbus::Message, hdr: &Header) -> Result<Self, AtspiError> {
		<LegacyAddAccessibleEvent as MessageConversionExt<crate::LegacyCacheItem>>::validate_interface(hdr)?;
		<LegacyAddAccessibleEvent as MessageConversionExt<crate::LegacyCacheItem>>::validate_member(hdr)?;
		<LegacyAddAccessibleEvent as MessageConversionExt<crate::LegacyCacheItem>>::validate_body(
			msg,
		)?;
		<LegacyAddAccessibleEvent as MessageConversion>::from_message_unchecked(msg, hdr)
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: RemoveAccessibleEvent, body: ObjectRef);
impl_from_dbus_message!(RemoveAccessibleEvent, Explicit);
impl_event_properties!(RemoveAccessibleEvent);
impl_to_dbus_message!(RemoveAccessibleEvent);
