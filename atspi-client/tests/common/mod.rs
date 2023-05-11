use std::{process::Output, time::Duration};

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
