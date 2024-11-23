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
use std::{
	fs::File,
	io::{BufReader, BufWriter, Read, Write},
};
use zbus::{
	zvariant::{
		serialized::{Context, Data, Format},
		Endian,
	},
	Message,
};

pub fn vec_of_all_atspi_messages() -> Vec<Message> {
	vec![
		// document events
		DocumentAttributesChangedEvent::default().try_into().unwrap(),
		ContentChangedEvent::default().try_into().unwrap(),
		LoadCompleteEvent::default().try_into().unwrap(),
		LoadStoppedEvent::default().try_into().unwrap(),
		PageChangedEvent::default().try_into().unwrap(),
		ReloadEvent::default().try_into().unwrap(),
		// focus events
		FocusEvent::default().try_into().unwrap(),
		// mouse events
		AbsEvent::default().try_into().unwrap(),
		ButtonEvent::default().try_into().unwrap(),
		RelEvent::default().try_into().unwrap(),
		// object events
		ActiveDescendantChangedEvent::default().try_into().unwrap(),
		AnnouncementEvent::default().try_into().unwrap(),
		AttributesChangedEvent::default().try_into().unwrap(),
		BoundsChangedEvent::default().try_into().unwrap(),
		ChildrenChangedEvent::default().try_into().unwrap(),
		ColumnDeletedEvent::default().try_into().unwrap(),
		ColumnInsertedEvent::default().try_into().unwrap(),
		ColumnReorderedEvent::default().try_into().unwrap(),
		LinkSelectedEvent::default().try_into().unwrap(),
		ModelChangedEvent::default().try_into().unwrap(),
		PropertyChangeEvent::default().try_into().unwrap(),
		RowDeletedEvent::default().try_into().unwrap(),
		RowInsertedEvent::default().try_into().unwrap(),
		RowReorderedEvent::default().try_into().unwrap(),
		SelectionChangedEvent::default().try_into().unwrap(),
		TextAttributesChangedEvent::default().try_into().unwrap(),
		TextBoundsChangedEvent::default().try_into().unwrap(),
		TextCaretMovedEvent::default().try_into().unwrap(),
		TextChangedEvent::default().try_into().unwrap(),
		TextSelectionChangedEvent::default().try_into().unwrap(),
		VisibleDataChangedEvent::default().try_into().unwrap(),
		// terminal events
		ApplicationChangedEvent::default().try_into().unwrap(),
		CharWidthChangedEvent::default().try_into().unwrap(),
		ColumnCountChangedEvent::default().try_into().unwrap(),
		LineChangedEvent::default().try_into().unwrap(),
		LineCountChangedEvent::default().try_into().unwrap(),
		// window events
		ActivateEvent::default().try_into().unwrap(),
		CloseEvent::default().try_into().unwrap(),
		CreateEvent::default().try_into().unwrap(),
		DeactivateEvent::default().try_into().unwrap(),
		DesktopCreateEvent::default().try_into().unwrap(),
		DesktopDestroyEvent::default().try_into().unwrap(),
		DestroyEvent::default().try_into().unwrap(),
		LowerEvent::default().try_into().unwrap(),
		MaximizeEvent::default().try_into().unwrap(),
		MinimizeEvent::default().try_into().unwrap(),
		MoveEvent::default().try_into().unwrap(),
		WindowPropertyChangeEvent::default().try_into().unwrap(),
		RaiseEvent::default().try_into().unwrap(),
		ReparentEvent::default().try_into().unwrap(),
		ResizeEvent::default().try_into().unwrap(),
		RestoreEvent::default().try_into().unwrap(),
		RestyleEvent::default().try_into().unwrap(),
		ShadeEvent::default().try_into().unwrap(),
		UUshadeEvent::default().try_into().unwrap(),
	]
}

pub fn generate_n_messages_rnd(n: usize) -> Vec<Message> {
	let all_messages = vec_of_all_atspi_messages();
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
