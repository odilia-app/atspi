//! ## Signified signal types
//!
//! The generic `AtspiEvent` has a specific meaning depending on its origin.
//! This module offers the signified types and their conversions from a generic `AtpiEvent`.
//!
//! The `TrySignify` macro implements a `TryFrom<Event>` on a per-name and member basis
//!

use crate::{
    events::{
        AddAccessibleEvent, AtspiEvent, AvailableEvent, CacheEvents, EventInterfaces,
        EventListenerDeregisteredEvent, EventListenerEvents, EventListenerRegisteredEvent,
        GenericEvent, RemoveAccessibleEvent,
    },
    identify::{
        document::{self, *},
        focus::{self, *},
        keyboard::{self, *},
        mouse::{self, *},
        object::{self, *},
        terminal::{self, *},
        window::{self, *},
    },
    AtspiError, Event,
};
use std::collections::HashMap;
use std::sync::Arc;
use zbus::{names::MemberName, zvariant, Message};
use zbus_names::{self, InterfaceName};
use zvariant::OwnedValue;

/// All Atspi / Qspi event types encapsulate `AtspiEvent`.
/// This trait allows access to the underlying item.
pub trait Signified {
    type Inner;

    fn inner(&self) -> &AtspiEvent;
    fn properties(&self) -> &HashMap<String, OwnedValue>;
    fn kind(&self) -> &str;
}

/// Shared functionality of Events, through its `Message` header
impl<T> GenericEvent for T
where
    T: Signified,
{
    /// Serialized bus message.
    #[must_use]
    fn message(&self) -> &Arc<Message> {
        &self.inner().message
    }

    // For now this returns the full interface name because the lifetimes in [`zbus_names`][zbus::names] are
    // wrong such that the `&str` you can get from a
    // [`zbus_names::InterfaceName`][zbus::names::InterfaceName] is tied to the lifetime of that
    // name, not to the lifetime of the message as it should be. In future, this will return only
    // the last component of the interface name (I.E. "Object" from
    // "org.a11y.atspi.Event.Object").

    /// The emitting interface.
    #[must_use]
    fn interface(&self) -> Option<InterfaceName<'_>> {
        self.inner().message.interface()
    }

    /// The interface member that dispatched this event / signal.
    ///
    /// Members of the interface are either signals, methods or properties.
    /// eg. `PropertyChanged` or `TextChanged`
    #[must_use]
    fn member(&self) -> Option<MemberName<'_>> {
        self.inner().message.member()
    }

    /// The object path to the object where the signal is emitted from.
    #[must_use]
    fn path(&self) -> std::option::Option<zbus::zvariant::ObjectPath<'_>> {
        self.inner().message.path()
    }

    /// Identifies the `sender` of the `Event`.
    /// # Errors
    /// - when deserializeing the header failed, or
    /// * When `zbus::get_field!` finds that 'sender' is an invalid field.
    fn sender(&self) -> Result<Option<zbus::names::UniqueName>, crate::AtspiError> {
        Ok(self.inner().message.header()?.sender()?.cloned())
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for document::AttributesChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Document(DocumentEvents::AttributesChanged(e))) = ev { Ok(e) } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for document::ContentChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Document(DocumentEvents::ContentChanged(event))) = ev { 
            Ok(event) 
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for document::LoadStoppedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Document(DocumentEvents::LoadStopped(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for document::PageChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event:: Interfaces(EventInterfaces::Document(DocumentEvents::PageChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for document::ReloadEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Document(DocumentEvents::Reload(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

// TODO: Remove me when the event is removed from crate!
#[rustfmt::skip]
impl TryFrom<Event> for focus::FocusEvent  {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Focus(FocusEvents::Focus(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for keyboard::ModifiersEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Keyboard(KeyboardEvents::Modifiers(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for mouse::AbsEvent  {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Mouse(MouseEvents::Abs(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for mouse::RelEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Mouse(MouseEvents::Rel(event))) = ev {            
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for mouse::ButtonEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Mouse(MouseEvents::Button(event))) = ev {            
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::ActiveDescendantChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ActiveDescendantChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::AnnouncementEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::Announcement(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::AttributesChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::AttributesChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::BoundsChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::BoundsChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::ChildrenChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ChildrenChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::ColumnDeletedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ColumnDeleted(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::ColumnInsertedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ColumnInserted(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::ColumnReorderedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ColumnReordered(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::LinkSelectedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::LinkSelected(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::ModelChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ModelChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::PropertyChangeEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::PropertyChange(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::RowDeletedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::RowDeleted(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::RowInsertedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::RowInserted(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::RowReorderedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::RowReordered(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::SelectionChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::SelectionChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::StateChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::StateChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::TextAttributesChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextAttributesChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::TextBoundsChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextBoundsChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::TextCaretMovedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextCaretMoved(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::TextChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::TextSelectionChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextSelectionChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for object::VisibleDataChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::VisibleDataChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for terminal::ApplicationChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Terminal(TerminalEvents::ApplicationChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for terminal::CharWidthChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Terminal(TerminalEvents::CharWidthChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for terminal::ColumnCountChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Terminal(TerminalEvents::ColumnCountChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for terminal::LineChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Terminal(TerminalEvents::LineChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for terminal::LineCountChangedEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Terminal(TerminalEvents::LineCountChanged(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::ActivateEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Activate(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::CloseEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Close(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::CreateEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Create(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::DeactivateEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Deactivate(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::DesktopCreateEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::DesktopCreate(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::DesktopDestroyEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::DesktopDestroy(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::DestroyEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Destroy(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::LowerEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Lower(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::MaximizeEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Maximize(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::MinimizeEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Minimize(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::MoveEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Move(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::PropertyChangeEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::PropertyChange(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::RaiseEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Raise(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::ReparentEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Reparent(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::ResizeEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Resize(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::RestoreEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Restore(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::RestyleEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Restyle(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::ShadeEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Shade(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for window::UUshadeEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Interfaces(EventInterfaces::Window(WindowEvents::UUshade(event))) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for AddAccessibleEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Cache(CacheEvents::Add(event)) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for RemoveAccessibleEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Cache(CacheEvents::Remove(event)) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for AvailableEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Available(event) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for EventListenerRegisteredEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Listener(EventListenerEvents::Registered(event)) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}

#[rustfmt::skip]
impl TryFrom<Event> for EventListenerDeregisteredEvent {
    type Error = AtspiError;
    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        if let Event::Listener(EventListenerEvents::Deregistered(event)) = ev {
            Ok(event)
        } else {
            Err(AtspiError::Conversion("invalid type"))
        }
    }
}
