#[cfg(feature = "zbus")]
use crate::error::AtspiError;
#[cfg(feature = "zbus")]
use crate::events::EventBody;
#[cfg(feature = "zbus")]
use crate::events::MessageConversion;
use crate::events::{
	DBusInterface, DBusMatchRule, DBusMember, EventBodyOwned, RegistryEventString,
};
#[cfg(feature = "zbus")]
use crate::EventProperties;
use crate::NonNullObjectRef;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct PropertyChangeEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
	pub property: String,
}

impl_event_type_properties_for_event!(PropertyChangeEvent<'_>);

/// The window has been minimized.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct MinimizeEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(MinimizeEvent<'_>);

/// The window has been maximized.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct MaximizeEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(MaximizeEvent<'_>);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct RestoreEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(RestoreEvent<'_>);

/// A window has been closed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct CloseEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(CloseEvent<'_>);

/// A new window has been created.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct CreateEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(CreateEvent<'_>);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ReparentEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(ReparentEvent<'_>);

/// A new virtual desktop has been created.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct DesktopCreateEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(DesktopCreateEvent<'_>);

/// A virtual desktop has been deleted.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct DesktopDestroyEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(DesktopDestroyEvent<'_>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct DestroyEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}
impl_event_type_properties_for_event!(DestroyEvent<'_>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct ActivateEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(ActivateEvent<'_>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct DeactivateEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(DeactivateEvent<'_>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct RaiseEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(RaiseEvent<'_>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct LowerEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(LowerEvent<'_>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct MoveEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(MoveEvent<'_>);

/// A window has been resized.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct ResizeEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(ResizeEvent<'_>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct ShadeEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(ShadeEvent<'_>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct UUshadeEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(UUshadeEvent<'_>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct RestyleEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(RestyleEvent<'_>);

impl_member_interface_registry_string_and_match_rule_for_event!(
	PropertyChangeEvent<'_>,
	"PropertyChange",
	"org.a11y.atspi.Event.Window",
	"window:property-change",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='PropertyChange'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for PropertyChangeEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'_>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		let mut body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item: item.into_owned(), property: body.take_kind() })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBody::Owned(EventBodyOwned { kind: self.property.clone(), ..Default::default() })
	}
}

impl_member_interface_registry_string_and_match_rule_for_event!(
	MinimizeEvent<'_>,
	"Minimize",
	"org.a11y.atspi.Event.Window",
	"window:minimize",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Minimize'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	MaximizeEvent<'_>,
	"Maximize",
	"org.a11y.atspi.Event.Window",
	"window:maximize",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Maximize'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RestoreEvent<'_>,
	"Restore",
	"org.a11y.atspi.Event.Window",
	"window:restore",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Restore'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	CloseEvent<'_>,
	"Close",
	"org.a11y.atspi.Event.Window",
	"window:close",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Close'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	CreateEvent<'_>,
	"Create",
	"org.a11y.atspi.Event.Window",
	"window:create",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Create'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ReparentEvent<'_>,
	"Reparent",
	"org.a11y.atspi.Event.Window",
	"window:reparent",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Reparent'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	DesktopCreateEvent<'_>,
	"DesktopCreate",
	"org.a11y.atspi.Event.Window",
	"window:desktop-create",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopCreate'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	DesktopDestroyEvent<'_>,
	"DesktopDestroy",
	"org.a11y.atspi.Event.Window",
	"window:desktop-destroy",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopDestroy'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	DestroyEvent<'_>,
	"Destroy",
	"org.a11y.atspi.Event.Window",
	"window:destroy",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Destroy'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ActivateEvent<'_>,
	"Activate",
	"org.a11y.atspi.Event.Window",
	"window:activate",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Activate'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	DeactivateEvent<'_>,
	"Deactivate",
	"org.a11y.atspi.Event.Window",
	"window:deactivate",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Deactivate'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RaiseEvent<'_>,
	"Raise",
	"org.a11y.atspi.Event.Window",
	"window:raise",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Raise'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LowerEvent<'_>,
	"Lower",
	"org.a11y.atspi.Event.Window",
	"window:lower",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Lower'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	MoveEvent<'_>,
	"Move",
	"org.a11y.atspi.Event.Window",
	"window:move",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Move'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ResizeEvent<'_>,
	"Resize",
	"org.a11y.atspi.Event.Window",
	"window:resize",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Resize'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ShadeEvent<'_>,
	"Shade",
	"org.a11y.atspi.Event.Window",
	"window:shade",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Shade'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	UUshadeEvent<'_>,
	"uUshade",
	"org.a11y.atspi.Event.Window",
	"window:uushade",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='uUshade'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RestyleEvent<'_>,
	"Restyle",
	"org.a11y.atspi.Event.Window",
	"window:restyle",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Restyle'"
);

event_test_cases!(PropertyChangeEvent);
impl_to_dbus_message!(PropertyChangeEvent<'_>);
impl_from_dbus_message!(PropertyChangeEvent<'_>);
impl_event_properties!(PropertyChangeEvent<'_>);
impl From<PropertyChangeEvent<'_>> for EventBodyOwned {
	fn from(event: PropertyChangeEvent) -> Self {
		EventBodyOwned { kind: event.property, ..Default::default() }
	}
}

event_test_cases!(MinimizeEvent);
impl_to_dbus_message!(MinimizeEvent<'_>);
impl_from_dbus_message!(MinimizeEvent<'_>);
impl_event_properties!(MinimizeEvent<'_>);
impl_from_object_ref!(MinimizeEvent<'_>);

event_test_cases!(MaximizeEvent);
impl_to_dbus_message!(MaximizeEvent<'_>);
impl_from_dbus_message!(MaximizeEvent<'_>);
impl_event_properties!(MaximizeEvent<'_>);
impl_from_object_ref!(MaximizeEvent<'_>);

event_test_cases!(RestoreEvent);
impl_to_dbus_message!(RestoreEvent<'_>);
impl_from_dbus_message!(RestoreEvent<'_>);
impl_event_properties!(RestoreEvent<'_>);
impl_from_object_ref!(RestoreEvent<'_>);

event_test_cases!(CloseEvent);
impl_to_dbus_message!(CloseEvent<'_>);
impl_from_dbus_message!(CloseEvent<'_>);
impl_event_properties!(CloseEvent<'_>);
impl_from_object_ref!(CloseEvent<'_>);

event_test_cases!(CreateEvent);
impl_to_dbus_message!(CreateEvent<'_>);
impl_from_dbus_message!(CreateEvent<'_>);
impl_event_properties!(CreateEvent<'_>);
impl_from_object_ref!(CreateEvent<'_>);

event_test_cases!(ReparentEvent);
impl_to_dbus_message!(ReparentEvent<'_>);
impl_from_dbus_message!(ReparentEvent<'_>);
impl_event_properties!(ReparentEvent<'_>);
impl_from_object_ref!(ReparentEvent<'_>);

event_test_cases!(DesktopCreateEvent);
impl_to_dbus_message!(DesktopCreateEvent<'_>);
impl_from_dbus_message!(DesktopCreateEvent<'_>);
impl_event_properties!(DesktopCreateEvent<'_>);
impl_from_object_ref!(DesktopCreateEvent<'_>);

event_test_cases!(DesktopDestroyEvent);
impl_to_dbus_message!(DesktopDestroyEvent<'_>);
impl_from_dbus_message!(DesktopDestroyEvent<'_>);
impl_event_properties!(DesktopDestroyEvent<'_>);
impl_from_object_ref!(DesktopDestroyEvent<'_>);

event_test_cases!(DestroyEvent);
impl_to_dbus_message!(DestroyEvent<'_>);
impl_from_dbus_message!(DestroyEvent<'_>);
impl_event_properties!(DestroyEvent<'_>);
impl_from_object_ref!(DestroyEvent<'_>);

event_test_cases!(ActivateEvent);
impl_to_dbus_message!(ActivateEvent<'_>);
impl_from_dbus_message!(ActivateEvent<'_>);
impl_event_properties!(ActivateEvent<'_>);
impl_from_object_ref!(ActivateEvent<'_>);

event_test_cases!(DeactivateEvent);
impl_to_dbus_message!(DeactivateEvent<'_>);
impl_from_dbus_message!(DeactivateEvent<'_>);
impl_event_properties!(DeactivateEvent<'_>);
impl_from_object_ref!(DeactivateEvent<'_>);

event_test_cases!(RaiseEvent);
impl_to_dbus_message!(RaiseEvent<'_>);
impl_from_dbus_message!(RaiseEvent<'_>);
impl_event_properties!(RaiseEvent<'_>);
impl_from_object_ref!(RaiseEvent<'_>);

event_test_cases!(LowerEvent);
impl_to_dbus_message!(LowerEvent<'_>);
impl_from_dbus_message!(LowerEvent<'_>);
impl_event_properties!(LowerEvent<'_>);
impl_from_object_ref!(LowerEvent<'_>);

event_test_cases!(MoveEvent);
impl_to_dbus_message!(MoveEvent<'_>);
impl_from_dbus_message!(MoveEvent<'_>);
impl_event_properties!(MoveEvent<'_>);
impl_from_object_ref!(MoveEvent<'_>);

event_test_cases!(ResizeEvent);
impl_to_dbus_message!(ResizeEvent<'_>);
impl_from_dbus_message!(ResizeEvent<'_>);
impl_event_properties!(ResizeEvent<'_>);
impl_from_object_ref!(ResizeEvent<'_>);

event_test_cases!(ShadeEvent);
impl_to_dbus_message!(ShadeEvent<'_>);
impl_from_dbus_message!(ShadeEvent<'_>);
impl_event_properties!(ShadeEvent<'_>);
impl_from_object_ref!(ShadeEvent<'_>);

event_test_cases!(UUshadeEvent);
impl_to_dbus_message!(UUshadeEvent<'_>);
impl_from_dbus_message!(UUshadeEvent<'_>);
impl_event_properties!(UUshadeEvent<'_>);
impl_from_object_ref!(UUshadeEvent<'_>);

event_test_cases!(RestyleEvent);
impl_to_dbus_message!(RestyleEvent<'_>);
impl_from_dbus_message!(RestyleEvent<'_>);
impl_event_properties!(RestyleEvent<'_>);
impl_from_object_ref!(RestyleEvent<'_>);

impl_msg_conversion_ext_for_target_type!(
	PropertyChangeEvent<'_>,
	MinimizeEvent<'_>,
	MaximizeEvent<'_>,
	RestoreEvent<'_>,
	CloseEvent<'_>,
	CreateEvent<'_>,
	ReparentEvent<'_>,
	DesktopCreateEvent<'_>,
	DesktopDestroyEvent<'_>,
	DestroyEvent<'_>,
	ActivateEvent<'_>,
	DeactivateEvent<'_>,
	RaiseEvent<'_>,
	LowerEvent<'_>,
	MoveEvent<'_>,
	ResizeEvent<'_>,
	ShadeEvent<'_>,
	UUshadeEvent<'_>,
	RestyleEvent<'_>,
);

impl_msg_conversion_for_types_built_from_object_ref!(
	MinimizeEvent<'_>,
	MaximizeEvent<'_>,
	RestoreEvent<'_>,
	CloseEvent<'_>,
	CreateEvent<'_>,
	ReparentEvent<'_>,
	DesktopCreateEvent<'_>,
	DesktopDestroyEvent<'_>,
	DestroyEvent<'_>,
	ActivateEvent<'_>,
	DeactivateEvent<'_>,
	RaiseEvent<'_>,
	LowerEvent<'_>,
	MoveEvent<'_>,
	ResizeEvent<'_>,
	ShadeEvent<'_>,
	UUshadeEvent<'_>,
	RestyleEvent<'_>,
);

impl_test_event!(
	PropertyChangeEvent<'_> { property },
	MinimizeEvent<'_>,
	MaximizeEvent<'_>,
	RestoreEvent<'_>,
	CloseEvent<'_>,
	CreateEvent<'_>,
	ReparentEvent<'_>,
	DesktopCreateEvent<'_>,
	DesktopDestroyEvent<'_>,
	DestroyEvent<'_>,
	ActivateEvent<'_>,
	DeactivateEvent<'_>,
	RaiseEvent<'_>,
	LowerEvent<'_>,
	MoveEvent<'_>,
	ResizeEvent<'_>,
	ShadeEvent<'_>,
	UUshadeEvent<'_>,
	RestyleEvent<'_>,
);
