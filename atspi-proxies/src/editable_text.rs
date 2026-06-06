//! # `EditableTextProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.EditableText`
//! interface. This interface provides access to and modification of the text
//! content of editable text fields (such as text boxes or editors).
//!
//! ## How to obtain an `EditableTextProxy`
//!
//! Because `EditableText` is implemented on individual, variable nodes within
//! the UI-tree, you rarely instantiate it directly. Instead, there are three
//! idiomatic ways to obtain a proxy:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for a node, you can safely query
//! and convert it to an `EditableTextProxy` using the [`ProxyExt`][pe] trait:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = accessible_node.proxies().await?;
//! let editable_text = proxies.editable_text().await?;
//! ```
//!
//! Note that proxies obtained through [`ProxyExt`][pe] share their underlying
//! [`zbus::Connection`]. As a consequence, if the [`AccessibleProxy`][ap]'s underlying
//! connection is a P2P connection, the proxies obtained through [`ProxyExt`][pe]
//! will also share that same P2P `zbus::Connection`.
//!
//! ### 2. Fast resolution via [`AccessibilityConnection`][ac] (P2P)
//! If the `p2p` feature is enabled and you have an [`ObjectRef`][or], you can resolve
//! it directly via the [`AccessibilityConnection`][ac] for maximum performance:
//!
//! ```rust,ignore
//! let editable_text = connection.object_as_accessible(&object_ref).await?;
//! ```
//!
//! ### 3. Manual construction using the `builder`
//! If you know the exact D-Bus service destination and object path, you can
//! construct the proxy manually:
//!
//! ```rust,ignore
//! let editable_text = EditableTextProxy::builder(&connection)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .build()
//!     .await?;
//! ```
//!
//! [ac]: atspi-connection::AccessibilityConnection
//! [ap]: atspi-proxies::AccessibilityProxy
//! [pe]: crate::proxy_ext::ProxyExt
//! [or]: atspi_common::ObjectRef
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
