//! # `SocketProxy`
//!
//! A handle for the object on the `Registry` service for the `org.a11y.atspi.Socket`
//! interface.
//!
//! The `Socket` interface is part of AT-SPI2's **Socket/Plug mechanism**, which is
//! used to seamlessly stitch together two distinct accessibility trees managed by
//! separate, out-of-process applications.
//!
//! ## The Socket/Plug Stitching Mechanism
//!
//! In modern desktop environments, applications often isolate their UI components
//! across process boundaries for security and stability. A prime example of this
//! is **GNOME Web (Epiphany)** and **`WebKitGTK`**:
//!
//! * **The Browser Shell** (the outer window, tabs, and URL bar) runs in one process.
//! * **The Web View** (the actual rendered HTML content) runs in a separate, sandboxed
//!   web process.
//!
//! To assistive technologies (like screen readers), these must appear as a single,
//! contiguous UI-tree.
//!
//! The entry point for the Plug/Embed mechanism is the `Embedded` method.
//!
//! 1. The `Embed` method is called by a socket application to inform the plug
//!    application that it is being embedded.
//!
//! 2. The registry imforms the plug application of the socket object.
//! 3. The plug application sets the socket object as its parent.
//! 4. The plug application calls `Socket::Embed` to identify itself.
//! 5. The Registry sets the `Id` property on the `org.a11y.atspi.Application`
//!    interface on the plug object.
//! 6. The method `Socket::Embed` returns with name and object path of the
//!    Registry's root object.
//!
//! ## Defaults
//!
//! The AT-SPI registry implements `Socket` on a known default path, the service
//! is also known.
//!
//! * **Service Destination**: `org.a11y.atspi.Registry`
//! * **Object Path**: `/org/a11y/atspi/accessible/root`
//!
//! Because these defaults are defined on the proxy, you can instantiate the global
//! registry socket with zero-configuration using `new`.
//!
//! ## How to instantiate the proxy
//!
//! ### 1. Shorthand construction using `new` (Default Registry Socket)
//!
//! ```rust,ignore
//! let socket = SocketProxy::new(&connection).await?;
//! ```

use atspi_common::object_ref::ObjectRefOwned;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
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
