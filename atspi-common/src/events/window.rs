use crate::{
	error::AtspiError,
	events::{
		BusProperties, EventBodyOwned, HasInterfaceName, HasMatchRule, HasRegistryEventString,
	},
	Event, EventProperties, EventTypeProperties,
};
#[cfg(feature = "zbus")]
use crate::{
	events::{EventWrapperMessageConversion, MessageConversion, TryFromMessage},
	ObjectRef,
};
use zbus_names::UniqueName;
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

impl EventTypeProperties for WindowEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.member(),
			Self::Minimize(inner) => inner.member(),
			Self::Maximize(inner) => inner.member(),
			Self::Restore(inner) => inner.member(),
			Self::Close(inner) => inner.member(),
			Self::Create(inner) => inner.member(),
			Self::Reparent(inner) => inner.member(),
			Self::DesktopCreate(inner) => inner.member(),
			Self::DesktopDestroy(inner) => inner.member(),
			Self::Destroy(inner) => inner.member(),
			Self::Activate(inner) => inner.member(),
			Self::Deactivate(inner) => inner.member(),
			Self::Raise(inner) => inner.member(),
			Self::Lower(inner) => inner.member(),
			Self::Move(inner) => inner.member(),
			Self::Resize(inner) => inner.member(),
			Self::Shade(inner) => inner.member(),
			Self::UUshade(inner) => inner.member(),
			Self::Restyle(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.interface(),
			Self::Minimize(inner) => inner.interface(),
			Self::Maximize(inner) => inner.interface(),
			Self::Restore(inner) => inner.interface(),
			Self::Close(inner) => inner.interface(),
			Self::Create(inner) => inner.interface(),
			Self::Reparent(inner) => inner.interface(),
			Self::DesktopCreate(inner) => inner.interface(),
			Self::DesktopDestroy(inner) => inner.interface(),
			Self::Destroy(inner) => inner.interface(),
			Self::Activate(inner) => inner.interface(),
			Self::Deactivate(inner) => inner.interface(),
			Self::Raise(inner) => inner.interface(),
			Self::Lower(inner) => inner.interface(),
			Self::Move(inner) => inner.interface(),
			Self::Resize(inner) => inner.interface(),
			Self::Shade(inner) => inner.interface(),
			Self::UUshade(inner) => inner.interface(),
			Self::Restyle(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.match_rule(),
			Self::Minimize(inner) => inner.match_rule(),
			Self::Maximize(inner) => inner.match_rule(),
			Self::Restore(inner) => inner.match_rule(),
			Self::Close(inner) => inner.match_rule(),
			Self::Create(inner) => inner.match_rule(),
			Self::Reparent(inner) => inner.match_rule(),
			Self::DesktopCreate(inner) => inner.match_rule(),
			Self::DesktopDestroy(inner) => inner.match_rule(),
			Self::Destroy(inner) => inner.match_rule(),
			Self::Activate(inner) => inner.match_rule(),
			Self::Deactivate(inner) => inner.match_rule(),
			Self::Raise(inner) => inner.match_rule(),
			Self::Lower(inner) => inner.match_rule(),
			Self::Move(inner) => inner.match_rule(),
			Self::Resize(inner) => inner.match_rule(),
			Self::Shade(inner) => inner.match_rule(),
			Self::UUshade(inner) => inner.match_rule(),
			Self::Restyle(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.registry_string(),
			Self::Minimize(inner) => inner.registry_string(),
			Self::Maximize(inner) => inner.registry_string(),
			Self::Restore(inner) => inner.registry_string(),
			Self::Close(inner) => inner.registry_string(),
			Self::Create(inner) => inner.registry_string(),
			Self::Reparent(inner) => inner.registry_string(),
			Self::DesktopCreate(inner) => inner.registry_string(),
			Self::DesktopDestroy(inner) => inner.registry_string(),
			Self::Destroy(inner) => inner.registry_string(),
			Self::Activate(inner) => inner.registry_string(),
			Self::Deactivate(inner) => inner.registry_string(),
			Self::Raise(inner) => inner.registry_string(),
			Self::Lower(inner) => inner.registry_string(),
			Self::Move(inner) => inner.registry_string(),
			Self::Resize(inner) => inner.registry_string(),
			Self::Shade(inner) => inner.registry_string(),
			Self::UUshade(inner) => inner.registry_string(),
			Self::Restyle(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for WindowEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::PropertyChange(inner) => inner.path(),
			Self::Minimize(inner) => inner.path(),
			Self::Maximize(inner) => inner.path(),
			Self::Restore(inner) => inner.path(),
			Self::Close(inner) => inner.path(),
			Self::Create(inner) => inner.path(),
			Self::Reparent(inner) => inner.path(),
			Self::DesktopCreate(inner) => inner.path(),
			Self::DesktopDestroy(inner) => inner.path(),
			Self::Destroy(inner) => inner.path(),
			Self::Activate(inner) => inner.path(),
			Self::Deactivate(inner) => inner.path(),
			Self::Raise(inner) => inner.path(),
			Self::Lower(inner) => inner.path(),
			Self::Move(inner) => inner.path(),
			Self::Resize(inner) => inner.path(),
			Self::Shade(inner) => inner.path(),
			Self::UUshade(inner) => inner.path(),
			Self::Restyle(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::PropertyChange(inner) => inner.sender(),
			Self::Minimize(inner) => inner.sender(),
			Self::Maximize(inner) => inner.sender(),
			Self::Restore(inner) => inner.sender(),
			Self::Close(inner) => inner.sender(),
			Self::Create(inner) => inner.sender(),
			Self::Reparent(inner) => inner.sender(),
			Self::DesktopCreate(inner) => inner.sender(),
			Self::DesktopDestroy(inner) => inner.sender(),
			Self::Destroy(inner) => inner.sender(),
			Self::Activate(inner) => inner.sender(),
			Self::Deactivate(inner) => inner.sender(),
			Self::Raise(inner) => inner.sender(),
			Self::Lower(inner) => inner.sender(),
			Self::Move(inner) => inner.sender(),
			Self::Resize(inner) => inner.sender(),
			Self::Shade(inner) => inner.sender(),
			Self::UUshade(inner) => inner.sender(),
			Self::Restyle(inner) => inner.sender(),
		}
	}
}

impl_from_interface_event_enum_for_event!(WindowEvents, Event::Window);
impl_try_from_event_for_user_facing_event_type!(WindowEvents, Event::Window);

event_wrapper_test_cases!(WindowEvents, MoveEvent);

impl HasMatchRule for WindowEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window'";
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct PropertyChangeEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
	pub property: String,
}

/// The window has been minimized.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MinimizeEvent {
	/// The application which has been minimized.
	pub item: crate::events::ObjectRef,
}

