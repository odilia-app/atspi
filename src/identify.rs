//! ## Signified signal types
//!
//! The generic `AtspiEvent` has a specific meaning depending on its origin.
//! This module offers the signified types and their conversions from a generic `AtpiEvent`.
//!
//! The `TrySignify` macro implements a `TryFrom<Event>` on a per-name and member basis
//!

use crate::error::AtspiError;
use crate::events::{AtspiEvent, GenericEvent};
use atspi_macros::TrySignify;
use std::collections::HashMap;
use std::sync::Arc;
use zbus::zvariant::OwnedObjectPath;
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
    fn path(&self) -> std::option::Option<zbus::zvariant::OwnedObjectPath> {
        let ev = self.inner();
        Some(OwnedObjectPath::from(ev.message.path().unwrap()))
    }

    /// Identifies the `sender` of the `Event`.
    /// # Errors
    /// - when deserializeing the header failed, or
    /// * When `zbus::get_field!` finds that 'sender' is an invalid field.
    fn sender(&self) -> Result<Option<zbus::names::UniqueName>, crate::AtspiError> {
        Ok(self.inner().message.header()?.sender()?.cloned())
    }
}

/// Any of the `Document` events.
///
/// If you are interested in `Event.Document` events, this enum
/// may help you select for these:
///
/// # Example
/// ```
/// // Boilerplate omitted.
/// use crate::identify::DocumentEvent;
///
/// while let Ok(Some(ev)) = event_stream.next().await? {
///     let doc_ev: DocumentEvent = ev.try_into()?;
/// }
///  ```
/// The event details encoded in the de-generalized types.
///
///
///
/// Event table for the contained types:
///
/// |Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties|
/// |:--|---|---|---|---|---|---|
/// |Document|LoadComplete|    |    |    |    |properties|
/// |Document|Reload|    |    |    |    |properties|
/// |Document|LoadStopped|    |    |    |    |properties|
/// |Document|ContentChanged|    |    |    |    |properties|
/// |Document|AttributesChanged|    |    |    |    |properties|
/// |Document|PageChanged|    |    |    |    |properties|
#[derive(Debug, Clone)]
pub enum DocumentEvents {
    LoadComplete(LoadCompleteEvent),
    Reload(ReloadEvent),
    LoadStopped(LoadStoppedEvent),
    ContentChanged(ContentChangedEvent),
    AttributesChanged(AttributesChangedEvent),
    PageChanged(PageChangedEvent),
}

impl TryFrom<AtspiEvent> for DocumentEvents {
    type Error = AtspiError;

    fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
        let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
        match member.as_str() {
            "LoadComplete" => Ok(DocumentEvents::LoadComplete(LoadCompleteEvent(ev))),
            "Reload" => Ok(DocumentEvents::Reload(ReloadEvent(ev))),
            "LoadStopped" => Ok(DocumentEvents::LoadStopped(LoadStoppedEvent(ev))),
            "ContentChanged" => Ok(DocumentEvents::ContentChanged(ContentChangedEvent(ev))),
            "AttributesChanged" => {
                Ok(DocumentEvents::AttributesChanged(AttributesChangedEvent(ev)))
            }
            "PageChanged" => Ok(DocumentEvents::PageChanged(PageChangedEvent(ev))),
            _ => Err(AtspiError::MemberMatch("No matching member for Document".into())),
        }
    }
}

