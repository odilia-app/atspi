#[macro_export]

/// Expands to a conversion given the enclosed event type and outer `Event` variant.
///
/// eg
/// ```ignore
/// impl_from_interface_event_enum_for_event!(ObjectEvents, Event::Object);
/// ```
/// expands to:
///
/// ```ignore
/// impl From<ObjectEvents> for Event {
///     fn from(event_variant: ObjectEvents) -> Event {
///         Event::Object(event_variant.into())
///     }
/// }
/// ```
macro_rules! impl_from_interface_event_enum_for_event {
	($outer_type:ty, $outer_variant:path) => {
		impl From<$outer_type> for Event {
			fn from(event_variant: $outer_type) -> Event {
				$outer_variant(event_variant.into())
			}
		}
	};
}

/// Expands to a conversion given the enclosed event enum type and outer `Event` variant.
///
/// eg
/// ```ignore
/// impl_try_from_event_for_user_facing_event_type!(ObjectEvents, Event::Object);
/// ```
/// expands to:
///
/// ```ignore
/// impl TryFrom<Event> for ObjectEvents {
///     type Error = AtspiError;
///     fn try_from(generic_event: Event) -> Result<ObjectEvents, Self::Error> {
///         if let Event::Object(event_type) = generic_event {
///             Ok(event_type)
///         } else {
///             Err(AtspiError::Conversion("Invalid type"))
///         }
///     }
/// }
/// ```
macro_rules! impl_try_from_event_for_user_facing_event_type {
	($outer_type:ty, $outer_variant:path) => {
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
}

/// Expands to a conversion given the user facing event type and outer `Event::Interface(<InterfaceEnum>)` variant.,
/// the enum type and outtermost variant.
///
/// ```ignore                                            user facing type,  enum type,    outer variant
/// impl_from_user_facing_event_for_interface_event_enum!(StateChangedEvent, ObjectEvents, ObjectEvents::StateChanged);
/// ```
///
/// expands to:
///
/// ```ignore
/// impl From<StateChangedEvent> for ObjectEvents {
///     fn from(specific_event: StateChangedEvent) -> ObjectEvents {
///         ObjectEvents::StateChanged(specific_event)
///     }
/// }
/// ```
macro_rules! impl_from_user_facing_event_for_interface_event_enum {
	($inner_type:ty, $outer_type:ty, $inner_variant:path) => {
		impl From<$inner_type> for $outer_type {
			fn from(specific_event: $inner_type) -> $outer_type {
				$inner_variant(specific_event)
			}
		}
	};
}

/// Expands to a conversion given two arguments,
/// 1. the user facing event type `(inner_type)`
/// which relies on a conversion to its interface variant enum type variant.
/// 2. the outer `Event::<Interface(<InterfaceEnum>)>` wrapper.,
/// the enum type and outtermost variant.
///
/// ```ignore                                   user facing type, outer event variant
/// impl_from_user_facing_type_for_event_enum!(StateChangedEvent, Event::Object);
/// ```
///
/// expands to:
///
/// ```ignore
/// impl From<StateChangedEvent> for Event {
///    fn from(event_variant: StateChangedEvent) -> Event {
///       Event::Object(ObjectEvents::StateChanged(event_variant))
///   }
/// }
/// ```
macro_rules! impl_from_user_facing_type_for_event_enum {
	($inner_type:ty, $outer_variant:path) => {
		impl From<$inner_type> for Event {
			fn from(event_variant: $inner_type) -> Event {
				$outer_variant(event_variant.into())
			}
		}
	};
}

/// Expands to a conversion given two arguments,
/// 1. the user facing event type `(inner_type)`
/// 2. the outer `Event::<Interface(<InterfaceEnum>)>` wrapper.
///
/// eg
/// ```ignore
/// impl_try_from_event_for_user_facing_type!(StateChangedEvent, ObjectEvents::StateChanged);
/// ```
/// expands to:
///
/// ```ignore
/// impl TryFrom<Event> for StateChangedEvent {
///    type Error = AtspiError;
///   fn try_from(generic_event: Event) -> Result<StateChangedEvent, Self::Error> {
///      if let Event::Object(ObjectEvents::StateChanged(specific_event)) = generic_event {
///          Ok(specific_event)
///         } else {
///          Err(AtspiError::Conversion("Invalid type"))
///         }
///   }
/// }
/// ```
macro_rules! impl_try_from_event_for_user_facing_type {
	($inner_type:ty, $inner_variant:path, $outer_variant:path) => {
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

/// Implements the `TryFrom` trait for a given event type.
/// Converts a user facing event type into a `zbus::Message`.
///
/// # Example
/// ```ignore
/// impl_to_dbus_message!(StateChangedEvent);
/// ```
/// expands to:
///
/// ```ignore
/// impl TryFrom<StateChangedEvent> for zbus::Message {
///   type Error = AtspiError;
///   fn try_from(event: StateChangedEvent) -> Result<Self, Self::Error> {
///     Ok(zbus::MessageBuilder::signal(
///         event.path(),
///         StateChangedEvent::DBUS_INTERFACE,
///         StateChangedEvent::DBUS_MEMBER,
///     )?
///     .sender(event.sender())?
///     .build(&event.body())?)
///  }
/// }
///
macro_rules! impl_to_dbus_message {
	($type:ty) => {
		#[cfg(feature = "zbus")]
		impl TryFrom<$type> for zbus::Message {
			type Error = AtspiError;
			fn try_from(event: $type) -> Result<Self, Self::Error> {
				Ok(zbus::MessageBuilder::signal(
					event.path(),
					<$type as GenericEvent>::DBUS_INTERFACE,
					<$type as GenericEvent>::DBUS_MEMBER,
				)?
				.sender(event.sender())?
				.build(&event.body())?)
			}
		}
	};
}

/// Implements the `TryFrom` trait for a given event type.
/// Converts a `zbus::Message` into a user facing event type.
///
/// # Example
/// ```ignore
/// impl_from_dbus_message!(StateChangedEvent);
/// ```
/// expands to:
///
/// ```ignore
/// impl TryFrom<&zbus::Message> for StateChangedEvent {
///   type Error = AtspiError;
///   fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
///    if msg.interface().ok_or(AtspiError::MissingInterface)? != StateChangedEvent::DBUS_INTERFACE {
///       return Err(AtspiError::InterfaceMatch(format!("The interface {} does not match the signal's interface: {}",
///         msg.interface().unwrap(),
///         StateChangedEvent::DBUS_INTERFACE)));
///     }
///     if msg.member().ok_or(AtspiError::MissingMember)? != StateChangedEvent::DBUS_MEMBER {
///       return Err(AtspiError::MemberMatch(format!("The member {} does not match the signal's member: {}",
///         msg.member().unwrap(),
///         StateChangedEvent::DBUS_MEMBER)));
///     }
///     StateChangedEvent::build(msg.try_into()?, msg.body::<StateChangedEvent::Body>()?)
///  }
/// }
/// ```
macro_rules! impl_from_dbus_message {
	($type:ty) => {
		#[cfg(feature = "zbus")]
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

/// Tests `Default` and `GenericEvent::build` for a given event struct.
///
/// Obtains a default for the given event struct.
/// Asserts that the path and sender are the default.
///
/// Breaks the struct down into item (the associated object) and body.
/// Then tests `GenericEvent::build` with the item and body.
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

/// Tests conversion to and from the `Event` enum.
///
/// Obtains a default for the given event struct.
/// Converts the struct into the `Event` enum, wrapping the struct.
/// Converts the `Event` enum into the given event struct.
/// Asserts that the original struct and the converted struct are equal.
#[cfg(test)]
macro_rules! event_enum_test_case {
	($type:ty) => {
		#[test]
		fn event_enum_conversion() {
			let struct_event = <$type>::default();
			let event = Event::from(struct_event.clone());
			let struct_event_back = <$type>::try_from(event)
				.expect("Could not convert from `Event` back to specific event type");
			assert_eq!(struct_event, struct_event_back);
		}
	};
}

#[cfg(test)]
macro_rules! zbus_message_test_case {
	($type:ty) => {
		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_to_specific_event_type() {
			let struct_event = <$type>::default();
			let msg: zbus::Message = zbus::Message::try_from(struct_event.clone())
				.expect("Could not convert event into a message");
			let struct_event_back =
				<$type>::try_from(&msg).expect("Could not convert message into an event");
			assert_eq!(struct_event, struct_event_back);
		}
		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_to_event_enum_type() {
			let struct_event = <$type>::default();
			let msg: zbus::Message = zbus::Message::try_from(struct_event.clone())
				.expect("Could not convert event into a message");
			let event_enum_back =
				Event::try_from(&msg).expect("Could not convert message into an event");
			let event_enum: Event = struct_event.into();
			assert_eq!(event_enum, event_enum_back);
		}
		// make want to consider parameterized tests here, no need for fuzz testing, but one level lower than that may be nice
		// try having a matching member, matching interface, path, or body type, but that has some other piece which is not right
		#[cfg(feature = "zbus")]
		#[test]
		#[should_panic(expected = "should panic")]
		fn zbus_msg_conversion_failure_fake_msg() -> () {
			let fake_msg = zbus::MessageBuilder::signal(
				"/org/a11y/sixtynine/fourtwenty",
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
		#[cfg(feature = "zbus")]
		#[test]
		#[should_panic(expected = "should panic")]
		fn zbus_msg_conversion_failure_correct_interface() -> () {
			let fake_msg = zbus::MessageBuilder::signal(
				"/org/a11y/sixtynine/fourtwenty",
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
		#[cfg(feature = "zbus")]
		#[test]
		#[should_panic(expected = "should panic")]
		fn zbus_msg_conversion_failure_correct_interface_and_member() -> () {
			let fake_msg = zbus::MessageBuilder::signal(
				"/org/a11y/sixtynine/fourtwenty",
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
		#[cfg(feature = "zbus")]
		#[test]
		#[should_panic(expected = "should panic")]
		fn zbus_msg_conversion_failure_correct_body() -> () {
			let fake_msg = zbus::MessageBuilder::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.accessible.technically.valid",
				"FakeMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$type>::default().body())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panic! Invalid event.");
		}
		#[cfg(feature = "zbus")]
		#[test]
		#[should_panic(expected = "should panic")]
		fn zbus_msg_conversion_failure_correct_body_and_member() -> () {
			let fake_msg = zbus::MessageBuilder::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.accessible.technically.valid",
				<$type as GenericEvent>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$type>::default().body())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
			event.expect("This should panic! Invalid event.");
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
			#[cfg(feature = "zbus")]
			#[test]
			#[should_panic(expected = "should panic")]
			fn zbus_msg_invalid_interface() {
				let fake_msg = zbus::MessageBuilder::signal(
					"/org/a11y/sixtynine/fourtwenty",
					"org.a11y.atspi.technically.valid.lol",
					<$any_subtype as GenericEvent>::DBUS_MEMBER,
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$any_subtype>::default().body())
				.unwrap();
				let mod_type = <$type>::try_from(&fake_msg);
				mod_type.expect(
					"This should panic! Could not convert message into a event wrapper type",
				);
			}
			#[cfg(feature = "zbus")]
			#[test]
			#[should_panic(expected = "should panic")]
			fn zbus_msg_invalid_member() {
				let fake_msg = zbus::MessageBuilder::signal(
					"/org/a11y/sixtynine/fourtwenty",
					<$any_subtype as GenericEvent>::DBUS_INTERFACE,
					"FakeFunctionLol",
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$any_subtype>::default().body())
				.unwrap();
				let mod_type = <$type>::try_from(&fake_msg);
				mod_type.expect(
					"This should panic! Could not convert message into a event wrapper type",
				);
			}
			#[cfg(feature = "zbus")]
			#[test]
			#[should_panic(expected = "should panic")]
			fn zbus_msg_invalid_member_and_interface() {
				let fake_msg = zbus::MessageBuilder::signal(
					"/org/a11y/sixtynine/fourtwenty",
					"org.a11y.atspi.technically.allowed",
					"FakeFunctionLol",
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$any_subtype>::default().body())
				.unwrap();
				let mod_type = <$type>::try_from(&fake_msg);
				mod_type.expect(
					"This should panic! Could not convert message into a event wrapper type",
				);
			}
			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_conversion() {
				let valid_msg = zbus::MessageBuilder::signal(
					"/org/a11y/sixtynine/fourtwenty",
					<$any_subtype as GenericEvent>::DBUS_INTERFACE,
					<$any_subtype as GenericEvent>::DBUS_MEMBER,
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$any_subtype>::default().body())
				.unwrap();
				let mod_type = <$type>::try_from(&valid_msg);
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
		#[cfg(feature = "zbus")]
		assert_impl_all!(zbus::Message: TryFrom<$type>);
	};
}

/// Asserts that the signatures are equal, but ignores the outer parentheses as
/// the difference between marshalled and unmarshalled signatures is often just one set of outer parentheses.
#[macro_export]
macro_rules! assert_eq_signatures {
	($lhs_sig:expr, $rhs_sig:expr) => {
		assert!(
			signatures_are_eq($lhs_sig, $rhs_sig),
			"Signatures are not equal (Lhs: {}, Rhs: {})",
			$lhs_sig,
			$rhs_sig
		);
	};
}
