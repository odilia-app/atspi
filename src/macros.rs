#[macro_export]

macro_rules! impl_event_conversions {
	($outer_type:ty, $outer_variant:path) => {
		impl From<$outer_type> for Event {
			fn from(event_variant: $outer_type) -> Event {
				$outer_variant(event_variant.into())
			}
		}
		impl TryFrom<Event> for $outer_type {
			type Error = AtspiError;
			fn try_from(generic_event: Event) -> Result<$outer_type, Self::Error> {
				if let $outer_variant(event_type) = generic_event {
					Ok(event_type)
				} else {
					Err(AtspiError::Conversion("Invalid type"))
				}
			}
		}
	};
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
		#[should_panic(expected = "should panic")]
		fn zbus_msg_conversion_failure_fake_msg() -> () {
			let fake_msg = zbus::MessageBuilder::signal(
				"/org/a11y/sixynine/fourtwenty",
				"org.a11y.atspi.technically.valid",
				"MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panic! Invalid event.");
		}
		#[test]
		#[should_panic(expected = "should panic")]
		fn zbus_msg_conversion_failure_correct_interface() -> () {
			let fake_msg = zbus::MessageBuilder::signal(
				"/org/a11y/sixynine/fourtwenty",
				<$type as GenericEvent>::DBUS_INTERFACE,
				"MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panic! Invalid event.");
		}
		#[test]
		#[should_panic(expected = "should panic")]
		fn zbus_msg_conversion_failure_correct_interface_and_member() -> () {
			let fake_msg = zbus::MessageBuilder::signal(
				"/org/a11y/sixynine/fourtwenty",
				<$type as GenericEvent>::DBUS_INTERFACE,
				<$type as GenericEvent>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panic! Invalid event.");
		}
		#[test]
		#[should_panic(expected = "should panic")]
		fn zbus_msg_conversion_failure_correct_body() -> () {
			let fake_msg = zbus::MessageBuilder::signal(
				"/org/a11y/sixynine/fourtwenty",
				"org.a11y.atspi.accesible.technically.valid",
				"FakeMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&(<$type>::default().body()))
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panic! Invalid event.");
		}
		#[test]
		#[should_panic(expected = "should panic")]
		fn zbus_msg_conversion_failure_correct_body_and_member() -> () {
			let fake_msg = zbus::MessageBuilder::signal(
				"/org/a11y/sixynine/fourtwenty",
				"org.a11y.atspi.accessible.technically.valid",
				<$type as GenericEvent>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&(<$type>::default().body()))
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panic! Invalid event.");
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

macro_rules! event_wrapper_test_cases {
	($type:ty, $any_subtype:ty) => {
		#[cfg(test)]
		#[rename_item::rename(name($type), prefix = "events_tests_", case = "snake")]
		mod foo {
			use super::{$any_subtype, $type, Event, GenericEvent};
			#[test]
			fn into_and_try_from_event() {
				let sub_type = <$any_subtype>::default();
				let mod_type = <$type>::from(sub_type);
				let event = Event::from(mod_type.clone());
				let mod_type2 = <$type>::try_from(event.clone())
					.expect("Could not create event type from event");
				assert_eq!(
					mod_type, mod_type2,
					"Events were able to be parsed and encapsulated, but they have changed value"
				);
			}
			#[test]
			#[should_panic]
			fn zbus_msg_invalid_interface() {
				let Ok(msg_builder) = zbus::MessageBuilder::signal(
								"/org/a11y/sixynine/fourtwenty",
								"org.a11y.atspi.technically.valid.lol",
								<$any_subtype as GenericEvent>::DBUS_MEMBER,
							) else {
							return ();
						};
				let Ok(msg_with_sender) = msg_builder.sender(":0.0") else {
							return ();
						};
				let Ok(fake_msg) = msg_with_sender.build(&(<$any_subtype>::default().body()),) else {
							return ();
						};
				let mod_type = <$type>::try_from(&fake_msg);
				mod_type.expect("Could not convert message into a event wrapper type");
			}
			#[test]
			#[should_panic]
			fn zbus_msg_invalid_member() {
				let Ok(msg_builder) = zbus::MessageBuilder::signal(
								"/org/a11y/sixynine/fourtwenty",
								<$any_subtype as GenericEvent>::DBUS_INTERFACE,
								"FakeFunctionLol",
							) else {
							return ();
						};
				let Ok(msg_with_sender) = msg_builder.sender(":0.0") else {
							return ();
						};
				let Ok(fake_msg) = msg_with_sender.build(&(<$any_subtype>::default().body()),) else {
							return ();
						};
				let mod_type = <$type>::try_from(&fake_msg);
				mod_type.expect("Could not convert message into a event wrapper type");
			}
			#[test]
			#[should_panic]
			fn zbus_msg_invalid_member_and_interface() {
				let Ok(msg_builder) = zbus::MessageBuilder::signal(
								"/org/a11y/sixynine/fourtwenty",
								"org.a11y.atspi.technically.allowed",
								"FakeFunctionLol",
							) else {
							return ();
						};
				let Ok(msg_with_sender) = msg_builder.sender(":0.0") else {
							return ();
						};
				let Ok(fake_msg) = msg_with_sender.build(&(<$any_subtype>::default().body()),) else {
							return ();
						};
				let mod_type = <$type>::try_from(&fake_msg);
				mod_type.expect("Could not convert message into a event wrapper type");
			}
			#[test]
			fn zbus_msg_conversion() {
				let Ok(msg_builder) = zbus::MessageBuilder::signal(
								"/org/a11y/sixynine/fourtwenty",
								<$any_subtype as GenericEvent>::DBUS_INTERFACE,
								<$any_subtype as GenericEvent>::DBUS_MEMBER,
							) else {
							return ();
						};
				let Ok(msg_with_sender) = msg_builder.sender(":0.0") else {
							return ();
						};
				let Ok(fake_msg) = msg_with_sender.build(&(<$any_subtype>::default().body()),) else {
							return ();
						};
				let mod_type = <$type>::try_from(&fake_msg);
				mod_type.expect("Could not convert message into a event wrapper type");
			}
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
		assert_impl_all!(
			$type: Clone,
			std::fmt::Debug,
			serde::Serialize,
			serde::Deserialize<'static>,
			Default,
			PartialEq,
			Eq,
			std::hash::Hash,
		);
		assert_impl_all!(zbus::Message: TryFrom<$type>);
	};
}
