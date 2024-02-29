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
use atspi::Event;
use zbus::Message;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn vec_of_all_atspi_messages() -> Vec<Message> {
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

fn generate_atspi_messages_random(n: usize) -> Vec<Message> {
	let all_messages = vec_of_all_atspi_messages();
	let mut messages: Vec<Message> = Vec::with_capacity(n);
	for _ in 0..n {
		let random_msg = all_messages[fastrand::usize(..all_messages.len())].clone();
		messages.push(random_msg);
	}
	messages
}

pub fn criterion_benchmark(c: &mut Criterion) {
	let random_messages = generate_atspi_messages_random(1000);

	c.bench_function("Parse 1000 Messages into Events", |b| {
		b.iter(|| {
			for msg in &random_messages {
				Event::try_from(black_box(msg)).unwrap();
			}
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
