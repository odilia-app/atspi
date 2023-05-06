#[macro_export]

macro_rules! impl_event_conversions {
	($inner_type:ty, $outer_type:ty, $inner_variant:path, $outer_variant:path) => {
		impl From<$inner_type> for $outer_type {
			fn from(specific_event: $inner_type) -> $outer_type {
				$inner_variant(specific_event)
			}
		}
		impl From<$inner_type> for Event {
			fn from(event_variant: $inner_type) -> Event {
				$outer_variant(event_variant.into())
			}
		}
		impl TryFrom<Event> for $inner_type {
			type Error = AtspiError;
			fn try_from(generic_event: Event) -> Result<$inner_type, Self::Error> {
				if let $outer_variant($inner_variant(specific_event)) = generic_event {
					Ok(specific_event)
				} else {
					Err(AtspiError::Conversion("Invalid type"))
				}
			}
		}
	};
}

macro_rules! impl_to_dbus_message {
	($type:ty) => {
		impl TryFrom<$type> for zbus::Message {
			type Error = AtspiError;
			fn try_from(event: $type) -> Result<Self, Self::Error> {
				Ok(zbus::MessageBuilder::signal(
					event.path(),
					<$type as GenericEvent>::DBUS_INTERFACE,
					<$type as GenericEvent>::DBUS_MEMBER,
				)?
				.sender(event.sender())?
				.build(&((event.body()),))?)
			}
		}
	};
}

macro_rules! impl_from_dbus_message {
	($type:ty) => {
		impl TryFrom<&zbus::Message> for $type {
			type Error = AtspiError;
			fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
				if msg.interface().ok_or(AtspiError::MissingInterface)?
					!= <$type as GenericEvent>::DBUS_INTERFACE
				{
					return Err(AtspiError::InterfaceMatch(format!(
						"The interface {} does not match the signal's interface: {}",
						msg.interface().unwrap(),
						<$type as GenericEvent>::DBUS_INTERFACE
					)));
				}
				if msg.member().ok_or(AtspiError::MissingMember)? != <$type>::DBUS_MEMBER {
					return Err(AtspiError::MemberMatch(format!(
						"The member {} does not match the signal's member: {}",
						// unwrap is safe here because of guard above
						msg.member().unwrap(),
						<$type as GenericEvent>::DBUS_MEMBER
					)));
				}
				<$type>::build(msg.try_into()?, msg.body::<<$type as GenericEvent>::Body>()?)
			}
		}
	};
}

#[cfg(test)]
macro_rules! generic_event_test_case {
	($type:ty) => {
		#[test]
		fn generic_event_uses() {
			let struct_event = <$type>::default();
			assert_eq!(struct_event.path().as_str(), "/org/a11y/atspi/accessible/null");
			assert_eq!(struct_event.sender().as_str(), ":0.0");
			let item = struct_event.item.clone();
			let body = struct_event.body();
			let build_struct = <$type>::build(item, body).expect("Could not build type from parts");
			assert_eq!(struct_event, build_struct);
		}
	};
}

#[cfg(test)]
macro_rules! event_enum_test_case {
	($type:ty) => {
		#[test]
		fn event_enum_conversion() {
			let struct_event = <$type>::default();
			let event = Event::from(struct_event.clone());
			let struct_event_back = <$type>::try_from(event)
				.expect("Could not convert from `Event` back to specifc event type");
			assert_eq!(struct_event, struct_event_back);
		}
	};
}

