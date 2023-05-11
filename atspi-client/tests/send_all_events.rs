use atspi_client::AccessibilityConnection;
use atspi_types::{
	events::{
		document::{
			LoadCompleteEvent,
			ReloadEvent,
			LoadStoppedEvent,
			ContentChangedEvent,
			AttributesChangedEvent as DocumentAttributesChangedEvent,
			PageChangedEvent,
		},
		object::{
			PropertyChangeEvent as ObjectPropertyChangeEvent,
			BoundsChangedEvent,
			LinkSelectedEvent,
			StateChangedEvent as ObjectStateChangedEvent,
			ChildrenChangedEvent,
			VisibleDataChangedEvent,
			SelectionChangedEvent,
			ModelChangedEvent,
			ActiveDescendantChangedEvent,
			AnnouncementEvent,
			AttributesChangedEvent as ObjectAttributesChangedEvent,
			RowInsertedEvent,
			RowReorderedEvent,
			RowDeletedEvent,
			ColumnInsertedEvent,
			ColumnReorderedEvent,
			ColumnDeletedEvent,
			TextBoundsChangedEvent,
			TextSelectionChangedEvent,
			TextChangedEvent,
			TextAttributesChangedEvent,
			TextCaretMovedEvent,
		},
		keyboard::{
			ModifiersEvent,
		},
		mouse::{
			AbsEvent,
			RelEvent,
			ButtonEvent,
		},
		terminal::{
			LineChangedEvent,
			ColumnCountChangedEvent,
			LineCountChangedEvent,
			ApplicationChangedEvent,
			CharWidthChangedEvent,
		},
		window::{
			PropertyChangeEvent,
			MinimizeEvent,
			MaximizeEvent,
			RestoreEvent,
			CloseEvent,
			CreateEvent,
			ReparentEvent,
			DesktopCreateEvent,
			DesktopDestroyEvent,
			DestroyEvent,
			ActivateEvent,
			DeactivateEvent,
			RaiseEvent,
			LowerEvent,
			MoveEvent,
			ResizeEvent,
			ShadeEvent,
			UUshadeEvent,
			RestyleEvent,
		},
	},
	events::GenericEvent,
};
use rename_item::rename;

macro_rules! end_to_end_test_case {
	($type:ty) => {
		#[rename(name($type), prefix="end_to_end_test_case_", case="snake")]
		#[tokio::test]
		async fn end_to_end() -> Result<(), Box<dyn std::error::Error>> {
			use futures_lite::StreamExt;
			let struct_event = <$type>::default();
			let con = AccessibilityConnection::open().await.unwrap();
			con.register_event::<$type>().await.expect("Could not register event");
			let mut events = con.event_stream();
			std::pin::pin!(&mut events);
			con.send_event(struct_event.clone())
				.await
				.expect("Could not send event struct");
			while let Some(Ok(ev)) = events.next().await {
				if let Ok(event) = <$type>::try_from(ev) {
					assert_eq!(struct_event.body(), event.body());
					break;
				// do things with your event here
				} else {
					panic!("The wrong event was received.")
				};
			}
			Ok(())
		}
	};
}

end_to_end_test_case!(ObjectPropertyChangeEvent);
end_to_end_test_case!(BoundsChangedEvent);
end_to_end_test_case!(LinkSelectedEvent);
end_to_end_test_case!(ObjectStateChangedEvent);
end_to_end_test_case!(ChildrenChangedEvent);
end_to_end_test_case!(VisibleDataChangedEvent);
end_to_end_test_case!(SelectionChangedEvent);
end_to_end_test_case!(ModelChangedEvent);
end_to_end_test_case!(ActiveDescendantChangedEvent);
end_to_end_test_case!(AnnouncementEvent);
end_to_end_test_case!(ObjectAttributesChangedEvent);
end_to_end_test_case!(RowInsertedEvent);
end_to_end_test_case!(RowReorderedEvent);
end_to_end_test_case!(RowDeletedEvent);
end_to_end_test_case!(ColumnInsertedEvent);
end_to_end_test_case!(ColumnReorderedEvent);
end_to_end_test_case!(ColumnDeletedEvent);
end_to_end_test_case!(TextBoundsChangedEvent);
end_to_end_test_case!(TextSelectionChangedEvent);
end_to_end_test_case!(TextChangedEvent);
end_to_end_test_case!(TextAttributesChangedEvent);
end_to_end_test_case!(TextCaretMovedEvent);
end_to_end_test_case!(LineChangedEvent);
end_to_end_test_case!(ColumnCountChangedEvent);
end_to_end_test_case!(LineCountChangedEvent);
end_to_end_test_case!(ApplicationChangedEvent);
end_to_end_test_case!(CharWidthChangedEvent);
end_to_end_test_case!(LoadCompleteEvent);
end_to_end_test_case!(ReloadEvent);
end_to_end_test_case!(LoadStoppedEvent);
end_to_end_test_case!(ContentChangedEvent);
end_to_end_test_case!(DocumentAttributesChangedEvent);
end_to_end_test_case!(PageChangedEvent);
end_to_end_test_case!(PropertyChangeEvent);
end_to_end_test_case!(MinimizeEvent);
end_to_end_test_case!(MaximizeEvent);
end_to_end_test_case!(RestoreEvent);
end_to_end_test_case!(CloseEvent);
end_to_end_test_case!(CreateEvent);
end_to_end_test_case!(ReparentEvent);
end_to_end_test_case!(DesktopCreateEvent);
end_to_end_test_case!(DesktopDestroyEvent);
end_to_end_test_case!(DestroyEvent);
end_to_end_test_case!(ActivateEvent);
end_to_end_test_case!(DeactivateEvent);
end_to_end_test_case!(RaiseEvent);
end_to_end_test_case!(LowerEvent);
end_to_end_test_case!(MoveEvent);
end_to_end_test_case!(ResizeEvent);
end_to_end_test_case!(ShadeEvent);
end_to_end_test_case!(UUshadeEvent);
end_to_end_test_case!(RestyleEvent);
end_to_end_test_case!(ModifiersEvent);
end_to_end_test_case!(AbsEvent);
end_to_end_test_case!(RelEvent);
end_to_end_test_case!(ButtonEvent);