/// Any of the `Object` events.
///
/// Event table for the contained types:
///
/// |Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties|
/// |:--|---|---|---|---|---|---|
/// |Object|PropertyChange|property|    |    |value|properties|
/// |Object|BoundsChanged|    |    |    |    |properties|
/// |Object|LinkSelected|    |    |    |    |properties|
/// |Object|StateChanged|state|enabled|    |    |properties|
/// |`Object|ChildrenChanged|operation|index_in_parent`|    |child|properties|
/// |Object|VisibleDataChanged|    |    |    |    |properties|
/// |Object|SelectionChanged|    |    |    |    |properties|
/// |Object|ModelChanged|    |    |    |    |properties|
/// |Object|ActiveDescendantChanged|    |    |    |child|properties|
/// |Object|Announcement|text|    |    |    |properties|
/// |Object|AttributesChanged|    |    |    |    |properties|
/// |Object|RowInserted|    |    |    |    |properties|
/// |Object|RowReordered|    |    |    |    |properties|
/// |Object|RowDeleted|    |    |    |    |properties|
/// |Object|ColumnInserted|    |    |    |    |properties|
/// |Object|ColumnReordered|    |    |    |    |properties|
/// |Object|ColumnDeleted|    |    |    |    |properties|
/// |Object|TextBoundsChanged|    |    |    |    |properties|
/// |Object|TextSelectionChanged|    |    |    |    |properties|
/// |`Object|TextChanged|detail|start_pos|end_pos|text|properties`|
/// |Object|TextAttributesChanged|    |    |    |    |properties|
/// |Object|TextCaretMoved|    |position|    |    |properties|
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ObjectEvents {
    PropertyChange(object::PropertyChangeEvent),
    BoundsChanged(object::BoundsChangedEvent),
    LinkSelected(object::LinkSelectedEvent),
    StateChanged(object::StateChangedEvent),
    ChildrenChanged(object::ChildrenChangedEvent),
    VisibleDataChanged(object::VisibleDataChangedEvent),
    SelectionChanged(object::SelectionChangedEvent),
    ModelChanged(object::ModelChangedEvent),
    ActiveDescendantChanged(object::ActiveDescendantChangedEvent),
    Announcement(object::AnnouncementEvent),
    AttributesChanged(object::AttributesChangedEvent),
    RowInserted(object::RowInsertedEvent),
    RowReordered(object::RowReorderedEvent),
    RowDeleted(object::RowDeletedEvent),
    ColumnInserted(object::ColumnInsertedEvent),
    ColumnReordered(object::ColumnReorderedEvent),
    ColumnDeleted(object::ColumnDeletedEvent),
    TextBoundsChanged(object::TextBoundsChangedEvent),
    TextSelectionChanged(object::TextSelectionChangedEvent),
    TextChanged(object::TextChangedEvent),
    TextAttributesChanged(object::TextAttributesChangedEvent),
    TextCaretMoved(object::TextCaretMovedEvent),
}

impl TryFrom<AtspiEvent> for ObjectEvents {
    type Error = AtspiError;

    fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
        let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
        match member.as_str() {
            "PropertyChange" => Ok(ObjectEvents::PropertyChange(object::PropertyChangeEvent(ev))),
            "BoundsChanged" => Ok(ObjectEvents::BoundsChanged(object::BoundsChangedEvent(ev))),
            "LinkSelected" => Ok(ObjectEvents::LinkSelected(object::LinkSelectedEvent(ev))),
            "StateChanged" => Ok(ObjectEvents::StateChanged(object::StateChangedEvent(ev))),
            "ChildrenChanged" => {
                Ok(ObjectEvents::ChildrenChanged(object::ChildrenChangedEvent(ev)))
            }
            "VisibleDataChanged" => {
                Ok(ObjectEvents::VisibleDataChanged(object::VisibleDataChangedEvent(ev)))
            }
            "SelectionChanged" => {
                Ok(ObjectEvents::SelectionChanged(object::SelectionChangedEvent(ev)))
            }
            "ModelChanged" => Ok(ObjectEvents::ModelChanged(object::ModelChangedEvent(ev))),
            "ActiveDescendantChanged" => {
                Ok(ObjectEvents::ActiveDescendantChanged(object::ActiveDescendantChangedEvent(ev)))
            }
            "Announcement" => Ok(ObjectEvents::Announcement(object::AnnouncementEvent(ev))),
            "AttributesChanged" => {
                Ok(ObjectEvents::AttributesChanged(object::AttributesChangedEvent(ev)))
            }
            "RowInserted" => Ok(ObjectEvents::RowInserted(object::RowInsertedEvent(ev))),
            "RowReordered" => Ok(ObjectEvents::RowReordered(object::RowReorderedEvent(ev))),
            "RowDeleted" => Ok(ObjectEvents::RowDeleted(object::RowDeletedEvent(ev))),
            "ColumnInserted" => Ok(ObjectEvents::ColumnInserted(object::ColumnInsertedEvent(ev))),
            "ColumnReordered" => {
                Ok(ObjectEvents::ColumnReordered(object::ColumnReorderedEvent(ev)))
            }
            "ColumnDeleted" => Ok(ObjectEvents::ColumnDeleted(object::ColumnDeletedEvent(ev))),
            "TextBoundsChanged" => {
                Ok(ObjectEvents::TextBoundsChanged(object::TextBoundsChangedEvent(ev)))
            }
            "TextSelectionChanged" => {
                Ok(ObjectEvents::TextSelectionChanged(object::TextSelectionChangedEvent(ev)))
            }
            "TextChanged" => Ok(ObjectEvents::TextChanged(object::TextChangedEvent(ev))),
            "TextAttributesChanged" => {
                Ok(ObjectEvents::TextAttributesChanged(object::TextAttributesChangedEvent(ev)))
            }
            "TextCaretMoved" => Ok(ObjectEvents::TextCaretMoved(object::TextCaretMovedEvent(ev))),
            _ => Err(AtspiError::MemberMatch("No matching member for Object".into())),
        }
    }
}

