use crate::{
	error::AtspiError,
	events::{ObjectReference, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
	Event,
};
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
impl_event_conversions!(TerminalEvents, Event::Terminal);
event_wrapper_test_cases!(TerminalEvents, LineChangedEvent);

impl HasMatchRule for TerminalEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal'";
}

/// A line of text has been changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LineChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::ObjectReference,
}

/// The width of a terminal emulator has changed sufficiently such that the number of characters
/// able to fit on one *visual* line has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnCountChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::ObjectReference,
}

/// The height of a terminal emulator has changed sufficiently such that the number of lines
/// able to fit within the terminal has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LineCountChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::ObjectReference,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ApplicationChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::ObjectReference,
}

/// The width of a terminal emulator has changed sufficiently such that the number of characters
/// able to fit on one *visual* line has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CharWidthChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::ObjectReference,
}

impl GenericEvent<'_> for LineChangedEvent {
	const DBUS_MEMBER: &'static str = "LineChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: ObjectReference, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl GenericEvent<'_> for ColumnCountChangedEvent {
	const DBUS_MEMBER: &'static str = "ColumncountChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: ObjectReference, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl GenericEvent<'_> for LineCountChangedEvent {
	const DBUS_MEMBER: &'static str = "LinecountChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: ObjectReference, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl GenericEvent<'_> for ApplicationChangedEvent {
	const DBUS_MEMBER: &'static str = "ApplicationChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: ObjectReference, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl GenericEvent<'_> for CharWidthChangedEvent {
	const DBUS_MEMBER: &'static str = "CharwidthChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: ObjectReference, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
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
		let member = ev
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

impl_event_conversions!(
	LineChangedEvent,
	TerminalEvents,
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
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(
	ColumnCountChangedEvent,
	TerminalEvents,
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
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(
	LineCountChangedEvent,
	TerminalEvents,
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
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(
	ApplicationChangedEvent,
	TerminalEvents,
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
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(
	CharWidthChangedEvent,
	TerminalEvents,
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
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl HasRegistryEventString for TerminalEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
}
