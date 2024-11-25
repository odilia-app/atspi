/// Expands to implement the required methods for the [`crate::EventProperties`] trait.
/// This depends on the struct to have an `item` field of type [`crate::ObjectRef`].
///
/// ```ignore
/// impl_from_interface_event_enum_for_event!(TextCaretMovedEvent);
/// ```
///
/// Expands to:
///
/// ```ignore
/// impl EventProperties for TextCaretMovedEvent {
///   fn sender(&self) -> UniqueName<'_> {
///     self.item.name.as_ref()
///   }
///   fn path(&self) -> ObjectPath<'_> {
///     self.item.path.as_ref()
///   }
/// }
/// ```
macro_rules! impl_event_properties {
	($type:ty) => {
		impl EventProperties for $type {
			fn sender(&self) -> UniqueName<'_> {
				self.item.name.as_ref()
			}
			fn path(&self) -> ObjectPath<'_> {
				self.item.path.as_ref()
			}
		}
	};
}

/// Expands to implement From for [`crate::ObjectRef`].
/// This depends on the struct to have an `item` field of type [`crate::ObjectRef`].
///
/// ```ignore
/// impl_from_object_ref!(TextCaretMovedEvent);
/// ```
///
/// Exapnds to:
///
/// ```ignore
/// impl From<ObjectRef> for TextCaretMovedItem {
///     fn from(obj_ref: ObjectRef) -> Self {
///         Self { item: obj_ref }
///     }
/// }
/// ```
macro_rules! impl_from_object_ref {
	($type:ty) => {
		impl From<crate::ObjectRef> for $type {
			fn from(obj_ref: crate::ObjectRef) -> Self {
				Self { item: obj_ref }
			}
		}
	};
}

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
///    which relies on a conversion to its interface variant enum type variant.
/// 2. the outer `Event::<Interface(<InterfaceEnum>)>` wrapper.,
///    the enum type and outtermost variant.
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
///     Ok(zbus::Message::signal(
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
				Ok(zbus::Message::signal(
					event.path(),
					<$type as BusProperties>::DBUS_INTERFACE,
					<$type as BusProperties>::DBUS_MEMBER,
				)?
				.sender(event.sender().to_string())?
				.build(&event.body())?)
			}
		}
	};
}

/// Implements the `TryFrom` trait for a given event type.
/// Converts a `zbus::Message` into a user facing event type.
///
/// See [`crate::events::MessageConversion`] for details on implementation.
///
/// # Example
/// ```ignore
/// impl_from_dbus_message!(StateChangedEvent);
/// ```
/// expands to:
///
/// ```ignore
/// impl TryFrom<&zbus::Message> for StateChangedEvents {
///   type Error = AtspiError;
///   fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
///     <$type as MessageConversion>::try_from_message(msg)
///   }
/// }
/// ```
///
/// There is also a variant that can be used for events whose [`crate::events::MessageConversion::Body`] is not
/// [`crate::events::EventBodyOwned`]. You can call this by setting the second parameter to `Explicit`.
macro_rules! impl_from_dbus_message {
	($type:ty) => {
		impl_from_dbus_message!($type, Auto);
	};
	($type:ty, Auto) => {
		#[cfg(feature = "zbus")]
		impl TryFrom<zbus::Message> for $type {
			type Error = AtspiError;
			fn try_from(msg: zbus::Message) -> Result<Self, Self::Error> {
        use zvariant::Type;

        Self::validate_interface(&msg)?;
        Self::validate_member(&msg)?;

        let body = msg.body();
        let body_signature = body.signature();
        let deser_body: <Self as MessageConversion>::Body = if body_signature == crate::events::QSPI_EVENT_SIGNATURE {
            let qtbody: crate::events::EventBodyQT = body.deserialize_unchecked()?;
            qtbody.into()
        } else if body_signature == crate::events::ATSPI_EVENT_SIGNATURE {
            body.deserialize_unchecked()?
        } else {
          return Err(AtspiError::SignatureMatch(format!(
            "The message signature {} does not match the signal's body signature: {}",
            body_signature,
            <Self as MessageConversion>::Body::SIGNATURE,
          )));
        };
        let name = msg.sender().ok_or(AtspiError::MissingName)?;
        // TODO: MissingName
        let path = msg.path().ok_or(AtspiError::MissingName)?;
        let item = crate::ObjectRef::new(name, path);
        Self::from_message_unchecked_parts(item, deser_body)
      }
    }
	};
	($type:ty, Explicit) => {
		#[cfg(feature = "zbus")]
		impl TryFrom<zbus::Message> for $type {
			type Error = AtspiError;
			fn try_from(msg: zbus::Message) -> Result<Self, Self::Error> {
				<$type as MessageConversionExt<<$type as MessageConversion>::Body>>::try_from_message(msg)
			}
		}
	};
}

