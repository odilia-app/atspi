use std::{process::Output, time::Duration};

use std::collections::HashMap;
use std::sync::Arc;
use zbus::zvariant::Value;
use zbus::zvariant::{ObjectPath, OwnedValue};
use zbus::Error;
use zbus::{Message, MessageBuilder};
use zbus_names::{InterfaceName, MemberName};

use atspi::events::EventBodyOwned;

pub fn test_event_body() -> EventBodyOwned {
    let any_data: OwnedValue =
        OwnedValue::try_from(Value::U16(0_u16)).expect("trivial value should convert");
    let props: HashMap<String, OwnedValue> = HashMap::new();
    EventBodyOwned {
        kind: "test body".to_string(),
        detail1: 0_i32,
        detail2: 0_i32,
        any_data,
        properties: props,
    }
}

/// An `Event` matching `method`, `interface` and `path`.
pub fn valid_mockup_message<'p, 'i, 'm, P, I, M>(method: M, iface: I, path: P) -> Arc<Message>
where
    P: TryInto<ObjectPath<'p>>,
    I: TryInto<InterfaceName<'i>>,
    M: TryInto<MemberName<'m>>,
    P::Error: Into<Error>,
    I::Error: Into<Error>,
    M::Error: Into<Error>,
{
    let body = test_event_body();
    let builder = MessageBuilder::signal(path, iface, method).unwrap();
    let msg = builder.build(&body).unwrap();
    Arc::new(msg)
}

pub fn a11y_bus_address() -> String {
    let output = std::process::Command::new("busctl")
        .arg("--user")
        .arg("call")
        .arg("org.a11y.Bus")
        .arg("/org/a11y/bus")
        .arg("org.a11y.Bus")
        .arg("GetAddress")
        .output()
        .unwrap();

    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code().unwrap(), 0,);

    let addr_string = String::from_utf8(output.stdout).unwrap();
    let addr_str = addr_string
        .strip_prefix("s \"")
        .unwrap()
        .trim()
        .strip_suffix('"')
        .unwrap();

    String::from(addr_str)
}

#[allow(clippy::too_many_arguments)]
pub fn create_command<'a>(
    address: &'a str,
    method: &'a str,
    kind: &'a str,
    detail1: &'a str,
    detail2: &'a str,
    valuekind: &'a str,
    value: &'a str,
    props: &'a str,
) -> Output {
    let mut base_cmd = std::process::Command::new("busctl");

    base_cmd
        .arg("--address")
        .arg(address)
        .arg("emit")
        .arg("/org/a11y/atspi/accessible/null")
        .arg("org.a11y.atspi.Event.Object")
        .arg(method)
        .arg("siiva{sv}")
        .arg(kind)
        .arg(detail1)
        .arg(detail2)
        .arg(valuekind)
        .arg(value)
        .arg(props)
        .output()
        .unwrap()
}
/// Yields `Err(())` on time-out.
pub async fn timeout(dur: Duration) -> Result<(), ()> {
    let start = std::time::Instant::now();
    let mut now = std::time::Instant::now();
    while now - start < dur {
        futures_lite::future::yield_now().await;
        now = std::time::Instant::now();
    }
    Err(())
}
