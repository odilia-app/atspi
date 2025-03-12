use atspi::{Event, Role, State, Operation, Politeness};
use atspi::events::document::{
	AttributesChangedEvent as DocumentAttributesChangedEvent, ContentChangedEvent,
	LoadCompleteEvent, LoadStoppedEvent, PageChangedEvent, ReloadEvent,
};
use atspi::events::focus::FocusEvent;
use atspi::events::mouse::{AbsEvent, ButtonEvent, RelEvent};
use atspi::events::object::{
    Property,
    StateChangedEvent,
	ActiveDescendantChangedEvent, AnnouncementEvent, AttributesChangedEvent, BoundsChangedEvent,
	ChildrenChangedEvent, ColumnDeletedEvent, ColumnInsertedEvent, ColumnReorderedEvent,
	LinkSelectedEvent, ModelChangedEvent, PropertyChangeEvent, RowDeletedEvent, RowInsertedEvent,
	RowReorderedEvent, SelectionChangedEvent, TextAttributesChangedEvent, TextBoundsChangedEvent,
	TextCaretMovedEvent, TextChangedEvent, TextSelectionChangedEvent, VisibleDataChangedEvent,
};
use atspi::events::terminal::{
	ApplicationChangedEvent, CharWidthChangedEvent, ColumnCountChangedEvent, LineChangedEvent,
	LineCountChangedEvent,
};
use atspi::events::window::{
	ActivateEvent, CloseEvent, CreateEvent, DeactivateEvent, DesktopCreateEvent,
	DesktopDestroyEvent, DestroyEvent, LowerEvent, MaximizeEvent, MinimizeEvent, MoveEvent,
	PropertyChangeEvent as WindowPropertyChangeEvent, RaiseEvent, ReparentEvent, ResizeEvent,
	RestoreEvent, RestyleEvent, ShadeEvent, UUshadeEvent,
};
use zbus::{
  zvariant::ObjectPath,
  names::{InterfaceName, MemberName, UniqueName},
};
use atspi::ObjectRef;
use proptest::prelude::*;
use proptest::{
    string::string_regex,
};
use zbus::zvariant::{Value, OwnedValue};

const OBJECT_PATH_PATTERN: &str = "(/[a-zA-Z0-9_]+)+";
const INTERFACE_NAME_PATTERN: &str = r"[a-zA-Z_]+(\.[a-zA-Z_])+";
const UNIQUE_NAME_PATTERN: &str = r":[a-zA-Z_]+(\.[a-zA-Z_])+";
const MEMBER_NAME_PATTERN: &str = "[a-zA-Z][a-zA-Z0-9_]+";

pub fn operation() -> impl Strategy<Value = Operation> {
    prop_oneof![
        Just(Operation::Insert),
        Just(Operation::Delete),
    ]
}

pub fn object_path() -> impl Strategy<Value = ObjectPath<'static>> {
    string_regex(OBJECT_PATH_PATTERN)
        .expect("Valid strategy!")
        .prop_map(|s| ObjectPath::try_from(s.clone())
            .unwrap_or_else(|_| panic!("Invlaid object path: {s}")))
}

pub fn interface_name() -> impl Strategy<Value = UniqueName<'static>> {
    string_regex(UNIQUE_NAME_PATTERN)
        .expect("Valid strategy!")
        .prop_map(|s| UniqueName::try_from(s.clone())
            .unwrap_or_else(|_| panic!("Invlaid bus name: {s}")))
}

pub fn object_ref() -> impl Strategy<Value = ObjectRef> {
    (object_path(), interface_name())
        .prop_map(|(p, n)| ObjectRef {
            name: n.into(),
            path: p.into(),
        })
}

