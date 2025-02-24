#[cfg(feature = "zbus")]
use crate::AtspiError;
use crate::ObjectRef;
#[cfg(feature = "zbus")]
use serde::{Deserialize, Serialize};
use zbus_names::UniqueName;
use zvariant::ObjectPath;
#[cfg(feature = "zbus")]
use zvariant::Type;

/// Describes properties of a specific event _type_.
///
/// - `DBus` member name
/// - `DBus` interface name
///
/// Together, the member and interface name can describe a specific event _type_.
/// Likewise, the path and sender bus name collectively make up an [`ObjectRef`], which is a way to uniquely identify an individual accessible item available to `atspi`.
/// The latter is available via the [`EventProperties`] trait.
///
/// This can also be generalized, for example this is implemented for [`crate::Event`] by dispatching to the matching variants.
/// NOTE: to use `EventProperties` on wrapper types, like `Event`, you must enable the "enum-dispatch" feature.
///
/// This trait *is* object-safe.
pub trait EventTypeProperties {
	fn member(&self) -> &'static str;
	fn interface(&self) -> &'static str;
	fn match_rule(&self) -> &'static str;
	fn registry_string(&self) -> &'static str;
}

/// `EventProperties` allows access to the internals of an event, specifically:
///
/// - The `DBUs` name which sent the event.
/// - The `ObjectPath`, a unique id for a given application.
/// - Collectively, this is called an [`ObjectRef`].
///
/// This trait *is* object-safe.
pub trait EventProperties {
	fn sender(&self) -> UniqueName<'_>;
	fn path(&self) -> ObjectPath<'_>;
	fn object_ref(&self) -> ObjectRef {
		ObjectRef { name: self.sender().into(), path: self.path().into() }
	}
}

/// Describes the `DBus`-related information about a given struct.
///
/// - `DBus` member name
/// - `DBus` interface name
/// - `DBus` match string: used to tell `DBus` you are interested in a particular signal
/// - accessibility registry event string: used to tell the accessibility registry that you are interested in a particular event
///
/// This trait *is not* object-safe.
/// For a similar, but object-safe trait, see [`EventProperties`].
pub trait BusProperties {
	/// The `DBus` member for the event.
	/// For example, for an [`crate::events::object::TextChangedEvent`] this should be `"TextChanged"`
	const DBUS_MEMBER: &'static str;
	/// The `DBus` interface name for this event.
	/// For example, for any event within [`crate::events::object`], this should be "org.a11y.atspi.Event.Object".
	const DBUS_INTERFACE: &'static str;
	/// A static match rule string for `DBus`.
	/// This should usually be a string that looks like this: `"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'"`;
	/// This should be deprecated in favour of composing the string from [`Self::DBUS_MEMBER`] and [`Self::DBUS_INTERFACE`].
	const MATCH_RULE_STRING: &'static str;
	/// A registry event string for registering for event receiving via the `RegistryProxy`.
	/// This should be deprecated in favour of composing the string from [`Self::DBUS_MEMBER`] and [`Self::DBUS_INTERFACE`].
	const REGISTRY_EVENT_STRING: &'static str;
}

/// A specific trait *only* to define an interface name.
/// This is useful for event wrappers like [`crate::events::ObjectEvents`], which, while it does not have other
/// information required to implement the [`crate::BusProperties`] trait, you can indeed attach in
/// interface name for all sub events of [`crate::events::ObjectEvents`].
///
/// This trait *is not* object-safe.
pub trait HasInterfaceName {
	/// A static interface string for `DBus`.
	/// This should usually be a string that looks like this: `"org.a11y.atspi.Event.*"`;
	const DBUS_INTERFACE: &'static str;
}