/// Any of the `Window` events.
///
/// Event table for the contained types:
///
/// |Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties|
/// |:--|---|---|---|---|---|---|
/// |Window|PropertyChange|property|    |    |    |properties|
/// |Window|Minimize|    |    |    |    |properties|
/// |Window|Maximize|    |    |    |    |properties|
/// |Window|Restore|    |    |    |    |properties|
/// |Window|Close|    |    |    |    |properties|
/// |Window|Create|    |    |    |    |properties|
/// |Window|Reparent|    |    |    |    |properties|
/// |Window|DesktopCreate|    |    |    |    |properties|
/// |Window|DesktopDestroy|    |    |    |    |properties|
/// |Window|Destroy|    |    |    |    |properties|
/// |Window|Activate|    |    |    |    |properties|
/// |Window|Deactivate|    |    |    |    |properties|
/// |Window|Raise|    |    |    |    |properties|
/// |Window|Lower|    |    |    |    |properties|
/// |Window|Move|    |    |    |    |properties|
/// |Window|Resize|    |    |    |    |properties|
/// |Window|Shade|    |    |    |    |properties|
/// |Window|uUshade|    |    |    |    |properties|
/// |Window|Restyle|    |    |    |    |properties|
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WindowEvents {
    PropertyChange(PropertyChangeEvent),
    Minimize(MinimizeEvent),
    Maximize(MaximizeEvent),
    Restore(RestoreEvent),
    Close(CloseEvent),
    Create(CreateEvent),
    Reparent(ReparentEvent),
    DesktopCreate(DesktopCreateEvent),
    DesktopDestroy(DesktopDestroyEvent),
    Destroy(DestroyEvent),
    Activate(ActivateEvent),
    Deactivate(DeactivateEvent),
    Raise(RaiseEvent),
    Lower(LowerEvent),
    Move(MoveEvent),
    Resize(ResizeEvent),
    Shade(ShadeEvent),
    UUshade(uUshadeEvent),
    Restyle(RestyleEvent),
}

impl TryFrom<AtspiEvent> for WindowEvents {
    type Error = AtspiError;

    fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
        let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
        match member.as_str() {
            "PropertyChange" => Ok(WindowEvents::PropertyChange(PropertyChangeEvent(ev))),
            "Minimize" => Ok(WindowEvents::Minimize(MinimizeEvent(ev))),
            "Maximize" => Ok(WindowEvents::Maximize(MaximizeEvent(ev))),
            "Restore" => Ok(WindowEvents::Restore(RestoreEvent(ev))),
            "Close" => Ok(WindowEvents::Close(CloseEvent(ev))),
            "Create" => Ok(WindowEvents::Create(CreateEvent(ev))),
            "Reparent" => Ok(WindowEvents::Reparent(ReparentEvent(ev))),
            "DesktopCreate" => Ok(WindowEvents::DesktopCreate(DesktopCreateEvent(ev))),
            "DesktopDestroy" => Ok(WindowEvents::DesktopDestroy(DesktopDestroyEvent(ev))),
            "Destroy" => Ok(WindowEvents::Destroy(DestroyEvent(ev))),
            "Activate" => Ok(WindowEvents::Activate(ActivateEvent(ev))),
            "Deactivate" => Ok(WindowEvents::Deactivate(DeactivateEvent(ev))),
            "Raise" => Ok(WindowEvents::Raise(RaiseEvent(ev))),
            "Lower" => Ok(WindowEvents::Lower(LowerEvent(ev))),
            "Move" => Ok(WindowEvents::Move(MoveEvent(ev))),
            "Resize" => Ok(WindowEvents::Resize(ResizeEvent(ev))),
            "Shade" => Ok(WindowEvents::Shade(ShadeEvent(ev))),
            "uUshade" => Ok(WindowEvents::UUshade(uUshadeEvent(ev))),
            "Restyle" => Ok(WindowEvents::Restyle(RestyleEvent(ev))),
            _ => Err(AtspiError::MemberMatch("No matching member for Window".into())),
        }
    }
}

