use crate::{
	error::AtspiError,
	events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
	Event,
};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

// IgnoreBlock start
/// # Example
///
/// Even though this example employs `Tokio`, any runtime will do.
///
/// Note that this example is minimized for rhe sake of brevity.
/// More complete examples may be found in the `examples/` directory.
///
/// ```
/// use atspi_common::events::Event;
/// use atspi_common::events::focus::FocusEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<FocusEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let output = std::process::Command::new("busctl")
/// #       .arg("--user")
/// #       .arg("call")
/// #       .arg("org.a11y.Bus")
/// #       .arg("/org/a11y/bus")
/// #       .arg("org.a11y.Bus")
/// #       .arg("GetAddress")
/// #       .output()
/// #       .unwrap();
/// #    let addr_string = String::from_utf8(output.stdout).unwrap();
/// #    let addr_str = addr_string
/// #        .strip_prefix("s \"")
/// #        .unwrap()
/// #        .trim()
/// #        .strip_suffix('"')
/// #        .unwrap();
/// #   let mut base_cmd = std::process::Command::new("busctl");
/// #   let thing = base_cmd
/// #       .arg("--address")
/// #       .arg(addr_str)
/// #       .arg("emit")
/// #       .arg("/org/a11y/atspi/accessible/null")
/// #       .arg("org.a11y.atspi.Event.Focus")
/// #       .arg("Focus")
/// #       .arg("siiva{sv}")
/// #       .arg("")
/// #       .arg("0")
/// #       .arg("0")
/// #       .arg("i")
/// #       .arg("0")
/// #       .arg("0")
/// #       .output()
/// #       .unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///          if let Event::Focus(_event) = ev {
/// #            break;
///              // do things with your event here
///          }
/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum FocusEvents {
	Focus(FocusEvent),
}
impl_event_conversions!(FocusEvents, Event::Focus);
event_wrapper_test_cases!(FocusEvents, FocusEvent);

impl HasMatchRule for FocusEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus'";
}

// IgnoreBlock start
/// # Example
///
/// Even though this example employs `Tokio`, any runtime will do.
///
/// Note that the example is minimized for rhe sake of brevity.
/// More complete examples may be found in the `examples/` directory.
///
/// ```
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::focus::FocusEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<FocusEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = FocusEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = FocusEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct FocusEvent {
	pub item: crate::events::Accessible,
}

impl GenericEvent<'_> for FocusEvent {
	const DBUS_MEMBER: &'static str = "Focus";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
	const REGISTRY_EVENT_STRING: &'static str = "Focus:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

/*
impl TryFrom<Event> for FocusEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Focus(FocusEvents::Focus(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for FocusEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let member = ev
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"Focus" => Ok(FocusEvents::Focus(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Focus".into())),
		}
	}
}

impl_event_conversions!(FocusEvent, FocusEvents, FocusEvents::Focus, Event::Focus);
event_test_cases!(FocusEvent);
impl_to_dbus_message!(FocusEvent);
impl_from_dbus_message!(FocusEvent);
impl From<FocusEvent> for EventBodyOwned {
	fn from(_event: FocusEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

/*impl HasMatchRule for FocusEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
}*/
/*impl HasRegistryEventString for FocusEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Focus:Focus";
}*/
impl HasRegistryEventString for FocusEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Focus:";
}
