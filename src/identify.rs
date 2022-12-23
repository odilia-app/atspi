//! ## Signified signal types
//!
//! The generic `AtspiEvent` has a specific meaning depending on its origin.
//! This module offers the signified types and their conversions from a generic `AtpiEvent`.
//!
//! The `TrySignify` macro implements a `TryFrom<Event>` on a per-name and member basis
//!

use atspi_macros::{Doc, Focus, Kbd, Mse, Obj, Term, TrySignify, Win};
use std::collections::HashMap;
use std::sync::Arc;
use zbus::names::InterfaceName;
use zbus::names::MemberName;
use zbus::zvariant;
use zbus::zvariant::OwnedObjectPath;
use zbus::Message;
use zvariant::OwnedValue;

use crate::events::AtspiEvent;

/// Exposes shared functionality over all Atspi / Qspi Signal events
/// on the non-generic / signified types.
pub trait Signified {
    type Inner;

    fn inner(&self) -> &AtspiEvent;
    fn properties(&self) -> &HashMap<String, OwnedValue>;
}

use crate::events::EventBodyOwned;
/// Shared functionality of Events, through its `Message` header
use crate::events::GenericEvent;

impl<T> GenericEvent for T
where
    T: Signified + ?Sized,
{
    /// Serialized bus message.
    #[must_use]
    fn message(&self) -> &Arc<Message> {
        &Signified::inner(self).message
    }

    /// For now this returns the full interface name because the lifetimes in [`zbus_names`][zbus::names] are
    /// wrong such that the `&str` you can get from a
    /// [`zbus_names::InterfaceName`][zbus::names::InterfaceName] is tied to the lifetime of that
    /// name, not to the lifetime of the message as it should be. In future, this will return only
    /// the last component of the interface name (I.E. "Object" from
    /// "org.a11y.atspi.Event.Object").
    #[must_use]
    fn interface(&self) -> Option<InterfaceName<'_>> {
        Signified::inner(self).message.interface()
    }

    /// Identifies this `Event`'s interface member name on the bus.
    /// Members of the interface are either signals, methods or properties.
    /// eg. `PropertyChanged` or `TextChanged`
    #[must_use]
    fn member(&self) -> Option<MemberName<'_>> {
        Signified::inner(self).message.member()
    }

    /// The object path to the object where the signal is emitted from.
    #[must_use]
    fn path(&self) -> std::option::Option<zbus::zvariant::OwnedObjectPath> {
        let ev = Signified::inner(self);
        Some(OwnedObjectPath::from(ev.message.path().unwrap()))
    }

    /// Identifies the `sender` of the `Event`.
    /// # Errors
    /// - when deserializeing the header failed, or
    /// - When `zbus::get_field!` finds that 'sender' is an invalid field.
    fn sender(&self) -> Result<Option<zbus::names::UniqueName>, crate::AtspiError> {
        Ok(Signified::inner(self).message.header()?.sender()?.cloned())
    }
}

/// Trait to allow grouping of `Document` signals
/// May contain any variant pertaining `Document` events.
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
/// | Interface  | Member  |  Kind  | Detail1   | Detail2  | Any_data |
/// |:-:|---|---|---|---|---|
/// | Document | LoadComplete  |   |   |   |
/// | Document | Reload |   |   |   |
/// | Document | LoadStopped |   |   |   |
/// | Document | ContentChanged  |   |   |   |
/// | Document | AttributesChanged  |   |   |   |
/// | Document | PageChanged  |   |   |   |
pub trait Doc {}

/// Trait to allow grouping of `Object` signals
pub trait Obj {}

/// Trait to allow grouping of `Win` signals
pub trait Win {}

/// Trait to allow grouping of `Mse` signals
/// Contains any variant pertaining `MouseEvent` events.
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
/// | Mouse |Abs |   | x | y |   |
/// | Mouse | Rel |   | delta_x  | delta_y   |   |
/// | Mouse | Button | p\[1..=5\] | x  | y  |   |
/// | Mouse | Button | r\[1..=5\] | x  | y  |   |
pub trait Mse {}

/// Trait to allow grouping of `Term` signals
/// May contain any variant pertaining `Terminal` events.
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
/// | Terminal | LineChanged  |   |   |   |
/// | Terminal | ColumncountChanged  |   |   |   |
/// | Terminal |LinecountChanged|   |   |   |
/// | Terminal | CharwidthChanged  |   |   |   |
pub trait Term {}

