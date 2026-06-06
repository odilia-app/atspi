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
//! ## Defaults
//!
//! The `Document` interface is implemented on individual, variable nodes within the
//! application's UI-tree. As a consequence, the object path varies per node and
//! no default path is applicable for this proxy.
//!
//! ## How to obtain a `DocumentProxy`
//!
//! There are two idiomatic ways to obtain a `DocumentProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for a document node, you can safely
//! query and convert it using the [`ProxyExt`][pe] trait:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = accessible_node.proxies().await?;
//! let document = proxies.document().await?;
//! ```
//!
//! All proxies obtained through [`ProxyExt`][pe] share their underlying
//! [`zbus::Connection`], inheriting any P2P configuration if applicable.
//!
//! ### 2. Manual construction using the `builder`
//! If you know the exact D-Bus service destination and object path, you can
//! construct the proxy manually:
//!
//! ```rust,ignore
//! let document = DocumentProxy::builder(&connection)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .build()
//!     .await?;
//! ```
//!
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy
//! [tp]: crate::text::TextProxy
//! [etp]: crate::editable_text::EditableTextProxy
//! [lc]: atspi_common::events::document::LoadCompleteEvent
//! [rl]: atspi_common::events::document::ReloadEvent

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