#[cfg(test)]
macro_rules! zbus_message_test_case {
	($type:ty) => {
		#[test]
		fn zbus_msg_conversion() {
			let struct_event = <$type>::default();
			let msg: zbus::Message = zbus::Message::try_from(struct_event.clone())
				.expect("Could not convert event into a message");
			let struct_event_back =
				<$type>::try_from(&msg).expect("Could not convert message into an event");
			assert_eq!(struct_event, struct_event_back);
		}
		// make want to consider paramaterized tests here, no need for fuzz testing, but one level lower than that may be nice
		// try having a matching member, matching interface, path, or body type, but that has some other piece which is not right
		#[test]
		#[should_panic]
		fn zbus_msg_conversion_failure_fake_msg() -> () {
			let Ok(msg_builder) = zbus::MessageBuilder::signal(
					"/org/a11y/sixynine/fourtwenty",
					"org.a11y.atspi.technically.valid",
					"MadeUpMember",
				) else {
				return ();
			};
			let Ok(msg_with_sender) = msg_builder.sender(":0.0") else {
				return ();
			};
			let Ok(fake_msg) = msg_with_sender.build(&(),) else {
				return ();
			};
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panci! Nothing about this event is valid");
		}
		#[test]
		#[should_panic]
		fn zbus_msg_conversion_failure_correct_interface() -> () {
			let Ok(msg_builder) = zbus::MessageBuilder::signal(
					"/org/a11y/sixynine/fourtwenty",
					<$type>::DBUS_INTERFACE,
					"MadeUpMember",
				) else {
				return ();
			};
			let Ok(msg_with_sender) = msg_builder.sender(":0.0") else {
				return ();
			};
			let Ok(fake_msg) = msg_with_sender.build(&(),) else {
				return ();
			};
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panci! Nothing about this event is valid");
		}
		#[test]
		#[should_panic]
		fn zbus_msg_conversion_failure_correct_interface_and_member() -> () {
			let Ok(msg_builder) = zbus::MessageBuilder::signal(
					"/org/a11y/sixynine/fourtwenty",
					<$type>::DBUS_INTERFACE,
					<$type>::DBUS_MEMBER,
				) else {
				return ();
			};
			let Ok(msg_with_sender) = msg_builder.sender(":0.0") else {
				return ();
			};
			let Ok(fake_msg) = msg_with_sender.build(&("i", "am", "cool", "and", "have", "a", "strange", "body"),) else {
				return ();
			};
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panci! Nothing about this event is valid");
		}
		#[test]
		#[should_panic]
		fn zbus_msg_conversion_failure_correct_body() -> () {
			let Ok(msg_builder) = zbus::MessageBuilder::signal(
					"/org/a11y/sixynine/fourtwenty",
					<$type>::DBUS_INTERFACE,
					"FakeMember",
				) else {
				return ();
			};
			let Ok(msg_with_sender) = msg_builder.sender(":0.0") else {
				return ();
			};
			let Ok(fake_msg) = msg_with_sender.build(&(<$type>::default().body()),) else {
				return ();
			};
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panci! Nothing about this event is valid");
		}
		#[test]
		#[should_panic]
		fn zbus_msg_conversion_failure_correct_body_and_member() -> () {
			let Ok(msg_builder) = zbus::MessageBuilder::signal(
					"/org/a11y/sixynine/fourtwenty",
					"org.a11y.atspi.accessible.technically.valid",
					<$type>::DBUS_MEMBER,
				) else {
				return ();
			};
			let Ok(msg_with_sender) = msg_builder.sender(":0.0") else {
				return ();
			};
			let Ok(fake_msg) = msg_with_sender.build(&(<$type>::default().body()),) else {
				return ();
			};
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panci! Nothing about this event is valid");
		}
	};
}

#[cfg(test)]
macro_rules! end_to_end_test_case {
	($type:ty) => {
		#[tokio::test]
		async fn end_to_end() -> Result<(), Box<dyn std::error::Error>> {
			use futures_lite::StreamExt;
			let struct_event = <$type>::default();
			let con = crate::AccessibilityConnection::open().await.unwrap();
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

macro_rules! event_test_cases {
	($type:ty) => {
		#[cfg(test)]
		#[rename_item::rename(name($type), prefix = "event_tests_", case = "snake")]
		mod foo {
			use super::{$type, Event, GenericEvent};

			generic_event_test_case!($type);
			event_enum_test_case!($type);
			zbus_message_test_case!($type);
			end_to_end_test_case!($type);
		}
	};
}