/// Any of the `Mouse` events.
///
/// Those interested in `Event.Mouse` events, this enum
/// may help select and specify for those on a stream:
///
/// # Example
/// ```
/// // Boilerplate omitted.
/// use crate::identify::MouseEvent;
///
/// while let Ok(Some(ev)) = event_stream.next().await? {
///   let mse_ev: MouseEvent = ev.try_into()?;
/// }
///  ```
/// Event table for the contained types:
///
/// |Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties|
/// |:--|---|---|---|---|---|---|
/// |Mouse|Abs|    |x|y|    |properties|
/// |Mouse|Rel|    |x|y|    |properties|
/// |`Mouse|Button|detail|mouse_x|mouse_y`|    |properties|
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MouseEvents {
    Abs(AbsEvent),
    Rel(RelEvent),
    Button(ButtonEvent),
}

impl TryFrom<AtspiEvent> for MouseEvents {
    type Error = AtspiError;

    fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
        let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
        match member.as_str() {
            "Abs" => Ok(MouseEvents::Abs(AbsEvent(ev))),
            "Rel" => Ok(MouseEvents::Rel(RelEvent(ev))),
            "Button" => Ok(MouseEvents::Button(ButtonEvent(ev))),
            _ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
        }
    }
}

/// Any of the `Terminal` events.
///
/// If you are interested in `Event.Terminal` events, this enum
/// may, for instance, help you select for those on a stream:
///
/// # Example
/// ```
/// // Boilerplate omitted.
/// use crate::identify::TerminalEvent;
///
/// while let Ok(Some(ev)) = event_stream.next().await? {
///   let term_ev: TerminalEvent = ev.try_into()?;
/// }
///  ```
/// Event table for the contained types:
///
/// |Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties|
/// |:--|---|---|---|---|---|---|
/// |Terminal|LineChanged|    |    |    |    |properties|
/// |Terminal|ColumncountChanged|    |    |    |    |properties|
/// |Terminal|LinecountChanged|    |    |    |    |properties|
/// |Terminal|ApplicationChanged|    |    |    |    |properties|
/// |Terminal|CharwidthChanged|    |    |    |    |properties|
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TerminalEvents {
    LineChanged(LineChangedEvent),
    ColumncountChanged(ColumncountChangedEvent),
    LinecountChanged(LinecountChangedEvent),
    ApplicationChanged(ApplicationChangedEvent),
    CharwidthChanged(CharwidthChangedEvent),
}

impl TryFrom<AtspiEvent> for TerminalEvents {
    type Error = AtspiError;

    fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
        let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
        match member.as_str() {
            "LineChanged" => Ok(TerminalEvents::LineChanged(LineChangedEvent(ev))),
            "ColumncountChanged" => {
                Ok(TerminalEvents::ColumncountChanged(ColumncountChangedEvent(ev)))
            }
            "LinecountChanged" => Ok(TerminalEvents::LinecountChanged(LinecountChangedEvent(ev))),
            "ApplicationChanged" => {
                Ok(TerminalEvents::ApplicationChanged(ApplicationChangedEvent(ev)))
            }
            "CharwidthChanged" => Ok(TerminalEvents::CharwidthChanged(CharwidthChangedEvent(ev))),
            _ => Err(AtspiError::MemberMatch("No matching member for Terminal".into())),
        }
    }
}

