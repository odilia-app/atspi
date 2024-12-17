use crate::{
	error::AtspiError,
	events::{BusProperties, EventBodyOwned},
	Event, EventProperties,
};
#[cfg(feature = "zbus")]
use crate::{
	events::{MessageConversion, MessageConversionExt},
	ObjectRef,
};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

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

	fn from_message_unchecked_parts(item: ObjectRef, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, property: body.kind })
	}
	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = if msg.body().signature() == crate::events::QSPI_EVENT_SIGNATURE {
			msg.body().deserialize::<crate::events::EventBodyQT>()?.into()
		} else {
			msg.body().deserialize()?
		};
		Self::from_message_unchecked_parts(item, body)
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

impl_from_user_facing_type_for_event_enum!(PropertyChangeEvent, Event::Window);
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

impl_from_user_facing_type_for_event_enum!(MinimizeEvent, Event::Window);
event_test_cases!(MinimizeEvent);
impl_to_dbus_message!(MinimizeEvent);
impl_from_dbus_message!(MinimizeEvent);
impl_event_properties!(MinimizeEvent);
impl_from_object_ref!(MinimizeEvent);

impl_from_user_facing_type_for_event_enum!(MaximizeEvent, Event::Window);
event_test_cases!(MaximizeEvent);
impl_to_dbus_message!(MaximizeEvent);
impl_from_dbus_message!(MaximizeEvent);
impl_event_properties!(MaximizeEvent);
impl_from_object_ref!(MaximizeEvent);

impl_from_user_facing_type_for_event_enum!(RestoreEvent, Event::Window);
event_test_cases!(RestoreEvent);
impl_to_dbus_message!(RestoreEvent);
impl_from_dbus_message!(RestoreEvent);
impl_event_properties!(RestoreEvent);
impl_from_object_ref!(RestoreEvent);

impl_from_user_facing_type_for_event_enum!(CloseEvent, Event::Window);
event_test_cases!(CloseEvent);
impl_to_dbus_message!(CloseEvent);
impl_from_dbus_message!(CloseEvent);
impl_event_properties!(CloseEvent);
impl_from_object_ref!(CloseEvent);

impl_from_user_facing_type_for_event_enum!(CreateEvent, Event::Window);
event_test_cases!(CreateEvent);
impl_to_dbus_message!(CreateEvent);
impl_from_dbus_message!(CreateEvent);
impl_event_properties!(CreateEvent);
impl_from_object_ref!(CreateEvent);

impl_from_user_facing_type_for_event_enum!(ReparentEvent, Event::Window);
event_test_cases!(ReparentEvent);
impl_to_dbus_message!(ReparentEvent);
impl_from_dbus_message!(ReparentEvent);
impl_event_properties!(ReparentEvent);
impl_from_object_ref!(ReparentEvent);

impl_from_user_facing_type_for_event_enum!(DesktopCreateEvent, Event::Window);
event_test_cases!(DesktopCreateEvent);
impl_to_dbus_message!(DesktopCreateEvent);
impl_from_dbus_message!(DesktopCreateEvent);
impl_event_properties!(DesktopCreateEvent);
impl_from_object_ref!(DesktopCreateEvent);

impl_from_user_facing_type_for_event_enum!(DesktopDestroyEvent, Event::Window);
event_test_cases!(DesktopDestroyEvent);
impl_to_dbus_message!(DesktopDestroyEvent);
impl_from_dbus_message!(DesktopDestroyEvent);
impl_event_properties!(DesktopDestroyEvent);
impl_from_object_ref!(DesktopDestroyEvent);

impl_from_user_facing_type_for_event_enum!(DestroyEvent, Event::Window);
event_test_cases!(DestroyEvent);
impl_to_dbus_message!(DestroyEvent);
impl_from_dbus_message!(DestroyEvent);
impl_event_properties!(DestroyEvent);
impl_from_object_ref!(DestroyEvent);

impl_from_user_facing_type_for_event_enum!(ActivateEvent, Event::Window);
event_test_cases!(ActivateEvent);
impl_to_dbus_message!(ActivateEvent);
impl_from_dbus_message!(ActivateEvent);
impl_event_properties!(ActivateEvent);
impl_from_object_ref!(ActivateEvent);

impl_from_user_facing_type_for_event_enum!(DeactivateEvent, Event::Window);
event_test_cases!(DeactivateEvent);
impl_to_dbus_message!(DeactivateEvent);
impl_from_dbus_message!(DeactivateEvent);
impl_event_properties!(DeactivateEvent);
impl_from_object_ref!(DeactivateEvent);

impl_from_user_facing_type_for_event_enum!(RaiseEvent, Event::Window);
event_test_cases!(RaiseEvent);
impl_to_dbus_message!(RaiseEvent);
impl_from_dbus_message!(RaiseEvent);
impl_event_properties!(RaiseEvent);
impl_from_object_ref!(RaiseEvent);

impl_from_user_facing_type_for_event_enum!(LowerEvent, Event::Window);
event_test_cases!(LowerEvent);
impl_to_dbus_message!(LowerEvent);
impl_from_dbus_message!(LowerEvent);
impl_event_properties!(LowerEvent);
impl_from_object_ref!(LowerEvent);

impl_from_user_facing_type_for_event_enum!(MoveEvent, Event::Window);
event_test_cases!(MoveEvent);
impl_to_dbus_message!(MoveEvent);
impl_from_dbus_message!(MoveEvent);
impl_event_properties!(MoveEvent);
impl_from_object_ref!(MoveEvent);

impl_from_user_facing_type_for_event_enum!(ResizeEvent, Event::Window);
event_test_cases!(ResizeEvent);
impl_to_dbus_message!(ResizeEvent);
impl_from_dbus_message!(ResizeEvent);
impl_event_properties!(ResizeEvent);
impl_from_object_ref!(ResizeEvent);

impl_from_user_facing_type_for_event_enum!(ShadeEvent, Event::Window);
event_test_cases!(ShadeEvent);
impl_to_dbus_message!(ShadeEvent);
impl_from_dbus_message!(ShadeEvent);
impl_event_properties!(ShadeEvent);
impl_from_object_ref!(ShadeEvent);

impl_from_user_facing_type_for_event_enum!(UUshadeEvent, Event::Window);
event_test_cases!(UUshadeEvent);
impl_to_dbus_message!(UUshadeEvent);
impl_from_dbus_message!(UUshadeEvent);
impl_event_properties!(UUshadeEvent);
impl_from_object_ref!(UUshadeEvent);

impl_from_user_facing_type_for_event_enum!(RestyleEvent, Event::Window);
event_test_cases!(RestyleEvent);
impl_to_dbus_message!(RestyleEvent);
impl_from_dbus_message!(RestyleEvent);
impl_event_properties!(RestyleEvent);
impl_from_object_ref!(RestyleEvent);
