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
use zbus::{names::MemberName, zvariant};
use zvariant::OwnedValue;

/// All Atspi / Qspi event types encapsulate `AtspiEvent`.
/// This trait allows access to the underlying item.
pub trait Signified {
    type Inner;

    fn inner(&self) -> &AtspiEvent;
    fn properties(&self) -> &HashMap<String, OwnedValue>;
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
/// TODO Catch signals and repair table below please!
///
/// Event table for the contained types:
///
/// Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties
/// |---|---|---|---|---|---|---
/// Document|LoadComplete|      |       |       |       |properties
/// Document|Reload|    |       |       |       |properties
/// Document|LoadStopped|       |       |       |       |properties
/// Document|ContentChanged|    |       |       |       |properties
/// Document|AttributesChanged| |       |       |       |properties
/// Document|PageChanged|       |       |       |       |properties
#[derive(Debug, Clone)]
pub enum DocumentEvents {
    LoadComplete(LoadCompleteEvent),
    Reload(ReloadEvent),
    LoadStopped(LoadStoppedEvent),
    ContentChanged(ContentChangedEvent),
    AttributesChanged(AttributesChangedEvent),
    PageChanged(PageChangedEvent),
}

impl From<AtspiEvent> for DocumentEvents {
    fn from(ev: AtspiEvent) -> Self {
        let member = ev.member().expect("signal w/o member");
        match member.as_str() {
            "LoadComplete" => Self::LoadComplete(LoadCompleteEvent(ev)),
            "Reload" => Self::Reload(ReloadEvent(ev)),
            "LoadStopped" => Self::LoadStopped(LoadStoppedEvent(ev)),
            "ContentChanged" => Self::ContentChanged(ContentChangedEvent(ev)),
            "AttributesChanged" => Self::AttributesChanged(AttributesChangedEvent(ev)),
            "PageChanged" => Self::PageChanged(PageChangedEvent(ev)),
            _ => panic!("Not a document event"),
        }
    }
}

/// Any of the `Object` events.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ObjectEvents {
    PropertyChange(PropertyChangeEvent),
    BoundsChanged(BoundsChangedEvent),
    LinkSelected(LinkSelectedEvent),
    StateChanged(StateChangedEvent),
    ChildrenChanged(ChildrenChangedEvent),
    VisibleDataChanged(VisibleDataChangedEvent),
    SelectionChanged(SelectionChangedEvent),
    ModelChanged(ModelChangedEvent),
    ActiveDescendantChanged(ActiveDescendantChangedEvent),
    Announcement(AnnouncementEvent),
    AttributesChanged(ObjectAttributesChangedEvent),
    RowInserted(RowInsertedEvent),
    RowReordered(RowReorderedEvent),
    RowDeleted(RowDeletedEvent),
    ColumnInserted(ColumnInsertedEvent),
    ColumnReordered(ColumnReorderedEvent),
    ColumnDeleted(ColumnDeletedEvent),
    TextBoundsChanged(TextBoundsChangedEvent),
    TextSelectionChanged(TextSelectionChangedEvent),
    TextChanged(TextChangedEvent),
    TextAttributesChanged(TextAttributesChangedEvent),
    TextCaretMoved(TextCaretMovedEvent),
}