/// The `Focus` event.
/// ## Deprecation notice!!
/// since: AT-SPI 2.9.4
/// This signal is deprecated and may be removed in the near future.
/// Monitor `StateChanged::Focused` signals instead.
///
/// Event table for the contained types:
///
/// |Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties|
/// |:--|---|---|---|---|---|---|
/// |Focus|Focus|    |    |    |    |properties|
#[derive(Debug, Clone)]
pub enum FocusEvents {
    Focus(FocusEvent),
}

impl TryFrom<AtspiEvent> for FocusEvents {
    type Error = AtspiError;

    fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
        let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
        match member.as_str() {
            "Focus" => Ok(FocusEvents::Focus(FocusEvent(ev))),
            _ => Err(AtspiError::MemberMatch("No matching member for Focus".into())),
        }
    }
}

/// The `Keyboard` events.
///
/// Contains the variant of the `Keyboard` event.
/// While this enum has only one item, it is defined nevertheless
/// to keep conversion requirements congruent over all types.
///
/// If you are interested in `Event.Keyboard` events, this enum
/// may, for instance, help you select for those on a stream:
///
/// # Example
/// ```
/// // Boilerplate omitted.
/// use crate::identify::KeyboardEvent;
///
/// while let Ok(Some(ev)) = event_stream.next().await? {
///   let kb_ev: KeyboardEvent = ev.try_into()?;
/// }
///  ```
/// Event table for the contained types:
///
/// Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties
/// |:--|---|---|---|---|---|---|
/// |Keyboard|Modifiers|    |`previous_modifiers`|`current_modifiers`|    |properties|
#[derive(Debug, Clone)]
pub enum KeyboardEvents {
    Modifiers(ModifiersEvent),
}

impl TryFrom<AtspiEvent> for KeyboardEvents {
    type Error = AtspiError;

    fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
        let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
        match member.as_str() {
            "Modifiers" => Ok(KeyboardEvents::Modifiers(ModifiersEvent(ev))),
            _ => Err(AtspiError::MemberMatch("No matching member for Keyboard".into())),
        }
    }
}

// ---------------> Object types:

pub mod object {
		use crate::{
			identify::Signified,
			events::AtspiEvent,
		};
    use atspi_macros::TrySignify;
    use zbus::zvariant::OwnedValue;

    #[derive(Debug, Eq, PartialEq, Clone, TrySignify)]
    pub struct PropertyChangeEvent(pub(crate) AtspiEvent);

