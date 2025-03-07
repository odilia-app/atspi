pub mod cache;
pub mod document;
#[cfg(feature = "wrappers")]
pub mod event_wrappers;

pub mod event_body;
pub mod focus;
pub mod keyboard;
pub mod mouse;
pub mod object;
pub mod terminal;
pub mod traits;
pub mod window;
pub use traits::*;

pub use crate::events::{
	cache::CacheEvents, document::DocumentEvents, focus::FocusEvents, keyboard::KeyboardEvents,
	mouse::MouseEvents, object::ObjectEvents, terminal::TerminalEvents, window::WindowEvents,
};
use crate::{AtspiError, AvailableEvent, EventListenerEvents, ObjectRef};
pub use event_body::{
	EventBody, EventBodyBorrowed, EventBodyOwned, EventBodyQtBorrowed, EventBodyQtOwned,
};
use zbus_names::UniqueName;
use zvariant::{ObjectPath, Type};

#[cfg(feature = "zbus")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

assert_obj_safe!(EventTypeProperties);
assert_obj_safe!(EventProperties);

#[cfg(feature = "zbus")]
pub trait MessageConversion<'a>: DBusProperties {
	/// What is the body type of this event.
	type Body<'msg>: Type + Deserialize<'msg> + Serialize
	where
		Self: 'msg;

	/// Build an event from a [`zbus::Message`] reference.
	/// This function will not check for any of the following error conditions:
	///
	/// - That the message has an interface: [`type@AtspiError::MissingInterface`]
	/// - That the message interface matches the one for the event: [`type@AtspiError::InterfaceMatch`]
	/// - That the message has an member: [`type@AtspiError::MissingMember`]
	/// - That the message member matches the one for the event: [`type@AtspiError::MemberMatch`]
	/// - That the message has an signature: [`type@AtspiError::MissingSignature`]
	/// - That the message signature matches the one for the event: [`type@AtspiError::SignatureMatch`]
	///
	/// Therefore, this should only be used when one has checked the above conditions.
	/// These must be checked manually.
	/// Alternatively, there is the [`MessageConversionExt::try_from_message`] that will check these
	/// conditions for you.
	///
	/// This type also implements `TryFrom<&zbus::Message>`; consider using this if you are not an
	/// internal developer.
	///
	/// # Errors
	///
	/// It is possible to get a [`type@AtspiError::Zvariant`] error if you do not check the proper
	/// conditions before calling this.
	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError>
	where
		Self: Sized + 'a;

	/// Build an event from an [`ObjectRef`] and [`Self::Body`].
	/// This function will not check for any of the following error conditions:
	///
	/// - That the message has an interface: [`type@AtspiError::MissingInterface`]
	/// - That the message interface matches the one for the event: [`type@AtspiError::InterfaceMatch`]
	/// - That the message has an member: [`type@AtspiError::MissingMember`]
	/// - That the message member matches the one for the event: [`type@AtspiError::MemberMatch`]
	///
	/// Therefore, this should only be used when one has checked the above conditions.
	///
	/// # Errors
	///
	/// Some [`Self::Body`] types may fallibly convert data fields contained in the body.
	/// If this happens, then the function will return an error.
	fn from_message_unchecked_parts(obj_ref: ObjectRef, body: DbusBody) -> Result<Self, AtspiError>
	where
		Self: Sized;

	/// The body of the object.
	fn body(&self) -> Self::Body<'_>;
}

