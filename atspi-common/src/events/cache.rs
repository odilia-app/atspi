use crate::cache::{CacheItem, LegacyCacheItem};
#[cfg(feature = "zbus")]
use crate::error::AtspiError;
use crate::events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString};
#[cfg(feature = "zbus")] // TODO: Should this be behind a feature, really?
use crate::object_ref::NonNullObjectRef;
#[cfg(feature = "zbus")]
use crate::EventProperties;
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

#[cfg(feature = "zbus")]
use super::{MessageConversion, MessageConversionExt};

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::LegacyCacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct LegacyAddAccessibleEvent<'a> {
	/// The [`crate::NonNullObjectRef`] the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
	/// A cache item to add to the internal cache.
	pub node_added: LegacyCacheItem,
}

impl_event_type_properties_for_event!(LegacyAddAccessibleEvent<'_>);

event_test_cases!(LegacyAddAccessibleEvent, [node_added], Explicit);
impl_from_dbus_message!(LegacyAddAccessibleEvent<'_>, Explicit);
impl_event_properties!(LegacyAddAccessibleEvent<'_>);
impl_to_dbus_message!(LegacyAddAccessibleEvent<'_>);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LegacyAddAccessibleEvent<'_>,
	"AddAccessible",
	"org.a11y.atspi.Cache",
	"cache:add",
	"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'"
);

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for LegacyAddAccessibleEvent<'_> {
	type Body<'msg>
		= LegacyCacheItem
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'_>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		Ok(Self {
			item: item.into_owned(),
			node_added: body.deserialize_unchecked::<Self::Body<'_>>()?,
		})
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct AddAccessibleEvent<'o> {
	/// The [`NonNullObjectRef`] the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'o>,
	/// A cache item to add to the internal cache.
	pub node_added: CacheItem,
}

impl_event_type_properties_for_event!(AddAccessibleEvent<'_>);

event_test_cases!(AddAccessibleEvent, [node_added], Explicit);

impl_member_interface_registry_string_and_match_rule_for_event!(
	AddAccessibleEvent<'_>,
	"AddAccessible",
	"org.a11y.atspi.Cache",
	"cache:add",
	"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for AddAccessibleEvent<'a> {
	type Body<'msg>
		= CacheItem
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'a>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		Ok(Self { item, node_added: body.deserialize_unchecked::<Self::Body<'_>>()? })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.node_added.clone()
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: AddAccessibleEvent<'_>, body: CacheItem);
impl_from_dbus_message!(AddAccessibleEvent<'_>, Explicit);
impl_event_properties!(AddAccessibleEvent<'_>);
impl_to_dbus_message!(AddAccessibleEvent<'_>);

/// `Cache::RemoveAccessible` signal event type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct RemoveAccessibleEvent<'o> {
	/// The application that emitted the signal
	/// The [`crate::NonNullObjectRef`] the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'o>,

	// It is not expected to receive a NULL here as that would not be a helpful signal.
	/// The node that was removed from the application tree  TODO Check Me
	pub node_removed: crate::ObjectRefOwned,
}

impl_event_type_properties_for_event!(RemoveAccessibleEvent<'_>);
event_test_cases!(RemoveAccessibleEvent, [node_removed], Explicit);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RemoveAccessibleEvent<'_>,
	"RemoveAccessible",
	"org.a11y.atspi.Cache",
	"cache:remove",
	"type='signal',interface='org.a11y.atspi.Cache',member='RemoveAccessible'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for RemoveAccessibleEvent<'a> {
	type Body<'msg>
		= crate::ObjectRefOwned
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'a>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		Ok(Self { item, node_removed: body.deserialize_unchecked::<Self::Body<'_>>()? })
	}

	fn from_message_unchecked(msg: &'a zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.node_removed.clone()
	}
}

#[cfg(feature = "zbus")]
impl<'a> MessageConversionExt<'a, LegacyCacheItem> for LegacyAddAccessibleEvent<'a> {
	fn try_from_message(msg: &'a zbus::Message, hdr: &'a Header) -> Result<Self, AtspiError> {
		<LegacyAddAccessibleEvent<'a> as MessageConversionExt<'a, crate::LegacyCacheItem>>::validate_interface(hdr)?;
		<LegacyAddAccessibleEvent<'a> as MessageConversionExt<'a, crate::LegacyCacheItem>>::validate_member(hdr)?;
		<LegacyAddAccessibleEvent<'a> as MessageConversionExt<'a, crate::LegacyCacheItem>>::validate_body(
			msg,
		)?;
		<LegacyAddAccessibleEvent<'a> as MessageConversion<'a>>::from_message_unchecked(msg, hdr)
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: RemoveAccessibleEvent<'_>, body: crate::ObjectRefOwned);
impl_from_dbus_message!(RemoveAccessibleEvent<'_>, Explicit);
impl_event_properties!(RemoveAccessibleEvent<'_>);
impl_to_dbus_message!(RemoveAccessibleEvent<'_>);

impl_test_event!(
	LegacyAddAccessibleEvent<'_> { node_added },
	AddAccessibleEvent<'_> { node_added },
	// RemoveAccessibleEvent<'_> { node_removed },
);

// Because `RemoveAccessibleEvent`'s body derived field is an `ObjectRefOwned` which does have a Default impl
// but it is Null, which should never occur for this event, therefor we just implement it manually:

impl<'o> RemoveAccessibleEvent<'o> {
	#[doc(hidden)]
	#[must_use]
	pub fn new_test_event(origin: &crate::NonNullObjectRef<'o>) -> Self {
		RemoveAccessibleEvent {
			item: origin.clone(),
			node_removed: crate::object_ref::TEST_DEFAULT_OBJECT_REF.into(),
		}
	}
}
