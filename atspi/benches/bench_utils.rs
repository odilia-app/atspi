use atspi::events::document::{
	AttributesChangedEvent as DocumentAttributesChangedEvent, ContentChangedEvent,
	LoadCompleteEvent, LoadStoppedEvent, PageChangedEvent, ReloadEvent,
};
use atspi::events::focus::FocusEvent;
use atspi::events::mouse::{AbsEvent, ButtonEvent, RelEvent};
use atspi::events::object::{
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
use atspi::NonNullObjectRef;
use std::{
	fs::File,
	io::{BufReader, BufWriter, Read, Write},
};
use zbus::names::UniqueName;
use zbus::zvariant::ObjectPath;
use zbus::{
	zvariant::{
		serialized::{Context, Data, Format},
		Endian,
	},
	Message,
};

pub fn vec_of_all_atspi_messages(origin: &NonNullObjectRef) -> Vec<Message> {
	vec![
		// document events
		DocumentAttributesChangedEvent::new_test_event(origin)
			.try_into()
			.unwrap(),
		ContentChangedEvent::new_test_event(origin).try_into().unwrap(),
		LoadCompleteEvent::new_test_event(origin).try_into().unwrap(),
		LoadStoppedEvent::new_test_event(origin).try_into().unwrap(),
		PageChangedEvent::new_test_event(origin).try_into().unwrap(),
		ReloadEvent::new_test_event(origin).try_into().unwrap(),
		// focus events
		FocusEvent::new_test_event(origin).try_into().unwrap(),
		// mouse events
		AbsEvent::new_test_event(origin).try_into().unwrap(),
		ButtonEvent::new_test_event(origin).try_into().unwrap(),
		RelEvent::new_test_event(origin).try_into().unwrap(),
		// object events
		ActiveDescendantChangedEvent::new_test_event(origin)
			.try_into()
			.unwrap(),
		AnnouncementEvent::new_test_event(origin).try_into().unwrap(),
		AttributesChangedEvent::new_test_event(origin).try_into().unwrap(),
		BoundsChangedEvent::new_test_event(origin).try_into().unwrap(),
		ChildrenChangedEvent::new_test_event(origin).try_into().unwrap(),
		ColumnDeletedEvent::new_test_event(origin).try_into().unwrap(),
		ColumnInsertedEvent::new_test_event(origin).try_into().unwrap(),
		ColumnReorderedEvent::new_test_event(origin).try_into().unwrap(),
		LinkSelectedEvent::new_test_event(origin).try_into().unwrap(),
		ModelChangedEvent::new_test_event(origin).try_into().unwrap(),
		PropertyChangeEvent::new_test_event(origin).try_into().unwrap(),
		RowDeletedEvent::new_test_event(origin).try_into().unwrap(),
		RowInsertedEvent::new_test_event(origin).try_into().unwrap(),
		RowReorderedEvent::new_test_event(origin).try_into().unwrap(),
		SelectionChangedEvent::new_test_event(origin).try_into().unwrap(),
		TextAttributesChangedEvent::new_test_event(origin).try_into().unwrap(),
		TextBoundsChangedEvent::new_test_event(origin).try_into().unwrap(),
		TextCaretMovedEvent::new_test_event(origin).try_into().unwrap(),
		TextChangedEvent::new_test_event(origin).try_into().unwrap(),
		TextSelectionChangedEvent::new_test_event(origin).try_into().unwrap(),
		VisibleDataChangedEvent::new_test_event(origin).try_into().unwrap(),
		// terminal events
		ApplicationChangedEvent::new_test_event(origin).try_into().unwrap(),
		CharWidthChangedEvent::new_test_event(origin).try_into().unwrap(),
		ColumnCountChangedEvent::new_test_event(origin).try_into().unwrap(),
		LineChangedEvent::new_test_event(origin).try_into().unwrap(),
		LineCountChangedEvent::new_test_event(origin).try_into().unwrap(),
		// window events
		ActivateEvent::new_test_event(origin).try_into().unwrap(),
		CloseEvent::new_test_event(origin).try_into().unwrap(),
		CreateEvent::new_test_event(origin).try_into().unwrap(),
		DeactivateEvent::new_test_event(origin).try_into().unwrap(),
		DesktopCreateEvent::new_test_event(origin).try_into().unwrap(),
		DesktopDestroyEvent::new_test_event(origin).try_into().unwrap(),
		DestroyEvent::new_test_event(origin).try_into().unwrap(),
		LowerEvent::new_test_event(origin).try_into().unwrap(),
		MaximizeEvent::new_test_event(origin).try_into().unwrap(),
		MinimizeEvent::new_test_event(origin).try_into().unwrap(),
		MoveEvent::new_test_event(origin).try_into().unwrap(),
		WindowPropertyChangeEvent::new_test_event(origin).try_into().unwrap(),
		RaiseEvent::new_test_event(origin).try_into().unwrap(),
		ReparentEvent::new_test_event(origin).try_into().unwrap(),
		ResizeEvent::new_test_event(origin).try_into().unwrap(),
		RestoreEvent::new_test_event(origin).try_into().unwrap(),
		RestyleEvent::new_test_event(origin).try_into().unwrap(),
		ShadeEvent::new_test_event(origin).try_into().unwrap(),
		UUshadeEvent::new_test_event(origin).try_into().unwrap(),
	]
}

pub fn generate_n_messages_rnd(n: usize) -> Vec<Message> {
	static NON_NULL_ORIGIN: NonNullObjectRef<'static> = NonNullObjectRef::Owned {
		name: UniqueName::from_static_str_unchecked(":0.0"),
		path: ObjectPath::from_static_str_unchecked("/org/atspi/test"),
	};

	let all_messages = vec_of_all_atspi_messages(&NON_NULL_ORIGIN);
	let mut messages: Vec<Message> = Vec::with_capacity(n);
	for _ in 0..n {
		let random_msg = all_messages[fastrand::usize(..all_messages.len())].clone();
		messages.push(random_msg);
	}
	messages
}

pub fn write_messages_to_file(messages: Vec<Message>, file: &str) {
	let file = File::create(file).unwrap();
	let mut writer = BufWriter::new(file);

	for msg in messages {
		let bytes = msg.data().bytes();
		let len = bytes.len() as u32;
		writer.write_all(&len.to_ne_bytes()).unwrap();
		writer.write_all(bytes).unwrap();
	}
	writer.flush().unwrap();
}

pub fn read_messages_from_file(file_path: &str) -> Vec<Message> {
	let file = File::open(file_path).unwrap();
	let mut slices = Vec::new();
	let mut reader = BufReader::new(file);

	loop {
		let mut buf = [0; 4];
		let n = reader.read(&mut buf).unwrap();
		if n == 0 {
			break;
		}
		let len = u32::from_ne_bytes(buf);

		let mut buf = vec![0; len as usize];
		reader.read_exact(&mut buf).unwrap();
		slices.push(buf);
	}

	let context = Context::new(Format::default(), Endian::native(), 0);

	slices
		.into_iter()
		.map(|slice| {
			let data = Data::new(slice, context);
			unsafe { Message::from_bytes(data).unwrap() }
		})
		.collect()
}