// We decorate the macro with a `#[cfg(test)]` attribute.
// This prevents Clippy from complaining about the macro not being used.
// It is being used, but only in test mode.
//
/// Tests `Default` and `BusProperties::from_message_unchecked` for a given event struct.
///
/// Obtains a default for the given event struct.
/// Asserts that the path and sender are the default.
///
/// Breaks the struct down into item (the associated object) and body.
/// Then tests `BusProperties::from_message_unchecked` with the item and body.
#[cfg(test)]
macro_rules! generic_event_test_case {
	($type:ty) => {
		#[test]
		fn generic_event_uses() {
			let struct_event = <$type>::default();
			assert_eq!(struct_event.path().as_str(), "/org/a11y/atspi/accessible/null");
			assert_eq!(struct_event.sender().as_str(), ":0.0");
			let body = struct_event.body();
			let body2 = Message::method_call(
				struct_event.path().as_str(),
				<$type as BusProperties>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(struct_event.sender().as_str())
			.unwrap()
			.build(&(body,))
			.unwrap();
			let build_struct = <$type>::from_message_unchecked(body2)
				.expect("<$type as Default>'s parts should build a valid ObjectRef");
			assert_eq!(struct_event, build_struct);
		}
	};
}

// We decorate the macro with a `#[cfg(test)]` attribute.
// This prevents Clippy from complaining about the macro not being used.
// It is being used, but only in test mode.
//
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
				.expect("Should convert event enum into specific event type because it was created from it. Check the `impl_from_interface_event_enum_for_event` macro");
			assert_eq!(struct_event, struct_event_back);
		}
	};
}

/// Tests transparency of the `EventTypeProperties` and `EventProperties` trait on the `Event` wrapper type.
///
/// Obtains a default for the given event struct.
/// Converts the struct into the `Event` enum, wrapping the struct.
/// Checks the equality of all four functions defined in the `EventTypeProiperties` and `EventProperties` traits:
///
/// - `member`
/// - `interface`
/// - `registry_string`
/// - `match_rule`
/// - `path`
/// - `sender`
///
/// It is imperitive that these items come through with no modifications from the wrappers.
///
#[cfg(test)]
macro_rules! event_enum_transparency_test_case {
	($type:ty) => {
		#[test]
		fn event_enum_transparency_test_case() {
			let specific_event = <$type>::default();
			let generic_event = Event::from(specific_event.clone());
			assert_eq!(
				specific_event.member(),
				generic_event.member(),
				"DBus member strings do not match."
			);
			assert_eq!(
				specific_event.interface(),
				generic_event.interface(),
				"Registry interfaces do not match."
			);
			assert_eq!(
				specific_event.registry_string(),
				generic_event.registry_string(),
				"Registry strings do not match."
			);
			assert_eq!(
				specific_event.match_rule(),
				generic_event.match_rule(),
				"Match rule strings do not match."
			);
			assert_eq!(specific_event.path(), generic_event.path(), "Pathsdo not match.");
			assert_eq!(specific_event.sender(), generic_event.sender(), "Senders do not match.");
		}
	};
}

#[cfg(test)]
macro_rules! zbus_message_qtspi_test_case {
    ($type:ty, Auto) => {
      #[cfg(feature = "zbus")]
     #[test]
    fn zbus_message_conversion_qtspi() {
      // in the case that the body type is EventBodyOwned, we need to also check successful
      // conversion from a QSPI-style body.
        let ev = <$type>::default();
          let qt: crate::events::EventBodyQT = ev.body().into();
          let msg = zbus::Message::signal(
            ev.path(),
            ev.interface(),
            ev.member(),
          )
          .unwrap()
          .sender(":0.0")
          .unwrap()
          .build(&(qt,))
          .unwrap();
          <$type>::try_from(msg).expect("Should be able to use an EventBodyQT for any type whose BusProperties::Body = EventBodyOwned");
        }
      #[cfg(feature = "zbus")]
     #[test]
    fn zbus_message_conversion_qtspi_event_enum() {
      // in the case that the body type is EventBodyOwned, we need to also check successful
      // conversion from a QSPI-style body.
        let ev = <$type>::default();
          let qt: crate::events::EventBodyQT = ev.body().into();
          let msg = zbus::Message::signal(
            ev.path(),
            ev.interface(),
            ev.member(),
          )
          .unwrap()
          .sender(":0.0")
          .unwrap()
          .build(&(qt,))
          .unwrap();
          assert_matches!(Event::try_from(msg), Ok(_));
        }
    };
    ($type:ty, Explicit) => {};
}

