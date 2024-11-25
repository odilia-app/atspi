#[cfg(feature = "zbus")]
use crate::events::{
	EventWrapperMessageConversion, MessageConversion, MessageConversionExt, TryFromMessage,
};
use crate::{
	cache::{CacheItem, LegacyCacheItem},
	error::AtspiError,
	events::{BusProperties, HasInterfaceName, HasMatchRule, HasRegistryEventString, ObjectRef},
	Event, EventProperties, EventTypeProperties,
};
use serde::{Deserialize, Serialize};
use zbus_names::UniqueName;
use zvariant::{ObjectPath, Type};

/// All events related to the `org.a11y.atspi.Cache` interface.
/// Note that these are not telling the client that an item *has been added* to a cache.
/// It is telling the client "here is a bunch of information to store it in your cache".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum CacheEvents<'a> {
	/// See: [`AddAccessibleEvent`].
	Add(AddAccessibleEvent<'a>),
	/// See: [`LegacyAddAccessibleEvent`].
	LegacyAdd(LegacyAddAccessibleEvent<'a>),
	/// See: [`RemoveAccessibleEvent`].
	Remove(RemoveAccessibleEvent<'a>),
}

impl HasMatchRule for CacheEvents<'_> {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Cache'";
}

impl HasRegistryEventString for CacheEvents<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "Cache";
}

impl HasInterfaceName for CacheEvents<'_> {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

impl EventTypeProperties for CacheEvents<'_> {
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

impl EventProperties for CacheEvents<'_> {
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
impl EventWrapperMessageConversion for CacheEvents<'_> {
	fn try_from_message_interface_checked(msg: zbus::Message) -> Result<Self, AtspiError> {
		let member = msg.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			AddAccessibleEvent::DBUS_MEMBER => {
				let sig = msg.signature();
				if sig == CacheItem::SIGNATURE {
					Ok(CacheEvents::Add(AddAccessibleEvent::from_message_unchecked(msg)?))
				} else if sig == LegacyCacheItem::SIGNATURE {
					Ok(CacheEvents::LegacyAdd(LegacyAddAccessibleEvent::from_message_unchecked(
						msg,
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
				Ok(CacheEvents::Remove(RemoveAccessibleEvent::from_message_unchecked(msg)?))
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
impl TryFrom<zbus::Message> for CacheEvents<'_> {
	type Error = AtspiError;
	fn try_from(msg: zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::LegacyCacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct LegacyAddAccessibleEvent<'a> {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	/// A cache item to add to the internal cache.
	pub node_added: LegacyCacheItem,
	_marker: core::marker::PhantomData<&'a u8>,
}

impl_from_user_facing_event_for_interface_event_enum_borrow!(
	LegacyAddAccessibleEvent<'a>,
	CacheEvents<'a>,
	CacheEvents::LegacyAdd
);
impl_from_user_facing_type_for_event_enum!(LegacyAddAccessibleEvent<'a>, Event::Cache);
impl_try_from_event_for_user_facing_type_borrow!(
	LegacyAddAccessibleEvent<'a>,
	CacheEvents::LegacyAdd,
	Event::Cache
);
event_test_cases_borrow!(LegacyAddAccessibleEvent, LegacyAddAccessibleEvent<'a>, Explicit);
impl_from_dbus_message!(LegacyAddAccessibleEvent<'_>, Explicit);
impl_event_properties!(LegacyAddAccessibleEvent<'_>);
impl_to_dbus_message!(LegacyAddAccessibleEvent<'_>);

impl BusProperties for LegacyAddAccessibleEvent<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Add";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'";
	const DBUS_MEMBER: &'static str = "AddAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

#[cfg(feature = "zbus")]
impl MessageConversion for LegacyAddAccessibleEvent<'_> {
	type Body = LegacyCacheItem;

	fn from_message_unchecked_parts(item: ObjectRef, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, node_added: body, _marker: core::marker::PhantomData })
	}
	fn from_message_unchecked(msg: zbus::Message) -> Result<Self, AtspiError> {
		let item = (&msg).try_into()?;
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
pub struct AddAccessibleEvent<'a> {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	/// A cache item to add to the internal cache.
	pub node_added: CacheItem,
	_marker: core::marker::PhantomData<&'a u8>,
}

impl_from_user_facing_event_for_interface_event_enum_borrow!(
	AddAccessibleEvent<'a>,
	CacheEvents<'a>,
	CacheEvents::Add
);
impl_from_user_facing_type_for_event_enum!(AddAccessibleEvent<'a>, Event::Cache);
impl_try_from_event_for_user_facing_type_borrow!(
	AddAccessibleEvent<'a>,
	CacheEvents::Add,
	Event::Cache
);
event_test_cases_borrow!(AddAccessibleEvent, AddAccessibleEvent<'a>, Explicit);

impl BusProperties for AddAccessibleEvent<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Add";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'";
	const DBUS_MEMBER: &'static str = "AddAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

#[cfg(feature = "zbus")]
impl MessageConversion for AddAccessibleEvent<'_> {
	type Body = CacheItem;

	fn from_message_unchecked_parts(item: ObjectRef, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, node_added: body, _marker: core::marker::PhantomData })
	}
	fn from_message_unchecked(msg: zbus::Message) -> Result<Self, AtspiError> {
		let item = (&msg).try_into()?;
		let body = msg.body().deserialize()?;
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body {
		self.node_added.clone()
	}
}

impl_from_dbus_message!(AddAccessibleEvent<'_>, Explicit);
impl_event_properties!(AddAccessibleEvent<'_>);
impl_to_dbus_message!(AddAccessibleEvent<'_>);

/// `Cache::RemoveAccessible` signal event type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct RemoveAccessibleEvent<'a> {
	/// The application that emitted the signal TODO Check Me
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	/// The node that was removed from the application tree  TODO Check Me
	pub node_removed: ObjectRef,
	_marker: core::marker::PhantomData<&'a u8>,
}

impl_from_user_facing_event_for_interface_event_enum_borrow!(
	RemoveAccessibleEvent<'a>,
	CacheEvents<'a>,
	CacheEvents::Remove
);
impl_from_user_facing_type_for_event_enum!(RemoveAccessibleEvent<'a>, Event::Cache);
impl_try_from_event_for_user_facing_type_borrow!(
	RemoveAccessibleEvent<'a>,
	CacheEvents::Remove,
	Event::Cache
);
event_test_cases_borrow!(RemoveAccessibleEvent, RemoveAccessibleEvent<'a>, Explicit);

impl BusProperties for RemoveAccessibleEvent<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "Cache:Remove";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Cache',member='RemoveAccessible'";
	const DBUS_MEMBER: &'static str = "RemoveAccessible";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

#[cfg(feature = "zbus")]
impl MessageConversion for RemoveAccessibleEvent<'_> {
	type Body = ObjectRef;

	fn from_message_unchecked_parts(item: ObjectRef, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, node_removed: body, _marker: core::marker::PhantomData })
	}
	fn from_message_unchecked(msg: zbus::Message) -> Result<Self, AtspiError> {
		let item = (&msg).try_into()?;
		let body = msg.body().deserialize()?;
		Self::from_message_unchecked_parts(item, body)
	}
	fn body(&self) -> Self::Body {
		self.node_removed.clone()
	}
}

impl_from_dbus_message!(RemoveAccessibleEvent<'_>, Explicit);
impl_event_properties!(RemoveAccessibleEvent<'_>);
impl_to_dbus_message!(RemoveAccessibleEvent<'_>);
