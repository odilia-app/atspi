use crate::{
	error::AtspiError,
	events::{BusProperties, EventBodyOwned, HasMatchRule, HasRegistryEventString, ObjectRef},
	Event,
};
use zbus_names::BusName;
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
	/// The [`ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// The width of a terminal emulator has changed sufficiently such that the number of characters
/// able to fit on one *visual* line has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnCountChangedEvent {
	/// The [`ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// The height of a terminal emulator has changed sufficiently such that the number of lines
/// able to fit within the terminal has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LineCountChangedEvent {
	/// The [`ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ApplicationChangedEvent {
	/// The [`ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// The width of a terminal emulator has changed sufficiently such that the number of characters
/// able to fit on one *visual* line has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CharWidthChangedEvent {
	/// The [`ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

impl BusProperties for LineChangedEvent {
	const DBUS_MEMBER: &'static str = "LineChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: ObjectRef, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> BusName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl BusProperties for ColumnCountChangedEvent {
	const DBUS_MEMBER: &'static str = "ColumncountChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: ObjectRef, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> BusName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl BusProperties for LineCountChangedEvent {
	const DBUS_MEMBER: &'static str = "LinecountChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: ObjectRef, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> BusName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl BusProperties for ApplicationChangedEvent {
	const DBUS_MEMBER: &'static str = "ApplicationChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: ObjectRef, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> BusName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl BusProperties for CharWidthChangedEvent {
	const DBUS_MEMBER: &'static str = "CharwidthChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: ObjectRef, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> BusName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for TerminalEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let header = ev.header();
		let member = header
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"LineChanged" => Ok(TerminalEvents::LineChanged(ev.try_into()?)),
			"ColumncountChanged" => Ok(TerminalEvents::ColumnCountChanged(ev.try_into()?)),
			"LinecountChanged" => Ok(TerminalEvents::LineCountChanged(ev.try_into()?)),
			"ApplicationChanged" => Ok(TerminalEvents::ApplicationChanged(ev.try_into()?)),
			"CharwidthChanged" => Ok(TerminalEvents::CharWidthChanged(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Terminal".into())),
		}
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
impl From<LineChangedEvent> for EventBodyOwned {
	fn from(_event: LineChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: u8::default().into(),
		}
	}
}

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
impl From<ColumnCountChangedEvent> for EventBodyOwned {
	fn from(_event: ColumnCountChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: u8::default().into(),
		}
	}
}

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
impl From<LineCountChangedEvent> for EventBodyOwned {
	fn from(_event: LineCountChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: u8::default().into(),
		}
	}
}

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
impl From<ApplicationChangedEvent> for EventBodyOwned {
	fn from(_event: ApplicationChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: u8::default().into(),
		}
	}
}

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
impl From<CharWidthChangedEvent> for EventBodyOwned {
	fn from(_event: CharWidthChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: u8::default().into(),
		}
	}
}

impl HasRegistryEventString for TerminalEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
}
