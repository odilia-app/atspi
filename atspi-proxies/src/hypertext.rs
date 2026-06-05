//! # `HypertextProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Hypertext`
//! interface.
//!
//! The `Hypertext` interface is implemented by text containers that contain embedded,
//! interactive links (such as web pages, HTML documents, or rich text fields). It
//! typically extends the standard [`TextProxy`][tp] interface.
//!
//! ## The Hypertext & Hyperlink Duo
//!
//! * **`Hypertext` (The Container)**: Manages the parent text block. It provides methods
//!   to query the total number of links ([`get_n_links`]), map character offsets to
//!   specific links ([`get_link_index`]), and retrieve the underlying D-Bus reference
//!   for a link ([`get_link`]).
//! * **[`Hyperlink`][hl] (The Link)**: Represents the individual interactive link itself.
//!   Once you obtain a link reference from `Hypertext`, you can instantiate a
//!   [`HyperlinkProxy`][hl] to query its URI, start/end boundaries, and target anchors.
//!
//! ## Defaults
//!
//! The `Hypertext` interface is implemented on individual, variable nodes within the
//! application's UI-tree. As a consequence, the object path varies per node and
//! no default path is applicable for this proxy.
//!
//! ## How to obtain a `HypertextProxy`
//!
//! There are two idiomatic ways to obtain a `HypertextProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for a hypertext node (such as a
//! paragraph in a browser), you can safely query and convert it:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = accessible_node.proxies().await?;
//! let hypertext = proxies.hypertext().await?;
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
//! let hypertext = HypertextProxy::builder(&connection)
//!     .destination(service_name)?
//!     .path(object_path)?
//!     .build()
//!     .await?;
//! ```
//!
//! [`get_n_links`]: HypertextProxy#method.get_n_links
//! [`get_link`]: HypertextProxy#method.get_link
//! [`get_link_index`]: HypertextProxy#method.get_link_index
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy
//! [tp]: crate::text::TextProxy
//! [hl]: crate::hyperlink::HyperlinkProxy

use atspi_common::object_ref::ObjectRefOwned;

// We explicitly disable the `assume_defaults` option to avoid generating default service/path defaults.
#[zbus::proxy(interface = "org.a11y.atspi.Hypertext", assume_defaults = false)]
pub trait Hypertext {
	/// `GetLink` method
	fn get_link(&self, link_index: i32) -> zbus::Result<ObjectRefOwned>;

	/// `GetLinkIndex` method
	fn get_link_index(&self, character_index: i32) -> zbus::Result<i32>;

	/// `GetNLinks` method
	#[zbus(name = "GetNLinks")]
	fn get_n_links(&self) -> zbus::Result<i32>;
}
