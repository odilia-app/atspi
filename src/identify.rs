//! ## Signified signal types
//!
//! The generic `Event` has a specific meaning depending on its origin.
//! This module offers the signified signal types and their conversions from a generic `Event`.
//!
//! The `TrySignify` macro implements a `TryFrom<Event>` on a per-name and member basis
//!

use crate::{events::Event, State};
use atspi_macros::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use zbus::names::MemberName;
use zbus::zvariant;
use zvariant::OwnedValue;

trait Signified {}

/// Shared functionality
trait GenericEvent {
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

// All types T : AtspiEvent should deref to [`crate::Event`]
//  This ensures all methods on Event are available for all.
impl Deref for dyn GenericEvent {
    type Target = crate::events::Event;

    fn deref(&self) -> &Self::Target {
        &*(self)
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

#[derive(Debug, TrySignify, Obj)]
pub struct PropertyChangeEvent(Event);

impl PropertyChangeEvent {
    fn property(&self) -> &str {
        self.0.kind()
    }
    fn value(&self) -> &OwnedValue {
        self.0.any_data()
    }
}

#[derive(Debug, Clone, Obj)]
pub struct BoundsChangedEvent(Event);
impl TryFrom<Event> for BoundsChangedEvent {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        if value.member() == Some(MemberName::from_static_str("BoundsChanged")?) {
            Ok(Self(value))
        } else {
            Err("error signifying event signal type".into())
        }
    }
}

impl Signified for BoundsChangedEvent {}

#[derive(Debug, Clone, Obj)]
pub struct LinkSelectedEvent(Event);

#[derive(Debug, Clone, Obj)]
pub struct StateChangedEvent(Event);
impl StateChangedEvent {
    fn state(&self) -> &str {
        self.0.kind()
    }
    //TODO checkme please!!
    fn enabled(&self) -> bool {
        self.0.detail1() == 0
    }
}

#[derive(Debug, Clone, Obj)]
pub struct ChildrenChangedEvent(Event);
impl ChildrenChangedEvent {
    fn operation(&self) -> &str {
        self.0.kind()
    }
    fn index_in_parent(&self) -> i32 {
        self.0.detail1()
    } // usizes ?
    fn child(&self) -> &OwnedValue {
        self.0.any_data()
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct VisibleDataChangedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct SelectionChangedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ModelChangedEvent(Event);

// TODO Check my impl please.
#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ActiveDescendantChangedEvent(Event);
impl ActiveDescendantChangedEvent {
    fn child(&self) -> &zvariant::OwnedValue {
        self.0.any_data() // TODO Make me a beter returner!
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct AnnouncementEvent(Event);
impl AnnouncementEvent {
    fn text(&self) -> &str {
        self.0.kind()
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct AttributesChangedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct RowInsertedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct RowReorderedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct RowDeletedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ColumnInsertedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ColumnReorderedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct ColumnDeletedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextBoundsChangedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextSelectionChangedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextChangedEvent(Event);
impl TextChangedEvent {
    fn detail(&self) -> &str {
        self.0.kind()
    }
    fn start_pos(&self) -> i32 {
        self.0.detail1()
    }
    fn end_pos(&self) -> i32 {
        self.0.detail2()
    }
    // TODO zvariant::Value -> String me please
    fn text(&self) -> &OwnedValue {
        self.0.any_data()
    }
}

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextAttributesChangedEvent(Event);

#[derive(Debug, Clone, TrySignify, Obj)]
pub struct TextCaretMovedEvent(Event);

impl TextCaretMovedEvent {
    fn position(&self) -> i32 {
        self.0.detail1()
    }
}

// ----------<- end of Obj signals
// ----------> Start of Win

mod win {
    use super::Win;
    use crate::events::Event;
    use crate::identify::MemberName;
    use crate::identify::Signified;
    use atspi_macros::TrySignify;

    //TODO Check my impl with bus signal
    #[derive(Debug, TrySignify, Win)]
    pub struct PropertyChangeEvent(Event);
    impl PropertyChangeEvent {
        fn property(&self) -> &str {
            self.0.kind()
        }
    }

    #[derive(Debug, TrySignify, Win)]
    pub struct MinimizeEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct MaximizeEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct RestoreEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct CloseEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct CreateEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct ReparentEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct DesktopCreateEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct DesktopDestroyEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct DestroyEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct ActivateEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct DeactivateEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct RaiseEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct LowerEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct MoveEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct ResizeEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct ShadeEvent(Event);

    #[allow(non_camel_case_types)]
    #[derive(Debug, TrySignify, Win)]
    pub struct uUshadeEvent(Event);

    #[derive(Debug, TrySignify, Win)]
    pub struct RestyleEvent(Event);
}

// ----------<- end of Win signals
// ----------> Start of Mse

#[derive(Debug, Clone, TrySignify, Mse)]
pub struct AbsEvent(Event);
impl AbsEvent {
    fn dx(&self) -> i32 {
        self.0.body().detail1
    }
    fn dy(&self) -> i32 {
        self.0.body().detail2
    }
}

#[derive(Debug, Clone, TrySignify, Mse)]
pub struct RelEvent(Event);
impl RelEvent {
    fn dx(&self) -> i32 {
        self.0.body().detail1
    }
    fn dy(&self) -> i32 {
        self.0.body().detail2
    }
}

#[derive(Debug, Clone, TrySignify, Mse)]
pub struct ButtonEvent(Event);
impl ButtonEvent {
    fn button(&self) -> &str {
        self.0.kind()
    }
    fn x(&self) -> i32 {
        self.0.body().detail1
    }
    fn y(&self) -> i32 {
        self.0.body().detail2
    }
}

// ----------<- end of Mse signals
// ----------> Start of Kbd

#[derive(Debug, Clone, TrySignify, Kbd)]
pub struct ModifiersEvent(Event);
impl ModifiersEvent {
    fn previous_modifiers(&self) -> i32 {
        self.0.body().detail1
    }
    fn current_modifiers(&self) -> i32 {
        self.0.body().detail2
    }
}

// ----------<- end of Kbd signals
// ----------> Start of Term

#[derive(Debug, Clone, TrySignify, Term)]
pub struct LineChangedEvent(Event);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct ColumncountChangedEvent(Event);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct LinecountChangedEvent(Event);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct ApplicationChangedEvent(Event);

#[derive(Debug, Clone, TrySignify, Term)]
pub struct CharwidthChangedEvent(Event);

// ----------<- end of Term signals
// ----------> Start of Doc

mod doc {
    use super::Doc;
    use crate::identify::MemberName;
    use crate::identify::Signified;
    use atspi_macros::TrySignify;

    use crate::identify::Event;

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct LoadCompleteEvent(Event);

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct ReloadEvent(Event);

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct LoadStoppedEvent(Event);

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct ContentChangedEvent(Event);

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct AttributesChangedEvent(Event);

    #[derive(Debug, Clone, TrySignify, Doc)]
    pub struct PageChangedEvent(Event);
}

#[derive(Debug, Clone, TrySignify, Focus)]
pub struct FocusEvent(Event);