/// Trait to allow grouping of `Focus` signals
/// ## Deprecation notice!!
/// since: AT-SPI 2.9.4
/// This signal is deprecated and may be removed in the near future.
/// Monitor `StateChanged::Focused` signals instead.
///
/// TODO: Catch an event, check table below.
///
/// | Interface  | Member  |  Kind | Detail1   | Detail2  | Any_data |
/// |:-:|---|---|---|---|---|
/// | Focus  | Focus |   |   |   |    |
pub trait Focus {}

/// Trait to allow grouping of `Kbd` signals
/// Contain any variant pertaining `Keyboard` events.
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
/// | Keyboard | Modifiers>  |   |  |   |   |
pub trait Kbd {}

#[derive(Debug, Clone, TrySignify, Obj)]
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

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct BoundsChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct LinkSelectedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
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

#[derive(Debug, Clone, TrySignify, Obj)]
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

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct VisibleDataChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct SelectionChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ModelChangedEvent(AtspiEvent);

// TODO Check my impl please.
#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ActiveDescendantChangedEvent(AtspiEvent);
impl ActiveDescendantChangedEvent {
    pub fn child(&self) -> &zvariant::OwnedValue {
        self.0.any_data() // TODO Make me a beter returner!
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct AnnouncementEvent(AtspiEvent);
impl AnnouncementEvent {
    #[must_use]
    pub fn text(&self) -> &str {
        self.0.kind()
    }
}

#[derive(Debug, Clone, Obj)]
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

// impl<'a> ObjectAttributesChangedEvent {
//     fn inner(&'a self) -> &'a AtspiEvent {
//         &self.0
//     }
// }

impl Signified for ObjectAttributesChangedEvent {
    type Inner = AtspiEvent;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn properties(&self) -> &HashMap<String, OwnedValue> {
        self.inner().properties()
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct RowInsertedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct RowReorderedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct RowDeletedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ColumnInsertedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ColumnReorderedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ColumnDeletedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextBoundsChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextSelectionChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextChangedEvent(AtspiEvent);
impl TextChangedEvent {
    pub fn detail(&self) -> &str {
        self.0.kind()
    }
    pub fn start_pos(&self) -> i32 {
        self.0.detail1()
    }
    pub fn end_pos(&self) -> i32 {
        self.0.detail2()
    }
    // TODO zvariant::Value -> String me please
    pub fn text(&self) -> &OwnedValue {
        self.0.any_data()
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextAttributesChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextCaretMovedEvent(AtspiEvent);

impl TextCaretMovedEvent {
    pub fn position(&self) -> i32 {
        self.0.detail1()
    }
}

// ----------<- end of Obj signals
// ----------> Start of Win

//TODO Check my impl with bus signal
#[derive(Debug, Win)]
pub struct WindowPropertyChangeEvent(AtspiEvent);
impl WindowPropertyChangeEvent {
    #[must_use]
    pub fn property(&self) -> &str {
        self.0.kind()
    }
}

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

impl<'a> WindowPropertyChangeEvent {
    fn inner(&'a self) -> &'a AtspiEvent {
        &self.0
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

#[derive(Debug, TrySignify, Win)]
pub struct MinimizeEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct MaximizeEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct RestoreEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct CloseEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct CreateEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct ReparentEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct DesktopCreateEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct DesktopDestroyEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct DestroyEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct ActivateEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct DeactivateEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct RaiseEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct LowerEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct MoveEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct ResizeEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct ShadeEvent(AtspiEvent);

#[allow(non_camel_case_types)]
#[derive(Debug, TrySignify, Win)]
pub struct uUshadeEvent(AtspiEvent);

#[derive(Debug, TrySignify, Win)]
pub struct RestyleEvent(AtspiEvent);

// ----------<- end of Win signals
// ----------> Start of Mse

#[derive(Debug, Clone, TrySignify, Mse)]
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

#[derive(Debug, Clone, TrySignify, Mse)]
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

#[derive(Debug, Clone, TrySignify, Mse)]
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

#[derive(Debug, Clone, TrySignify, Kbd)]
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

#[derive(Debug, Clone, TrySignify, Term)]
pub struct LineChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct ColumncountChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct LinecountChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct ApplicationChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct CharwidthChangedEvent(AtspiEvent);

// ----------<- end of Term signals
// ----------> Start of Doc

#[derive(Debug, Clone, TrySignify, Doc)]
pub struct LoadCompleteEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Doc)]
pub struct ReloadEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Doc)]
pub struct LoadStoppedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Doc)]
pub struct ContentChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Doc)]
pub struct AttributesChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Doc)]
pub struct PageChangedEvent(AtspiEvent);

#[derive(Debug, Clone, TrySignify, Focus)]
pub struct FocusEvent(AtspiEvent);