pub fn body_object_ref_event() -> impl Strategy<Value = Event> {
    prop_oneof![
        object_ref().prop_map(DocumentAttributesChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(DocumentAttributesChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ContentChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(LoadCompleteEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(LoadStoppedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(PageChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ReloadEvent::from)
            .prop_map(Event::from),
        // focus events
        object_ref().prop_map(FocusEvent::from)
            .prop_map(Event::from),
        // object events
        object_ref().prop_map(AttributesChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(BoundsChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ColumnDeletedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ColumnInsertedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ColumnReorderedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(LinkSelectedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ModelChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(RowDeletedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(RowInsertedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(RowReorderedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(SelectionChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(TextAttributesChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(TextBoundsChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(TextSelectionChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(VisibleDataChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ApplicationChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(CharWidthChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ColumnCountChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(LineChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(LineCountChangedEvent::from)
            .prop_map(Event::from),
		object_ref().prop_map(ActivateEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(CloseEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(CreateEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(DeactivateEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(DesktopCreateEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(DesktopDestroyEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(DestroyEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(LowerEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(MaximizeEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(MinimizeEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(MoveEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(RaiseEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(ReparentEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(ResizeEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(RestoreEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(RestyleEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(ShadeEvent::from)
        .prop_map(Event::from),
		object_ref().prop_map(UUshadeEvent::from)
        .prop_map(Event::from),
    ]
}

fn state() -> impl Strategy<Value = State> {
    prop_oneof![
      Just(State::Invalid),
      Just(State::Active),
      Just(State::Armed),
      Just(State::Busy),
      Just(State::Checked),
      Just(State::Collapsed),
      Just(State::Defunct),
      Just(State::Editable),
      Just(State::Enabled),
      Just(State::Expandable),
      Just(State::Expanded),
      Just(State::Focusable),
      Just(State::Focused),
      Just(State::HasTooltip),
      Just(State::Horizontal),
      Just(State::Iconified),
      Just(State::Modal),
      Just(State::MultiLine),
      Just(State::Multiselectable),
      Just(State::Opaque),
      Just(State::Pressed),
      Just(State::Resizable),
      Just(State::Selectable),
      Just(State::Selected),
      Just(State::Sensitive),
      Just(State::Showing),
      Just(State::SingleLine),
      Just(State::Stale),
      Just(State::Transient),
      Just(State::Vertical),
      Just(State::Visible),
      Just(State::ManagesDescendants),
      Just(State::Indeterminate),
      Just(State::Required),
      Just(State::Truncated),
      Just(State::Animated),
      Just(State::InvalidEntry),
      Just(State::SupportsAutocompletion),
      Just(State::SelectableText),
      Just(State::IsDefault),
      Just(State::Visited),
      Just(State::Checkable),
      Just(State::HasPopup),
      Just(State::ReadOnly),
    ]
}

fn role() -> impl Strategy<Value = Role> {
	prop_oneof![
		Just(Role::Invalid),
		Just(Role::AcceleratorLabel),
		Just(Role::Alert),
		Just(Role::Animation),
		Just(Role::Arrow),
		Just(Role::Calendar),
		Just(Role::Canvas),
		Just(Role::CheckBox),
		Just(Role::CheckMenuItem),
		Just(Role::ColorChooser),
		Just(Role::ColumnHeader),
		Just(Role::ComboBox),
		Just(Role::DateEditor),
		Just(Role::DesktopIcon),
		Just(Role::DesktopFrame),
		Just(Role::Dial),
		Just(Role::Dialog),
		Just(Role::DirectoryPane),
		Just(Role::DrawingArea),
		Just(Role::FileChooser),
		Just(Role::Filler),
		Just(Role::FocusTraversable),
		Just(Role::FontChooser),
		Just(Role::Frame),
		Just(Role::GlassPane),
		Just(Role::HTMLContainer),
		Just(Role::Icon),
		Just(Role::Image),
		Just(Role::InternalFrame),
		Just(Role::Label),
		Just(Role::LayeredPane),
		Just(Role::List),
		Just(Role::ListItem),
		Just(Role::Menu),
		Just(Role::MenuBar),
		Just(Role::MenuItem),
		Just(Role::OptionPane),
		Just(Role::PageTab),
		Just(Role::PageTabList),
		Just(Role::Panel),
		Just(Role::PasswordText),
		Just(Role::PopupMenu),
		Just(Role::ProgressBar),
		Just(Role::Button),
		Just(Role::RadioButton),
		Just(Role::RadioMenuItem),
		Just(Role::RootPane),
		Just(Role::RowHeader),
		Just(Role::ScrollBar),
		Just(Role::ScrollPane),
		Just(Role::Separator),
		Just(Role::Slider),
		Just(Role::SpinButton),
		Just(Role::SplitPane),
		Just(Role::StatusBar),
		Just(Role::Table),
		Just(Role::TableCell),
		Just(Role::TableColumnHeader),
		Just(Role::TableRowHeader),
		Just(Role::TearoffMenuItem),
		Just(Role::Terminal),
		Just(Role::Text),
		Just(Role::ToggleButton),
		Just(Role::ToolBar),
		Just(Role::ToolTip),
		Just(Role::Tree),
		Just(Role::TreeTable),
		Just(Role::Unknown),
		Just(Role::Viewport),
		Just(Role::Window),
		Just(Role::Extended),
		Just(Role::Header),
		Just(Role::Footer),
		Just(Role::Paragraph),
		Just(Role::Ruler),
		Just(Role::Application),
		Just(Role::Autocomplete),
		Just(Role::Editbar),
		Just(Role::Embedded),
		Just(Role::Entry),
		Just(Role::CHART),
		Just(Role::Caption),
		Just(Role::DocumentFrame),
		Just(Role::Heading),
		Just(Role::Page),
		Just(Role::Section),
		Just(Role::RedundantObject),
		Just(Role::Form),
		Just(Role::Link),
		Just(Role::InputMethodWindow),
		Just(Role::TableRow),
		Just(Role::TreeItem),
		Just(Role::DocumentSpreadsheet),
		Just(Role::DocumentPresentation),
		Just(Role::DocumentText),
		Just(Role::DocumentWeb),
		Just(Role::DocumentEmail),
		Just(Role::Comment),
		Just(Role::ListBox),
		Just(Role::Grouping),
		Just(Role::ImageMap),
		Just(Role::Notification),
		Just(Role::InfoBar),
		Just(Role::LevelBar),
		Just(Role::TitleBar),
		Just(Role::BlockQuote),
		Just(Role::Audio),
		Just(Role::Video),
		Just(Role::Definition),
		Just(Role::Article),
		Just(Role::Landmark),
		Just(Role::Log),
		Just(Role::Marquee),
		Just(Role::Math),
		Just(Role::Rating),
		Just(Role::Timer),
		Just(Role::Static),
		Just(Role::MathFraction),
		Just(Role::MathRoot),
		Just(Role::Subscript),
		Just(Role::Superscript),
		Just(Role::DescriptionList),
		Just(Role::DescriptionTerm),
		Just(Role::DescriptionValue),
		Just(Role::Footnote),
		Just(Role::ContentDeletion),
		Just(Role::ContentInsertion),
		Just(Role::Mark),
		Just(Role::Suggestion),
		Just(Role::PushButtonMenu),
	]
}

fn value() -> impl Strategy<Value = OwnedValue> {
    prop_oneof![
        any::<u8>().prop_map(|int| 
            Value::U8(int).try_into().expect("Valud owned value!")),
        any::<u16>().prop_map(|int| 
            Value::U16(int).try_into().expect("Valud owned value!")),
        any::<u32>().prop_map(|int| 
            Value::U32(int).try_into().expect("Valud owned value!")),
        any::<u64>().prop_map(|int| 
            Value::U64(int).try_into().expect("Valud owned value!")),
        any::<i16>().prop_map(|int| 
            Value::I16(int).try_into().expect("Valud owned value!")),
        any::<i32>().prop_map(|int| 
            Value::I32(int).try_into().expect("Valud owned value!")),
        any::<i64>().prop_map(|int| 
            Value::I64(int).try_into().expect("Valud owned value!")),
        any::<f64>().prop_map(|int| 
            Value::F64(int).try_into().expect("Valud owned value!")),
        any::<String>().prop_map(|s| 
            Value::Str(s.into()).try_into().expect("Valud owned value!")),
        object_path().prop_map(|op|
            Value::ObjectPath(op).try_into().expect("Valud owned value!")),
        value().prop_map(|v| 
            Value::Value(Box::new(v.into())).try_into().expect("Valud owned value!")),
        // TODO: signature, array, dictionary, structure, maybe, fd
    ]
}

fn property() -> impl Strategy<Value = Property> {
    prop_oneof![
        any::<String>().prop_map(|name| Property::Name(name)),
        any::<String>().prop_map(|desc| Property::Description(desc)),
        role().prop_map(|r| Property::Role(r)),
        object_ref().prop_map(|or| Property::Parent(or)),
        any::<String>().prop_map(|tc| Property::TableCaption(tc)),
        any::<String>().prop_map(|tcd| Property::TableColumnDescription(tcd)),
        any::<String>().prop_map(|tch| Property::TableColumnHeader(tch)),
        any::<String>().prop_map(|trd| Property::TableRowDescription(trd)),
        any::<String>().prop_map(|trh| Property::TableRowHeader(trh)),
        any::<String>().prop_map(|ts| Property::TableSummary(ts)),
        (any::<String>(), value())
            .prop_map(|(name, val)| Property::Other((name, val)))
    ]
}

fn politeness() -> impl Strategy<Value = Politeness> {
    prop_oneof![
        Just(Politeness::None),
        Just(Politeness::Polite),
        Just(Politeness::Assertive),
    ]
}

fn one_object_ref_event<T>() -> impl Strategy<Value = Event> 
where T: From<ObjectRef> + std::fmt::Debug,
Event: From<T>  {
    object_ref().prop_map(T::from).prop_map(Event::from)
}

fn object_event() -> impl Strategy<Value = Event> {
  prop_oneof![
        (property(), object_ref(), any::<String>())
            .prop_map(|(value, item, property)| PropertyChangeEvent {
                item,
                property,
                value,
            })
            .prop_map(Event::from),
        (state(), any::<bool>(), object_ref())
            .prop_map(|(state, enabled, item)| StateChangedEvent {
                state, enabled, item
            })
            .prop_map(Event::from),
        (object_ref(), object_ref(), any::<i32>(), operation())
            .prop_map(|(item, child, index_in_parent, operation)| ChildrenChangedEvent {
                item, child, index_in_parent, operation
            })
            .prop_map(Event::from),
        (object_ref(), object_ref())
            .prop_map(|(item, child)| ActiveDescendantChangedEvent {
                item, child
            })
            .prop_map(Event::from),
        (object_ref(), any::<String>(), politeness())
            .prop_map(|(item, text, live)| AnnouncementEvent {
                item, text, live
            })
            .prop_map(Event::from),
        (object_ref(), operation(), any::<i32>(), any::<i32>(), any::<String>())
            .prop_map(|(item, operation, start_pos, length, text)| TextChangedEvent {
                item, operation, start_pos, length, text
            })
            .prop_map(Event::from),
        (object_ref(), any::<i32>())
            .prop_map(|(item, position)| TextCaretMovedEvent {
                item, position
            })
            .prop_map(Event::from),
        object_ref().prop_map(AttributesChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(BoundsChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ColumnDeletedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ColumnInsertedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ColumnReorderedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(LinkSelectedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(ModelChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(RowDeletedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(RowInsertedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(RowReorderedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(SelectionChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(TextAttributesChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(TextBoundsChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(TextSelectionChangedEvent::from)
            .prop_map(Event::from),
        object_ref().prop_map(VisibleDataChangedEvent::from)
            .prop_map(Event::from),
  ]
}

proptest! {
#[test]
    fn op_event(
        ope in body_object_ref_event()
    ) {
        println!("{ope:?}");
    }
#[test]
    fn test_path(
        path in OBJECT_PATH_PATTERN,
    ) {
        let Ok(_op) = ObjectPath::try_from(path.clone()) else {
            panic!("Invalid object path: {path}");
        };
    }
#[test]
    fn test_interface(
        name in INTERFACE_NAME_PATTERN,
    ) {
        let Ok(_in) = InterfaceName::try_from(name.clone()) else {
            panic!("Invalid name: {name}");
        };
    }
#[test]
    fn test_member(
        name in MEMBER_NAME_PATTERN,
    ) {
        let Ok(_mn) = MemberName::try_from(name.clone()) else {
            panic!("Invalid name: {name}");
        };
    }
#[test]
    fn test_unique(
        name in UNIQUE_NAME_PATTERN,
    ) {
        let Ok(_un) = UniqueName::try_from(name.clone()) else {
            panic!("Invalid name: {name}");
        };
    }
}

