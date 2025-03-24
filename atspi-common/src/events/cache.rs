#[cfg(feature = "zbus")]
use crate::events::{
	EventWrapperMessageConversion, MessageConversion, MessageConversionExt, TryFromMessage,
};
use crate::{
	cache::{CacheItem, LegacyCacheItem},
	error::AtspiError,
	events::{DBusInterface, DBusMatchRule, ObjectRef, RegistryEventString},
	Event, EventProperties, EventTypeProperties,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};
use zbus_names::UniqueName;
use zvariant::{ObjectPath, Type};

use super::DBusMember;

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

#[cfg(feature = "zbus")]
impl TryFromMessage for CacheEvents {
	fn try_from_message(msg: &zbus::Message) -> Result<CacheEvents, AtspiError> {
		let header = msg.header();
		let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
		if interface != Self::DBUS_INTERFACE {
			return Err(AtspiError::InterfaceMatch(format!(
				"Interface {} does not match require interface for event: {}",
				interface,
				Self::DBUS_INTERFACE
			)));
		}
		Self::try_from_message_interface_checked(msg, &header)
	}
}

impl DBusMatchRule for CacheEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Cache'";
}

impl DBusInterface for CacheEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

impl RegistryEventString for CacheEvents {
	const REGISTRY_EVENT_STRING: &'static str = "cache:";
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
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Add(inner) => inner.sender(),
			Self::LegacyAdd(inner) => inner.sender(),
			Self::Remove(inner) => inner.sender(),
		}
	}
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for CacheEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			AddAccessibleEvent::DBUS_MEMBER => {
				let body = msg.body();
				let sig = body.signature();
				if sig == CacheItem::SIGNATURE {
					Ok(CacheEvents::Add(AddAccessibleEvent::from_message_unchecked(msg, hdr)?))
				} else if sig == LegacyCacheItem::SIGNATURE {
					Ok(CacheEvents::LegacyAdd(LegacyAddAccessibleEvent::from_message_unchecked(
						msg, hdr,
					)?))
				} else {
					Err(AtspiError::SignatureMatch(format!(
						"No matching event for signature {} in interface {}",
						&sig.to_string(),
						Self::DBUS_INTERFACE
					)))
				}
			}
			RemoveAccessibleEvent::DBUS_MEMBER => {
				Ok(CacheEvents::Remove(RemoveAccessibleEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch(format!(
				"No member {} in {}",
				member.as_str(),
				Self::DBUS_INTERFACE
			))),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for CacheEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
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
event_test_cases!(LegacyAddAccessibleEvent, Explicit);
impl_from_dbus_message!(LegacyAddAccessibleEvent, Explicit);
impl_event_properties!(LegacyAddAccessibleEvent);
impl_to_dbus_message!(LegacyAddAccessibleEvent);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LegacyAddAccessibleEvent,
	"AddAccessible",
	"org.a11y.atspi.Cache",
	"cache:add",
	"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'"
);

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

impl_from_user_facing_event_for_interface_event_enum!(
	AddAccessibleEvent,
	CacheEvents,
	CacheEvents::Add
);
impl_from_user_facing_type_for_event_enum!(AddAccessibleEvent, Event::Cache);
impl_try_from_event_for_user_facing_type!(AddAccessibleEvent, CacheEvents::Add, Event::Cache);
event_test_cases!(AddAccessibleEvent, Explicit);

impl_member_interface_registry_string_and_match_rule_for_event!(
	AddAccessibleEvent,
	"AddAccessible",
	"org.a11y.atspi.Cache",
	"cache:add",
	"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'"
);

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

impl_from_user_facing_event_for_interface_event_enum!(
	RemoveAccessibleEvent,
	CacheEvents,
	CacheEvents::Remove
);
impl_from_user_facing_type_for_event_enum!(RemoveAccessibleEvent, Event::Cache);
impl_try_from_event_for_user_facing_type!(RemoveAccessibleEvent, CacheEvents::Remove, Event::Cache);
event_test_cases!(RemoveAccessibleEvent, Explicit);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RemoveAccessibleEvent,
	"RemoveAccessible",
	"org.a11y.atspi.Cache",
	"cache:remove",
	"type='signal',interface='org.a11y.atspi.Cache',member='RemoveAccessible'"
);

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
