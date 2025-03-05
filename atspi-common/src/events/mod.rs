pub mod cache;
pub mod document;
#[cfg(feature = "wrappers")]
pub mod event_wrappers;

pub mod event_body;
pub use event_body::{EventBodyBorrowed, EventBodyOwned, EventBodyQtBorrowed, EventBodyQtOwned};
#[cfg(feature = "wrappers")]
pub use event_wrappers::{
	CacheEvents, DocumentEvents, Event, FocusEvents, KeyboardEvents, MouseEvents, ObjectEvents,
	TerminalEvents, WindowEvents,
};

pub mod event_listeners;
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
use crate::{AtspiError, ObjectRef};
pub use event_body::{
	EventBody, EventBodyBorrowed, EventBodyOwned, EventBodyQtBorrowed, EventBodyQtOwned,
};
pub use event_listeners::{
	EventListenerDeregisteredEvent, EventListenerEvents, EventListenerRegisteredEvent,
};
use zbus_names::UniqueName;
use zvariant::{ObjectPath, Type};

#[cfg(feature = "zbus")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

/// Encapsulates the various different accessibility bus signal types.
///
/// Assumes being non exhaustive to allow for future- or custom signals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Event {
	/// See: [`DocumentEvents`].
	Document(DocumentEvents),
	/// See: [`FocusEvents`].
	Focus(FocusEvents),
	/// See: [`KeyboardEvents`].
	Keyboard(KeyboardEvents),
	/// See: [`MouseEvents`].
	Mouse(MouseEvents),
	/// See: [`ObjectEvents`].
	Object(ObjectEvents),
	/// See: [`TerminalEvents`].
	Terminal(TerminalEvents),
	/// See: [`WindowEvents`].
	Window(WindowEvents),
	/// See: [`AvailableEvent`].
	Available(AvailableEvent),
	/// See: [`CacheEvents`].
	Cache(CacheEvents),
	/// See: [`EventListenerEvents`].
	Listener(EventListenerEvents),
}

