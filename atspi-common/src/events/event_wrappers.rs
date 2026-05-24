use crate::events::registry::socket::AvailableEvent;

use crate::events::registry::{EventListenerDeregisteredEvent, EventListenerRegisteredEvent};
#[cfg(feature = "zbus")]
use crate::events::traits::EventWrapperMessageConversion;
#[cfg(feature = "zbus")]
use crate::events::MessageConversion;
use crate::{
	error::AtspiError,
	events::{
		cache::{AddAccessibleEvent, LegacyAddAccessibleEvent, RemoveAccessibleEvent},
		document::{
			AttributesChangedEvent as DocumentAttributesChangedEvent, ContentChangedEvent,
			LoadCompleteEvent, LoadStoppedEvent, PageChangedEvent, ReloadEvent,
		},
		focus::FocusEvent,
		keyboard::ModifiersEvent,
		mouse::{AbsEvent, ButtonEvent, RelEvent},
		object::{
			ActiveDescendantChangedEvent, AnnouncementEvent,
			AttributesChangedEvent as ObjectAttributesChangedEvent, BoundsChangedEvent,
			ChildrenChangedEvent, ColumnDeletedEvent, ColumnInsertedEvent, ColumnReorderedEvent,
			LinkSelectedEvent, ModelChangedEvent, PropertyChangeEvent as ObjectPropertyChangeEvent,
			RowDeletedEvent, RowInsertedEvent, RowReorderedEvent, SelectionChangedEvent,
			StateChangedEvent, TextAttributesChangedEvent, TextBoundsChangedEvent,
			TextCaretMovedEvent, TextChangedEvent, TextSelectionChangedEvent,
			VisibleDataChangedEvent,
		},
		terminal::{
			ApplicationChangedEvent, CharWidthChangedEvent, ColumnCountChangedEvent,
			LineChangedEvent, LineCountChangedEvent,
		},
		window::{
			ActivateEvent, CloseEvent, CreateEvent, DeactivateEvent, DesktopCreateEvent,
			DesktopDestroyEvent, DestroyEvent, LowerEvent, MaximizeEvent, MinimizeEvent, MoveEvent,
			PropertyChangeEvent as WindowPropertyChangeEvent, RaiseEvent, ReparentEvent,
			ResizeEvent, RestoreEvent, RestyleEvent, ShadeEvent, UUshadeEvent,
		},
		DBusInterface, DBusMatchRule, EventTypeProperties, RegistryEventString,
	},
	EventProperties,
};
#[cfg(feature = "zbus")]
use crate::{events::DBusMember, CacheItem, LegacyCacheItem};
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zbus::message::Header;
use zbus_names::UniqueName;
use zvariant::ObjectPath;
#[cfg(feature = "zbus")]
use zvariant::Type;

impl_from_user_facing_event_for_interface_event_enum!(
	EventListenerRegisteredEvent<'_>,
	EventListenerEvents<'_>,
	EventListenerEvents::Registered
);

