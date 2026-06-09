//! # `DocumentProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Document`
//! interface.
//!
//! The `Document` interface provides access to a document's metadata and global
//! structure (such as page count, locale, and custom attributes). It is typically
//! implemented by document-viewing containers (like PDF readers, web pages, or
//! word processors).
//!
//! ## Document Events vs. Text Interfaces
//!
//! * **Document Lifecycle**: Unlike the [`TextProxy`][tp] or [`EditableTextProxy`][etp]
//!   interfaces (which deal with character-level reading and text manipulation),
//!   the `Document` interface represents the document container as a whole.
//! * **Events**: Its companion events (such as [`LoadCompleteEvent`][lc] or [`ReloadEvent`][rl]
//!   found in `atspi-common/events/document.rs`) focus on the lifecycle states of
//!   loading, saving, and rendering rather than active text editing.
//!
//! ## D-Bus Addressing
//!
//! Since this interface is implemented dynamically on individual nodes within an
//! application's UI-tree, its D-Bus addressing (the unique bus name and object path)
//! varies per node. There is no static, well-known service destination or object path
//! applicable; address details must be resolved dynamically at runtime.
//!
//! ## How to obtain a `DocumentProxy`
//!
//! There are two idiomatic ways to obtain a `DocumentProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for a document node, you can safely
//! query and convert it using the [`ProxyExt`][pe] trait:
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
//! // Establish an `AccessibleProxy` for the document node
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//! let accessible_node = obj_ref.into_accessible_proxy(&conn).await?;
//!
//! // Convert to `DocumentProxy` safely
//! let proxies = accessible_node.proxies().await?;
//! let document = proxies.document().await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! All proxies obtained through [`ProxyExt`][pe] share their underlying
//! [`zbus::Connection`], inheriting any P2P configuration if applicable.
//!
//! ### 2. Manual construction using the `builder`
//! If you know the exact D-Bus service destination and object path, you can
//! construct the proxy manually:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::document::DocumentProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! let bus_name = ":1.1001";
//! let object_path = "/org/a11y/atspi/accessible/root";
//!
//! let document = DocumentProxy::builder(&conn)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy
//! [tp]: crate::text::TextProxy
//! [etp]: crate::editable_text::EditableTextProxy
//! [lc]: https://docs.rs/atspi-common/latest/atspi_common/events/document/struct.LoadCompleteEvent.html
//! [rl]: https://docs.rs/atspi-common/latest/atspi_common/events/document/struct.ReloadEvent.html

use crate::common::TextSelection;

// `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/latest/source/src/proxy.rs#197-199>
#[zbus::proxy(interface = "org.a11y.atspi.Document", assume_defaults = false)]
pub trait Document {
	/// `GetTextSelections` method
	fn get_text_selections(&self) -> zbus::Result<Vec<TextSelection>>;

	/// `SetTextSelections` method
	fn set_text_selections(&self, selections: Vec<TextSelection>) -> zbus::Result<bool>;

	/// `GetAttributeValue` method
	fn get_attribute_value(&self, attributename: &str) -> zbus::Result<String>;

	/// `GetAttributes` method
	fn get_attributes(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

	/// `GetLocale` method
	fn get_locale(&self) -> zbus::Result<String>;

	/// `CurrentPageNumber` property
	#[zbus(property)]
	fn current_page_number(&self) -> zbus::Result<i32>;

	/// `PageCount` property
	#[zbus(property)]
	fn page_count(&self) -> zbus::Result<i32>;
}
