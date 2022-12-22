//! ## Signified signal types
//!
//! The generic `AtspiEvent` has a specific meaning depending on its origin.
//! This module offers the signified types and their conversions from a generic `AtpiEvent`.
//!
//! The `TrySignify` macro implements a `TryFrom<Event>` on a per-name and member basis
//!

use atspi_macros::{Doc, Focus, Kbd, Mse, Obj, Term, TrySignify, Win};
use std::collections::HashMap;
use std::ops::Deref;
use zbus::names::MemberName;
use zbus::zvariant;
use zvariant::OwnedValue;

pub trait Signified {}

/// Shared functionality
pub trait GenericEvent {
    fn properties(&self) -> &HashMap<String, OwnedValue>;
}

impl<T> GenericEvent for T
where
    T: Signified,
{
    fn properties(&self) -> &HashMap<String, OwnedValue> {
        (*self).properties()
    }
}

// CONSIDERED BAD PRACTICE!
// All types T : AtspiEvent should deref to [`crate::Event`]
//  This ensures all methods on Event are available for all.
impl Deref for dyn GenericEvent {
    type Target = crate::events::AtspiEvent;

    fn deref(&self) -> &Self::Target {
        &**self
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

#[derive(Debug, Obj)]
pub struct PropertyChangeEvent(AtspiEvent);

impl TryFrom<AtspiEvent> for PropertyChangeEvent {
    type Error = crate::AtspiError;

    fn try_from(msg: AtspiEvent) -> Result<Self, Self::Error> {
        let msg_member = msg.member();
        if msg_member == Some(MemberName::from_static_str(r#"PropertyChange"#)?) {
            return Ok(Self(msg));
        };

        let tname = std::any::type_name::<Self>().to_string();
        let member = tname.strip_suffix("Event").unwrap();
        let error = format!("specific type's member: {member} != msg type member: {msg_member:?}");
        Err(crate::AtspiError::MemberMatchError(error))
    }
}

impl<'a> PropertyChangeEvent {
    fn inner(&'a self) -> &'a AtspiEvent {
        &self.0
    }
}

impl PropertyChangeEvent {
    pub fn property(&self) -> &str {
        self.0.kind()
    }
    pub fn value(&self) -> &OwnedValue {
        self.0.any_data()
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct BoundsChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct LinkSelectedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct StateChangedEvent(pub(crate) AtspiEvent);
impl StateChangedEvent {
    pub fn state(&self) -> &str {
        self.0.kind()
    }
    //TODO checkme please!!
    pub fn enabled(&self) -> bool {
        self.0.detail1() == 0
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ChildrenChangedEvent(pub(crate) AtspiEvent);
impl ChildrenChangedEvent {
    pub fn operation(&self) -> &str {
        self.0.kind()
    }
    pub fn index_in_parent(&self) -> i32 {
        self.0.detail1()
    } // usizes ?
    pub fn child(&self) -> &OwnedValue {
        self.0.any_data()
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct VisibleDataChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct SelectionChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ModelChangedEvent(pub(crate) AtspiEvent);

// TODO Check my impl please.
#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ActiveDescendantChangedEvent(pub(crate) AtspiEvent);
impl ActiveDescendantChangedEvent {
    pub fn child(&self) -> &zvariant::OwnedValue {
        self.0.any_data() // TODO Make me a beter returner!
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct AnnouncementEvent(pub(crate) AtspiEvent);
impl AnnouncementEvent {
    pub fn text(&self) -> &str {
        self.0.kind()
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct AttributesChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct RowInsertedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct RowReorderedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct RowDeletedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ColumnInsertedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ColumnReorderedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ColumnDeletedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextBoundsChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextSelectionChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextChangedEvent(pub(crate) AtspiEvent);
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
pub struct TextAttributesChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextCaretMovedEvent(pub(crate) AtspiEvent);

impl TextCaretMovedEvent {
    pub fn position(&self) -> i32 {
        self.0.detail1()
    }
}

// ----------<- end of Obj signals
// ----------> Start of Win

mod win {
    use super::Win;
    use crate::events::AtspiEvent;
    use crate::identify::MemberName;
    use crate::identify::Signified;
    use atspi_macros::TrySignify;

    //TODO Check my impl with bus signal
    #[derive(Debug, TrySignify, Win)]
    pub struct PropertyChangeEvent(pub(crate) AtspiEvent);
    impl PropertyChangeEvent {
        pub fn property(&self) -> &str {
            self.0.kind()
        }
    }

    #[derive(Debug, TrySignify, Win)]
    pub struct MinimizeEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct MaximizeEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct RestoreEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct CloseEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct CreateEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct ReparentEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct DesktopCreateEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct DesktopDestroyEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct DestroyEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct ActivateEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct DeactivateEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct RaiseEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct LowerEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct MoveEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct ResizeEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct ShadeEvent(pub(crate) AtspiEvent);

    #[allow(non_camel_case_types)]
    #[derive(Debug, TrySignify, Win)]
    pub struct uUshadeEvent(pub(crate) AtspiEvent);

    #[derive(Debug, TrySignify, Win)]
    pub struct RestyleEvent(pub(crate) AtspiEvent);
}

// ----------<- end of Win signals
// ----------> Start of Mse

#[derive(Debug, Clone, TrySignify, Mse)]
pub struct AbsEvent(pub(crate) AtspiEvent);
impl AbsEvent {
    pub fn x(&self) -> i32 {
        self.0.body().detail1
    }
    pub fn y(&self) -> i32 {
        self.0.body().detail2
    }
}

#[derive(Debug, Clone, TrySignify, Mse)]
pub struct RelEvent(pub(crate) AtspiEvent);
impl RelEvent {
    pub fn dx(&self) -> i32 {
        self.0.body().detail1
    }
    pub fn dy(&self) -> i32 {
        self.0.body().detail2
    }
}

#[derive(Debug, Clone, TrySignify, Mse)]
pub struct ButtonEvent(pub(crate) AtspiEvent);
impl ButtonEvent {
    pub fn button(&self) -> &str {
        self.0.kind()
    }
    pub fn x(&self) -> i32 {
        self.0.body().detail1
    }
    pub fn y(&self) -> i32 {
        self.0.body().detail2
    }
}

// ----------<- end of Mse signals
// ----------> Start of Kbd

#[derive(Debug, Clone, TrySignify, Kbd)]
pub struct ModifiersEvent(pub(crate) AtspiEvent);
impl ModifiersEvent {
    pub fn previous_modifiers(&self) -> i32 {
        self.0.body().detail1
    }
    pub fn current_modifiers(&self) -> i32 {
        self.0.body().detail2
    }
}

// ----------<- end of Kbd signals
// ----------> Start of Term

#[derive(Debug, Clone, TrySignify, Term)]
pub struct LineChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct ColumncountChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct LinecountChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct ApplicationChangedEvent(pub(crate) AtspiEvent);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct CharwidthChangedEvent(pub(crate) AtspiEvent);

// ----------<- end of Term signals
// ----------> Start of Doc

mod doc {
    use super::Doc;
    use crate::events::AtspiEvent;
    use crate::identify::MemberName;
    use crate::identify::Signified;
    use atspi_macros::TrySignify;

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct LoadCompleteEvent(pub(crate) AtspiEvent);

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct ReloadEvent(pub(crate) AtspiEvent);

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct LoadStoppedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct ContentChangedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct AttributesChangedEvent(pub(crate) AtspiEvent);

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct PageChangedEvent(pub(crate) AtspiEvent);
}

use crate::events::AtspiEvent;

#[derive(Debug, Clone, TrySignify, Focus)]
pub struct FocusEvent(pub(crate) AtspiEvent);