    impl PropertyChangeEvent {
        #[must_use]
        pub fn property(&self) -> &str {
            self.0.kind()
        }
        #[must_use]
        pub fn value(&self) -> &OwnedValue {
            self.0.any_data()
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct BoundsChangedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct LinkSelectedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct StateChangedEvent(pub(crate) AtspiEvent);
    impl StateChangedEvent {
        #[must_use]
        pub fn state(&self) -> &str {
            self.0.kind()
        }
        //TODO checkme please!!
        #[must_use]
        pub fn enabled(&self) -> bool {
            self.0.detail1() == 0
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct ChildrenChangedEvent(pub(crate) AtspiEvent);
    impl ChildrenChangedEvent {
        #[must_use]
        pub fn operation(&self) -> &str {
            self.0.kind()
        }
        #[must_use]
        pub fn index_in_parent(&self) -> i32 {
            self.0.detail1()
        } // usizes ?
        #[must_use]
        pub fn child(&self) -> &OwnedValue {
            self.0.any_data()
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct VisibleDataChangedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct SelectionChangedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct ModelChangedEvent(pub(crate) AtspiEvent);

    // TODO Check my impl please.
    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct ActiveDescendantChangedEvent(pub(crate) AtspiEvent);
    impl ActiveDescendantChangedEvent {
        #[must_use]
        pub fn child(&self) -> &OwnedValue {
            self.0.any_data() // TODO Make me a beter returner!
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct AnnouncementEvent(pub(crate) AtspiEvent);
    impl AnnouncementEvent {
        #[must_use]
        pub fn text(&self) -> &str {
            self.0.kind()
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct AttributesChangedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct RowInsertedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct RowReorderedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct RowDeletedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct ColumnInsertedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct ColumnReorderedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct ColumnDeletedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct TextBoundsChangedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct TextSelectionChangedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct TextChangedEvent(pub(crate) AtspiEvent);
    impl TextChangedEvent {
        #[must_use]
        pub fn detail(&self) -> &str {
            self.0.kind()
        }
        #[must_use]
        pub fn start_pos(&self) -> i32 {
            self.0.detail1()
        }
        #[must_use]
        pub fn end_pos(&self) -> i32 {
            self.0.detail2()
        }
        // TODO zvariant::Value -> String me please
        #[must_use]
        pub fn text(&self) -> &OwnedValue {
            self.0.any_data()
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct TextAttributesChangedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
    pub struct TextCaretMovedEvent(pub(crate) AtspiEvent);

    impl TextCaretMovedEvent {
        #[must_use]
        pub fn position(&self) -> i32 {
            self.0.detail1()
        }
    }
}

// ------<- end of Obj signals
// ----------> Start of Win

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct PropertyChangeEvent(pub(crate) AtspiEvent);
impl PropertyChangeEvent {
    #[must_use]
    pub fn property(&self) -> &str {
        self.0.kind()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct MinimizeEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct MaximizeEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct RestoreEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct CloseEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct CreateEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ReparentEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct DesktopCreateEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct DesktopDestroyEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct DestroyEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ActivateEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct DeactivateEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct RaiseEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct LowerEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct MoveEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ResizeEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ShadeEvent(pub(crate) AtspiEvent);

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct uUshadeEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct RestyleEvent(pub(crate) AtspiEvent);

// ----------<- end of Win signals
// ----------> Start of Mse

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct AbsEvent(pub(crate) AtspiEvent);
impl AbsEvent {
    /// X-coordinate of mouse button event
    ///  Coordinates are absolute, with the origin in the top-left of the 'root window'
    #[must_use]
    pub fn x(&self) -> i32 {
        self.inner().detail1()
    }
    /// Y-coordinate of mouse button event
    /// Coordinates are absolute, with the origin in the top-left of the 'root window'
    #[must_use]
    pub fn y(&self) -> i32 {
        self.inner().detail2()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct RelEvent(pub(crate) AtspiEvent);
impl RelEvent {
    #[must_use]
    pub fn dx(&self) -> i32 {
        self.inner().detail1()
    }
    #[must_use]
    pub fn dy(&self) -> i32 {
        self.inner().detail2()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ButtonEvent(pub(crate) AtspiEvent);
impl ButtonEvent {
    /// Button being used 1..X
    /// The suffix may either be 'p', for pressed, or 'r' for rekeased.
    #[must_use]
    pub fn button(&self) -> &str {
        self.inner().kind()
    }
    /// X-coordinate of mouse button event
    #[must_use]
    pub fn x(&self) -> i32 {
        self.inner().detail1()
    }
    /// Y-coordinate of mouse button event
    #[must_use]
    pub fn y(&self) -> i32 {
        self.inner().detail2()
    }
}

// ----------<- end of Mse signals
// ----------> Start of Kbd

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ModifiersEvent(pub(crate) AtspiEvent);
impl ModifiersEvent {
    #[must_use]
    pub fn previous_modifiers(&self) -> i32 {
        self.inner().detail1()
    }
    #[must_use]
    pub fn current_modifiers(&self) -> i32 {
        self.inner().detail2()
    }
}

// ----------<- end of Kbd signals
// ----------> Start of Term

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct LineChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ColumncountChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct LinecountChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ApplicationChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct CharwidthChangedEvent(pub(crate) AtspiEvent);

// -------<- end of Term signals
// ----------> Start of Doc

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct LoadCompleteEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ReloadEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct LoadStoppedEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ContentChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct AttributesChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct PageChangedEvent(pub(crate) AtspiEvent);

// ---------- End of Doc
// ----------> Start of Focus

// #[deprecated(note = "Users are advised to monitor Object:StateChanged:focused instead.")]
#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct FocusEvent(pub(crate) AtspiEvent);
