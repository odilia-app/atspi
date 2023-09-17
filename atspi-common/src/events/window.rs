use crate::{
	error::AtspiError,
	events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
	Event,
};
use zvariant::ObjectPath;

/// All events on the `org.a11y.atspi.Event.Window` interface.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum WindowEvents {
	/// See: [`PropertyChangeEvent`].
	PropertyChange(PropertyChangeEvent),
	/// See: [`MinimizeEvent`].
	Minimize(MinimizeEvent),
	/// See: [`MaximizeEvent`].
	Maximize(MaximizeEvent),
	/// See: [`RestoreEvent`].
	Restore(RestoreEvent),
	/// See: [`CloseEvent`].
	Close(CloseEvent),
	/// See: [`CreateEvent`].
	Create(CreateEvent),
	/// See: [`ReparentEvent`].
	Reparent(ReparentEvent),
	/// See: [`DesktopCreateEvent`].
	DesktopCreate(DesktopCreateEvent),
	/// See: [`DesktopDestroyEvent`].
	DesktopDestroy(DesktopDestroyEvent),
	/// See: [`DestroyEvent`].
	Destroy(DestroyEvent),
	/// See: [`ActivateEvent`].
	Activate(ActivateEvent),
	/// See: [`DeactivateEvent`].
	Deactivate(DeactivateEvent),
	/// See: [`RaiseEvent`].
	Raise(RaiseEvent),
	/// See: [`LowerEvent`].
	Lower(LowerEvent),
	/// See: [`MoveEvent`].
	Move(MoveEvent),
	/// See: [`ResizeEvent`].
	Resize(ResizeEvent),
	/// See: [`ShadeEvent`].
	Shade(ShadeEvent),
	/// See: [`UUshadeEvent`].
	UUshade(UUshadeEvent),
	/// See: [`RestyleEvent`].
	Restyle(RestyleEvent),
}

impl_from_interface_event_enum_for_event!(WindowEvents, Event::Window);
impl_try_from_event_for_user_facing_event_type!(WindowEvents, Event::Window);

event_wrapper_test_cases!(WindowEvents, MoveEvent);

impl HasMatchRule for WindowEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window'";
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct PropertyChangeEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	pub property: String,
}

/// The window has been minimized.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MinimizeEvent {
	/// The application which has been minimized.
	pub item: crate::events::Accessible,
}

/// The window has been maximized.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MaximizeEvent {
	/// The application which has been maximized.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RestoreEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

/// A window has been closed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CloseEvent {
	/// The application which has been closed.
	pub item: crate::events::Accessible,
}

/// A new window has been created.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CreateEvent {
	/// An application to query for additional events from.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ReparentEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

/// A new virtual desktop has been created.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DesktopCreateEvent {
	/// A reference to a new desktop
	pub item: crate::events::Accessible,
}

/// A virtual desktop has been deleted.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DesktopDestroyEvent {
	/// A reference to the destroyed desktop.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DestroyEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ActivateEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DeactivateEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RaiseEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LowerEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MoveEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

/// A window has been resized.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ResizeEvent {
	/// The application which has been resized.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ShadeEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct UUshadeEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RestyleEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

impl GenericEvent<'_> for PropertyChangeEvent {
	const DBUS_MEMBER: &'static str = "PropertyChange";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='PropertyChange'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, property: body.kind })
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