/// A specific trait *only* to define match rules.
/// This is useful for event wrappers like [`crate::events::ObjectEvents`], which, while it does not have other
/// information required to implement the [`crate::BusProperties`] trait, you can indeed add a match rule
/// to the `DBus` connection to capture all sub events of [`crate::events::ObjectEvents`].
///
/// This trait *is not* object-safe.
pub trait HasMatchRule {
	/// A static match rule string for `DBus`.
	/// This should usually be a string that looks like this: `"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'"`;
	/// This should be deprecated in favour of composing the string from [`BusProperties::DBUS_MEMBER`] and [`BusProperties::DBUS_INTERFACE`].
	const MATCH_RULE_STRING: &'static str;
}

/// A specific trait *only* to define registry event matches.
/// This is useful for event wrappers like [`crate::events::ObjectEvents`], which, while it does not have other
/// information required to implement the [`crate::BusProperties`] trait, you can indeed add a match rule
/// to the AT-SPI connection to subscribe to all sub events of [`crate::events::ObjectEvents`].
///
/// This trait *is not* object-safe.
pub trait HasRegistryEventString {
	/// A registry event string for registering for event receiving via the `RegistryProxy`.
	/// This should be deprecated in favour of composing the string from [`BusProperties::DBUS_MEMBER`] and [`BusProperties::DBUS_INTERFACE`].
	const REGISTRY_EVENT_STRING: &'static str;
}

/// An way to convert a [`zbus::Message`] without checking its interface.
#[cfg(feature = "zbus")]
pub(crate) trait EventWrapperMessageConversion {
	/// # Errors
	/// Will fail if no matching member or body signature is found.
	fn try_from_message_interface_checked(msg: &zbus::Message) -> Result<Self, AtspiError>
	where
		Self: Sized;
}

#[cfg(feature = "zbus")]
pub(crate) trait TryFromMessage {
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError>
	where
		Self: Sized;
}

#[cfg(feature = "zbus")]
pub trait MessageConversionExt<B>: MessageConversion<Body = B>
where
	B: Type + Serialize + for<'a> Deserialize<'a>,
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
	fn try_from_message(msg: &zbus::Message) -> Result<Self, AtspiError>
	where
		Self: Sized;
	/// Validate the interface string via [`zbus::message::Header::interface`] against `Self`'s assignment of [`BusProperties::DBUS_INTERFACE`]
	///
	/// # Errors
	///
	/// - [`type@AtspiError::MissingInterface`] if there is no interface
	/// - [`type@AtspiError::InterfaceMatch`] if the interfaces do not match
	fn validate_interface(msg: &zbus::Message) -> Result<(), AtspiError> {
		let header = msg.header();
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
	fn validate_member(msg: &zbus::Message) -> Result<(), AtspiError> {
		let header = msg.header();
		let member = header.member().ok_or(AtspiError::MissingMember)?;
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
	fn validate_body(msg: &zbus::Message) -> Result<(), AtspiError> {
		let body = msg.body();
		let body_signature = body.signature();
		if body_signature != Self::Body::SIGNATURE {
			return Err(AtspiError::SignatureMatch(format!(
				"The message signature {} does not match the signal's body signature: {:?}",
				body_signature,
				Self::Body::SIGNATURE
			)));
		}
		Ok(())
	}
}

#[cfg(feature = "zbus")]
pub trait MessageConversion: BusProperties {
	/// What is the body type of this event.
	type Body: Type + Serialize + for<'a> Deserialize<'a>;

	/// Build an event from a [`zbus::Message`] reference.
	/// This function will not check for any of the following error conditions:
	///
	/// - That the message has an interface: [`type@AtspiError::MissingInterface`]
	/// - That the message interface matches the one for the event: [`type@AtspiError::InterfaceMatch`]
	/// - That the message has an member: [`type@AtspiError::MissingMember`]
	/// - That the message member matches the one for the event: [`type@AtspiError::MemberMatch`]
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
	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError>
	where
		Self: Sized;

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
	fn from_message_unchecked_parts(
		obj_ref: ObjectRef,
		body: Self::Body,
	) -> Result<Self, AtspiError>
	where
		Self: Sized;

	/// The body of the object.
	fn body(&self) -> Self::Body;
}
