//! # `SocketProxy`
//!
//! `org.a11y.atspi.Socket` provides methods for accessible applications to
//! register and deregister with the `Registry`.
//!
//! ## Defaults
//!
//! "org.a11y.atspi.Spcket" is implemented for once on the `Registry`
//!
//! All three, interface, `default_service` and `default_path` are provided to the
//! macro that builds `RegistryProxy`, so no defaults need to be derived.

use atspi_common::object_ref::ObjectRefOwned;

#[zbus::proxy(
	interface = "org.a11y.atspi.Socket",
	default_path = "/org/a11y/atspi/accessible/root",
	default_service = "org.a11y.atspi.Registry",
	assume_defaults = false
)]
pub trait Socket {
	/// @plug: a string for the unique bus name of the application, and an object path
	/// for the application's' root object.
	///
	/// This is the entry point for an application that wants to register itself against
	/// the accessibility registry.  The application's root object, which it passes in
	/// @plug, must support the org.a11y.atspi.Application interface.
	///
	/// When an application calls this method on the registry, the following handshake happens:
	///
	/// * Application calls this method on the registry to identify itself.
	/// * The registry sets the "Id" property on the org.a11y.atspi.Application interface on the @plug object.
	/// * The Embed method returns with the bus name and object path for the registry's root object.
	/// Returns: the bus name and object path of the registry's root object.
	fn embed(&self, plug: &(&str, zbus::zvariant::ObjectPath<'_>)) -> zbus::Result<ObjectRefOwned>;

	/// This method is called by a socket to inform the plug that it is being
	/// embedded. The plug should register the embedding socket as its parent.
	fn embedded(&self, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

	/// Unembed method
	/// @plug: a string for the unique bus name of the application, and an object path
	/// for the application's' root object.
	///
	/// Unregisters an application from the accesibility registry.  It is not necessary to
	/// call this method; the accessibility registry detects when an application
	/// disconnects from the bus.
	fn unembed(&self, plug: &(&str, zbus::zvariant::ObjectPath<'_>)) -> zbus::Result<()>;
}