impl_from_user_facing_type_for_event_enum!(EventListenerRegisteredEvent<'_>, Event::Listener);

impl_try_from_event_for_user_facing_type!(
	EventListenerRegisteredEvent<'_>,
	EventListenerEvents::Registered,
	Event::Listener
);

impl_from_user_facing_event_for_interface_event_enum!(
	EventListenerDeregisteredEvent<'_>,
	EventListenerEvents<'_>,
	EventListenerEvents::Deregistered
);

impl_from_user_facing_type_for_event_enum!(EventListenerDeregisteredEvent<'_>, Event::Listener);
impl_try_from_event_for_user_facing_type!(
	EventListenerDeregisteredEvent<'_>,
	EventListenerEvents::Deregistered,
	Event::Listener
);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum KeyboardEvents<'a> {
	/// See: [`ModifiersEvent`].
	#[serde(borrow)]
	Modifiers(ModifiersEvent<'a>),
}

impl_tryfrommessage_for_event_wrapper!(KeyboardEvents<'_>);
impl_try_from_event_for_interface_enum!(KeyboardEvents<'_>, Event::Keyboard);
impl_from_interface_event_enum_for_event!(KeyboardEvents, Event::Keyboard);

impl_from_user_facing_event_for_interface_event_enum!(
	ModifiersEvent<'_>,
	KeyboardEvents<'_>,
	KeyboardEvents::Modifiers
);
impl EventTypeProperties for KeyboardEvents<'_> {
	fn member(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.member(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.match_rule(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.interface(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for KeyboardEvents<'_> {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Modifiers(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Modifiers(inner) => inner.sender(),
		}
	}
}

event_wrapper_test_cases!(KeyboardEvents, ModifiersEvent);

impl DBusMatchRule for KeyboardEvents<'_> {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Keyboard'";
}

impl DBusInterface for KeyboardEvents<'_> {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Keyboard";
}

impl RegistryEventString for KeyboardEvents<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "keyboard:";
}

impl_from_user_facing_type_for_event_enum!(ModifiersEvent<'_>, Event::Keyboard);

impl_try_from_event_for_user_facing_type!(
	ModifiersEvent<'_>,
	KeyboardEvents::Modifiers,
	Event::Keyboard
);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum MouseEvents<'a> {
	/// See: [`AbsEvent`].
	#[serde(borrow)]
	Abs(AbsEvent<'a>),

	/// See: [`RelEvent`].
	#[serde(borrow)]
	Rel(RelEvent<'a>),

	/// See: [`ButtonEvent`].
	#[serde(borrow)]
	Button(ButtonEvent<'a>),
}

impl DBusMatchRule for MouseEvents<'_> {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse'";
}

impl DBusInterface for MouseEvents<'_> {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
}

impl RegistryEventString for MouseEvents<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "mouse:";
}

impl_tryfrommessage_for_event_wrapper!(MouseEvents<'_>);

#[cfg(feature = "zbus")]
impl<'a> EventWrapperMessageConversion<'a> for KeyboardEvents<'a> {
	fn try_from_message_interface_checked(
		msg: &'a zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			ModifiersEvent::DBUS_MEMBER => {
				Ok(KeyboardEvents::Modifiers(ModifiersEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Keyboard".into())),
		}
	}
}

impl EventProperties for MouseEvents<'_> {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Abs(inner) => inner.path(),
			Self::Rel(inner) => inner.path(),
			Self::Button(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Abs(inner) => inner.sender(),
			Self::Rel(inner) => inner.sender(),
			Self::Button(inner) => inner.sender(),
		}
	}
}

impl EventTypeProperties for MouseEvents<'_> {
	fn member(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.member(),
			Self::Rel(inner) => inner.member(),
			Self::Button(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.interface(),
			Self::Rel(inner) => inner.interface(),
			Self::Button(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.match_rule(),
			Self::Rel(inner) => inner.match_rule(),
			Self::Button(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.registry_string(),
			Self::Rel(inner) => inner.registry_string(),
			Self::Button(inner) => inner.registry_string(),
		}
	}
}

event_wrapper_test_cases!(MouseEvents, AbsEvent);
impl_try_from_event_for_interface_enum!(MouseEvents<'_>, Event::Mouse);
impl_from_interface_event_enum_for_event!(MouseEvents<'_>, Event::Mouse);

impl_from_user_facing_event_for_interface_event_enum!(
	RelEvent<'_>,
	MouseEvents<'_>,
	MouseEvents::Rel
);
impl_try_from_event_for_user_facing_type!(RelEvent<'_>, MouseEvents::Rel, Event::Mouse);
impl_from_user_facing_event_for_interface_event_enum!(
	ButtonEvent<'_>,
	MouseEvents<'_>,
	MouseEvents::Button
);
impl_try_from_event_for_user_facing_type!(ButtonEvent<'_>, MouseEvents::Button, Event::Mouse);

#[cfg(feature = "zbus")]
impl<'a> EventWrapperMessageConversion<'a> for MouseEvents<'a> {
	fn try_from_message_interface_checked(
		msg: &'a zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			AbsEvent::DBUS_MEMBER => {
				Ok(MouseEvents::Abs(AbsEvent::from_message_unchecked(msg, hdr)?))
			}
			RelEvent::DBUS_MEMBER => {
				Ok(MouseEvents::Rel(RelEvent::from_message_unchecked(msg, hdr)?))
			}
			ButtonEvent::DBUS_MEMBER => {
				Ok(MouseEvents::Button(ButtonEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
		}
	}
}

impl_from_user_facing_type_for_event_enum!(ButtonEvent<'_>, Event::Mouse);
impl_from_user_facing_type_for_event_enum!(RelEvent<'_>, Event::Mouse);
impl_from_user_facing_type_for_event_enum!(AbsEvent<'_>, Event::Mouse);

impl_from_user_facing_event_for_interface_event_enum!(
	AbsEvent<'_>,
	MouseEvents<'_>,
	MouseEvents::Abs
);
impl_try_from_event_for_user_facing_type!(AbsEvent<'_>, MouseEvents::Abs, Event::Mouse);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum ObjectEvents<'a> {
	/// See: [`ObjectPropertyChangeEvent`].
	#[serde(borrow)]
	PropertyChange(ObjectPropertyChangeEvent<'a>),

	/// See: [`BoundsChangedEvent`].
	#[serde(borrow)]
	BoundsChanged(BoundsChangedEvent<'a>),

	/// See: [`LinkSelectedEvent`].
	#[serde(borrow)]
	LinkSelected(LinkSelectedEvent<'a>),

	/// See: [`StateChangedEvent`].
	#[serde(borrow)]
	StateChanged(StateChangedEvent<'a>),

	/// See: [`ChildrenChangedEvent`].
	#[serde(borrow)]
	ChildrenChanged(ChildrenChangedEvent<'a>),

	/// See: [`VisibleDataChangedEvent`].
	#[serde(borrow)]
	VisibleDataChanged(VisibleDataChangedEvent<'a>),

	/// See: [`SelectionChangedEvent`].
	#[serde(borrow)]
	SelectionChanged(SelectionChangedEvent<'a>),

	/// See: [`ModelChangedEvent`].
	#[serde(borrow)]
	ModelChanged(ModelChangedEvent<'a>),

	/// See: [`ActiveDescendantChangedEvent`].
	#[serde(borrow)]
	ActiveDescendantChanged(ActiveDescendantChangedEvent<'a>),

	/// See: [`AnnouncementEvent`].
	#[serde(borrow)]
	Announcement(AnnouncementEvent<'a>),

	/// See: [`ObjectAttributesChangedEvent`].
	#[serde(borrow)]
	AttributesChanged(ObjectAttributesChangedEvent<'a>),

	/// See: [`RowInsertedEvent`].
	#[serde(borrow)]
	RowInserted(RowInsertedEvent<'a>),
	/// See: [`RowReorderedEvent`].
	#[serde(borrow)]
	RowReordered(RowReorderedEvent<'a>),

	/// See: [`RowDeletedEvent`].
	#[serde(borrow)]
	RowDeleted(RowDeletedEvent<'a>),

	/// See: [`ColumnInsertedEvent`].
	#[serde(borrow)]
	ColumnInserted(ColumnInsertedEvent<'a>),

	/// See: [`ColumnReorderedEvent`].
	#[serde(borrow)]
	ColumnReordered(ColumnReorderedEvent<'a>),

	/// See: [`ColumnDeletedEvent`].
	#[serde(borrow)]
	ColumnDeleted(ColumnDeletedEvent<'a>),

	/// See: [`TextBoundsChangedEvent`].
	#[serde(borrow)]
	TextBoundsChanged(TextBoundsChangedEvent<'a>),

	/// See: [`TextSelectionChangedEvent`].
	#[serde(borrow)]
	TextSelectionChanged(TextSelectionChangedEvent<'a>),

	/// See: [`TextChangedEvent`].
	#[serde(borrow)]
	TextChanged(TextChangedEvent<'a>),

	/// See: [`TextAttributesChangedEvent`].
	#[serde(borrow)]
	TextAttributesChanged(TextAttributesChangedEvent<'a>),

	/// See: [`TextCaretMovedEvent`].
	#[serde(borrow)]
	TextCaretMoved(TextCaretMovedEvent<'a>),
}

impl_tryfrommessage_for_event_wrapper!(ObjectEvents<'_>);

impl EventTypeProperties for ObjectEvents<'_> {
	fn member(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.member(),
			Self::BoundsChanged(inner) => inner.member(),
			Self::LinkSelected(inner) => inner.member(),
			Self::StateChanged(inner) => inner.member(),
			Self::ChildrenChanged(inner) => inner.member(),
			Self::VisibleDataChanged(inner) => inner.member(),
			Self::SelectionChanged(inner) => inner.member(),
			Self::ModelChanged(inner) => inner.member(),
			Self::ActiveDescendantChanged(inner) => inner.member(),
			Self::Announcement(inner) => inner.member(),
			Self::AttributesChanged(inner) => inner.member(),
			Self::RowInserted(inner) => inner.member(),
			Self::RowReordered(inner) => inner.member(),
			Self::RowDeleted(inner) => inner.member(),
			Self::ColumnInserted(inner) => inner.member(),
			Self::ColumnReordered(inner) => inner.member(),
			Self::ColumnDeleted(inner) => inner.member(),
			Self::TextBoundsChanged(inner) => inner.member(),
			Self::TextSelectionChanged(inner) => inner.member(),
			Self::TextChanged(inner) => inner.member(),
			Self::TextAttributesChanged(inner) => inner.member(),
			Self::TextCaretMoved(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.interface(),
			Self::BoundsChanged(inner) => inner.interface(),
			Self::LinkSelected(inner) => inner.interface(),
			Self::StateChanged(inner) => inner.interface(),
			Self::ChildrenChanged(inner) => inner.interface(),
			Self::VisibleDataChanged(inner) => inner.interface(),
			Self::SelectionChanged(inner) => inner.interface(),
			Self::ModelChanged(inner) => inner.interface(),
			Self::ActiveDescendantChanged(inner) => inner.interface(),
			Self::Announcement(inner) => inner.interface(),
			Self::AttributesChanged(inner) => inner.interface(),
			Self::RowInserted(inner) => inner.interface(),
			Self::RowReordered(inner) => inner.interface(),
			Self::RowDeleted(inner) => inner.interface(),
			Self::ColumnInserted(inner) => inner.interface(),
			Self::ColumnReordered(inner) => inner.interface(),
			Self::ColumnDeleted(inner) => inner.interface(),
			Self::TextBoundsChanged(inner) => inner.interface(),
			Self::TextSelectionChanged(inner) => inner.interface(),
			Self::TextChanged(inner) => inner.interface(),
			Self::TextAttributesChanged(inner) => inner.interface(),
			Self::TextCaretMoved(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.match_rule(),
			Self::BoundsChanged(inner) => inner.match_rule(),
			Self::LinkSelected(inner) => inner.match_rule(),
			Self::StateChanged(inner) => inner.match_rule(),
			Self::ChildrenChanged(inner) => inner.match_rule(),
			Self::VisibleDataChanged(inner) => inner.match_rule(),
			Self::SelectionChanged(inner) => inner.match_rule(),
			Self::ModelChanged(inner) => inner.match_rule(),
			Self::ActiveDescendantChanged(inner) => inner.match_rule(),
			Self::Announcement(inner) => inner.match_rule(),
			Self::AttributesChanged(inner) => inner.match_rule(),
			Self::RowInserted(inner) => inner.match_rule(),
			Self::RowReordered(inner) => inner.match_rule(),
			Self::RowDeleted(inner) => inner.match_rule(),
			Self::ColumnInserted(inner) => inner.match_rule(),
			Self::ColumnReordered(inner) => inner.match_rule(),
			Self::ColumnDeleted(inner) => inner.match_rule(),
			Self::TextBoundsChanged(inner) => inner.match_rule(),
			Self::TextSelectionChanged(inner) => inner.match_rule(),
			Self::TextChanged(inner) => inner.match_rule(),
			Self::TextAttributesChanged(inner) => inner.match_rule(),
			Self::TextCaretMoved(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.registry_string(),
			Self::BoundsChanged(inner) => inner.registry_string(),
			Self::LinkSelected(inner) => inner.registry_string(),
			Self::StateChanged(inner) => inner.registry_string(),
			Self::ChildrenChanged(inner) => inner.registry_string(),
			Self::VisibleDataChanged(inner) => inner.registry_string(),
			Self::SelectionChanged(inner) => inner.registry_string(),
			Self::ModelChanged(inner) => inner.registry_string(),
			Self::ActiveDescendantChanged(inner) => inner.registry_string(),
			Self::Announcement(inner) => inner.registry_string(),
			Self::AttributesChanged(inner) => inner.registry_string(),
			Self::RowInserted(inner) => inner.registry_string(),
			Self::RowReordered(inner) => inner.registry_string(),
			Self::RowDeleted(inner) => inner.registry_string(),
			Self::ColumnInserted(inner) => inner.registry_string(),
			Self::ColumnReordered(inner) => inner.registry_string(),
			Self::ColumnDeleted(inner) => inner.registry_string(),
			Self::TextBoundsChanged(inner) => inner.registry_string(),
			Self::TextSelectionChanged(inner) => inner.registry_string(),
			Self::TextChanged(inner) => inner.registry_string(),
			Self::TextAttributesChanged(inner) => inner.registry_string(),
			Self::TextCaretMoved(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for ObjectEvents<'_> {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::PropertyChange(inner) => inner.path(),
			Self::BoundsChanged(inner) => inner.path(),
			Self::LinkSelected(inner) => inner.path(),
			Self::StateChanged(inner) => inner.path(),
			Self::ChildrenChanged(inner) => inner.path(),
			Self::VisibleDataChanged(inner) => inner.path(),
			Self::SelectionChanged(inner) => inner.path(),
			Self::ModelChanged(inner) => inner.path(),
			Self::ActiveDescendantChanged(inner) => inner.path(),
			Self::Announcement(inner) => inner.path(),
			Self::AttributesChanged(inner) => inner.path(),
			Self::RowInserted(inner) => inner.path(),
			Self::RowReordered(inner) => inner.path(),
			Self::RowDeleted(inner) => inner.path(),
			Self::ColumnInserted(inner) => inner.path(),
			Self::ColumnReordered(inner) => inner.path(),
			Self::ColumnDeleted(inner) => inner.path(),
			Self::TextBoundsChanged(inner) => inner.path(),
			Self::TextSelectionChanged(inner) => inner.path(),
			Self::TextChanged(inner) => inner.path(),
			Self::TextAttributesChanged(inner) => inner.path(),
			Self::TextCaretMoved(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::PropertyChange(inner) => inner.sender(),
			Self::BoundsChanged(inner) => inner.sender(),
			Self::LinkSelected(inner) => inner.sender(),
			Self::StateChanged(inner) => inner.sender(),
			Self::ChildrenChanged(inner) => inner.sender(),
			Self::VisibleDataChanged(inner) => inner.sender(),
			Self::SelectionChanged(inner) => inner.sender(),
			Self::ModelChanged(inner) => inner.sender(),
			Self::ActiveDescendantChanged(inner) => inner.sender(),
			Self::Announcement(inner) => inner.sender(),
			Self::AttributesChanged(inner) => inner.sender(),
			Self::RowInserted(inner) => inner.sender(),
			Self::RowReordered(inner) => inner.sender(),
			Self::RowDeleted(inner) => inner.sender(),
			Self::ColumnInserted(inner) => inner.sender(),
			Self::ColumnReordered(inner) => inner.sender(),
			Self::ColumnDeleted(inner) => inner.sender(),
			Self::TextBoundsChanged(inner) => inner.sender(),
			Self::TextSelectionChanged(inner) => inner.sender(),
			Self::TextChanged(inner) => inner.sender(),
			Self::TextAttributesChanged(inner) => inner.sender(),
			Self::TextCaretMoved(inner) => inner.sender(),
		}
	}
}

impl_try_from_event_for_interface_enum!(ObjectEvents<'_>, Event::Object);
impl_from_interface_event_enum_for_event!(ObjectEvents, Event::Object);

event_wrapper_test_cases!(ObjectEvents, ObjectPropertyChangeEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	TextChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::TextChanged
);
impl_try_from_event_for_user_facing_type!(
	TextChangedEvent<'_>,
	ObjectEvents::TextChanged,
	Event::Object
);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum DocumentEvents<'a> {
	/// See: [`LoadCompleteEvent`].
	#[serde(borrow)]
	LoadComplete(LoadCompleteEvent<'a>),

	/// See: [`ReloadEvent`].
	#[serde(borrow)]
	Reload(ReloadEvent<'a>),

	/// See: [`LoadStoppedEvent`].
	#[serde(borrow)]
	LoadStopped(LoadStoppedEvent<'a>),

	/// See: [`ContentChangedEvent`].
	#[serde(borrow)]
	ContentChanged(ContentChangedEvent<'a>),

	/// See: [`DocumentAttributesChangedEvent`].
	#[serde(borrow)]
	AttributesChanged(DocumentAttributesChangedEvent<'a>),

	/// See: [`PageChangedEvent`].
	#[serde(borrow)]
	PageChanged(PageChangedEvent<'a>),
}

impl_tryfrommessage_for_event_wrapper!(DocumentEvents<'_>);

impl DBusInterface for DocumentEvents<'_> {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
}

impl DBusMatchRule for DocumentEvents<'_> {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document'";
}

impl RegistryEventString for DocumentEvents<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}

impl EventTypeProperties for DocumentEvents<'_> {
	fn member(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.member(),
			Self::Reload(inner) => inner.member(),
			Self::LoadStopped(inner) => inner.member(),
			Self::ContentChanged(inner) => inner.member(),
			Self::AttributesChanged(inner) => inner.member(),
			Self::PageChanged(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.interface(),
			Self::Reload(inner) => inner.interface(),
			Self::LoadStopped(inner) => inner.interface(),
			Self::ContentChanged(inner) => inner.interface(),
			Self::AttributesChanged(inner) => inner.interface(),
			Self::PageChanged(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.match_rule(),
			Self::Reload(inner) => inner.match_rule(),
			Self::LoadStopped(inner) => inner.match_rule(),
			Self::ContentChanged(inner) => inner.match_rule(),
			Self::AttributesChanged(inner) => inner.match_rule(),
			Self::PageChanged(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.registry_string(),
			Self::Reload(inner) => inner.registry_string(),
			Self::LoadStopped(inner) => inner.registry_string(),
			Self::ContentChanged(inner) => inner.registry_string(),
			Self::AttributesChanged(inner) => inner.registry_string(),
			Self::PageChanged(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for DocumentEvents<'_> {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::LoadComplete(inner) => inner.path(),
			Self::Reload(inner) => inner.path(),
			Self::LoadStopped(inner) => inner.path(),
			Self::ContentChanged(inner) => inner.path(),
			Self::AttributesChanged(inner) => inner.path(),
			Self::PageChanged(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::LoadComplete(inner) => inner.sender(),
			Self::Reload(inner) => inner.sender(),
			Self::LoadStopped(inner) => inner.sender(),
			Self::ContentChanged(inner) => inner.sender(),
			Self::AttributesChanged(inner) => inner.sender(),
			Self::PageChanged(inner) => inner.sender(),
		}
	}
}

impl_from_user_facing_type_for_event_enum!(PageChangedEvent<'_>, Event::Document);
impl_from_user_facing_type_for_event_enum!(DocumentAttributesChangedEvent<'_>, Event::Document);
impl_from_user_facing_type_for_event_enum!(ContentChangedEvent<'_>, Event::Document);
impl_from_user_facing_type_for_event_enum!(LoadStoppedEvent<'_>, Event::Document);
impl_from_user_facing_type_for_event_enum!(ReloadEvent<'_>, Event::Document);
impl_from_user_facing_type_for_event_enum!(LoadCompleteEvent<'_>, Event::Document);

impl_try_from_event_for_interface_enum!(DocumentEvents<'_>, Event::Document);
impl_from_interface_event_enum_for_event!(DocumentEvents<'_>, Event::Document);

event_wrapper_test_cases!(DocumentEvents, LoadCompleteEvent);

#[cfg(feature = "zbus")]
impl<'a> EventWrapperMessageConversion<'a> for DocumentEvents<'a> {
	fn try_from_message_interface_checked(
		msg: &'a zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			LoadCompleteEvent::DBUS_MEMBER => Ok(DocumentEvents::LoadComplete(
				LoadCompleteEvent::from_message_unchecked(msg, hdr)?,
			)),
			ReloadEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::Reload(ReloadEvent::from_message_unchecked(msg, hdr)?))
			}
			LoadStoppedEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::LoadStopped(LoadStoppedEvent::from_message_unchecked(msg, hdr)?))
			}
			ContentChangedEvent::DBUS_MEMBER => Ok(DocumentEvents::ContentChanged(
				ContentChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			DocumentAttributesChangedEvent::DBUS_MEMBER => Ok(DocumentEvents::AttributesChanged(
				DocumentAttributesChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			PageChangedEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::PageChanged(PageChangedEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Document".into())),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	LoadCompleteEvent<'_>,
	DocumentEvents<'_>,
	DocumentEvents::LoadComplete
);
impl_try_from_event_for_user_facing_type!(
	LoadCompleteEvent<'_>,
	DocumentEvents::LoadComplete,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	ReloadEvent<'_>,
	DocumentEvents<'_>,
	DocumentEvents::Reload
);
impl_try_from_event_for_user_facing_type!(ReloadEvent<'_>, DocumentEvents::Reload, Event::Document);

impl_from_user_facing_event_for_interface_event_enum!(
	LoadStoppedEvent<'_>,
	DocumentEvents<'_>,
	DocumentEvents::LoadStopped
);
impl_try_from_event_for_user_facing_type!(
	LoadStoppedEvent<'_>,
	DocumentEvents::LoadStopped,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	ContentChangedEvent<'_>,
	DocumentEvents<'_>,
	DocumentEvents::ContentChanged
);
impl_try_from_event_for_user_facing_type!(
	ContentChangedEvent<'_>,
	DocumentEvents::ContentChanged,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	DocumentAttributesChangedEvent<'_>,
	DocumentEvents<'_>,
	DocumentEvents::AttributesChanged
);
impl_try_from_event_for_user_facing_type!(
	DocumentAttributesChangedEvent<'_>,
	DocumentEvents::AttributesChanged,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	PageChangedEvent<'_>,
	DocumentEvents<'_>,
	DocumentEvents::PageChanged
);
impl_try_from_event_for_user_facing_type!(
	PageChangedEvent<'_>,
	DocumentEvents::PageChanged,
	Event::Document
);

/// Encapsulates the various different accessibility bus signal types.
///
/// Assumes being non exhaustive to allow for future- or custom signals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Event<'a> {
	/// See: [`DocumentEvents`].
	#[serde(borrow)]
	Document(DocumentEvents<'a>),

	/// See: [`FocusEvents`].
	#[serde(borrow)]
	Focus(FocusEvents<'a>),

	/// See: [`KeyboardEvents`].
	#[serde(borrow)]
	Keyboard(KeyboardEvents<'a>),

	/// See: [`MouseEvents`].
	#[serde(borrow)]
	Mouse(MouseEvents<'a>),

	/// See: [`ObjectEvents`].
	#[serde(borrow)]
	Object(ObjectEvents<'a>),

	/// See: [`TerminalEvents`].
	#[serde(borrow)]
	Terminal(TerminalEvents<'a>),

	/// See: [`WindowEvents`].
	#[serde(borrow)]
	Window(WindowEvents<'a>),

	/// See: [`AvailableEvent`].
	#[serde(borrow)]
	Available(AvailableEvent<'a>),

	/// See: [`CacheEvents`].
	#[serde(borrow)]
	Cache(CacheEvents<'a>),

	/// See: [`EventListenerEvents`].
	#[serde(borrow)]
	Listener(EventListenerEvents<'a>),
}

impl EventTypeProperties for Event<'_> {
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

impl EventProperties for Event<'_> {
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

#[cfg(feature = "zbus")]
impl<'a> TryFrom<&'a zbus::Message> for Event<'a> {
	type Error = AtspiError;

	fn try_from(msg: &'a zbus::Message) -> Result<Event<'a>, AtspiError> {
		let header = msg.header();
		let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
		let interface_str = interface.as_str();

		match interface_str {
			<ObjectEvents as DBusInterface>::DBUS_INTERFACE => {
				Ok(Event::Object(ObjectEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<FocusEvents as DBusInterface>::DBUS_INTERFACE => {
				Ok(Event::Focus(FocusEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<CacheEvents as DBusInterface>::DBUS_INTERFACE => {
				Ok(Event::Cache(CacheEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<WindowEvents as DBusInterface>::DBUS_INTERFACE => {
				Ok(Event::Window(WindowEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<MouseEvents as DBusInterface>::DBUS_INTERFACE => {
				Ok(Event::Mouse(MouseEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<TerminalEvents as DBusInterface>::DBUS_INTERFACE => Ok(Event::Terminal(
				TerminalEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<DocumentEvents as DBusInterface>::DBUS_INTERFACE => Ok(Event::Document(
				DocumentEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<KeyboardEvents as DBusInterface>::DBUS_INTERFACE => Ok(Event::Keyboard(
				KeyboardEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<EventListenerEvents as DBusInterface>::DBUS_INTERFACE => Ok(Event::Listener(
				EventListenerEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<AvailableEvent as DBusInterface>::DBUS_INTERFACE => {
				Ok(AvailableEvent::try_from(msg)?.into())
			}
			_ => Err(AtspiError::InterfaceMatch(format!(
				"No events found with interface {interface_str}"
			))),
		}
	}
}

impl_from_user_facing_type_for_event_enum!(TextCaretMovedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(TextAttributesChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(TextChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(TextSelectionChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(TextBoundsChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(ColumnDeletedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(ColumnReorderedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(ColumnInsertedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(RowDeletedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(RowReorderedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(RowInsertedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(ObjectAttributesChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(AnnouncementEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(ActiveDescendantChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(ModelChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(SelectionChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(VisibleDataChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(ChildrenChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(StateChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(LinkSelectedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(BoundsChangedEvent<'_>, Event::Object);
impl_from_user_facing_type_for_event_enum!(ObjectPropertyChangeEvent<'_>, Event::Object);

impl_from_user_facing_event_for_interface_event_enum!(
	ObjectPropertyChangeEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::PropertyChange
);
impl_try_from_event_for_user_facing_type!(
	ObjectPropertyChangeEvent<'_>,
	ObjectEvents::PropertyChange,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	BoundsChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::BoundsChanged
);
impl_try_from_event_for_user_facing_type!(
	BoundsChangedEvent<'_>,
	ObjectEvents::BoundsChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	LinkSelectedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::LinkSelected
);
impl_try_from_event_for_user_facing_type!(
	LinkSelectedEvent<'_>,
	ObjectEvents::LinkSelected,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	StateChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::StateChanged
);
impl_try_from_event_for_user_facing_type!(
	StateChangedEvent<'_>,
	ObjectEvents::StateChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ChildrenChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::ChildrenChanged
);
impl_try_from_event_for_user_facing_type!(
	ChildrenChangedEvent<'_>,
	ObjectEvents::ChildrenChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	VisibleDataChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::VisibleDataChanged
);
impl_try_from_event_for_user_facing_type!(
	VisibleDataChangedEvent<'_>,
	ObjectEvents::VisibleDataChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	SelectionChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::SelectionChanged
);
impl_try_from_event_for_user_facing_type!(
	SelectionChangedEvent<'_>,
	ObjectEvents::SelectionChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ModelChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::ModelChanged
);
impl_try_from_event_for_user_facing_type!(
	ModelChangedEvent<'_>,
	ObjectEvents::ModelChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ActiveDescendantChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::ActiveDescendantChanged
);
impl_try_from_event_for_user_facing_type!(
	ActiveDescendantChangedEvent<'_>,
	ObjectEvents::ActiveDescendantChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	AnnouncementEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::Announcement
);
impl_try_from_event_for_user_facing_type!(
	AnnouncementEvent<'_>,
	ObjectEvents::Announcement,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ObjectAttributesChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::AttributesChanged
);
impl_try_from_event_for_user_facing_type!(
	ObjectAttributesChangedEvent<'_>,
	ObjectEvents::AttributesChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	RowInsertedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::RowInserted
);
impl_try_from_event_for_user_facing_type!(
	RowInsertedEvent<'_>,
	ObjectEvents::RowInserted,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	RowReorderedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::RowReordered
);
impl_try_from_event_for_user_facing_type!(
	RowReorderedEvent<'_>,
	ObjectEvents::RowReordered,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	RowDeletedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::RowDeleted
);
impl_try_from_event_for_user_facing_type!(
	RowDeletedEvent<'_>,
	ObjectEvents::RowDeleted,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnInsertedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::ColumnInserted
);
impl_try_from_event_for_user_facing_type!(
	ColumnInsertedEvent<'_>,
	ObjectEvents::ColumnInserted,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnReorderedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::ColumnReordered
);
impl_try_from_event_for_user_facing_type!(
	ColumnReorderedEvent<'_>,
	ObjectEvents::ColumnReordered,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnDeletedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::ColumnDeleted
);
impl_try_from_event_for_user_facing_type!(
	ColumnDeletedEvent<'_>,
	ObjectEvents::ColumnDeleted,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextBoundsChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::TextBoundsChanged
);
impl_try_from_event_for_user_facing_type!(
	TextBoundsChangedEvent<'_>,
	ObjectEvents::TextBoundsChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextSelectionChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::TextSelectionChanged
);
impl_try_from_event_for_user_facing_type!(
	TextSelectionChangedEvent<'_>,
	ObjectEvents::TextSelectionChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextAttributesChangedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::TextAttributesChanged
);
impl_try_from_event_for_user_facing_type!(
	TextAttributesChangedEvent<'_>,
	ObjectEvents::TextAttributesChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextCaretMovedEvent<'_>,
	ObjectEvents<'_>,
	ObjectEvents::TextCaretMoved
);
impl_try_from_event_for_user_facing_type!(
	TextCaretMovedEvent<'_>,
	ObjectEvents::TextCaretMoved,
	Event::Object
);

impl DBusMatchRule for ObjectEvents<'_> {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object'";
}

impl DBusInterface for ObjectEvents<'_> {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
}

impl RegistryEventString for ObjectEvents<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "object:";
}

#[cfg(feature = "zbus")]
impl<'a> EventWrapperMessageConversion<'a> for ObjectEvents<'a> {
	fn try_from_message_interface_checked(
		msg: &'a zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			ObjectPropertyChangeEvent::DBUS_MEMBER => Ok(ObjectEvents::PropertyChange(
				ObjectPropertyChangeEvent::from_message_unchecked(msg, hdr)?,
			)),
			BoundsChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::BoundsChanged(
				BoundsChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			LinkSelectedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::LinkSelected(LinkSelectedEvent::from_message_unchecked(msg, hdr)?))
			}
			StateChangedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::StateChanged(StateChangedEvent::from_message_unchecked(msg, hdr)?))
			}
			ChildrenChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::ChildrenChanged(
				ChildrenChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			VisibleDataChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::VisibleDataChanged(
				VisibleDataChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			SelectionChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::SelectionChanged(
				SelectionChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			ModelChangedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::ModelChanged(ModelChangedEvent::from_message_unchecked(msg, hdr)?))
			}
			ActiveDescendantChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::ActiveDescendantChanged(
				ActiveDescendantChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			AnnouncementEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::Announcement(AnnouncementEvent::from_message_unchecked(msg, hdr)?))
			}
			ObjectAttributesChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::AttributesChanged(
				ObjectAttributesChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			RowInsertedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::RowInserted(RowInsertedEvent::from_message_unchecked(msg, hdr)?))
			}
			RowReorderedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::RowReordered(RowReorderedEvent::from_message_unchecked(msg, hdr)?))
			}
			RowDeletedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::RowDeleted(RowDeletedEvent::from_message_unchecked(msg, hdr)?))
			}
			ColumnInsertedEvent::DBUS_MEMBER => Ok(ObjectEvents::ColumnInserted(
				ColumnInsertedEvent::from_message_unchecked(msg, hdr)?,
			)),
			ColumnReorderedEvent::DBUS_MEMBER => Ok(ObjectEvents::ColumnReordered(
				ColumnReorderedEvent::from_message_unchecked(msg, hdr)?,
			)),
			ColumnDeletedEvent::DBUS_MEMBER => Ok(ObjectEvents::ColumnDeleted(
				ColumnDeletedEvent::from_message_unchecked(msg, hdr)?,
			)),
			TextBoundsChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextBoundsChanged(
				TextBoundsChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			TextSelectionChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextSelectionChanged(
				TextSelectionChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			TextChangedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::TextChanged(TextChangedEvent::from_message_unchecked(msg, hdr)?))
			}
			TextAttributesChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextAttributesChanged(
				TextAttributesChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			TextCaretMovedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextCaretMoved(
				TextCaretMovedEvent::from_message_unchecked(msg, hdr)?,
			)),
			_ => Err(AtspiError::MemberMatch(format!(
				"No matching member {member} for interface {}",
				Self::DBUS_INTERFACE,
			))),
		}
	}
}

/// All events related to the `org.a11y.atspi.Cache` interface.
/// Note that these are not telling the client that an item *has been added* to a cache.
/// It is telling the client "here is a bunch of information to store it in your cache".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum CacheEvents<'a> {
	/// See: [`AddAccessibleEvent`].
	#[serde(borrow)]
	Add(AddAccessibleEvent<'a>),

	/// See: [`LegacyAddAccessibleEvent`].
	#[serde(borrow)]
	LegacyAdd(LegacyAddAccessibleEvent<'a>),

	/// See: [`RemoveAccessibleEvent`].
	#[serde(borrow)]
	Remove(RemoveAccessibleEvent<'a>),
}

impl_from_user_facing_type_for_event_enum!(RemoveAccessibleEvent<'_>, Event::Cache);
impl_from_user_facing_type_for_event_enum!(AddAccessibleEvent<'_>, Event::Cache);
impl_from_user_facing_type_for_event_enum!(LegacyAddAccessibleEvent<'_>, Event::Cache);

impl DBusMatchRule for CacheEvents<'_> {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Cache'";
}

impl RegistryEventString for CacheEvents<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "Cache";
}

impl DBusInterface for CacheEvents<'_> {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

impl EventTypeProperties for CacheEvents<'_> {
	fn member(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.member(),
			Self::LegacyAdd(inner) => inner.member(),
			Self::Remove(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.interface(),
			Self::LegacyAdd(inner) => inner.interface(),
			Self::Remove(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.match_rule(),
			Self::LegacyAdd(inner) => inner.match_rule(),
			Self::Remove(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.registry_string(),
			Self::LegacyAdd(inner) => inner.registry_string(),
			Self::Remove(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for CacheEvents<'_> {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Add(inner) => inner.path(),
			Self::LegacyAdd(inner) => inner.path(),
			Self::Remove(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Add(inner) => inner.sender(),
			Self::LegacyAdd(inner) => inner.sender(),
			Self::Remove(inner) => inner.sender(),
		}
	}
}

#[cfg(feature = "zbus")]
impl<'a> EventWrapperMessageConversion<'a> for CacheEvents<'a> {
	fn try_from_message_interface_checked(
		msg: &'a zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			AddAccessibleEvent::DBUS_MEMBER => {
				let body = msg.body();
				let sig = body.signature();
				if sig == CacheItem::SIGNATURE {
					Ok(CacheEvents::Add(AddAccessibleEvent::from_message_unchecked(msg, hdr)?))
				} else if sig == LegacyCacheItem::SIGNATURE {
					Ok(CacheEvents::LegacyAdd(LegacyAddAccessibleEvent::from_message_unchecked(
						msg, hdr,
					)?))
				} else {
					Err(AtspiError::SignatureMatch(format!(
						"No matching event for signature {} in interface {}",
						sig.to_string(),
						Self::DBUS_INTERFACE
					)))
				}
			}
			RemoveAccessibleEvent::DBUS_MEMBER => {
				Ok(CacheEvents::Remove(RemoveAccessibleEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch(format!(
				"No member {} in {}",
				member.as_str(),
				Self::DBUS_INTERFACE
			))),
		}
	}
}

impl_tryfrommessage_for_event_wrapper!(CacheEvents<'_>);

impl_from_user_facing_event_for_interface_event_enum!(
	LegacyAddAccessibleEvent<'_>,
	CacheEvents<'_>,
	CacheEvents::LegacyAdd
);
impl_try_from_event_for_user_facing_type!(
	LegacyAddAccessibleEvent<'_>,
	CacheEvents::LegacyAdd,
	Event::Cache
);
impl_from_user_facing_event_for_interface_event_enum!(
	AddAccessibleEvent<'_>,
	CacheEvents<'_>,
	CacheEvents::Add
);
impl_try_from_event_for_user_facing_type!(AddAccessibleEvent<'_>, CacheEvents::Add, Event::Cache);
impl_from_user_facing_event_for_interface_event_enum!(
	RemoveAccessibleEvent<'_>,
	CacheEvents<'_>,
	CacheEvents::Remove
);
impl_try_from_event_for_user_facing_type!(
	RemoveAccessibleEvent<'_>,
	CacheEvents::Remove,
	Event::Cache
);
impl_try_from_event_for_interface_enum!(CacheEvents<'_>, Event::Cache);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum FocusEvents<'a> {
	/// See: [`FocusEvent`].
	#[serde(borrow)]
	Focus(FocusEvent<'a>),
}

impl_tryfrommessage_for_event_wrapper!(FocusEvents<'_>);

impl_from_interface_event_enum_for_event!(FocusEvents, Event::Focus);
impl_try_from_event_for_user_facing_type!(FocusEvent<'_>, FocusEvents::Focus, Event::Focus);

impl EventTypeProperties for FocusEvents<'_> {
	fn member(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.member(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.match_rule(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.interface(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for FocusEvents<'_> {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Focus(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Focus(inner) => inner.sender(),
		}
	}
}

impl_try_from_event_for_interface_enum!(FocusEvents<'_>, Event::Focus);

impl_from_user_facing_event_for_interface_event_enum!(
	FocusEvent<'_>,
	FocusEvents<'_>,
	FocusEvents::Focus
);
impl_from_user_facing_type_for_event_enum!(FocusEvent<'_>, Event::Focus);
event_wrapper_test_cases!(FocusEvents, FocusEvent);

#[cfg(feature = "zbus")]
impl<'a> EventWrapperMessageConversion<'a> for FocusEvents<'a> {
	fn try_from_message_interface_checked(
		msg: &'a zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			FocusEvent::DBUS_MEMBER => {
				Ok(FocusEvents::Focus(FocusEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch(format!(
				"No matching member {member} for interface {}",
				Self::DBUS_INTERFACE,
			))),
		}
	}
}

impl DBusMatchRule for FocusEvents<'_> {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus'";
}

impl RegistryEventString for FocusEvents<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "focus:";
}

impl DBusInterface for FocusEvents<'_> {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
}

/// All events related to the `org.a11y.atspi.Event.Terminal` interface.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum TerminalEvents<'a> {
	/// See: [`LineChangedEvent`].
	#[serde(borrow)]
	LineChanged(LineChangedEvent<'a>),

	/// See: [`ColumnCountChangedEvent`].
	#[serde(borrow)]
	ColumnCountChanged(ColumnCountChangedEvent<'a>),

	/// See: [`LineCountChangedEvent`].
	#[serde(borrow)]
	LineCountChanged(LineCountChangedEvent<'a>),

	/// See: [`ApplicationChangedEvent`].
	#[serde(borrow)]
	ApplicationChanged(ApplicationChangedEvent<'a>),

	/// See: [`CharWidthChangedEvent`].
	#[serde(borrow)]
	CharWidthChanged(CharWidthChangedEvent<'a>),
}

impl_tryfrommessage_for_event_wrapper!(TerminalEvents<'_>);

impl EventTypeProperties for TerminalEvents<'_> {
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

impl EventProperties for TerminalEvents<'_> {
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

impl_from_user_facing_type_for_event_enum!(CharWidthChangedEvent<'_>, Event::Terminal);
impl_from_user_facing_type_for_event_enum!(ApplicationChangedEvent<'_>, Event::Terminal);
impl_from_user_facing_type_for_event_enum!(LineCountChangedEvent<'_>, Event::Terminal);
impl_from_user_facing_type_for_event_enum!(ColumnCountChangedEvent<'_>, Event::Terminal);
impl_from_user_facing_type_for_event_enum!(LineChangedEvent<'_>, Event::Terminal);

impl_try_from_event_for_interface_enum!(TerminalEvents<'_>, Event::Terminal);
impl_from_interface_event_enum_for_event!(TerminalEvents, Event::Terminal);

event_wrapper_test_cases!(TerminalEvents, LineChangedEvent);

impl DBusMatchRule for TerminalEvents<'_> {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal'";
}

impl RegistryEventString for TerminalEvents<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "terminal:";
}

impl DBusInterface for TerminalEvents<'_> {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
}

#[cfg(feature = "zbus")]
impl<'a> EventWrapperMessageConversion<'a> for TerminalEvents<'a> {
	fn try_from_message_interface_checked(
		msg: &'a zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			LineChangedEvent::DBUS_MEMBER => {
				Ok(TerminalEvents::LineChanged(LineChangedEvent::from_message_unchecked(msg, hdr)?))
			}
			ColumnCountChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::ColumnCountChanged(
				ColumnCountChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			LineCountChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::LineCountChanged(
				LineCountChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			ApplicationChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::ApplicationChanged(
				ApplicationChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			CharWidthChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::CharWidthChanged(
				CharWidthChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			_ => Err(AtspiError::MemberMatch("No matching member for Terminal".into())),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	LineChangedEvent<'_>,
	TerminalEvents<'_>,
	TerminalEvents::LineChanged
);
impl_try_from_event_for_user_facing_type!(
	LineChangedEvent<'_>,
	TerminalEvents::LineChanged,
	Event::Terminal
);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnCountChangedEvent<'_>,
	TerminalEvents<'_>,
	TerminalEvents::ColumnCountChanged
);
impl_try_from_event_for_user_facing_type!(
	ColumnCountChangedEvent<'_>,
	TerminalEvents::ColumnCountChanged,
	Event::Terminal
);
impl_from_user_facing_event_for_interface_event_enum!(
	LineCountChangedEvent<'_>,
	TerminalEvents<'_>,
	TerminalEvents::LineCountChanged
);
impl_try_from_event_for_user_facing_type!(
	LineCountChangedEvent<'_>,
	TerminalEvents::LineCountChanged,
	Event::Terminal
);
impl_from_user_facing_event_for_interface_event_enum!(
	ApplicationChangedEvent<'_>,
	TerminalEvents<'_>,
	TerminalEvents::ApplicationChanged
);
impl_try_from_event_for_user_facing_type!(
	ApplicationChangedEvent<'_>,
	TerminalEvents::ApplicationChanged,
	Event::Terminal
);
impl_from_user_facing_event_for_interface_event_enum!(
	CharWidthChangedEvent<'_>,
	TerminalEvents<'_>,
	TerminalEvents::CharWidthChanged
);
impl_try_from_event_for_user_facing_type!(
	CharWidthChangedEvent<'_>,
	TerminalEvents::CharWidthChanged,
	Event::Terminal
);

/// All events on the `org.a11y.atspi.Event.Window` interface.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum WindowEvents<'a> {
	/// See: [`WindowPropertyChangeEvent`].
	#[serde(borrow)]
	PropertyChange(WindowPropertyChangeEvent<'a>),

	/// See: [`MinimizeEvent`].
	#[serde(borrow)]
	Minimize(MinimizeEvent<'a>),

	/// See: [`MaximizeEvent`].
	#[serde(borrow)]
	Maximize(MaximizeEvent<'a>),

	/// See: [`RestoreEvent`].
	#[serde(borrow)]
	Restore(RestoreEvent<'a>),

	/// See: [`CloseEvent`].
	#[serde(borrow)]
	Close(CloseEvent<'a>),
	/// See: [`CreateEvent`].
	#[serde(borrow)]
	Create(CreateEvent<'a>),

	/// See: [`ReparentEvent`].
	#[serde(borrow)]
	Reparent(ReparentEvent<'a>),

	/// See: [`DesktopCreateEvent`].
	#[serde(borrow)]
	DesktopCreate(DesktopCreateEvent<'a>),

	/// See: [`DesktopDestroyEvent`].
	#[serde(borrow)]
	DesktopDestroy(DesktopDestroyEvent<'a>),

	/// See: [`DestroyEvent`].
	#[serde(borrow)]
	Destroy(DestroyEvent<'a>),

	/// See: [`ActivateEvent`].
	#[serde(borrow)]
	Activate(ActivateEvent<'a>),

	/// See: [`DeactivateEvent`].
	#[serde(borrow)]
	Deactivate(DeactivateEvent<'a>),
	/// See: [`RaiseEvent`].
	#[serde(borrow)]
	Raise(RaiseEvent<'a>),

	/// See: [`LowerEvent`].
	#[serde(borrow)]
	Lower(LowerEvent<'a>),

	/// See: [`MoveEvent`].
	#[serde(borrow)]
	Move(MoveEvent<'a>),

	/// See: [`ResizeEvent`].
	#[serde(borrow)]
	Resize(ResizeEvent<'a>),

	/// See: [`ShadeEvent`].
	#[serde(borrow)]
	Shade(ShadeEvent<'a>),

	/// See: [`UUshadeEvent`].
	#[serde(borrow)]
	UUshade(UUshadeEvent<'a>),

	/// See: [`RestyleEvent`].
	#[serde(borrow)]
	Restyle(RestyleEvent<'a>),
}

impl_tryfrommessage_for_event_wrapper!(WindowEvents<'_>);

impl EventTypeProperties for WindowEvents<'_> {
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

impl EventProperties for WindowEvents<'_> {
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

impl_from_user_facing_type_for_event_enum!(ReparentEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(CloseEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(RestoreEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(MaximizeEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(MinimizeEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(WindowPropertyChangeEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(RestyleEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(UUshadeEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(ShadeEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(ResizeEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(MoveEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(LowerEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(RaiseEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(DeactivateEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(ActivateEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(DestroyEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(DesktopDestroyEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(DesktopCreateEvent<'_>, Event::Window);
impl_from_user_facing_type_for_event_enum!(CreateEvent<'_>, Event::Window);

impl_try_from_event_for_interface_enum!(WindowEvents<'_>, Event::Window);
impl_from_interface_event_enum_for_event!(WindowEvents, Event::Window);

event_wrapper_test_cases!(WindowEvents, MoveEvent);

impl DBusMatchRule for WindowEvents<'_> {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window'";
}

impl DBusInterface for WindowEvents<'_> {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
}

impl RegistryEventString for WindowEvents<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "window:";
}

#[cfg(feature = "zbus")]
impl<'a> EventWrapperMessageConversion<'a> for WindowEvents<'a> {
	fn try_from_message_interface_checked(
		msg: &'a zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			WindowPropertyChangeEvent::DBUS_MEMBER => Ok(WindowEvents::PropertyChange(
				WindowPropertyChangeEvent::from_message_unchecked(msg, hdr)?,
			)),
			MinimizeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Minimize(MinimizeEvent::from_message_unchecked(msg, hdr)?))
			}
			MaximizeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Maximize(MaximizeEvent::from_message_unchecked(msg, hdr)?))
			}
			RestoreEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Restore(RestoreEvent::from_message_unchecked(msg, hdr)?))
			}
			CloseEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Close(CloseEvent::from_message_unchecked(msg, hdr)?))
			}
			CreateEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Create(CreateEvent::from_message_unchecked(msg, hdr)?))
			}
			ReparentEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Reparent(ReparentEvent::from_message_unchecked(msg, hdr)?))
			}
			DesktopCreateEvent::DBUS_MEMBER => Ok(WindowEvents::DesktopCreate(
				DesktopCreateEvent::from_message_unchecked(msg, hdr)?,
			)),
			DesktopDestroyEvent::DBUS_MEMBER => Ok(WindowEvents::DesktopDestroy(
				DesktopDestroyEvent::from_message_unchecked(msg, hdr)?,
			)),
			DestroyEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Destroy(DestroyEvent::from_message_unchecked(msg, hdr)?))
			}
			ActivateEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Activate(ActivateEvent::from_message_unchecked(msg, hdr)?))
			}
			DeactivateEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Deactivate(DeactivateEvent::from_message_unchecked(msg, hdr)?))
			}
			RaiseEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Raise(RaiseEvent::from_message_unchecked(msg, hdr)?))
			}
			LowerEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Lower(LowerEvent::from_message_unchecked(msg, hdr)?))
			}
			MoveEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Move(MoveEvent::from_message_unchecked(msg, hdr)?))
			}
			ResizeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Resize(ResizeEvent::from_message_unchecked(msg, hdr)?))
			}
			ShadeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Shade(ShadeEvent::from_message_unchecked(msg, hdr)?))
			}
			UUshadeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::UUshade(UUshadeEvent::from_message_unchecked(msg, hdr)?))
			}
			RestyleEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Restyle(RestyleEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Window".into())),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	WindowPropertyChangeEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::PropertyChange
);
impl_try_from_event_for_user_facing_type!(
	WindowPropertyChangeEvent<'_>,
	WindowEvents::PropertyChange,
	Event::Window
);
impl_from_user_facing_event_for_interface_event_enum!(
	MinimizeEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Minimize
);
impl_try_from_event_for_user_facing_type!(MinimizeEvent<'_>, WindowEvents::Minimize, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	MaximizeEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Maximize
);
impl_try_from_event_for_user_facing_type!(MaximizeEvent<'_>, WindowEvents::Maximize, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	RestoreEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Restore
);
impl_try_from_event_for_user_facing_type!(RestoreEvent<'_>, WindowEvents::Restore, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	CloseEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Close
);
impl_try_from_event_for_user_facing_type!(CloseEvent<'_>, WindowEvents::Close, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	CreateEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Create
);
impl_try_from_event_for_user_facing_type!(CreateEvent<'_>, WindowEvents::Create, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	ReparentEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Reparent
);
impl_try_from_event_for_user_facing_type!(ReparentEvent<'_>, WindowEvents::Reparent, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	DesktopCreateEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::DesktopCreate
);
impl_try_from_event_for_user_facing_type!(
	DesktopCreateEvent<'_>,
	WindowEvents::DesktopCreate<'_>,
	Event::Window
);
impl_from_user_facing_event_for_interface_event_enum!(
	DesktopDestroyEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::DesktopDestroy
);
impl_try_from_event_for_user_facing_type!(
	DesktopDestroyEvent<'_>,
	WindowEvents::DesktopDestroy<'_>,
	Event::Window
);
impl_from_user_facing_event_for_interface_event_enum!(
	DestroyEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Destroy
);
impl_try_from_event_for_user_facing_type!(DestroyEvent<'_>, WindowEvents::Destroy, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	ActivateEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Activate
);
impl_try_from_event_for_user_facing_type!(ActivateEvent<'_>, WindowEvents::Activate, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	DeactivateEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Deactivate
);
impl_try_from_event_for_user_facing_type!(
	DeactivateEvent<'_>,
	WindowEvents::Deactivate<'_>,
	Event::Window
);
impl_from_user_facing_event_for_interface_event_enum!(
	RaiseEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Raise
);
impl_try_from_event_for_user_facing_type!(RaiseEvent<'_>, WindowEvents::Raise, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	LowerEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Lower
);
impl_try_from_event_for_user_facing_type!(LowerEvent<'_>, WindowEvents::Lower, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	MoveEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Move
);
impl_try_from_event_for_user_facing_type!(MoveEvent<'_>, WindowEvents::Move, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	ResizeEvent<'_>,
	WindowEvents<'_>,
	WindowEvents<'_>::Resize
);
impl_try_from_event_for_user_facing_type!(ResizeEvent<'_>, WindowEvents::Resize, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	ShadeEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Shade
);
impl_try_from_event_for_user_facing_type!(ShadeEvent<'_>, WindowEvents::Shade, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	UUshadeEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::UUshade
);
impl_try_from_event_for_user_facing_type!(UUshadeEvent<'_>, WindowEvents::UUshade, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	RestyleEvent<'_>,
	WindowEvents<'_>,
	WindowEvents::Restyle
);
impl_try_from_event_for_user_facing_type!(RestyleEvent<'_>, WindowEvents::Restyle, Event::Window);

/// The events that can be emitted by the registry daemon.
/// This enum is used to wrap the events that are emitted by the registry daemon.
/// The events are [`EventListenerRegisteredEvent`] and [`EventListenerDeregisteredEvent`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum EventListenerEvents<'a> {
	/// See: [`EventListenerRegisteredEvent`].
	#[serde(borrow)]
	Registered(EventListenerRegisteredEvent<'a>),

	/// See: [`EventListenerDeregisteredEvent`].
	#[serde(borrow)]
	Deregistered(EventListenerDeregisteredEvent<'a>),
}

impl_tryfrommessage_for_event_wrapper!(EventListenerEvents<'_>);

impl DBusInterface for EventListenerEvents<'_> {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Registry";
}

impl DBusMatchRule for EventListenerEvents<'_> {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Registry'";
}

impl RegistryEventString for EventListenerEvents<'_> {
	const REGISTRY_EVENT_STRING: &'static str = "Event";
}

impl EventTypeProperties for EventListenerEvents<'_> {
	fn member(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.member(),
			Self::Deregistered(inner) => inner.member(),
		}
	}

	fn match_rule(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.match_rule(),
			Self::Deregistered(inner) => inner.match_rule(),
		}
	}

	fn interface(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.interface(),
			Self::Deregistered(inner) => inner.interface(),
		}
	}

	fn registry_string(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.registry_string(),
			Self::Deregistered(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for EventListenerEvents<'_> {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Registered(inner) => inner.path(),
			Self::Deregistered(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Registered(inner) => inner.sender(),
			Self::Deregistered(inner) => inner.sender(),
		}
	}
}

#[cfg(feature = "zbus")]
impl<'a> EventWrapperMessageConversion<'a> for EventListenerEvents<'a> {
	fn try_from_message_interface_checked(
		msg: &'a zbus::Message,
		hdr: &zbus::message::Header,
	) -> Result<Self, crate::AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			EventListenerRegisteredEvent::DBUS_MEMBER => Ok(EventListenerEvents::Registered(
				EventListenerRegisteredEvent::from_message_unchecked(msg, hdr)?,
			)),
			EventListenerDeregisteredEvent::DBUS_MEMBER => Ok(EventListenerEvents::Deregistered(
				EventListenerDeregisteredEvent::from_message_unchecked(msg, hdr)?,
			)),
			_ => Err(AtspiError::MemberMatch(format!(
				"No member {} in {}",
				member.as_str(),
				Self::DBUS_INTERFACE
			))),
		}
	}
}

impl_try_from_event_for_interface_enum!(EventListenerEvents<'_>, Event::Listener);
impl_from_interface_event_enum_for_event!(EventListenerEvents, Event::Listener);

impl<'a> From<AvailableEvent<'a>> for Event<'a> {
	fn from(ev: AvailableEvent) -> Event {
		Event::Available(ev)
	}
}

#[cfg(feature = "zbus")]
impl<'a> TryFrom<Event<'a>> for AvailableEvent<'a> {
	type Error = AtspiError;
	fn try_from(generic_event: Event) -> Result<AvailableEvent, Self::Error> {
		if let Event::Available(specific_event) = generic_event {
			Ok(specific_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}

event_wrapper_test_cases!(EventListenerEvents, EventListenerRegisteredEvent);