impl EventTypeProperties for Event {
	fn member(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.member(),
			Self::Focus(inner) => inner.member(),
			Self::Keyboard(inner) => inner.member(),
			Self::Mouse(inner) => inner.member(),
			Self::Object(inner) => inner.member(),
			Self::Terminal(inner) => inner.member(),
			Self::Window(inner) => inner.member(),
			Self::Available(inner) => inner.member(),
			Self::Cache(inner) => inner.member(),
			Self::Listener(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.interface(),
			Self::Focus(inner) => inner.interface(),
			Self::Keyboard(inner) => inner.interface(),
			Self::Mouse(inner) => inner.interface(),
			Self::Object(inner) => inner.interface(),
			Self::Terminal(inner) => inner.interface(),
			Self::Window(inner) => inner.interface(),
			Self::Available(inner) => inner.interface(),
			Self::Cache(inner) => inner.interface(),
			Self::Listener(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.match_rule(),
			Self::Focus(inner) => inner.match_rule(),
			Self::Keyboard(inner) => inner.match_rule(),
			Self::Mouse(inner) => inner.match_rule(),
			Self::Object(inner) => inner.match_rule(),
			Self::Terminal(inner) => inner.match_rule(),
			Self::Window(inner) => inner.match_rule(),
			Self::Available(inner) => inner.match_rule(),
			Self::Cache(inner) => inner.match_rule(),
			Self::Listener(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.registry_string(),
			Self::Focus(inner) => inner.registry_string(),
			Self::Keyboard(inner) => inner.registry_string(),
			Self::Mouse(inner) => inner.registry_string(),
			Self::Object(inner) => inner.registry_string(),
			Self::Terminal(inner) => inner.registry_string(),
			Self::Window(inner) => inner.registry_string(),
			Self::Available(inner) => inner.registry_string(),
			Self::Cache(inner) => inner.registry_string(),
			Self::Listener(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for Event {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Document(inner) => inner.path(),
			Self::Focus(inner) => inner.path(),
			Self::Keyboard(inner) => inner.path(),
			Self::Mouse(inner) => inner.path(),
			Self::Object(inner) => inner.path(),
			Self::Terminal(inner) => inner.path(),
			Self::Window(inner) => inner.path(),
			Self::Available(inner) => inner.path(),
			Self::Cache(inner) => inner.path(),
			Self::Listener(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Document(inner) => inner.sender(),
			Self::Focus(inner) => inner.sender(),
			Self::Keyboard(inner) => inner.sender(),
			Self::Mouse(inner) => inner.sender(),
			Self::Object(inner) => inner.sender(),
			Self::Terminal(inner) => inner.sender(),
			Self::Window(inner) => inner.sender(),
			Self::Available(inner) => inner.sender(),
			Self::Cache(inner) => inner.sender(),
			Self::Listener(inner) => inner.sender(),
		}
	}
}

impl<T: BusProperties> HasMatchRule for T {
	const MATCH_RULE_STRING: &'static str = <T as BusProperties>::MATCH_RULE_STRING;
}
impl<T: BusProperties> HasRegistryEventString for T {
	const REGISTRY_EVENT_STRING: &'static str = <T as BusProperties>::REGISTRY_EVENT_STRING;
}
impl<T: BusProperties> HasInterfaceName for T {
	const DBUS_INTERFACE: &'static str = <T as BusProperties>::DBUS_INTERFACE;
}

/// An event that is emitted when the registry daemon has started.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Eq, Hash)]
pub struct AvailableEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRef,
	pub socket: ObjectRef,
}

#[cfg(feature = "wrappers")]
impl From<AvailableEvent> for Event {
	fn from(ev: AvailableEvent) -> Event {
		Event::Available(ev)
	}
}

#[cfg(feature = "wrappers")]
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

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let socket = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item, socket })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
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

#[cfg(all(feature = "zbus", feature = "wrappers"))]
impl TryFrom<&zbus::Message> for Event {
	type Error = AtspiError;

	fn try_from(msg: &zbus::Message) -> Result<Event, AtspiError> {
		let header = msg.header();
		let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
		let interface_str = interface.as_str();

		match interface_str {
			<ObjectEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Object(ObjectEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<FocusEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Focus(FocusEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<CacheEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Cache(CacheEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<WindowEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Window(WindowEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<MouseEvents as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(Event::Mouse(MouseEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<TerminalEvents as HasInterfaceName>::DBUS_INTERFACE => Ok(Event::Terminal(
				TerminalEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<DocumentEvents as HasInterfaceName>::DBUS_INTERFACE => Ok(Event::Document(
				DocumentEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<KeyboardEvents as HasInterfaceName>::DBUS_INTERFACE => Ok(Event::Keyboard(
				KeyboardEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<EventListenerEvents as HasInterfaceName>::DBUS_INTERFACE => Ok(Event::Listener(
				EventListenerEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<AvailableEvent as HasInterfaceName>::DBUS_INTERFACE => {
				Ok(AvailableEvent::try_from(msg)?.into())
			}
			_ => Err(AtspiError::InterfaceMatch(format!(
				"No events found with interface {interface_str}"
			))),
		}
	}
}

impl<T: BusProperties> EventTypeProperties for T {
	fn member(&self) -> &'static str {
		<T>::DBUS_MEMBER
	}
	fn interface(&self) -> &'static str {
		<T>::DBUS_INTERFACE
	}
	fn match_rule(&self) -> &'static str {
		<T>::MATCH_RULE_STRING
	}
	fn registry_string(&self) -> &'static str {
		<T>::REGISTRY_EVENT_STRING
	}
}

assert_obj_safe!(EventTypeProperties);

assert_obj_safe!(EventProperties);

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
	/// For example, for an [`object::TextChangedEvent`] this should be `"TextChanged"`
	const DBUS_MEMBER: &'static str;
	/// The `DBus` interface name for this event.
	/// For example, for any event within [`object`], this should be "org.a11y.atspi.Event.Object".
	const DBUS_INTERFACE: &'static str;
	/// A static match rule string for `DBus`.
	/// This should usually be a string that looks like this: `"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'"`;
	/// This should be deprecated in favour of composing the string from [`Self::DBUS_MEMBER`] and [`Self::DBUS_INTERFACE`].
	const MATCH_RULE_STRING: &'static str;
	/// A registry event string for registering for event receiving via the `RegistryProxy`.
	/// This should be deprecated in favour of composing the string from [`Self::DBUS_MEMBER`] and [`Self::DBUS_INTERFACE`].
	const REGISTRY_EVENT_STRING: &'static str;
}

#[cfg(feature = "zbus")]
pub trait MessageConversion<'a>: BusProperties {
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

/// A specific trait *only* to define an interface name.
/// This is useful for event wrappers like [`ObjectEvents`], which, while it does not have other
/// information required to implement the [`BusProperties`] trait, you can indeed attach in
/// interface name for all sub events of [`ObjectEvents`].
///
/// This trait *is not* object-safe.
pub trait HasInterfaceName {
	/// A static interface string for `DBus`.
	/// This should usually be a string that looks like this: `"org.a11y.atspi.Event.*"`;
	const DBUS_INTERFACE: &'static str;
}

/// A specific trait *only* to define match rules.
///
/// This is useful for event wrappers like [`ObjectEvents`], which, while it does not have other
/// information required to implement the [`BusProperties`] trait, you can indeed add a match rule
/// to the `DBus` connection to capture all sub events of [`ObjectEvents`].
///
/// This trait *is not* object-safe.
pub trait HasMatchRule {
	/// A static match rule string for `DBus`.
	/// This should usually be a string that looks like this: `"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'"`;
	/// This should be deprecated in favour of composing the string from [`BusProperties::DBUS_MEMBER`] and [`BusProperties::DBUS_INTERFACE`].
	const MATCH_RULE_STRING: &'static str;
}

/// A specific trait *only* to define registry event matches.
///
/// This is useful for event wrappers like [`ObjectEvents`], which, while it does not have other
/// information required to implement the [`BusProperties`] trait, you can indeed add a match rule
/// to the AT-SPI connection to subscribe to all sub events of [`ObjectEvents`].
///
/// This trait *is not* object-safe.
pub trait HasRegistryEventString {
	/// A registry event string for registering for event receiving via the `RegistryProxy`.
	/// This should be deprecated in favour of composing the string from [`BusProperties::DBUS_MEMBER`] and [`BusProperties::DBUS_INTERFACE`].
	const REGISTRY_EVENT_STRING: &'static str;
}

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
