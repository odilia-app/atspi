//! # `HyperlinkProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Hyperlink`
//! interface.
//!
//! The `Hyperlink` interface represents an individual, interactive link embedded
//! within a [`Hypertext`][ht] container (such as an `<a>` tag on a web page).
//!
//! ## The Hypertext & Hyperlink Duo
//!
//! * **[`Hypertext`][ht] (The Container)**: Manages the flow of text and tells you
//!   which characters are part of a link.
//! * **`Hyperlink` (The Link)**: Provides metadata about the link itself, such as
//!   its target URI ([`get_uri`]), its start index in the parent text ([`start_index`]),
//!   and its end index ([`end_index`]).
//!
//! ## Defaults
//!
//! The `Hyperlink` interface is implemented on individual, variable nodes within the
//! application's UI-tree. As a consequence, the object path varies per node and
//! no default path is applicable for this proxy.
//!
//! ## How to obtain a `HyperlinkProxy`
//!
//! There are three idiomatic ways to obtain a `HyperlinkProxy`:
//!
//! ### 1. Resolved from a [`HypertextProxy`][ht] (Most Common)
//! Usually, you discover hyperlinks by querying a [`Hypertext`][ht] container.
//! Once you have a link's index, you can retrieve its object reference and resolve it:
//!
//! ```rust,ignore
//! // Get the reference of the first link in the hypertext container:
//! let link_ref = hypertext.get_link(0).await?;
//!
//! // Resolve it directly (P2P-aware):
//! let hyperlink = connection.object_as_accessible(&link_ref).await?;
//!
//! // Or manually instantiate using the builder:
//! let hyperlink = HyperlinkProxy::builder(&connection)
//!     .destination(link_ref.name)?
//!     .path(link_ref.path)?
//!     .build()
//!     .await?;
//! ```
//!
//! ### 2. Safe conversion via [`ProxyExt`][pe]
//! If you already have an [`AccessibleProxy`][ap] for a link node, you can safely
//! convert it using the [`ProxyExt`][pe] trait:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = accessible_node.proxies().await?;
//! let hyperlink = proxies.hyperlink().await?;
//! ```
//!
//! ### 3. Manual construction using the `builder`
//!
//! ```rust,ignore
//! let hyperlink = HyperlinkProxy::builder(&connection)
//!     .destination(service_name)?
//!     .path(object_path)?
//!     .build()
//!     .await?;
//! ```
//!
//! [`get_uri`]: HyperlinkProxy#method.get_uri
//! [`start_index`]: HyperlinkProxy#method.start_index
//! [`end_index`]: HyperlinkProxy#method.end_index
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy
//! [ht]: crate::hypertext::HypertextProxy

use atspi_common::object_ref::ObjectRefOwned;

// We explicitly disable the `assume_defaults` option to avoid generating default service/path methods.
#[zbus::proxy(interface = "org.a11y.atspi.Hyperlink", assume_defaults = false)]
pub trait Hyperlink {
	/// `GetObject` method
	fn get_object(&self, i: i32) -> zbus::Result<ObjectRefOwned>;

	/// `GetURI` method
	#[zbus(name = "GetURI")]
	fn get_uri(&self, i: i32) -> zbus::Result<String>;

	/// `IsValid` method
	fn is_valid(&self) -> zbus::Result<bool>;

	/// `EndIndex` property
	#[zbus(property)]
	fn end_index(&self) -> zbus::Result<i32>;

	/// `NAnchors` property
	#[zbus(property, name = "NAnchors")]
	fn n_anchors(&self) -> zbus::Result<i16>;

	/// `StartIndex` property
	#[zbus(property)]
	fn start_index(&self) -> zbus::Result<i32>;
}
