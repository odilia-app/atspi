/// Expands to implement the required methods for the [`crate::EventProperties`] trait.
/// This depends on the struct to have an `item` field of type [`crate::ObjectRefOwned`].
///
/// # Example
/// ```ignore
/// impl_event_properties!(TextCaretMovedEvent);
/// ```
///
/// expands to:
///
/// ```ignore
/// impl EventProperties for TextCaretMovedEvent {
///     fn sender(&self) -> UniqueName<'_> {
///         self.item.name().as_ref()
///     }
///     fn path(&self) -> ObjectPath<'_> {
///         self.item.path().as_ref()
///     }
/// }
/// ```
macro_rules! impl_event_properties {
	($type:ty) => {
		impl crate::EventProperties for $type {
			fn sender(&self) -> zbus_names::UniqueName<'_> {
				self.item.name().as_ref()
			}

			fn path(&self) -> zvariant::ObjectPath<'_> {
				self.item.path().as_ref()
			}
		}
	};
}

/// Expands to implement `From` for [`crate::NonNullObjectRef<'o>`] for a given event type.
/// This macro is only applicable to event types without extra body data.
///
/// For types with a lifetime, it preserves the borrow.
/// For owned types, it automatically calls `.into_owned()`.
///
/// # Example
/// ```ignore
/// // owned:
/// impl_from_object_ref!(TextAttributesChangedEvent);
///
/// // borrowed:
/// impl_from_object_ref!(AddAccessibleEvent<'_>);
/// ```
///
/// expands to:
///
/// ```ignore
/// // owned
/// impl From<crate::NonNullObjectRef<'_>> for TextAttributesChangedEvent {
///     fn from(item: crate::NonNullObjectRef<'_>) -> Self {
///         Self { item: item.into_owned() }
///     }
/// }
///
/// // borrowed
/// impl<'o> From<crate::NonNullObjectRef<'o>> for AddAccessibleEvent<'o> {
///     fn from(item: crate::NonNullObjectRef<'o>) -> Self {
///         Self { item }
///     }
/// }
/// ```
macro_rules! impl_from_object_ref {
	// Pattern for types with a lifetime parameter: LoadCompleteEvent<'_>
	// The trick is to match `$target` as identifier with or without a lifetime parameter, then use `Self: 'o` to bind `Self` to `'o`
	($target:ident<'_>) => {
		impl<'o> From<crate::NonNullObjectRef<'o>> for $target<'o>
		where
			Self: 'o, // `Self` lifetime bound
		{
			fn from(item: crate::NonNullObjectRef<'o>) -> Self {
				Self { item }
			}
		}
	};

	// Variant for owned types (no lifetime parameter): PropertyChangeEvent
	($target:ident) => {
		impl From<crate::NonNullObjectRef<'_>> for $target {
			fn from(item: crate::NonNullObjectRef<'_>) -> Self {
				Self { item: item.into_owned() }
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Implements `From<InterfaceEvents<'a>>` for the global `Event<'a>` enum.
///
/// This macro handles the conversion from interface-specific enums (like `ObjectEvents<'a>`)
/// to the global `Event<'a>` enum, ensuring lifetimes are correctly unified.
///
/// # Examples
///
/// ```ignore
/// // For an interface enum with a lifetime:
/// impl_from_interface_event_enum_for_event!(ObjectEvents<'_>, Event::Object);
///
/// // For an owned interface enum (no lifetime):
/// impl_from_interface_event_enum_for_event!(FocusEvents, Event::Focus);
/// ```
macro_rules! impl_from_interface_event_enum_for_event {
	// Pattern for interface enums with a lifetime parameter (e.g., ObjectEvents<'_>)
	($outer_type:ident<'_>, $outer_variant:path) => {
		#[cfg(feature = "wrappers")]
		impl<'a> From<$outer_type<'a>> for Event<'a> {
			fn from(event_variant: $outer_type<'a>) -> Self {
				$outer_variant(event_variant)
			}
		}
	};

	// Pattern for interface enums that DO have a lifetime, but where we didn't use the <'_> marker
	($outer_type:ident, $outer_variant:path) => {
		#[cfg(feature = "wrappers")]
		impl<'a> From<$outer_type<'a>> for Event<'a> {
			fn from(event_variant: $outer_type<'a>) -> Event<'a> {
				$outer_variant(event_variant)
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Implements `TryFrom<Event>` for a specific interface event enum.
///
/// This macro simplifies the conversion from the global `Event` enum to an
/// interface-specific enum (like `ObjectEvents` or `WindowEvents`).
///
/// It supports both interface enums with lifetimes (preserving a borrow on
/// `zbus::Message`) and owned interface enums.
///
/// # Examples
///
/// Note that the macro matches on the anonymous lifetime parameter (`'_`),
/// this means at call site, `FooEvent<'_>`  is required.
///
/// ```ignore
/// // For an interface enum with a lifetime:
/// impl_try_from_event_for_interface_enum!(ObjectEvents<'_>, Event::Object);
///
/// // For an owned interface enum:
/// impl_try_from_event_for_interface_enum!(FocusEvents, Event::Focus);
/// ```
macro_rules! impl_try_from_event_for_interface_enum {
	// Pattern for interface enums with a lifetime (e.g., ObjectEvents<'_>)
	($outer_type:ident<'_>, $outer_variant:path) => {
		impl<'a> TryFrom<Event<'a>> for $outer_type<'a> {
			type Error = AtspiError;
			fn try_from(generic_event: Event<'a>) -> Result<Self, Self::Error> {
				if let $outer_variant(event_type) = generic_event {
					Ok(event_type)
				} else {
					Err(AtspiError::Conversion("Invalid type"))
				}
			}
		}
	};

	// Pattern for owned interface enums (no lifetime parameter)
	($outer_type:ident, $outer_variant:path) => {
		// Voor owned types matchen we op de 'static variant van Event
		impl TryFrom<Event<'static>> for $outer_type {
			type Error = AtspiError;
			fn try_from(generic_event: Event<'static>) -> Result<Self, Self::Error> {
				if let $outer_variant(event_type) = generic_event {
					Ok(event_type)
				} else {
					Err(AtspiError::Conversion("Invalid type"))
				}
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Implements `From<SpecificEvent<'a>>` for an interface event enum (e.g., `ObjectEvents<'a>`).
///
/// This macro handles the first step of the conversion hierarchy, wrapping a
/// specific event into its corresponding interface-level enum.
///
/// # Example
///
/// ```ignore
/// impl_from_user_facing_event_for_interface_event_enum!(
///     StateChangedEvent<'_>,
///     ObjectEvents<'_>,
///     ObjectEvents::StateChanged
/// );
/// ```
macro_rules! impl_from_user_facing_event_for_interface_event_enum {
	// Pattern for types with a lifetime (zero-copy)
	($inner_type:ident<'_>, $outer_type:ident<'_>, $inner_variant:path) => {
		impl<'a> From<$inner_type<'a>> for $outer_type<'a> {
			fn from(specific_event: $inner_type<'a>) -> Self {
				$inner_variant(specific_event)
			}
		}
	};

	// Pattern for owned types (no lifetime)
	($inner_type:ident, $outer_type:ident, $inner_variant:path) => {
		impl From<$inner_type> for $outer_type {
			fn from(specific_event: $inner_type) -> Self {
				$inner_variant(specific_event)
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Implements `From<SpecificEvent>` for the global `Event` enum.
///
/// This macro handles the two-step conversion from a specific event type
/// (e.g., `StateChangedEvent`) through its interface enum (e.g., `ObjectEvents`)
/// to the global `Event` enum.
///
/// It supports both events with lifetimes and owned events, ensuring that
/// any borrowed data remains valid throughout the conversion.
///
/// # Examples
///
/// ```ignore
/// // For an event with a lifetime (preserving the borrow):
/// impl_from_user_facing_type_for_event_enum!(StateChangedEvent<'_>, Event::Object);
///
/// // For an owned event:
/// impl_from_user_facing_type_for_event_enum!(FocusEvent, Event::Focus);
/// ```
///
/// # Technical Note
/// For types with a lifetime parameter, this macro unifies the lifetime of
/// the specific event with that of the global `Event<'a>` enum. This ensures
/// that any borrow from the underlying D-Bus message is preserved throughout
/// the entire conversion hierarchy.
macro_rules! impl_from_user_facing_type_for_event_enum {
	// Pattern for types with a lifetime parameter: StateChangedEvent<'_>
	($inner_type:ident<'_>, $outer_variant:path) => {
		#[cfg(feature = "wrappers")]
		impl<'a> From<$inner_type<'a>> for Event<'a> {
			fn from(event_variant: $inner_type<'a>) -> Self {
				$outer_variant(event_variant.into())
			}
		}
	};

	// Pattern for owned types (no lifetime parameter)
	($inner_type:ident, $outer_variant:path) => {
		#[cfg(feature = "wrappers")]
		impl<'a> From<$inner_type<'_>> for Event<'_> {
			fn from(event_variant: $inner_type) -> Self {
				$outer_variant(event_variant.into())
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Expands to a `TryFrom<Event> for T` where T is the user facing type.
/// The macro takes three arguments:
///
/// 1. The user facing type.
/// 2. The inner variant of the user facing type.
/// 3. The outer variant of the `Event` enum.
///
/// ```ignore
/// impl_try_from_event_for_user_facing_type!(
///     LoadCompleteEvent<'_>,
///     Document::LoadComplete,
///     Event::Document
/// );
/// ```
/// expands to:
///
/// ```ignore
/// impl TryFrom<Event> for StateChangedEvent {
///     type Error = AtspiError;
///     fn try_from(generic_event: Event) -> Result<StateChangedEvent, Self::Error> {
///         if let Event::Object(ObjectEvents::StateChanged(specific_event)) = generic_event {
///             Ok(specific_event)
///         } else {
///             Err(AtspiError::Conversion("Invalid type"))
///         }
///     }
/// }
/// ```
macro_rules! impl_try_from_event_for_user_facing_type {
	// For types with a lifetime (ie. LoadCompleteEvent<'_>)
	($inner_type:ident<'_>, $inner_variant:path, $outer_variant:path) => {
		#[cfg(feature = "wrappers")]
		impl<'a> TryFrom<crate::Event<'a>> for $inner_type<'a> {
			type Error = AtspiError;
			fn try_from(generic_event: crate::Event<'a>) -> Result<Self, Self::Error> {
				if let $outer_variant($inner_variant(specific_event)) = generic_event {
					// We moeten er hier wel op vertrouwen dat de data in Event
					// compatibel is met de gevraagde lifetime 'a.
					// Meestal is de data in Event 'static, wat een subtype is van 'a.
					Ok(specific_event)
				} else {
					Err(AtspiError::Conversion("Invalid type"))
				}
			}
		}
	};

	// Owned types pattern.
	($inner_type:ident, $inner_variant:path, $outer_variant:path) => {
		#[cfg(feature = "wrappers")]
		impl TryFrom<crate::Event> for $inner_type {
			type Error = AtspiError;
			fn try_from(generic_event: crate::Event) -> Result<Self, Self::Error> {
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
				use crate::events::{DBusInterface, DBusMember, MessageConversion};
				Ok(zbus::Message::signal(
					event.path(),
					<$type as DBusInterface>::DBUS_INTERFACE,
					<$type as DBusMember>::DBUS_MEMBER,
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
///    let hdr = msg.header();
///     <$type as MessageConversion>::try_from_message(msg, hdr)
///   }
/// }
/// ```
///
/// There is also a variant that can be used for events whose [`crate::events::MessageConversion::Body`] is not
/// [`crate::events::event_body::EventBodyOwned`]. You can call this by setting the second parameter to `Explicit`.
macro_rules! impl_from_dbus_message {
	($type:ty) => {
		impl_from_dbus_message!($type, Auto);
	};

	($type:ty, Auto) => {
		#[cfg(feature = "zbus")]
		impl<'msg> TryFrom<&'msg zbus::Message> for $type {
			type Error = AtspiError;
			fn try_from(msg: &'msg zbus::Message) -> Result<Self, Self::Error> {
				use crate::events::traits::{MessageConversion, MessageConversionExt};
				use crate::events::{EventBody, EventBodyQtBorrowed};
				use crate::NonNullObjectRef;
				use zvariant::Type;

				let hdr = msg.header();
				<Self as MessageConversionExt<'_,<Self as MessageConversion<'_>>::Body<'_>>>::validate_interface(&hdr)?;
				<Self as MessageConversionExt<'_,<Self as MessageConversion<'_>>::Body<'_>>>::validate_member(&hdr)?;
				let item = NonNullObjectRef::try_from(&hdr)?.into_owned();

				let dbus_body = msg.body();
				let signature = dbus_body.signature();

				if signature == EventBody::SIGNATURE || signature == EventBodyQtBorrowed::SIGNATURE
				{
					Ok(Self::from_message_unchecked_parts(item, dbus_body)?)
				} else {
					Err(AtspiError::SignatureMatch(format!(
						"signature mismatch: expected: {}, signal body: {}",
						msg.body().signature(),
						<Self as MessageConversion<'_>>::Body::SIGNATURE,
					)))
				}
			}
		}
	};

	($type:ty, Explicit) => {
		#[cfg(feature = "zbus")]
		impl<'msg> TryFrom<&'msg zbus::Message> for $type {
			type Error = crate::AtspiError;
			fn try_from(msg: &'msg zbus::Message) -> Result<Self, Self::Error> {
				use crate::events::traits::{MessageConversion, MessageConversionExt};
				let hdr = msg.header();
				<Self as MessageConversionExt<'_,<Self as MessageConversion<'_>>::Body<'_>,>>::validate_interface(&hdr)?;
				<Self as MessageConversionExt<'_,<Self as MessageConversion<'_>>::Body<'_>,>>::validate_member(&hdr)?;
				<Self as MessageConversionExt<'_,<Self as MessageConversion<'_>>::Body<'_>,>>::validate_body(msg)?;

				let item = crate::NonNullObjectRef::try_from(&hdr)?.into_owned();
				let dbus_body = msg.body();
				Self::from_message_unchecked_parts(item, dbus_body)
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
/// A macro to generate a test case for generic event properties.
///
/// It verifies that an event can be deconstructed into its parts (path, sender, body)
/// and reconstructed from a message without losing information.
#[cfg(test)]
macro_rules! generic_event_test_case {
	($ufet:ty, [ $($field:ident),* ]) => {
		#[test]
		fn generic_event_uses() {
			use crate::events::traits::MessageConversion;
			use crate::object_ref::{NonNullObjectRef, TEST_OBJECT_BUS_NAME, TEST_OBJECT_PATH_STR};

			let test_origin = NonNullObjectRef::from_static_str_unchecked(
				TEST_OBJECT_BUS_NAME,
				TEST_OBJECT_PATH_STR,
			);

			let event_struct = <$ufet>::new_test_event(&test_origin);

			assert_eq!(event_struct.path().as_str(), TEST_OBJECT_PATH_STR);
			assert_eq!(event_struct.sender().as_str(), TEST_OBJECT_BUS_NAME);

			let body = event_struct.body();
			let body2 = zbus::Message::signal(
				event_struct.path(),
				<$ufet as crate::events::traits::DBusInterface>::DBUS_INTERFACE,
				<$ufet as crate::events::traits::DBusMember>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(event_struct.sender().as_str())
			.unwrap()
			.build(&(body,))
			.unwrap();

			let header = body2.header();
			let build_struct = <$ufet>::from_message_unchecked(&body2, &header)
				.expect("Should build a valid event from its own parts");
			assert_eq!(event_struct, build_struct);
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
/// Verifies that the `DBus` interface and member names match the XML definitions.
#[cfg(test)]
macro_rules! event_has_matching_xml_definition {
	($ufet:ty) => {
		#[test]
		fn event_has_matching_xml_definition() {
			use crate::events::traits::{DBusInterface, DBusMember};
			use zbus_xml;

			let fname = match <$ufet>::DBUS_INTERFACE.split('.').last().expect("Has last section") {
				"Cache" => "xml/Cache.xml",
				"Socket" => "xml/Socket.xml",
				"Registry" => "xml/Registry.xml",
				_ => "xml/Event.xml",
			};

			let reader = std::fs::File::open(fname).expect("Valid file path!");
			let xml = zbus_xml::Node::from_reader(reader).expect("Valid DBus XML file!");
			let interface = xml
				.interfaces()
				.iter()
				.find(|int| int.name() == <$ufet>::DBUS_INTERFACE)
				.unwrap_or_else(|| {
					let possible_names: Vec<String> = xml
						.interfaces()
						.iter()
						.map(|int| int.name().as_str().to_string())
						.collect();
					panic!(
						"{} has interface name {}, but it was not found in XML: {:?}",
						std::any::type_name::<$ufet>(),
						<$ufet>::DBUS_INTERFACE,
						possible_names
					);
				});

			let _signal = interface
				.signals()
				.iter()
				.find(|sig| sig.name() == <$ufet>::DBUS_MEMBER)
				.unwrap_or_else(|| {
					let possible_names: Vec<String> = interface
						.signals()
						.iter()
						.map(|sig| sig.name().as_str().to_string())
						.collect();
					panic!(
						"{} has member name {}, but it was not found in XML interface {}: {:?}",
						std::any::type_name::<$ufet>(),
						<$ufet>::DBUS_MEMBER,
						<$ufet>::DBUS_INTERFACE,
						possible_names
					);
				});
		}
	};
}

/// Generates test cases for QTSPI-specific body conversion.
#[cfg(test)]
macro_rules! zbus_message_qtspi_test_case {
	($type:ty, [ $($field:ident),* ], Auto) => {
		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_message_conversion_qtspi() {
		    use crate::events::traits::MessageConversion;
			use crate::object_ref::{NonNullObjectRef, TEST_OBJECT_BUS_NAME, TEST_OBJECT_PATH_STR};

			let test_origin = NonNullObjectRef::from_static_str_unchecked(
				TEST_OBJECT_BUS_NAME,
				TEST_OBJECT_PATH_STR,
			);

			let ev = <$type>::new_test_event(&test_origin);

			let qt: crate::events::EventBodyQtOwned = ev.body().into();
			let msg = zbus::Message::signal(ev.path(), ev.interface(), ev.member())
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&(qt,))
				.unwrap();

			<$type>::try_from(&msg).expect("Should be able to use an EventBodyQtOwned for any type whose body is EventBodyOwned");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_message_conversion_qtspi_event_enum() {
			use crate::events::traits::MessageConversion;
			use crate::object_ref::{NonNullObjectRef, TEST_OBJECT_BUS_NAME, TEST_OBJECT_PATH_STR};
			use crate::Event;

			let test_origin = NonNullObjectRef::from_static_str_unchecked(
				TEST_OBJECT_BUS_NAME,
				TEST_OBJECT_PATH_STR,
			);

			let ev = <$type>::new_test_event(&test_origin);

			let qt: crate::events::EventBodyQtOwned = ev.body().into();
			let msg = zbus::Message::signal(ev.path(), ev.interface(), ev.member())
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&(qt,))
				.unwrap();

			assert_matches::assert_matches!(Event::try_from(&msg), Ok(_));
		}
	};

	// This macro generates tests for events with a generic `Qspi` body types,
	// `Explicit` refers to bespoke body types,not the generic `EventBody[Qt][Owned]`.
	// That is why this pattern does yield code.
	($type:ty, [ $($field:ident),* ], Explicit) => {};
}

/// Generates test cases for zbus Message conversion.
#[cfg(test)]
macro_rules! zbus_message_test_case {
	($type:ty, [ $($field:ident),* ]) => {
		zbus_message_test_case!($type, [ $($field),* ], Auto);
	};
	($type:ty, [ $($field:ident),* ], $extra:tt) => {
		zbus_message_qtspi_test_case!($type, [ $($field),* ], $extra);

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_to_specific_event_type() {
			use crate::object_ref::{NonNullObjectRef, TEST_OBJECT_BUS_NAME, TEST_OBJECT_PATH_STR};
			let test_origin = NonNullObjectRef::from_static_str_unchecked(
				TEST_OBJECT_BUS_NAME,
				TEST_OBJECT_PATH_STR,
			);

			let struct_event = <$type>::new_test_event(&test_origin);

			let msg = zbus::Message::try_from(struct_event.clone())
				.expect("Should convert event into a message");

			let struct_event_back = <$type>::try_from(&msg)
				.expect("Should convert message back into specific event type");
			assert_eq!(struct_event, struct_event_back);
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_to_event_enum_type() {
			use crate::object_ref::{NonNullObjectRef, TEST_OBJECT_BUS_NAME, TEST_OBJECT_PATH_STR};
			let test_origin = NonNullObjectRef::from_static_str_unchecked(
				TEST_OBJECT_BUS_NAME,
				TEST_OBJECT_PATH_STR,
			);

			let struct_event = <$type>::new_test_event(&test_origin);

			let msg = zbus::Message::try_from(struct_event.clone())
				.expect("Should convert event into a message");

			let event_enum_back = crate::Event::try_from(&msg)
				.expect("Should convert message into global event enum");

			let event_enum: crate::Event = struct_event.into();
			assert_eq!(event_enum, event_enum_back);
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_fake_msg() {
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
			let event = <$type>::try_from(&fake_msg);
			assert_matches::assert_matches!(event, Err(_));
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_validated_message_with_body() {
			use crate::events::traits::MessageConversion;
			use crate::object_ref::{NonNullObjectRef, TEST_OBJECT_BUS_NAME, TEST_OBJECT_PATH_STR};

			let test_origin = NonNullObjectRef::from_static_str_unchecked(
				TEST_OBJECT_BUS_NAME,
				TEST_OBJECT_PATH_STR,
			);

			let event_to_test = <$type>::new_test_event(&test_origin);

			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.technically.valid",
				"MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&event_to_test.body())
			.unwrap();

			let hdr = fake_msg.header();
			let event = <$type>::from_message_unchecked(&fake_msg, &hdr);
			event.expect("from_message_unchecked should work despite mismatching interface/member");
		}
	};
}

/// Generates test cases for event wrapper conversions.
///
/// This macro verifies the round-trip conversion between a user-facing event type (UFTE),
/// an interface-specific event enum, and the overarching `Event` enum.
///
/// Expands to five tests:
/// 1. `into_and_try_from_user_facing_event`
/// 2. `zbus_msg_invalid_interface`
/// 3. `zbus_msg_invalid_member`
/// 4. `zbus_msg_invalid_member_and_interface`
/// 5. `zbus_msg_conversion`
///
/// # Arguments
///
/// * `$iface_enum`: The enum representing a specific AT-SPI interface (e.g., `DocumentEvents`).
/// * `$ufet`: The user-facing event struct (e.g., `LoadCompleteEvent`).
/// * `[ $($field:ident),* ]`: An optional list of fields present in the UFTE struct
///   (excluding the mandatory `item` field). These will be initialized with `Default::default()`.
macro_rules! event_wrapper_test_cases {
    ($iface_enum:ty, $ufet:ty) => {
		event_wrapper_test_cases!($iface_enum, $ufet, []);
	};

	($iface_enum:ty, $ufet:ty, [ $($field:ident),* ]) => {
		#[cfg(test)]
		#[rename_item::rename(name($iface_enum), prefix = "events_tests_", case = "snake")]
		mod test_module {
			use super::{$ufet, $iface_enum, AtspiError, Event, MessageConversion};
			use crate::object_ref::{NonNullObjectRef, TEST_OBJECT_BUS_NAME, TEST_OBJECT_PATH_STR};
			use assert_matches::assert_matches;

			#[test]
			fn into_and_try_from_user_facing_event() {
				let test_origin = NonNullObjectRef::from_static_str_unchecked(
					TEST_OBJECT_BUS_NAME,
					TEST_OBJECT_PATH_STR,
				);

				let event_to_test = <$ufet>::new_test_event(&test_origin);

				let mod_type = <$iface_enum>::from(event_to_test);
				let hint_iface = "Check macro `impl_from_user_facing_event_for_interface_event_enum!`";

				let event = Event::from(mod_type.clone());
				let hint_event = "Check macro `impl_from_interface_event_enum_for_event!`";

				let hint_event_try = "Check macro `impl_try_from_event_for_interface_enum!`";
				let mod_type2 = <$iface_enum>::try_from(event.clone())
					.expect(&format!("Should convert global `Event` enum into interface enum. Hints: {hint_event} and {hint_event_try}"));

				assert_eq!(
					mod_type, mod_type2,
					"Interface enums should match. Hints: {hint_iface}, {hint_event} and {hint_event_try}"
				);
			}

			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_invalid_interface() {
				let test_origin = NonNullObjectRef::from_static_str_unchecked(
					TEST_OBJECT_BUS_NAME,
					TEST_OBJECT_PATH_STR,
				);

				let event_to_test = <$ufet>::new_test_event(&test_origin);
				let test_body = event_to_test.body();

				let fake_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					"org.a11y.atspi.technically.valid.lol",
					<$ufet as crate::events::traits::DBusMember>::DBUS_MEMBER,
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&test_body)
				.unwrap();

				let mod_type = <$iface_enum>::try_from(&fake_msg);
				let event_type = Event::try_from(&fake_msg);

				assert_matches!(mod_type, Err(AtspiError::InterfaceMatch(_)));
				assert_matches!(event_type, Err(AtspiError::InterfaceMatch(_)));
			}

			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_invalid_member() {
				let test_origin = NonNullObjectRef::from_static_str_unchecked(
					TEST_OBJECT_BUS_NAME,
					TEST_OBJECT_PATH_STR,
				);

				let event_to_test = <$ufet>::new_test_event(&test_origin);
				let test_body = event_to_test.body();

				let fake_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					<$ufet as crate::events::traits::DBusInterface>::DBUS_INTERFACE,
					"FakeMemberName",
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&test_body)
				.unwrap();

				let mod_type = <$iface_enum>::try_from(&fake_msg);
				assert_matches!(mod_type, Err(AtspiError::MemberMatch(_)));
			}

			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_invalid_member_and_interface() {
				let test_origin = NonNullObjectRef::from_static_str_unchecked(
					TEST_OBJECT_BUS_NAME,
					TEST_OBJECT_PATH_STR,
				);

				let event_to_test = <$ufet>::new_test_event(&test_origin);

				let fake_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					"org.a11y.atspi.technically.allowed",
					"FakeMemberName",
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&event_to_test.body())
				.unwrap();

				let mod_type = <$iface_enum>::try_from(&fake_msg);
				assert_matches!(mod_type, Err(AtspiError::InterfaceMatch(_)));
			}

			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_conversion() {
				let test_origin = NonNullObjectRef::from_static_str_unchecked(
					TEST_OBJECT_BUS_NAME,
					TEST_OBJECT_PATH_STR,
				);

				let event_to_test = <$ufet>::new_test_event(&test_origin);

				let valid_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					<$ufet as crate::events::traits::DBusInterface>::DBUS_INTERFACE,
					<$ufet as crate::events::traits::DBusMember>::DBUS_MEMBER,
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&event_to_test.body())
				.unwrap();

				let mod_type = <$iface_enum>::try_from(&valid_msg);
				mod_type.expect("Should convert from valid `Message` back into a interface event enum variant");
			}
		}
	};
}

/// Generates several tests for a specific event type.
///
/// This macro creates a module named `event_tests_{ufet_snake_case}` containing several tests
/// that ensure the event type correctly implements standard AT-SPI properties and conversions.
///
/// # Generated Tests
///
/// * `event_enum_conversion`: Verifies round-trip conversion between the specific event type and the global `Event` enum.
/// * `event_enum_transparency_test_case`: Checks that `DBus` properties (member, interface, etc.) match between the specific type and its `Event` wrapper.
/// * `event_has_matching_xml_definition`: Validates that the interface and member names match the project's `DBus` XML definitions.
/// * `generic_event_uses`: Verifies deconstruction and reconstruction of the event from its constituent parts.
/// * `zbus_msg_conversion_to_specific_event_type`: Ensures the event can be converted to a `zbus::Message` and back.
/// * `zbus_msg_conversion_to_event_enum_type`: Verifies conversion from a `zbus::Message` to the global `Event` enum.
/// * `zbus_msg_conversion_failure_fake_msg`: Confirms that invalid messages fail to parse.
/// * `zbus_msg_conversion_validated_message_with_body`: Tests message parsing with a valid body but potentially mismatching headers.
///
/// # Arguments
///
/// * `$ufet`: The user-facing event struct type.
/// * `[ $($field:ident),* ]`: (Optional) A list of fields in the struct (excluding `item`) to be initialized with `Default::default()` during tests.
/// * `$qt`: (Optional) Body handling mode. Can be `Auto` (default) or `Explicit` for non-generic body types.
///
/// # Examples
///
/// ```ignore
/// // Simple event with only the mandatory 'item' field
/// event_test_cases!(LoadCompleteEvent);
///
/// // Event with additional fields
/// event_test_cases!(AbsEvent, [x, y]);
/// ```
macro_rules! event_test_cases {
   	($ufet:ident, $qt:tt) => {
		event_test_cases!($ufet, [], $qt);
	};

	($ufet:ident, [ $($field:ident),* ], $qt:tt) => {
		#[cfg(test)]
		#[rename_item::rename(name($ufet), prefix = "event_tests_", case = "snake")]
		mod unique_module_name {
			use super::EventProperties;
			use super::$ufet;
			use crate::events::traits::EventTypeProperties;
			use crate::object_ref::{NonNullObjectRef, TEST_OBJECT_BUS_NAME, TEST_OBJECT_PATH_STR};
			use crate::Event;


			#[test]
			#[cfg(feature = "wrappers")]
			fn event_enum_conversion() {
				let test_origin = NonNullObjectRef::from_static_str_unchecked(
					TEST_OBJECT_BUS_NAME,
					TEST_OBJECT_PATH_STR,
				);

				let event_struct = <$ufet>::new_test_event(&test_origin);

				let event_enum = Event::from(event_struct.clone());
				let event_struct_back = <$ufet>::try_from(event_enum)
					.expect("Should convert event enum back into specific type");
				assert_eq!(event_struct, event_struct_back);
			}

			#[test]
			#[cfg(feature = "wrappers")]
			fn event_enum_transparency_test_case() {
				let test_origin = NonNullObjectRef::from_static_str_unchecked(
					TEST_OBJECT_BUS_NAME,
					TEST_OBJECT_PATH_STR,
				);

				let specific_event = <$ufet>::new_test_event(&test_origin);
				let event_enum = Event::from(specific_event.clone());

				let hint = "Check macro `impl_from_user_facing_type_for_event_enum!`";

				assert_eq!(specific_event.member(), event_enum.member(), "Members mismatch: {hint}");
				assert_eq!(specific_event.interface(), event_enum.interface(), "Interfaces mismatch: {hint}");
				assert_eq!(specific_event.registry_string(), event_enum.registry_string(), "Registry mismatch: {hint}");

				assert_eq!(specific_event.match_rule(), event_enum.match_rule(), "Match rule mismatch: {hint}");
				assert_eq!(specific_event.path(), event_enum.path(), "Paths mismatch: {hint}");
				assert_eq!(specific_event.sender(), event_enum.sender(), "Senders mismatch: {hint}");
			}

			zbus_message_test_case!($ufet, [ $($field),* ], $qt);
			event_has_matching_xml_definition!($ufet);
			generic_event_test_case!($ufet, [ $($field),* ]);
		}

		assert_impl_all!(
			$ufet<'static>: Clone,
			std::fmt::Debug,
			serde::Serialize,
			serde::Deserialize<'static>,
			PartialEq,
			Eq,
			std::hash::Hash,
			crate::events::traits::EventProperties,
			crate::events::traits::EventTypeProperties,
			crate::events::traits::DBusInterface,
			crate::events::traits::DBusMember,
			crate::events::traits::DBusMatchRule,
			crate::events::traits::RegistryEventString
		);

		#[cfg(feature = "zbus")]
		assert_impl_all!(zbus::Message: TryFrom<$ufet<'static>>);
	};

	// Shorthand "Type + [fields]" pattern -> into main pattern
	($ufet:ident, [ $($field:ident),* ]) => {
		event_test_cases!($ufet, [ $($field),* ], Auto);
	};

	// Shorthand "Type + Explicit" pattern -> into main pattern
	($ufet:ident, Explicit) => {
		event_test_cases!($ufet, [], Explicit);
	};

	// shorthand "Type + Auto" pattern -> into main pattern
	($ufet:ident, Auto) => {
		event_test_cases!($ufet, [], Auto);
	};

	// Shortest pattern shorthand "Type" only pattern -> into main pattern
	($ufet:ident) => {
		event_test_cases!($ufet, [], Auto);
	};
}

/// Implements `MessageConversionExt` for a given target event type with a given body type.
///
/// # Example
/// ```ignore
/// # // 'ignored for brevity because `impl`s require that the *example crate* defines traits `MessageConversionExt`,
/// # // `MessageConversion` as well as the body and target types.
///
/// impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: RemoveAccessibleEvent, body: ObjectRef);
/// ```
/// expands to:
///
/// ```ignore
/// #[cfg(feature = "zbus")]
/// impl<'a> MessageConversionExt<'a, ObjectRef> for RemoveAccessibleEvent {
///     fn try_from_message(msg: &zbus::Message, hdr: &'_ Header) -> Result<Self, AtspiError> {
///         <Self as MessageConversionExt<$body_type>>::validate_interface(hdr)?;
///         <Self as MessageConversionExt<$body_type>>::validate_member(hdr)?;
///         <Self as MessageConversionExt<$body_type>>::validate_body(msg)?;
///         <Self as MessageConversion<'_>>::from_message_unchecked(msg, hdr)
///     }
/// }
/// ```
macro_rules! impl_msg_conversion_ext_for_target_type_with_specified_body_type {
	// We need to match on the identifier, not the type to ensure the macro
	// decides the names of the borrows. We _could_ match on :lt (life time)
	// but this is simpler than having a target_ident and a target_life_time pair,
	(target: $target_type:ident<'_>, body: $body_type:ty) => {
		#[cfg(feature = "zbus")]
		impl<'a> crate::events::traits::MessageConversionExt<'a, $body_type> for $target_type<'a> {
			fn try_from_message(
				msg: &'a zbus::Message,
				hdr: &'a zbus::message::Header,
			) -> Result<Self, crate::error::AtspiError>
			where
				Self: 'a,
			{
				use crate::events::traits::MessageConversionExt;
				<Self as MessageConversionExt<$body_type>>::validate_interface(hdr)?;
				<Self as MessageConversionExt<$body_type>>::validate_member(hdr)?;
				<Self as MessageConversionExt<$body_type>>::validate_body(msg)?;
				<Self as crate::events::traits::MessageConversion<'a>>::from_message_unchecked(
					msg, hdr,
				)
			}
		}
	};
}

/// Implements `MessageConversionExt` for a list of event types using standard `EventBody`.
///
/// This macro handles both types with a lifetime (preserving the zero-copy borrow)
/// and owned types.
///
/// # Examples
///
/// ```ignore
/// // Bulk implement for multiple types:
/// impl_msg_conversion_ext_for_target_type!(
///     LoadCompleteEvent<'_>,
///     FocusEvent,
///     LinkSelectedEvent<'_>
/// );
/// ```
macro_rules! impl_msg_conversion_ext_for_target_type {
	($($target:ident$(<$lt:lifetime>)?),* $(,)?) => {
		$(
		    // Note how "@impl" is added to the recursive call to avoid matching on this, outer pattern again.
			impl_msg_conversion_ext_for_target_type!(@impl $target $(<$lt>)?);
		)*
	};

	// Helper arm for partially borrowed types (with lifetime)
	(@impl $target:ident <$lt:lifetime>) => {
		#[cfg(feature = "zbus")]
		impl<'msg> crate::events::traits::MessageConversionExt<'msg, crate::events::EventBody<'msg>>
			for $target<'msg>
		{
			fn try_from_message(
				msg: &'msg zbus::Message,
				header: &'msg zbus::message::Header,
			) -> Result<Self, crate::error::AtspiError> {
				use crate::events::traits::MessageConversion;
				use zbus::zvariant::Type;

				Self::validate_interface(header)?;
				Self::validate_member(header)?;

				let item = crate::object_ref::NonNullObjectRef::try_from(header)?;
				let msg_body = msg.body();
				let signature = msg_body.signature();

				if signature == crate::events::EventBodyOwned::SIGNATURE
					|| signature == crate::events::EventBodyQtOwned::SIGNATURE
				{
					Self::from_message_unchecked_parts(item, msg_body)
				} else {
					Err(crate::error::AtspiError::SignatureMatch(format!(
						"Signature \"{}\" does not match a common: \"{}\" or Qt: \"{}\" signature.",
						signature,
						crate::events::EventBodyOwned::SIGNATURE,
						crate::events::EventBodyQtOwned::SIGNATURE,
					)))
				}
			}
		}
	};

	// Helper arm for owned types (no lifetime)
	(@impl $target:ident) => {
		#[cfg(feature = "zbus")]
		impl<'msg> crate::events::traits::MessageConversionExt<'msg, crate::events::EventBody<'msg>>
			for $target
		{
			fn try_from_message(
				msg: &'msg zbus::Message,
				header: &'msg zbus::message::Header,
			) -> Result<Self, crate::error::AtspiError> {
				use crate::events::traits::MessageConversion;
				use crate::events::traits::MessageConversionExt;
				use zbus::zvariant::Type;

				Self::validate_interface(header)?;
				Self::validate_member(header)?;

				let item = crate::object_ref::NonNullObjectRef::try_from(header)?;
				let msg_body = msg.body();
				let signature = msg_body.signature();

				if signature == crate::events::EventBodyOwned::SIGNATURE
					|| signature == crate::events::EventBodyQtOwned::SIGNATURE
				{
					Self::from_message_unchecked_parts(item, msg_body)
				} else {
					Err(crate::error::AtspiError::SignatureMatch(format!(
						"Signature \"{}\" does not match a common: \"{}\" or Qt: \"{}\" signature.",
						signature,
						crate::events::EventBodyOwned::SIGNATURE,
						crate::events::EventBodyQtOwned::SIGNATURE,
					)))
				}
			}
		}
	};
}

/// Implements `TryFromMessage` for a given event wrapper type.
///
/// This macro handles the top-level conversion from a raw `zbus::Message` to
/// an interface-specific event wrapper (like `ObjectEvents` or `DocumentEvents`).
///
/// It supports both borrowed wrappers (preserving the borrow on the message)
/// and owned wrappers.
///
/// # Examples
///
/// ```ignore
/// // For a wrapper with a lifetime (preserving the borrow):
/// impl_tryfrommessage_for_event_wrapper!(ObjectEvents<'_>);
///
/// // For an owned wrapper:
/// impl_tryfrommessage_for_event_wrapper!(FocusEvents);
/// ```
///
/// # Technical Note
/// For borrowed types, the macro unifies the lifetime of the wrapper with
/// the lifetime of the `zbus::Message` allowing users to receive events that
/// point directly into the message buffer.
macro_rules! impl_tryfrommessage_for_event_wrapper {
	// Pattern for wrappers with a lifetime parameter: ObjectEvents<'_>
	($wrapper:ident<'_>) => {
		#[cfg(feature = "zbus")]
		impl<'a> TryFrom<&'a zbus::Message> for $wrapper<'a> {
			type Error = AtspiError;
			fn try_from(msg: &'a zbus::Message) -> Result<$wrapper<'a>, AtspiError> {
				use crate::events::traits::EventWrapperMessageConversion;

				let header = msg.header();
				let interface =
					header.interface().ok_or(crate::error::AtspiError::MissingInterface)?;
				if interface != Self::DBUS_INTERFACE {
					return Err(crate::error::AtspiError::InterfaceMatch(format!(
						"Interface {} does not match required interface: {}",
						interface,
						Self::DBUS_INTERFACE
					)));
				}
				Self::try_from_message_interface_checked(msg, &header)
			}
		}
	};

	// Pattern for owned wrappers (no lifetime parameter)
	($wrapper:ident) => {
		#[cfg(feature = "zbus")]
		impl TryFrom<&zbus::Message> for $wrapper {
			type Error = AtspiError;

			fn try_from(msg: &zbus::Message) -> Result<Self, AtspiError> {
				use crate::events::traits::EventWrapperMessageConversion;

				let header = msg.header();
				let interface =
					header.interface().ok_or(crate::error::AtspiError::MissingInterface)?;
				if interface != Self::DBUS_INTERFACE {
					return Err(crate::error::AtspiError::InterfaceMatch(format!(
						"Interface {} does not match required interface: {}",
						interface,
						Self::DBUS_INTERFACE
					)));
				}
				Self::try_from_message_interface_checked(msg, &header)
			}
		}
	};
}

/// Implement the `MessageConversion` trait for types built solely from a `NonNullObjectRef`.
///
/// This macro handles both types with a lifetime (preserving the zero-copy borrow)
/// and owned types (automatically calling `.into_owned()`).
///
/// # Examples
///
/// ```ignore
/// // For a mix of types:
/// impl_msg_conversion_for_types_built_from_object_ref!(
///     LoadCompleteEvent<'_>,
///     FocusEvent,
///     LinkSelectedEvent<'_>
/// );
/// ```
///
/// # Technical Details (for zero-copy)
///
/// For types marked with `<'_>`, the macro ensures that the lifetime of the resulting
/// event is unified with the lifetime of the `zbus::Message`. This allows us to
/// store `NonNullObjectRef<'a>` directly in the event struct without allocations.
macro_rules! impl_msg_conversion_for_types_built_from_object_ref {
	() => {};

	// Pattern for types with lifetime.
	// Note that we match an identifier (not a type)
	// and that ident is paired with the anonymous lifetime `<'_>` to match on a type with a lifetime.
	($target:ident <'_> $(, $($rest:tt)*)?) => {
		#[cfg(feature = "zbus")]
		impl<'a> crate::events::traits::MessageConversion<'a> for $target<'a> {
			type Body<'msg> = crate::events::EventBody<'msg> where Self: 'msg;

			fn from_message_unchecked_parts(
				obj_ref: crate::object_ref::NonNullObjectRef<'a>,
				_body: zbus::message::Body,
			) -> Result<Self, crate::error::AtspiError> {
				Ok(Self { item: obj_ref })
			}

			fn from_message_unchecked(_: &'a zbus::Message, header: &zbus::message::Header) -> Result<Self, crate::error::AtspiError> {
				let obj_ref: crate::object_ref::NonNullObjectRef<'_> = header.try_into()?;
				Ok( Self { item: obj_ref.into_owned() })
			}

			fn body(&self) -> Self::Body<'_> {
				crate::events::EventBodyOwned::default().into()
			}
		}
		impl_msg_conversion_for_types_built_from_object_ref!($($($rest)*)?);
	};

	// Pattern for types without lifetime.
	// Note the absense of a lifetime here.
	($target:ident $(, $($rest:tt)*)?) => {
		#[cfg(feature = "zbus")]
		impl<'a> crate::events::traits::MessageConversion<'a> for $target {
			type Body<'msg> = crate::events::EventBody<'msg> where Self: 'msg;

			fn from_message_unchecked_parts(
				obj_ref: crate::object_ref::NonNullObjectRef<'a>,
				_body: zbus::message::Body,
			) -> Result<Self, crate::error::AtspiError> {
				Ok(Self { item: obj_ref.into_owned() })
			}

			fn from_message_unchecked(_: &'a zbus::Message, header: &zbus::message::Header) -> Result<Self, crate::error::AtspiError> {
				let obj_ref: crate::object_ref::NonNullObjectRef<'_> = header.try_into()?;
				Ok( Self { item: obj_ref.into_owned() })
			}

			fn body(&self) -> Self::Body<'_> {
				crate::events::EventBodyOwned::default().into()
			}
		}
		impl_msg_conversion_for_types_built_from_object_ref!($($($rest)*)?);
	};
}

/// Implement `DBusMember`, `DBusInterface`, `DBusMatchRule`, and `RegistryEventString`
/// for a given event type.
///
/// This macro takes 5 arguments in the order:
/// - The target type
/// - The member string
/// - The interface string
/// - The registry string
/// - The match rule string
///
/// # Example
/// ```ignore
/// impl_member_interface_registry_string_and_match_rule_for_event!(
/// FocusEvent, "Focus", "org.a11y.atspi.Event.Focus", "focus",
/// "type='signal',interface='org.a11y.atspi.Event.Focus'");
/// ```
/// expands to:
///
/// ```ignore
/// impl DBusMember for FocusEvent {
///    const DBUS_MEMBER: &'static str = "Focus";
/// }
/// impl DBusInterface for FocusEvent {
///   const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
/// }
/// impl MatchRule for FocusEvent {
///  const MATCH_RULE: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus'";
/// }
/// impl RegistryEventString for FocusEvent {
///  const REGISTRY_STRING: &'static str = "focus";
/// }
/// impl DBusProperties for FocusEvent {}
/// ```
macro_rules! impl_member_interface_registry_string_and_match_rule_for_event {
	($target_type:ty, $member_str:literal, $interface_str:literal, $registry_str:literal, $match_rule_str:literal) => {
		impl crate::events::traits::DBusMember for $target_type {
			const DBUS_MEMBER: &'static str = $member_str;
		}
		impl crate::events::traits::DBusInterface for $target_type {
			const DBUS_INTERFACE: &'static str = $interface_str;
		}
		impl crate::events::traits::DBusMatchRule for $target_type {
			const MATCH_RULE_STRING: &'static str = $match_rule_str;
		}
		impl crate::events::traits::RegistryEventString for $target_type {
			const REGISTRY_EVENT_STRING: &'static str = $registry_str;
		}
		impl crate::events::traits::DBusProperties for $target_type {}
	};
}

/// Implement `EventTypeProperties` for a given event type.
///
/// This macro takes one argument: the target type.
///
/// # Example
/// ```ignore
/// impl_event_type_properties_for_event!(FocusEvent);
/// ```
/// expands to:
///
/// ```ignore
/// impl EventTypeProperties for FocusEvent {
/// fn member(&self) -> &'static str {
///   Self::DBUS_MEMBER
/// }
/// fn interface(&self) -> &'static str {
///   Self::DBUS_INTERFACE
/// }
/// fn registry_string(&self) -> &'static str {
///   Self::REGISTRY_EVENT_STRING
/// }
/// fn match_rule(&self) -> &'static str {
///     Self::MATCH_RULE_STRING
///   }
/// }
/// ```
macro_rules! impl_event_type_properties_for_event {
	($target_type:ty) => {
		impl crate::events::traits::EventTypeProperties for $target_type {
			fn member(&self) -> &'static str {
				Self::DBUS_MEMBER
			}
			fn interface(&self) -> &'static str {
				Self::DBUS_INTERFACE
			}
			fn registry_string(&self) -> &'static str {
				Self::REGISTRY_EVENT_STRING
			}
			fn match_rule(&self) -> &'static str {
				Self::MATCH_RULE_STRING
			}
		}
	};
}

/// Implements a `new_test_event` associated function for the given event types.
/// This is used to create an event for testing purposes, filling fields with defaults.
///
/// We hide it from the documentation because we do not want to promote creation of events
/// with non-existent object references.
///
/// # Examples
///
/// ```ignore
/// // For a type with a lifetime and no extra fields:
/// impl_test_event!(LoadCompleteEvent<'_>);
///
/// // For a type with a lifetime and extra fields:
/// impl_test_event!(ModifiersEvent<'_> { previous_modifiers, current_modifiers });
///
/// // For an owned type (no lifetime) with extra fields:
/// impl_test_event!(PropertyChangeEvent { property, value });
///
/// // You can also batch multiple types in one call:
/// impl_test_event!(
///     LoadCompleteEvent<'_>,
///     ModifiersEvent<'_> { previous_modifiers, current_modifiers },
///     PropertyChangeEvent { property, value }
/// );
/// ```
macro_rules! impl_test_event {
	() => {};

	// Pattern for types with lifetime parameter and additional fields.
	// Generates a generic `impl<'o>` where the `item` is borrowed from the origin.
	($target_type:ident<'_> { $($field:ident),* $(,)? } $(, $($rest:tt)*)?) => {
		impl<'o> $target_type<'o> {
			#[doc(hidden)]
			pub fn new_test_event(origin: &crate::NonNullObjectRef<'o>) -> Self {
				Self {
					item: origin.clone(),
					$( $field: Default::default() ),*
				}
			}
		}
		impl_test_event!($($($rest)*)?);
	};

	// Pattern for types with a lifetime parameter but no additional fields.
	($target_type:ident<'_> $(, $($rest:tt)*)?) => {
		impl<'o> $target_type<'o> {
			#[doc(hidden)]
			pub fn new_test_event(origin: &crate::NonNullObjectRef<'o>) -> Self {
				Self {
					item: origin.clone(),
				}
			}
		}
		impl_test_event!($($($rest)*)?);
	};

	// Pattern for owned types (no lifetime parameter) with additional fields.
	// These types require `into_owned()` to store the `NonNullObjectRef` as a 'static value.
	($target_type:ident { $($field:ident),* $(,)? } $(, $($rest:tt)*)?) => {
		impl $target_type {
			#[doc(hidden)]
			pub fn new_test_event(origin: &crate::NonNullObjectRef<'_>) -> Self {
				Self {
					item: origin.clone().into_owned(),
					$( $field: Default::default() ),*
				}
			}
		}
		impl_test_event!($($($rest)*)?);
	};

	// Pattern for owned types (no lifetime parameter) and no additional fields.
	($target_type:ident $(, $($rest:tt)*)?) => {
		impl $target_type {
			#[doc(hidden)]
			pub fn new_test_event(origin: &crate::NonNullObjectRef<'_>) -> Self {
				Self {
					item: origin.clone().into_owned(),
				}
			}
		}
		impl_test_event!($($($rest)*)?);
	};
}