impl GenericEvent<'_> for MinimizeEvent {
	const DBUS_MEMBER: &'static str = "Minimize";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Minimize'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for MaximizeEvent {
	const DBUS_MEMBER: &'static str = "Maximize";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Maximize'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for RestoreEvent {
	const DBUS_MEMBER: &'static str = "Restore";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Restore'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for CloseEvent {
	const DBUS_MEMBER: &'static str = "Close";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Close'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for CreateEvent {
	const DBUS_MEMBER: &'static str = "Create";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Create'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for ReparentEvent {
	const DBUS_MEMBER: &'static str = "Reparent";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Reparent'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for DesktopCreateEvent {
	const DBUS_MEMBER: &'static str = "DesktopCreate";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopCreate'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for DesktopDestroyEvent {
	const DBUS_MEMBER: &'static str = "DesktopDestroy";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopDestroy'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for DestroyEvent {
	const DBUS_MEMBER: &'static str = "Destroy";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Destroy'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for ActivateEvent {
	const DBUS_MEMBER: &'static str = "Activate";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Activate'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for DeactivateEvent {
	const DBUS_MEMBER: &'static str = "Deactivate";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Deactivate'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for RaiseEvent {
	const DBUS_MEMBER: &'static str = "Raise";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Raise'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for LowerEvent {
	const DBUS_MEMBER: &'static str = "Lower";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Lower'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for MoveEvent {
	const DBUS_MEMBER: &'static str = "Move";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Move'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for ResizeEvent {
	const DBUS_MEMBER: &'static str = "Resize";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Resize'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for ShadeEvent {
	const DBUS_MEMBER: &'static str = "Shade";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Shade'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for UUshadeEvent {
	const DBUS_MEMBER: &'static str = "uUshade";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='uUshade'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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

impl GenericEvent<'_> for RestyleEvent {
	const DBUS_MEMBER: &'static str = "Restyle";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Restyle'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
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
impl TryFrom<&zbus::Message> for WindowEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let member = ev
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"PropertyChange" => Ok(WindowEvents::PropertyChange(ev.try_into()?)),
			"Minimize" => Ok(WindowEvents::Minimize(ev.try_into()?)),
			"Maximize" => Ok(WindowEvents::Maximize(ev.try_into()?)),
			"Restore" => Ok(WindowEvents::Restore(ev.try_into()?)),
			"Close" => Ok(WindowEvents::Close(ev.try_into()?)),
			"Create" => Ok(WindowEvents::Create(ev.try_into()?)),
			"Reparent" => Ok(WindowEvents::Reparent(ev.try_into()?)),
			"DesktopCreate" => Ok(WindowEvents::DesktopCreate(ev.try_into()?)),
			"DesktopDestroy" => Ok(WindowEvents::DesktopDestroy(ev.try_into()?)),
			"Destroy" => Ok(WindowEvents::Destroy(ev.try_into()?)),
			"Activate" => Ok(WindowEvents::Activate(ev.try_into()?)),
			"Deactivate" => Ok(WindowEvents::Deactivate(ev.try_into()?)),
			"Raise" => Ok(WindowEvents::Raise(ev.try_into()?)),
			"Lower" => Ok(WindowEvents::Lower(ev.try_into()?)),
			"Move" => Ok(WindowEvents::Move(ev.try_into()?)),
			"Resize" => Ok(WindowEvents::Resize(ev.try_into()?)),
			"Shade" => Ok(WindowEvents::Shade(ev.try_into()?)),
			"uUshade" => Ok(WindowEvents::UUshade(ev.try_into()?)),
			"Restyle" => Ok(WindowEvents::Restyle(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Window".into())),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	PropertyChangeEvent,
	WindowEvents,
	WindowEvents::PropertyChange
);
impl_from_user_facing_type_for_event_enum!(PropertyChangeEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(
	PropertyChangeEvent,
	WindowEvents::PropertyChange,
	Event::Window
);
event_test_cases!(PropertyChangeEvent);
impl_to_dbus_message!(PropertyChangeEvent);
impl_from_dbus_message!(PropertyChangeEvent);
impl From<PropertyChangeEvent> for EventBodyOwned {
	fn from(event: PropertyChangeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.property,
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	MinimizeEvent,
	WindowEvents,
	WindowEvents::Minimize
);
impl_from_user_facing_type_for_event_enum!(MinimizeEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(MinimizeEvent, WindowEvents::Minimize, Event::Window);
event_test_cases!(MinimizeEvent);
impl_to_dbus_message!(MinimizeEvent);
impl_from_dbus_message!(MinimizeEvent);
impl From<MinimizeEvent> for EventBodyOwned {
	fn from(_event: MinimizeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	MaximizeEvent,
	WindowEvents,
	WindowEvents::Maximize
);
impl_from_user_facing_type_for_event_enum!(MaximizeEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(MaximizeEvent, WindowEvents::Maximize, Event::Window);
event_test_cases!(MaximizeEvent);
impl_to_dbus_message!(MaximizeEvent);
impl_from_dbus_message!(MaximizeEvent);
impl From<MaximizeEvent> for EventBodyOwned {
	fn from(_event: MaximizeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	RestoreEvent,
	WindowEvents,
	WindowEvents::Restore
);
impl_from_user_facing_type_for_event_enum!(RestoreEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(RestoreEvent, WindowEvents::Restore, Event::Window);
event_test_cases!(RestoreEvent);
impl_to_dbus_message!(RestoreEvent);
impl_from_dbus_message!(RestoreEvent);
impl From<RestoreEvent> for EventBodyOwned {
	fn from(_event: RestoreEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	CloseEvent,
	WindowEvents,
	WindowEvents::Close
);
impl_from_user_facing_type_for_event_enum!(CloseEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(CloseEvent, WindowEvents::Close, Event::Window);
event_test_cases!(CloseEvent);
impl_to_dbus_message!(CloseEvent);
impl_from_dbus_message!(CloseEvent);
impl From<CloseEvent> for EventBodyOwned {
	fn from(_event: CloseEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	CreateEvent,
	WindowEvents,
	WindowEvents::Create
);
impl_from_user_facing_type_for_event_enum!(CreateEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(CreateEvent, WindowEvents::Create, Event::Window);
event_test_cases!(CreateEvent);
impl_to_dbus_message!(CreateEvent);
impl_from_dbus_message!(CreateEvent);
impl From<CreateEvent> for EventBodyOwned {
	fn from(_event: CreateEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	ReparentEvent,
	WindowEvents,
	WindowEvents::Reparent
);
impl_from_user_facing_type_for_event_enum!(ReparentEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(ReparentEvent, WindowEvents::Reparent, Event::Window);
event_test_cases!(ReparentEvent);
impl_to_dbus_message!(ReparentEvent);
impl_from_dbus_message!(ReparentEvent);
impl From<ReparentEvent> for EventBodyOwned {
	fn from(_event: ReparentEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	DesktopCreateEvent,
	WindowEvents,
	WindowEvents::DesktopCreate
);
impl_from_user_facing_type_for_event_enum!(DesktopCreateEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(
	DesktopCreateEvent,
	WindowEvents::DesktopCreate,
	Event::Window
);
event_test_cases!(DesktopCreateEvent);
impl_to_dbus_message!(DesktopCreateEvent);
impl_from_dbus_message!(DesktopCreateEvent);
impl From<DesktopCreateEvent> for EventBodyOwned {
	fn from(_event: DesktopCreateEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	DesktopDestroyEvent,
	WindowEvents,
	WindowEvents::DesktopDestroy
);
impl_from_user_facing_type_for_event_enum!(DesktopDestroyEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(
	DesktopDestroyEvent,
	WindowEvents::DesktopDestroy,
	Event::Window
);
event_test_cases!(DesktopDestroyEvent);
impl_to_dbus_message!(DesktopDestroyEvent);
impl_from_dbus_message!(DesktopDestroyEvent);
impl From<DesktopDestroyEvent> for EventBodyOwned {
	fn from(_event: DesktopDestroyEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	DestroyEvent,
	WindowEvents,
	WindowEvents::Destroy
);
impl_from_user_facing_type_for_event_enum!(DestroyEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(DestroyEvent, WindowEvents::Destroy, Event::Window);
event_test_cases!(DestroyEvent);
impl_to_dbus_message!(DestroyEvent);
impl_from_dbus_message!(DestroyEvent);
impl From<DestroyEvent> for EventBodyOwned {
	fn from(_event: DestroyEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	ActivateEvent,
	WindowEvents,
	WindowEvents::Activate
);
impl_from_user_facing_type_for_event_enum!(ActivateEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(ActivateEvent, WindowEvents::Activate, Event::Window);
event_test_cases!(ActivateEvent);
impl_to_dbus_message!(ActivateEvent);
impl_from_dbus_message!(ActivateEvent);
impl From<ActivateEvent> for EventBodyOwned {
	fn from(_event: ActivateEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	DeactivateEvent,
	WindowEvents,
	WindowEvents::Deactivate
);
impl_from_user_facing_type_for_event_enum!(DeactivateEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(DeactivateEvent, WindowEvents::Deactivate, Event::Window);
event_test_cases!(DeactivateEvent);
impl_to_dbus_message!(DeactivateEvent);
impl_from_dbus_message!(DeactivateEvent);
impl From<DeactivateEvent> for EventBodyOwned {
	fn from(_event: DeactivateEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	RaiseEvent,
	WindowEvents,
	WindowEvents::Raise
);
impl_from_user_facing_type_for_event_enum!(RaiseEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(RaiseEvent, WindowEvents::Raise, Event::Window);
event_test_cases!(RaiseEvent);
impl_to_dbus_message!(RaiseEvent);
impl_from_dbus_message!(RaiseEvent);
impl From<RaiseEvent> for EventBodyOwned {
	fn from(_event: RaiseEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	LowerEvent,
	WindowEvents,
	WindowEvents::Lower
);
impl_from_user_facing_type_for_event_enum!(LowerEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(LowerEvent, WindowEvents::Lower, Event::Window);
event_test_cases!(LowerEvent);
impl_to_dbus_message!(LowerEvent);
impl_from_dbus_message!(LowerEvent);
impl From<LowerEvent> for EventBodyOwned {
	fn from(_event: LowerEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(MoveEvent, WindowEvents, WindowEvents::Move);
impl_from_user_facing_type_for_event_enum!(MoveEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(MoveEvent, WindowEvents::Move, Event::Window);
event_test_cases!(MoveEvent);
impl_to_dbus_message!(MoveEvent);
impl_from_dbus_message!(MoveEvent);
impl From<MoveEvent> for EventBodyOwned {
	fn from(_event: MoveEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	ResizeEvent,
	WindowEvents,
	WindowEvents::Resize
);
impl_from_user_facing_type_for_event_enum!(ResizeEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(ResizeEvent, WindowEvents::Resize, Event::Window);
event_test_cases!(ResizeEvent);
impl_to_dbus_message!(ResizeEvent);
impl_from_dbus_message!(ResizeEvent);
impl From<ResizeEvent> for EventBodyOwned {
	fn from(_event: ResizeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	ShadeEvent,
	WindowEvents,
	WindowEvents::Shade
);
impl_from_user_facing_type_for_event_enum!(ShadeEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(ShadeEvent, WindowEvents::Shade, Event::Window);
event_test_cases!(ShadeEvent);
impl_to_dbus_message!(ShadeEvent);
impl_from_dbus_message!(ShadeEvent);
impl From<ShadeEvent> for EventBodyOwned {
	fn from(_event: ShadeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	UUshadeEvent,
	WindowEvents,
	WindowEvents::UUshade
);
impl_from_user_facing_type_for_event_enum!(UUshadeEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(UUshadeEvent, WindowEvents::UUshade, Event::Window);
event_test_cases!(UUshadeEvent);
impl_to_dbus_message!(UUshadeEvent);
impl_from_dbus_message!(UUshadeEvent);
impl From<UUshadeEvent> for EventBodyOwned {
	fn from(_event: UUshadeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	RestyleEvent,
	WindowEvents,
	WindowEvents::Restyle
);
impl_from_user_facing_type_for_event_enum!(RestyleEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(RestyleEvent, WindowEvents::Restyle, Event::Window);
event_test_cases!(RestyleEvent);
impl_to_dbus_message!(RestyleEvent);
impl_from_dbus_message!(RestyleEvent);
impl From<RestyleEvent> for EventBodyOwned {
	fn from(_event: RestyleEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl HasRegistryEventString for WindowEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}
