//! # `BusProxy`
//!
//! `org.a11y.Bus` (the service) exposes two interfaces on the session bus.
//! This small eaemon is AT-SPI2's only handle on the session bus.
//!
//! `org.a11y.Status` provides a simple status interface, while `org.a11y.Bus`
//! offers a single method to obtain the address of the accessibility bus.
//!
//! ## Defaults
//!
//! The service is a single small daemon provided by the AT-SPI implementation.
//! Both interfaces have a fixed service name, path and interface.
//!
//! All three, interface, `default_service` and `default_path` are provided to the
//! macro that builds `BusProxy`, so no defaults need to be derived.

#[zbus::proxy(
	interface = "org.a11y.Status",
	default_service = "org.a11y.Bus",
	default_path = "/org/a11y/bus",
	assume_defaults = false
)]
pub trait Status {
	/// `IsEnabled` property
	#[zbus(property)]
	fn is_enabled(&self) -> zbus::Result<bool>;
	#[zbus(property)]
	fn set_is_enabled(&self, value: bool) -> zbus::Result<()>;

	/// `ScreenReaderEnabled` property
	#[zbus(property)]
	fn screen_reader_enabled(&self) -> zbus::Result<bool>;
	#[zbus(property)]
	fn set_screen_reader_enabled(&self, value: bool) -> zbus::Result<()>;
}

#[zbus::proxy(
	interface = "org.a11y.Bus",
	default_service = "org.a11y.Bus",
	default_path = "/org/a11y/bus",
	assume_defaults = false
)]
pub trait Bus {
	/// `GetAddress` method
	fn get_address(&self) -> zbus::Result<String>;
}