impl From<AtspiEvent> for ObjectEvents {
    fn from(ev: AtspiEvent) -> Self {
        let member = ev.member().expect("signal w/o member");
        match member.as_str() {
            "PropertyChange" => Self::PropertyChange(PropertyChangeEvent(ev)),
            "BoundsChanged" => Self::BoundsChanged(BoundsChangedEvent(ev)),
            "LinkSelected" => Self::LinkSelected(LinkSelectedEvent(ev)),
            "StateChanged" => Self::StateChanged(StateChangedEvent(ev)),
            "ChildrenChanged" => Self::ChildrenChanged(ChildrenChangedEvent(ev)),
            "VisibleDataChanged" => Self::VisibleDataChanged(VisibleDataChangedEvent(ev)),
            "SelectionChanged" => Self::SelectionChanged(SelectionChangedEvent(ev)),
            "ModelChanged" => Self::ModelChanged(ModelChangedEvent(ev)),
            "ActiveDescendantChanged" => {
                Self::ActiveDescendantChanged(ActiveDescendantChangedEvent(ev))
            }
            "Announcement" => Self::Announcement(AnnouncementEvent(ev)),
            "AttributesChanged" => Self::AttributesChanged(ObjectAttributesChangedEvent(ev)),
            "RowInserted" => Self::RowInserted(RowInsertedEvent(ev)),
            "RowReordered" => Self::RowReordered(RowReorderedEvent(ev)),
            "RowDeleted" => Self::RowDeleted(RowDeletedEvent(ev)),
            "ColumnInserted" => Self::ColumnInserted(ColumnInsertedEvent(ev)),
            "ColumnReordered" => Self::ColumnReordered(ColumnReorderedEvent(ev)),
            "ColumnDeleted" => Self::ColumnDeleted(ColumnDeletedEvent(ev)),
            "TextBoundsChanged" => Self::TextBoundsChanged(TextBoundsChangedEvent(ev)),
            "TextSelectionChanged" => Self::TextSelectionChanged(TextSelectionChangedEvent(ev)),
            "TextChanged" => Self::TextChanged(TextChangedEvent(ev)),
            "TextAttributesChanged" => Self::TextAttributesChanged(TextAttributesChangedEvent(ev)),
            "TextCaretMoved" => Self::TextCaretMoved(TextCaretMovedEvent(ev)),
            _ => panic!("Not an object event"),
        }
    }
}

/// Any of the `Window` events.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WindowEvents {
    PropertyChange(WindowPropertyChangeEvent),
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