/// The window has been maximized.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MaximizeEvent {
	/// The application which has been maximized.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RestoreEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// A window has been closed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CloseEvent {
	/// The application which has been closed.
	pub item: crate::events::ObjectRef,
}

/// A new window has been created.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CreateEvent {
	/// An application to query for additional events from.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ReparentEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// A new virtual desktop has been created.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DesktopCreateEvent {
	/// A reference to a new desktop
	pub item: crate::events::ObjectRef,
}

/// A virtual desktop has been deleted.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DesktopDestroyEvent {
	/// A reference to the destroyed desktop.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DestroyEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ActivateEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DeactivateEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RaiseEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LowerEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MoveEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// A window has been resized.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ResizeEvent {
	/// The application which has been resized.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ShadeEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct UUshadeEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RestyleEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

impl BusProperties for PropertyChangeEvent {
	const DBUS_MEMBER: &'static str = "PropertyChange";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='PropertyChange'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

#[cfg(feature = "zbus")]
impl MessageConversion for PropertyChangeEvent {
	type Body = EventBodyOwned;

	fn try_from_validated_message_parts(
		item: ObjectRef,
		body: Self::Body,
	) -> Result<Self, AtspiError> {
		Ok(Self { item, property: body.kind })
	}
	fn try_from_validated_message(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = msg.body().deserialize()?;
		Self::try_from_validated_message_parts(item, body)
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl BusProperties for MinimizeEvent {
	const DBUS_MEMBER: &'static str = "Minimize";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Minimize'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for MaximizeEvent {
	const DBUS_MEMBER: &'static str = "Maximize";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Maximize'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for RestoreEvent {
	const DBUS_MEMBER: &'static str = "Restore";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Restore'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for CloseEvent {
	const DBUS_MEMBER: &'static str = "Close";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Close'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for CreateEvent {
	const DBUS_MEMBER: &'static str = "Create";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Create'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for ReparentEvent {
	const DBUS_MEMBER: &'static str = "Reparent";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Reparent'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for DesktopCreateEvent {
	const DBUS_MEMBER: &'static str = "DesktopCreate";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopCreate'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for DesktopDestroyEvent {
	const DBUS_MEMBER: &'static str = "DesktopDestroy";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopDestroy'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for DestroyEvent {
	const DBUS_MEMBER: &'static str = "Destroy";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Destroy'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for ActivateEvent {
	const DBUS_MEMBER: &'static str = "Activate";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Activate'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for DeactivateEvent {
	const DBUS_MEMBER: &'static str = "Deactivate";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Deactivate'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for RaiseEvent {
	const DBUS_MEMBER: &'static str = "Raise";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Raise'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for LowerEvent {
	const DBUS_MEMBER: &'static str = "Lower";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Lower'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for MoveEvent {
	const DBUS_MEMBER: &'static str = "Move";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Move'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for ResizeEvent {
	const DBUS_MEMBER: &'static str = "Resize";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Resize'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for ShadeEvent {
	const DBUS_MEMBER: &'static str = "Shade";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Shade'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for UUshadeEvent {
	const DBUS_MEMBER: &'static str = "uUshade";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='uUshade'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl BusProperties for RestyleEvent {
	const DBUS_MEMBER: &'static str = "Restyle";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Restyle'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}

impl HasInterfaceName for WindowEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for WindowEvents {
	fn try_from_message_interface_checked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let header = msg.header();
		let member = header.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			PropertyChangeEvent::DBUS_MEMBER => Ok(WindowEvents::PropertyChange(
				PropertyChangeEvent::try_from_validated_message(msg)?,
			)),
			MinimizeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Minimize(MinimizeEvent::try_from_validated_message(msg)?))
			}
			MaximizeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Maximize(MaximizeEvent::try_from_validated_message(msg)?))
			}
			RestoreEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Restore(RestoreEvent::try_from_validated_message(msg)?))
			}
			"Close" => Ok(WindowEvents::Close(CloseEvent::try_from_validated_message(msg)?)),
			CreateEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Create(CreateEvent::try_from_validated_message(msg)?))
			}
			ReparentEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Reparent(ReparentEvent::try_from_validated_message(msg)?))
			}
			"DesktopCreate" => Ok(WindowEvents::DesktopCreate(
				DesktopCreateEvent::try_from_validated_message(msg)?,
			)),
			"DesktopDestroy" => Ok(WindowEvents::DesktopDestroy(
				DesktopDestroyEvent::try_from_validated_message(msg)?,
			)),
			"Destroy" => Ok(WindowEvents::Destroy(DestroyEvent::try_from_validated_message(msg)?)),
			"Activate" => {
				Ok(WindowEvents::Activate(ActivateEvent::try_from_validated_message(msg)?))
			}
			"Deactivate" => {
				Ok(WindowEvents::Deactivate(DeactivateEvent::try_from_validated_message(msg)?))
			}
			"Raise" => Ok(WindowEvents::Raise(RaiseEvent::try_from_validated_message(msg)?)),
			"Lower" => Ok(WindowEvents::Lower(LowerEvent::try_from_validated_message(msg)?)),
			"Move" => Ok(WindowEvents::Move(MoveEvent::try_from_validated_message(msg)?)),
			"Resize" => Ok(WindowEvents::Resize(ResizeEvent::try_from_validated_message(msg)?)),
			"Shade" => Ok(WindowEvents::Shade(ShadeEvent::try_from_validated_message(msg)?)),
			"uUshade" => Ok(WindowEvents::UUshade(UUshadeEvent::try_from_validated_message(msg)?)),
			"Restyle" => Ok(WindowEvents::Restyle(RestyleEvent::try_from_validated_message(msg)?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Window".into())),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for WindowEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
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
impl_event_properties!(PropertyChangeEvent);
impl From<PropertyChangeEvent> for EventBodyOwned {
	fn from(event: PropertyChangeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.property,
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: u8::default().into(),
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
impl_event_properties!(MinimizeEvent);
impl_from_object_ref!(MinimizeEvent);

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
impl_event_properties!(MaximizeEvent);
impl_from_object_ref!(MaximizeEvent);

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
impl_event_properties!(RestoreEvent);
impl_from_object_ref!(RestoreEvent);

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
impl_event_properties!(CloseEvent);
impl_from_object_ref!(CloseEvent);

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
impl_event_properties!(CreateEvent);
impl_from_object_ref!(CreateEvent);

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
impl_event_properties!(ReparentEvent);
impl_from_object_ref!(ReparentEvent);

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
impl_event_properties!(DesktopCreateEvent);
impl_from_object_ref!(DesktopCreateEvent);

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
impl_event_properties!(DesktopDestroyEvent);
impl_from_object_ref!(DesktopDestroyEvent);

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
impl_event_properties!(DestroyEvent);
impl_from_object_ref!(DestroyEvent);

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
impl_event_properties!(ActivateEvent);
impl_from_object_ref!(ActivateEvent);

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
impl_event_properties!(DeactivateEvent);
impl_from_object_ref!(DeactivateEvent);

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
impl_event_properties!(RaiseEvent);
impl_from_object_ref!(RaiseEvent);

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
impl_event_properties!(LowerEvent);
impl_from_object_ref!(LowerEvent);

impl_from_user_facing_event_for_interface_event_enum!(MoveEvent, WindowEvents, WindowEvents::Move);
impl_from_user_facing_type_for_event_enum!(MoveEvent, Event::Window);
impl_try_from_event_for_user_facing_type!(MoveEvent, WindowEvents::Move, Event::Window);
event_test_cases!(MoveEvent);
impl_to_dbus_message!(MoveEvent);
impl_from_dbus_message!(MoveEvent);
impl_event_properties!(MoveEvent);
impl_from_object_ref!(MoveEvent);

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
impl_event_properties!(ResizeEvent);
impl_from_object_ref!(ResizeEvent);

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
impl_event_properties!(ShadeEvent);
impl_from_object_ref!(ShadeEvent);

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
impl_event_properties!(UUshadeEvent);
impl_from_object_ref!(UUshadeEvent);

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
impl_event_properties!(RestyleEvent);
impl_from_object_ref!(RestyleEvent);

impl HasRegistryEventString for WindowEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}