#[cfg(feature = "zbus")]
pub trait MessageConversionExt<'a, B>: 'a + MessageConversion<'a, Body<'a> = B>
where
	B: Type + Serialize + Deserialize<'a>,
{
	/// Convert a [`zbus::Message`] into this event type.
	/// Does all the validation for you.
	///
	/// # Errors
	///
	/// - The message does not have an interface: [`type@AtspiError::MissingInterface`]
	/// - The message interface does not match the one for the event: [`type@AtspiError::InterfaceMatch`]
	/// - The message does not have an member: [`type@AtspiError::MissingMember`]
	/// - The message member does not match the one for the event: [`type@AtspiError::MemberMatch`]
	/// - The message signature does not match the one for the event: [`type@AtspiError::SignatureMatch`]
	///
	/// See [`MessageConversion::from_message_unchecked`] for info on panic condition that should never
	/// happen.
	fn try_from_message(msg: &'a zbus::Message, hdr: &Header) -> Result<Self, AtspiError>
	where
		Self: Sized + 'a;

	/// Validate the interface string via [`zbus::message::Header::interface`] against `Self`'s assignment of [`BusProperties::DBUS_INTERFACE`]
	///
	/// # Errors
	///
	/// - [`type@AtspiError::MissingInterface`] if there is no interface
	/// - [`type@AtspiError::InterfaceMatch`] if the interfaces do not match
	#[inline]
	fn validate_interface(header: &Header) -> Result<(), AtspiError> {
		let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
		if interface != Self::DBUS_INTERFACE {
			return Err(AtspiError::InterfaceMatch(format!(
				"The interface {} does not match the signal's interface: {}",
				interface,
				Self::DBUS_INTERFACE,
			)));
		}
		Ok(())
	}

	/// Validate the member string via [`zbus::message::Header::member`] against `Self`'s assignment of [`BusProperties::DBUS_MEMBER`]
	///
	/// # Errors
	///
	/// - [`type@AtspiError::MissingMember`] if there is no member
	/// - [`type@AtspiError::MemberMatch`] if the members do not match
	fn validate_member(hdr: &Header) -> Result<(), AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		if member != Self::DBUS_MEMBER {
			return Err(AtspiError::MemberMatch(format!(
				"The member {} does not match the signal's member: {}",
				// unwrap is safe here because of guard above
				member,
				Self::DBUS_MEMBER,
			)));
		}
		Ok(())
	}

	/// Validate the body signature against the [`zvariant::Signature`] of [`MessageConversion::Body`]
	///
	/// # Errors
	///
	/// - [`type@AtspiError::SignatureMatch`] if the signatures do not match
	#[inline]
	fn validate_body(msg: &zbus::Message) -> Result<(), AtspiError> {
		let body = msg.body();
		let body_signature = body.signature();

		let expected_signature = B::SIGNATURE;
		if body_signature != expected_signature {
			return Err(AtspiError::SignatureMatch(format!(
				"The message signature {} does not match the signal's body signature: {}",
				body_signature,
				&expected_signature.to_string(),
			)));
		}
		Ok(())
	}
}

/// The `DBus` member for the event.
/// For example, for an [`object::TextChangedEvent`] this should be `"TextChanged"`
pub trait DBusMember {
	const DBUS_MEMBER: &'static str;
}

/// The `DBus` interface name for an event - or a wrapper type.
/// For example, for any event within [`object`], this should be "org.a11y.atspi.Event.Object".
pub trait DBusInterface {
	/// A static interface string for `DBus`.
	/// This should usually be a string that looks like this: `"org.a11y.atspi.Event.*"`;
	const DBUS_INTERFACE: &'static str;
}

/// A static `DBus` match rule string.
/// This should usually be a string that looks like this:
/// `"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'"`;
// We cannot concat! consts, so we (at time of writing) need to have a separate trait for this.
// Otherwise composing from `DBusMember` and `DBusInterface` would be preferred.
pub trait DBusMatchRule {
	/// A static match rule string for `DBus`.
	const MATCH_RULE_STRING: &'static str;
}

/// A static `Registry` event string for registering with the `RegistryProxy` for receiving events.
pub trait RegistryEventString {
	/// A registry event string for registering for event receiving via the `RegistryProxy`.
	/// This should be deprecated in favour of composing the string from [`BusProperties::DBUS_MEMBER`] and [`BusProperties::DBUS_INTERFACE`].
	const REGISTRY_EVENT_STRING: &'static str;
}

/// A 'alias'-trait that combines all the `DBus` related traits.
pub trait DBusProperties: DBusMember + DBusInterface + DBusMatchRule + RegistryEventString {}

/// A way to convert a [`zbus::Message`] without checking its interface.
#[cfg(feature = "zbus")]
pub(crate) trait EventWrapperMessageConversion {
	/// # Errors
	/// Will fail if no matching member or body signature is found.
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError>
	where
		Self: Sized;
}

#[cfg(feature = "zbus")]
pub(crate) trait TryFromMessage {
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError>
	where
		Self: Sized;
}