impl From<AtspiEvent> for WindowEvents {
    fn from(ev: AtspiEvent) -> Self {
        let member = ev.member().expect("signal w/o member");
        match member.as_str() {
            "PropertyChange" => Self::PropertyChange(WindowPropertyChangeEvent(ev)),
            "Minimize" => Self::Minimize(MinimizeEvent(ev)),
            "Maximize" => Self::Maximize(MaximizeEvent(ev)),
            "Restore" => Self::Restore(RestoreEvent(ev)),
            "Close" => Self::Close(CloseEvent(ev)),
            "Create" => Self::Create(CreateEvent(ev)),
            "Reparent" => Self::Reparent(ReparentEvent(ev)),
            "DesktopCreate" => Self::DesktopCreate(DesktopCreateEvent(ev)),
            "DesktopDestroy" => Self::DesktopDestroy(DesktopDestroyEvent(ev)),
            "Destroy" => Self::Destroy(DestroyEvent(ev)),
            "Activate" => Self::Activate(ActivateEvent(ev)),
            "Deactivate" => Self::Deactivate(DeactivateEvent(ev)),
            "Raise" => Self::Raise(RaiseEvent(ev)),
            "Lower" => Self::Lower(LowerEvent(ev)),
            "Move" => Self::Move(MoveEvent(ev)),
            "Resize" => Self::Resize(ResizeEvent(ev)),
            "Shade" => Self::Shade(ShadeEvent(ev)),
            "uUshade" => Self::UUshade(uUshadeEvent(ev)),
            "Restyle" => Self::Restyle(RestyleEvent(ev)),
            _ => panic!("Not a window event"),
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
///
/// | Interface  | Member  |  kind | Detail1   | Detail2  | Any_data |
/// |:-:|---|---|---|---|---|
/// | Mouse |`Abs` |   | x | y |   |
/// | Mouse |`Rel` |   | `delta_x` | `delta_y`  |   |
/// | Mouse | `Button` | p\[1..=5\] | x  | y  |   |
/// | Mouse | `Button` | r\[1..=5\] | x  | y  |   |
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MouseEvents {
    Abs(AbsEvent),
    Rel(RelEvent),
    Button(ButtonEvent),
}

impl From<AtspiEvent> for MouseEvents {
    fn from(ev: AtspiEvent) -> Self {
        let member = ev.member().expect("signal w/o member");
        match member.as_str() {
            "Abs" => Self::Abs(AbsEvent(ev)),
            "Rel" => Self::Rel(RelEvent(ev)),
            "Button" => Self::Button(ButtonEvent(ev)),
            _ => panic!("Not a mouse event"),
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
/// | Interface  | Member  |  Detail1   | Detail2  | Any_data |
/// |:-:|---|---|---|---|
/// | Terminal | `LineChanged`  |   |   |   |
/// | Terminal | `ColumncountChanged`  |   |   |   |
/// | Terminal |`LinecountChanged`|   |   |   |
/// | Terminal | `CharwidthChanged`  |   |   |   |
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TerminalEvents {
    LineChanged(LineChangedEvent),
    ColumncountChanged(ColumncountChangedEvent),
    LinecountChanged(LinecountChangedEvent),
    ApplicationChanged(ApplicationChangedEvent),
    CharwidthChanged(CharwidthChangedEvent),
}

impl From<AtspiEvent> for TerminalEvents {
    fn from(ev: AtspiEvent) -> Self {
        let member = ev.member().expect("signal w/o member");
        match member.as_str() {
            "LineChanged" => Self::LineChanged(LineChangedEvent(ev)),
            "ColumncountChanged" => Self::ColumncountChanged(ColumncountChangedEvent(ev)),
            "LinecountChanged" => Self::LinecountChanged(LinecountChangedEvent(ev)),
            "ApplicationChanged" => Self::ApplicationChanged(ApplicationChangedEvent(ev)),
            "CharwidthChanged" => Self::CharwidthChanged(CharwidthChangedEvent(ev)),
            _ => panic!("Not a terminal event"),
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
/// Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties
/// ---|---|---|---|---|---|---
/// Focus|Focus|        |       |       |       |properties
#[derive(Debug, Clone)]
pub enum FocusEvents {
    Focus(FocusEvent),
}

impl From<AtspiEvent> for FocusEvents {
    fn from(ev: AtspiEvent) -> Self {
        let member = ev.member().expect("signal w/o member");
        match member.as_str() {
            "Focus" => Self::Focus(FocusEvent(ev)),
            _ => panic!("Not a focus event"),
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
/// | Interface  | Member  | Kind |  Detail1   | Detail2  | Any_data |
/// |:-:|---|---|---|---|---|
/// | Keyboard | Modifiers |   |   |   |   |
#[derive(Debug, Clone)]
pub enum KeyboardEvents {
    Modifiers(ModifiersEvent),
}

impl From<AtspiEvent> for KeyboardEvents {
    fn from(ev: AtspiEvent) -> Self {
        let member = ev.member().expect("signal w/o member");
        match member.as_str() {
            "Modifiers" => Self::Modifiers(ModifiersEvent(ev)),
            _ => panic!("Not a keyboad event"),
        }
    }
}

// ---------------> Object types:

#[derive(Debug, Eq, PartialEq, Clone, TrySignify)]
pub struct PropertyChangeEvent(AtspiEvent);

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
pub struct BoundsChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct LinkSelectedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Clone, TrySignify)]
pub struct StateChangedEvent(AtspiEvent);
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

impl Eq for StateChangedEvent {}

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ChildrenChangedEvent(AtspiEvent);
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
pub struct VisibleDataChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct SelectionChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ModelChangedEvent(AtspiEvent);

// TODO Check my impl please.
#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ActiveDescendantChangedEvent(AtspiEvent);
impl ActiveDescendantChangedEvent {
    #[must_use]
    pub fn child(&self) -> &zvariant::OwnedValue {
        self.0.any_data() // TODO Make me a beter returner!
    }
}

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct AnnouncementEvent(AtspiEvent);
impl AnnouncementEvent {
    #[must_use]
    pub fn text(&self) -> &str {
        self.0.kind()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectAttributesChangedEvent(AtspiEvent);

// Gets manual impl, because its name is changed,
// the proc-macro does not handle changed names that well
impl TryFrom<AtspiEvent> for ObjectAttributesChangedEvent {
    type Error = crate::AtspiError;

    fn try_from(msg: AtspiEvent) -> Result<Self, Self::Error> {
        let msg_member = msg.message.member();
        if msg_member == Some(MemberName::from_static_str("AttributesChanged")?) {
            return Ok(Self(msg));
        };

        let error =
            format!("specific type's member: AttributesChanged != msg type member: {msg_member:?}");
        Err(crate::AtspiError::MemberMatch(error))
    }
}

impl Eq for ObjectAttributesChangedEvent {}

impl Signified for ObjectAttributesChangedEvent {
    type Inner = AtspiEvent;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn properties(&self) -> &HashMap<String, OwnedValue> {
        self.0.properties()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct RowInsertedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct RowReorderedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct RowDeletedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ColumnInsertedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ColumnReorderedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ColumnDeletedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct TextBoundsChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct TextSelectionChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct TextChangedEvent(AtspiEvent);
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
pub struct TextAttributesChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct TextCaretMovedEvent(AtspiEvent);

impl TextCaretMovedEvent {
    #[must_use]
    pub fn position(&self) -> i32 {
        self.0.detail1()
    }
}

// ------<- end of Obj signals
// ----------> Start of Win

//TODO Check my impl with bus signal
#[derive(Debug, PartialEq, Clone)]
pub struct WindowPropertyChangeEvent(AtspiEvent);
impl WindowPropertyChangeEvent {
    #[must_use]
    pub fn property(&self) -> &str {
        self.0.kind()
    }
}

impl Eq for WindowPropertyChangeEvent {}

// Gets manual impl, because its name is changed,
// the proc-macro does not handle changed names that well
impl TryFrom<AtspiEvent> for WindowPropertyChangeEvent {
    type Error = crate::AtspiError;

    fn try_from(msg: AtspiEvent) -> Result<Self, Self::Error> {
        let msg_member = msg.message.member();
        if msg_member == Some(MemberName::from_static_str("PropertyChange")?) {
            return Ok(Self(msg));
        };

        let error =
            format!("specific type's member: PropertyChange != msg type member: {msg_member:?}");
        Err(crate::AtspiError::MemberMatch(error))
    }
}

impl Signified for WindowPropertyChangeEvent {
    type Inner = AtspiEvent;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn properties(&self) -> &HashMap<String, OwnedValue> {
        self.inner().properties()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct MinimizeEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct MaximizeEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct RestoreEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct CloseEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct CreateEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ReparentEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct DesktopCreateEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct DesktopDestroyEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct DestroyEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ActivateEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct DeactivateEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct RaiseEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct LowerEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct MoveEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ResizeEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ShadeEvent(AtspiEvent);

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct uUshadeEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct RestyleEvent(AtspiEvent);

// ----------<- end of Win signals
// ----------> Start of Mse

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct AbsEvent(AtspiEvent);
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
pub struct RelEvent(AtspiEvent);
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
pub struct ButtonEvent(AtspiEvent);
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
pub struct ModifiersEvent(AtspiEvent);
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
pub struct LineChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ColumncountChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct LinecountChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ApplicationChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct CharwidthChangedEvent(AtspiEvent);

// -------<- end of Term signals
// ----------> Start of Doc

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct LoadCompleteEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ReloadEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct LoadStoppedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct ContentChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct AttributesChangedEvent(AtspiEvent);

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct PageChangedEvent(AtspiEvent);

// ---------- End of Doc
// ----------> Start of Focus

#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
pub struct FocusEvent(AtspiEvent);
