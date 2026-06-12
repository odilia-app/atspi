//! # `EditableTextProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.EditableText`
//! interface. This interface provides access to and modification of the text
//! content of editable text fields (such as text boxes or editors).
//!
//! ## D-Bus Addressing
//!
//! Since this interface is implemented dynamically on individual nodes within an
//! application's UI-tree, its D-Bus addressing (the unique bus name and object path)
//! varies per node. There is no static, well-known service destination or object path
//! applicable; address details must be resolved dynamically at runtime.
//!
//! ## How to obtain an `EditableTextProxy`
//!
//! There are three idiomatic ways to obtain an `EditableTextProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for a node, you can safely query
//! and convert it to an `EditableTextProxy` using the [`ProxyExt`][pe] trait:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::proxy_ext::ProxyExt;
//! use atspi_proxies::accessible::ObjectRefExt;
//! use atspi_common::ObjectRefOwned;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! // Establish an `AccessibleProxy` for the node
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//! let accessible_node = obj_ref.into_accessible_proxy(&conn).await?;
//!
//! // Get the associated interface proxies
//! let proxies = accessible_node.proxies().await?;
//! let editable_text = proxies.editable_text().await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! Note that proxies obtained through [`ProxyExt`][pe] share their underlying
//! [`zbus::Connection`]. As a consequence, if the [`AccessibleProxy`][ap]'s underlying
//! connection is a P2P connection, the proxies obtained through [`ProxyExt`][pe]
//! will also share that same P2P `zbus::Connection`.
//!
//! ### 2. Fast resolution via [`AccessibilityConnection`][ac] (P2P)
//! If the `p2p` feature is enabled and you have an [`ObjectRefOwned`][or], you can resolve
//! it directly via the [`AccessibilityConnection`][ac] for maximum performance:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::{AccessibilityConnection, P2P};
//! use atspi_common::ObjectRefOwned;
//! use atspi_proxies::proxy_ext::ProxyExt;
//!
//! # block_on( async {
//! let connection = AccessibilityConnection::new().await?;
//! let object_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//!
//! // Obtain the P2P connected `AccessibleProxy`
//! let accessible = connection.object_as_accessible(&object_ref).await?;
//!
//! // Convert to `EditableTextProxy`
//! let proxies = accessible.proxies().await?;
//! let _editable_text = proxies.editable_text().await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! ### 3. Manual construction using the `builder`
//! If you know the exact D-Bus service destination and object path, you can
//! construct the proxy manually:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::editable_text::EditableTextProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! let bus_name = ":1.1001";
//! let object_path = "/org/a11y/atspi/accessible/root";
//!
//! let editable_text = EditableTextProxy::builder(&conn)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [ac]: https://docs.rs/atspi-connection/latest/atspi_connection/struct.AccessibilityConnection.html
//! [ap]: crate::accessible::AccessibleProxy
//! [pe]: crate::proxy_ext::ProxyExt
//! [or]: atspi_common::object_ref::ObjectRefOwned
//! [tp]: crate::text::TextProxy

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
#[zbus::proxy(interface = "org.a11y.atspi.EditableText", assume_defaults = false)]
pub trait EditableText {
	/// `CopyText` method
	fn copy_text(&self, start_pos: i32, end_pos: i32) -> zbus::Result<()>;

	/// `CutText` method
	fn cut_text(&self, start_pos: i32, end_pos: i32) -> zbus::Result<bool>;

	/// `DeleteText` method
	fn delete_text(&self, start_pos: i32, end_pos: i32) -> zbus::Result<bool>;

	/// `InsertText` method
	fn insert_text(&self, position: i32, text: &str, length: i32) -> zbus::Result<bool>;

	/// `PasteText` method
	fn paste_text(&self, position: i32) -> zbus::Result<bool>;

	/// `SetTextContents` method
	fn set_text_contents(&self, new_contents: &str) -> zbus::Result<bool>;
}