// We decorate the macro with a `#[cfg(test)]` attribute.
// This prevents Clippy from complaining about the macro not being used.
// It is being used, but only in test mode.
//
/// As of writing, this macro is expanded only once: in the `event_test_cases!` macro.
#[cfg(test)]
macro_rules! zbus_message_test_case {
  ($type:ty) => {
      zbus_message_test_case!($type, Auto);
    };
	($type:ty, $extra:tt) => {
    zbus_message_qtspi_test_case!($type, $extra);
		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_to_specific_event_type() {
			let struct_event = <$type>::default();
			let msg: zbus::Message = zbus::Message::try_from(struct_event.clone())
				.expect("Should convert a `$type::default()` into a message. Check the `impl_to_dbus_message` macro .");
			let struct_event_back =
				<$type>::try_from(msg).expect("Should convert from `$type::default()` originated `Message` back into a specific event type. Check the `impl_from_dbus_message` macro.");
        assert_eq!(struct_event, struct_event_back, "Events converted into a message and back must be the same");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_to_event_enum_type() {
			let struct_event = <$type>::default();
			let msg: zbus::Message = zbus::Message::try_from(struct_event.clone()).expect("Should convert a `$type::default()` into a message. Check the `impl_to_dbus_message` macro .");
			let event_enum_back =
				Event::try_from(msg).expect("Should convert a from `$type::default()` built `Message` into an event enum. Check the `impl_from_dbus_message` macro .");
			let event_enum: Event = struct_event.into();
			assert_eq!(event_enum, event_enum_back);
		}
		// make want to consider parameterized tests here, no need for fuzz testing, but one level lower than that may be nice
		// try having a matching member, matching interface, path, or body type, but that has some other piece which is not right
		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_fake_msg() -> () {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.technically.valid",
				"MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&())
			.unwrap();
			let event = <$type>::try_from(fake_msg);
      assert_matches!(event, Err(_), "This conversion should fail");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_validated_message_with_body() -> () {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.technically.valid",
				"MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$type>::default().body())
			.unwrap();
			let event = <$type>::from_message_unchecked(fake_msg);
      event.expect("The from_message_unchecked function should work, despite mismatching interface and member");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_interface() -> () {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				<$type as BusProperties>::DBUS_INTERFACE,
				"MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&())
			.unwrap();
			let event = <$type>::try_from(fake_msg);
      assert_matches!(event, Err(AtspiError::MemberMatch(_)), "Wrong kind of error");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_interface_and_member() -> () {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				<$type as BusProperties>::DBUS_INTERFACE,
				<$type as BusProperties>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&())
			.unwrap();
			let event = <$type>::try_from(fake_msg);
      assert_matches!(event, Err(AtspiError::SignatureMatch(_)), "Wrong kind of error");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_interface_and_member_invalid_body() -> () {
      // known invalid body for AT-SPI events
      let invalid_body: (i32, u64, String, String) = (0, 0, String::new(), String::new());
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				<$type as BusProperties>::DBUS_INTERFACE,
				<$type as BusProperties>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&invalid_body)
			.unwrap();
			let event = <$type>::try_from(fake_msg);
      assert_matches!(event, Err(AtspiError::SignatureMatch(_)), "Wrong kind of error");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_body() -> () {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.accessible.technically.valid",
				"FakeMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$type>::default().body())
			.unwrap();
			let event = <$type>::try_from(fake_msg);
      assert_matches!(event, Err(_));
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_body_and_member() -> () {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.accessible.technically.valid",
				<$type as BusProperties>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$type>::default().body())
			.unwrap();
			let event = <$type>::try_from(fake_msg);
      assert_matches!(event, Err(AtspiError::InterfaceMatch(_)), "Wrong kind of error");
		}
		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_body_and_interface() -> () {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				<$type as BusProperties>::DBUS_INTERFACE,
        "MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$type>::default().body())
			.unwrap();
			let event = <$type>::try_from(fake_msg);
      assert_matches!(event, Err(AtspiError::MemberMatch(_)), "Wrong kind of error");
		}
	};
}

/// Expands to five tests:
///
/// 1. `into_and_try_from_event`
/// 2. `zbus_msg_invalid_interface`
/// 3. `zbus_msg_invalid_member`
/// 4. `zbus_msg_invalid_member_and_interface`
/// 5. `zbus_msg_conversion`
///
/// # Examples
///
/// ```ignore
/// event_wrapper_test_cases!(MouseEvents, AbsEvent);
/// ```
/// In the macro, its first argument `$type` is the event enum type.  
/// The second argument `$any_subtype` is the event struct type.
///
/// For each of the types, the macro will create a module with the name `events_tests_{foo}`
/// where `{foo}` is the snake case of the 'interface enum' name.
macro_rules! event_wrapper_test_cases {
	($type:ty, $any_subtype:ty) => {
		#[cfg(test)]
		#[rename_item::rename(name($type), prefix = "events_tests_", case = "snake")]
		mod foo {
			use super::{$any_subtype, $type, AtspiError, Event, BusProperties, MessageConversion};
      // TODO: replace with [`std::assert_matches::assert_matches`] when stabailized
      use assert_matches::assert_matches;
			#[test]
			fn into_and_try_from_event() {
				// Create a default event struct from its type's `Default::default()` impl.
				let sub_type = <$any_subtype>::default();
				// Wrap the event struct in the event enum
				let mod_type = <$type>::from(sub_type);
				// Wrap the inner event enum into the `Event` enum.
				let event = Event::from(mod_type.clone());
				// Unwrap the `Event` enum into the inner event enum.
				let mod_type2 = <$type>::try_from(event.clone())
					.expect("Should convert outer `Event` enum into interface enum because it was created from it. Check the `impl_try_from_event_for_user_facing_event_type` macro");
				assert_eq!(
					mod_type, mod_type2,
					"Events were able to be parsed and encapsulated, but they have changed value"
				);
			}
			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_invalid_interface() {
				let fake_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					"org.a11y.atspi.technically.valid.lol",
					<$any_subtype as BusProperties>::DBUS_MEMBER,
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$any_subtype>::default().body())
				.unwrap();

				// It is hard to see what eventually is tested here. Let's unravel it:
				//
				// Below we call `TryFrom<&zbus::Message> for $type` where `$type` the interface enum name. (eg. `MouseEvents`, `ObjectEvents`, etc.) and
				// `mod_type` is an 'interface enum' variant (eg. `MouseEvents::Abs(AbsEvent)`).
				// This conversion is found in the `/src/events/{iface_name}.rs`` file.
				// This conversion in turn leans on the `impl_from_dbus_message` macro.
				// In `MouseEvents::Abs(msg.try_into()?)`, it is the `msg.try_into()?` that should fail.
				// The `msg.try_into()?` is provided through the `impl_from_dbus_message` macro.
        // Additioanlly, we check against the same method in `Event`; the overarchive enum that
        // contains all other events as variants.
				let mod_type = <$type>::try_from(fake_msg.clone());
				let event_type = Event::try_from(fake_msg);
        assert_matches!(mod_type, Err(AtspiError::InterfaceMatch(_)), "Wrong kind of error");
        assert_matches!(event_type, Err(AtspiError::InterfaceMatch(_)), "Wrong kind of error");
			}
			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_invalid_member() {
				let fake_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					<$any_subtype as BusProperties>::DBUS_INTERFACE,
					"FakeFunctionLol",
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$any_subtype>::default().body())
				.unwrap();
				// As above, the `msg.try_into()?` is provided through the `impl_from_dbus_message` macro.
				let mod_type = <$type>::try_from(fake_msg);
        assert_matches!(mod_type, Err(AtspiError::MemberMatch(_)), "Wrong kind of error");
			}
			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_invalid_member_and_interface() {
				let fake_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					"org.a11y.atspi.technically.allowed",
					"FakeFunctionLol",
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$any_subtype>::default().body())
				.unwrap();
				// As above, the `msg.try_into()?` is provided through the `impl_from_dbus_message` macro.
				let mod_type = <$type>::try_from(fake_msg);

				// Note that the non-matching interface is the first error, so the member match error is not reached.
        assert_matches!(mod_type, Err(AtspiError::InterfaceMatch(_)), "Wrong kind of error");
			}
			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_conversion() {
				let valid_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					<$any_subtype as BusProperties>::DBUS_INTERFACE,
					<$any_subtype as BusProperties>::DBUS_MEMBER,
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$any_subtype>::default().body())
				.unwrap();
				// As above, the `msg.try_into()?` is provided through the `impl_from_dbus_message` macro.
				let mod_type = <$type>::try_from(valid_msg);
				mod_type.expect("Should convert from `$any_subtype::default()` built `Message` back into a interface event enum variant wrapping an inner type. Check the `impl_from_dbus_message` macro.");
			}
		}
	};
}

macro_rules! event_test_cases {
  ($type:ty) => {
      event_test_cases!($type, Auto);
  };
	($type:ty, $qt:tt) => {
		#[cfg(test)]
		#[rename_item::rename(name($type), prefix = "event_tests_", case = "snake")]
		mod foo {
			use super::{$type, AtspiError, Event, BusProperties, MessageConversion, EventProperties, EventTypeProperties};
      use zbus::Message;
      // TODO: use [`std::assert_matches::assert_matches`] when stabalized
      use assert_matches::assert_matches;

			generic_event_test_case!($type);
			event_enum_test_case!($type);
			zbus_message_test_case!($type, $qt);
			event_enum_transparency_test_case!($type);
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
			crate::EventProperties,
			crate::EventTypeProperties,
			crate::BusProperties,
		);
		#[cfg(feature = "zbus")]
		assert_impl_all!(zbus::Message: TryFrom<$type>);
	};
}
