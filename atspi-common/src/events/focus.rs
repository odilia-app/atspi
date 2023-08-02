use crate::{
	error::AtspiError,
	events::{EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString, ObjectReference},
	Event,
};
use zvariant::ObjectPath;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum FocusEvents {
	/// See: [`FocusEvent`].
	Focus(FocusEvent),
}
impl_event_conversions!(FocusEvents, Event::Focus);
event_wrapper_test_cases!(FocusEvents, FocusEvent);

impl HasMatchRule for FocusEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus'";
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct FocusEvent {
	/// The [`ObjectReference`] which the event applies to.
	pub item: crate::events::ObjectReference,
}

impl GenericEvent<'_> for FocusEvent {
	const DBUS_MEMBER: &'static str = "Focus";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
	const REGISTRY_EVENT_STRING: &'static str = "Focus:";

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
impl TryFrom<&zbus::Message> for FocusEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let member = ev
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"Focus" => Ok(FocusEvents::Focus(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Focus".into())),
		}
	}
}

impl_event_conversions!(FocusEvent, FocusEvents, FocusEvents::Focus, Event::Focus);
event_test_cases!(FocusEvent);
impl_to_dbus_message!(FocusEvent);
impl_from_dbus_message!(FocusEvent);
impl From<FocusEvent> for EventBodyOwned {
	fn from(_event: FocusEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl HasRegistryEventString for FocusEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Focus:";
}
