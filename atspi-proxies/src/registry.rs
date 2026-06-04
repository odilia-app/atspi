//! # `RegistryProxy`
//!
//! `org.a11y.atspi.Registry` provides access to the AT-SPI registry.
//!
//! With this interface one can register, deregister and query
//! registered events on the accessibility bus.
//!
//! ## Defaults
//!
//! The service is the daemon specifically for the accessibility bus.
//! This has a fixed service name, path and interface.
//!
//! All three, interface, `default_service` and `default_path` are provided to the
//! macro that builds `RegistryProxy`, so no defaults need to be derived.

use zbus::names::OwnedBusName;

#[zbus::proxy(
	interface = "org.a11y.atspi.Registry",
	default_service = "org.a11y.atspi.Registry",
	default_path = "/org/a11y/atspi/registry",
	assume_defaults = false
)]
pub trait Registry {
	/// `DeregisterEvent` method
	fn deregister_event(&self, event: &str) -> zbus::Result<()>;

	/// `GetRegisteredEvents` method
	#[zbus(name = "GetRegisteredEvents")]
	fn registered_events(&self) -> zbus::Result<Vec<(OwnedBusName, String)>>;

	/// `RegisterEvent` method
	fn register_event(&self, event: &str) -> zbus::Result<()>;
}
