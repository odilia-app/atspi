#[cfg(feature = "zbus")]
use crate::events::{
	EventWrapperMessageConversion, MessageConversion, MessageConversionExt, TryFromMessage,
};
use crate::{
	error::AtspiError,
	events::{BusProperties, HasInterfaceName, HasMatchRule, HasRegistryEventString},
	Event, EventProperties, EventTypeProperties,
};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

/// All events related to the `org.a11y.atspi.Event.Terminal` interface.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum TerminalEvents {
	/// See: [`LineChangedEvent`].
	LineChanged(LineChangedEvent),
	/// See: [`ColumnCountChangedEvent`].
	ColumnCountChanged(ColumnCountChangedEvent),
	/// See: [`LineCountChangedEvent`].
	LineCountChanged(LineCountChangedEvent),
	/// See: [`ApplicationChangedEvent`].
	ApplicationChanged(ApplicationChangedEvent),
	/// See: [`CharWidthChangedEvent`].
	CharWidthChanged(CharWidthChangedEvent),
}

impl EventTypeProperties for TerminalEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::LineChanged(inner) => inner.member(),
			Self::ColumnCountChanged(inner) => inner.member(),
			Self::LineCountChanged(inner) => inner.member(),
			Self::ApplicationChanged(inner) => inner.member(),
			Self::CharWidthChanged(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::LineChanged(inner) => inner.interface(),
			Self::ColumnCountChanged(inner) => inner.interface(),
			Self::LineCountChanged(inner) => inner.interface(),
			Self::ApplicationChanged(inner) => inner.interface(),
			Self::CharWidthChanged(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::LineChanged(inner) => inner.match_rule(),
			Self::ColumnCountChanged(inner) => inner.match_rule(),
			Self::LineCountChanged(inner) => inner.match_rule(),
			Self::ApplicationChanged(inner) => inner.match_rule(),
			Self::CharWidthChanged(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::LineChanged(inner) => inner.registry_string(),
			Self::ColumnCountChanged(inner) => inner.registry_string(),
			Self::LineCountChanged(inner) => inner.registry_string(),
			Self::ApplicationChanged(inner) => inner.registry_string(),
			Self::CharWidthChanged(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for TerminalEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::LineChanged(inner) => inner.path(),
			Self::ColumnCountChanged(inner) => inner.path(),
			Self::LineCountChanged(inner) => inner.path(),
			Self::ApplicationChanged(inner) => inner.path(),
			Self::CharWidthChanged(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::LineChanged(inner) => inner.sender(),
			Self::ColumnCountChanged(inner) => inner.sender(),
			Self::LineCountChanged(inner) => inner.sender(),
			Self::ApplicationChanged(inner) => inner.sender(),
			Self::CharWidthChanged(inner) => inner.sender(),
		}
	}
}

impl_from_interface_event_enum_for_event!(TerminalEvents, Event::Terminal);
impl_try_from_event_for_user_facing_event_type!(TerminalEvents, Event::Terminal);

event_wrapper_test_cases!(TerminalEvents, LineChangedEvent);

impl HasMatchRule for TerminalEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal'";
}

/// A line of text has been changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LineChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// The width of a terminal emulator has changed sufficiently such that the number of characters
/// able to fit on one *visual* line has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnCountChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// The height of a terminal emulator has changed sufficiently such that the number of lines
/// able to fit within the terminal has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LineCountChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ApplicationChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// The width of a terminal emulator has changed sufficiently such that the number of characters
/// able to fit on one *visual* line has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CharWidthChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

impl BusProperties for LineChangedEvent {
	const DBUS_MEMBER: &'static str = "LineChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
}

impl BusProperties for ColumnCountChangedEvent {
	const DBUS_MEMBER: &'static str = "ColumncountChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
}

impl BusProperties for LineCountChangedEvent {
	const DBUS_MEMBER: &'static str = "LinecountChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
}

impl BusProperties for ApplicationChangedEvent {
	const DBUS_MEMBER: &'static str = "ApplicationChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
}

impl BusProperties for CharWidthChangedEvent {
	const DBUS_MEMBER: &'static str = "CharwidthChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
}

impl HasInterfaceName for TerminalEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for TerminalEvents {
	fn try_from_message_interface_checked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let header = msg.header();
		let member = header
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			LineChangedEvent::DBUS_MEMBER => {
				Ok(TerminalEvents::LineChanged(LineChangedEvent::from_message_unchecked(msg)?))
			}
			ColumnCountChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::ColumnCountChanged(
				ColumnCountChangedEvent::from_message_unchecked(msg)?,
			)),
			LineCountChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::LineCountChanged(
				LineCountChangedEvent::from_message_unchecked(msg)?,
			)),
			ApplicationChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::ApplicationChanged(
				ApplicationChangedEvent::from_message_unchecked(msg)?,
			)),
			CharWidthChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::CharWidthChanged(
				CharWidthChangedEvent::from_message_unchecked(msg)?,
			)),
			_ => Err(AtspiError::MemberMatch("No matching member for Terminal".into())),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for TerminalEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	LineChangedEvent,
	TerminalEvents,
	TerminalEvents::LineChanged
);
impl_from_user_facing_type_for_event_enum!(LineChangedEvent, Event::Terminal);
impl_try_from_event_for_user_facing_type!(
	LineChangedEvent,
	TerminalEvents::LineChanged,
	Event::Terminal
);
event_test_cases!(LineChangedEvent);
impl_to_dbus_message!(LineChangedEvent);
impl_from_dbus_message!(LineChangedEvent);
impl_event_properties!(LineChangedEvent);
impl_from_object_ref!(LineChangedEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	ColumnCountChangedEvent,
	TerminalEvents,
	TerminalEvents::ColumnCountChanged
);
impl_from_user_facing_type_for_event_enum!(ColumnCountChangedEvent, Event::Terminal);
impl_try_from_event_for_user_facing_type!(
	ColumnCountChangedEvent,
	TerminalEvents::ColumnCountChanged,
	Event::Terminal
);
event_test_cases!(ColumnCountChangedEvent);
impl_to_dbus_message!(ColumnCountChangedEvent);
impl_from_dbus_message!(ColumnCountChangedEvent);
impl_event_properties!(ColumnCountChangedEvent);
impl_from_object_ref!(ColumnCountChangedEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	LineCountChangedEvent,
	TerminalEvents,
	TerminalEvents::LineCountChanged
);
impl_from_user_facing_type_for_event_enum!(LineCountChangedEvent, Event::Terminal);
impl_try_from_event_for_user_facing_type!(
	LineCountChangedEvent,
	TerminalEvents::LineCountChanged,
	Event::Terminal
);
event_test_cases!(LineCountChangedEvent);
impl_to_dbus_message!(LineCountChangedEvent);
impl_from_dbus_message!(LineCountChangedEvent);
impl_event_properties!(LineCountChangedEvent);
impl_from_object_ref!(LineCountChangedEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	ApplicationChangedEvent,
	TerminalEvents,
	TerminalEvents::ApplicationChanged
);
impl_from_user_facing_type_for_event_enum!(ApplicationChangedEvent, Event::Terminal);
impl_try_from_event_for_user_facing_type!(
	ApplicationChangedEvent,
	TerminalEvents::ApplicationChanged,
	Event::Terminal
);
event_test_cases!(ApplicationChangedEvent);
impl_to_dbus_message!(ApplicationChangedEvent);
impl_from_dbus_message!(ApplicationChangedEvent);
impl_event_properties!(ApplicationChangedEvent);
impl_from_object_ref!(ApplicationChangedEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	CharWidthChangedEvent,
	TerminalEvents,
	TerminalEvents::CharWidthChanged
);
impl_from_user_facing_type_for_event_enum!(CharWidthChangedEvent, Event::Terminal);
impl_try_from_event_for_user_facing_type!(
	CharWidthChangedEvent,
	TerminalEvents::CharWidthChanged,
	Event::Terminal
);
event_test_cases!(CharWidthChangedEvent);
impl_to_dbus_message!(CharWidthChangedEvent);
impl_from_dbus_message!(CharWidthChangedEvent);
impl_event_properties!(CharWidthChangedEvent);
impl_from_object_ref!(CharWidthChangedEvent);

impl HasRegistryEventString for TerminalEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
}
