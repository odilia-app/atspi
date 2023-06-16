use crate::{
	error::AtspiError,
	events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
	Event,
};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum TerminalEvents {
	LineChanged(LineChangedEvent),
	ColumnCountChanged(ColumnCountChangedEvent),
	LineCountChanged(LineCountChangedEvent),
	ApplicationChanged(ApplicationChangedEvent),
	CharWidthChanged(CharWidthChangedEvent),
}
impl_event_conversions!(TerminalEvents, Event::Terminal);
event_wrapper_test_cases!(TerminalEvents, LineChangedEvent);

impl HasMatchRule for TerminalEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal'";
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LineChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnCountChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LineCountChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ApplicationChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CharWidthChangedEvent {
	pub item: crate::events::Accessible,
}

impl GenericEvent<'_> for LineChangedEvent {
	const DBUS_MEMBER: &'static str = "LineChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> UniqueName<'_> {
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

/*
impl TryFrom<Event> for LineChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Terminal(TerminalEvents::LineChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ColumnCountChangedEvent {
	const DBUS_MEMBER: &'static str = "ColumncountChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> UniqueName<'_> {
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

/*
impl TryFrom<Event> for ColumnCountChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Terminal(TerminalEvents::ColumnCountChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for LineCountChangedEvent {
	const DBUS_MEMBER: &'static str = "LinecountChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> UniqueName<'_> {
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

/*
impl TryFrom<Event> for LineCountChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Terminal(TerminalEvents::LineCountChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ApplicationChangedEvent {
	const DBUS_MEMBER: &'static str = "ApplicationChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> UniqueName<'_> {
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

/*
impl TryFrom<Event> for ApplicationChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Terminal(TerminalEvents::ApplicationChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for CharWidthChangedEvent {
	const DBUS_MEMBER: &'static str = "CharwidthChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> UniqueName<'_> {
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

/*
impl TryFrom<Event> for CharWidthChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Terminal(TerminalEvents::CharWidthChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

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

/*impl HasMatchRule for LineChangedEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'";
}*/
/*impl HasMatchRule for ColumnCountChangedEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'";
}*/
/*impl HasMatchRule for LineCountChangedEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'";
}*/
/*impl HasMatchRule for ApplicationChangedEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'";
}*/
/*impl HasMatchRule for CharWidthChangedEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'";
}*/
/*impl HasRegistryEventString for LineChangedEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:LineChanged";
}*/
/*impl HasRegistryEventString for ColumnCountChangedEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:ColumncountChanged";
}*/
/*impl HasRegistryEventString for LineCountChangedEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:LinecountChanged";
}*/
/*impl HasRegistryEventString for ApplicationChangedEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:ApplicationChanged";
}*/
/*impl HasRegistryEventString for CharWidthChangedEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:CharwidthChanged";
}*/
impl HasRegistryEventString for TerminalEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
}
