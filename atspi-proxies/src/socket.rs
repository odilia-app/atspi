//! # `SocketProxy`
//!
//! A handle for the object on the `Registry` service for the `org.a11y.atspi.Socket`
//! interface.
//!
//! The `Socket` D-Bus interface bundles two largely unrelated concerns:
//!
//!  - General application registration (`Embed`, which every server app calls on the registry).
//!  - The cross-process stitching (`Embedded`).
//!
//! Only `Embedded` is part of the actual stitching.
//!
//! ## The Socket/Plug Stitching Mechanism
//!
#![doc = include_str!("../doc/socket-plug.md")]
//!
//! ## D-Bus Addressing
//!
//! For Assistive Technology (AT) clients—such as screen readers—the `Socket` interface
//! is not directly relevant. ATs act purely as consumers on the accessibility bus; they
//! do not register as accessible servers, and the socket/plug stitching is handled
//! entirely on the application side.
//!
//! For accessible applications exposing a UI, the interface serves two distinct purposes:
//!
//! ### 1. Central Registry Socket (General Application Registration)
//! Every application must register on startup by calling `embed` on the central registry.
//! In this role, the interface is hosted by the central registry daemon at a fixed,
//! well-known location:
//!
//! * **Service Destination**: `org.a11y.atspi.Registry`
//! * **Object Path**: `/org/a11y/atspi/accessible/root`
//!
//! Because these central details are standardized on the proxy, the global registry
//! socket can be instantiated directly using the shorthand `new` constructor.
//!
//! ### 2. Plug Process Socket (Cross-Process Stitching)
//! During cross-process embedding, a host process calls `embedded` directly on the
//! embedded plug's root object. Here, the plug must implement the `Socket` interface
//! locally. The D-Bus destination and object path are dynamic, pointing to the unique
//! bus name and root path of the plug process. To communicate with the plug, the proxy
//! must be instantiated manually using the `builder` to supply these runtime coordinates.
//!
//! ## How to instantiate the proxy
//!
//! ### 1. Shorthand construction using `new` (Default Registry Socket)
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::socket::SocketProxy;
//!
//! # block_on( async {
//! let a11y_connection = AccessibilityConnection::new().await?;
//! let conn = a11y_connection.connection();
//!
//! let _socket = SocketProxy::new(conn).await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
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
	/// Registers an application against the accessibility registry.
	///
	/// `plug` is an `(so)` reference (bus name + object path) to the application's root
	/// object, which must implement `org.a11y.atspi.Application`.
	///
	/// Despite living on the `Socket` interface, this call is the *general application
	/// registration* that every server app performs on startup; on its own it is not the
	/// cross-process plug/socket stitching (see the module-level docs).
	///
	/// On success, the registry assigns the application its `Id` (on the
	/// `org.a11y.atspi.Application` interface) and returns its own root object reference.
	///
	/// member: `Embed`, type: method
	fn embed(&self, plug: &ObjectRefOwned) -> zbus::Result<ObjectRefOwned>;

	/// Informs a plug that it is being embedded by a socket.
	///
	/// Called by the host (socket) directly on the embedded application's (plug) root
	/// object. On receiving this call, the plug registers the embedding socket as its parent,
	/// after which it emits `object:property-change:accessible-parent`. This is the entry
	/// point of the actual plug/socket stitching (see the module-level docs).
	///
	/// # Wire type note
	///
	/// `path` is the socket's *object path*, but the D-Bus interface declares the argument
	/// as a plain string (`s`), not an object path (`o`). It is therefore typed as `&str`
	/// so the marshalled signature matches the server; passing an `ObjectPath` would send
	/// `o` and the call would fail.
	///
	/// See: [at-spi2-core XML definitions on Socket::Embedded](https://gitlab.gnome.org/GNOME/at-spi2-core/-/blob/main/xml/Socket.xml#L51-54)
	///
	/// member: `Embedded`, type: method
	fn embedded(&self, path: &str) -> zbus::Result<()>;

	/// Unregisters an application from the accessibility registry.
	///
	/// `plug` is an `(so)` reference (bus name + object path) to the application's root
	/// object. Calling this is optional: the registry also detects when an application
	/// disconnects from the bus.
	///
	/// member: `Unembed`, type: method
	fn unembed(&self, plug: &ObjectRefOwned) -> zbus::Result<()>;
}
