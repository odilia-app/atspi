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
//! ## D-Bus Addressing
//!
//! Since this interface is implemented dynamically on individual nodes within an
//! application's UI-tree, its D-Bus addressing (the unique bus name and object path)
//! varies per node. There is no static, well-known service destination or object path
//! applicable; address details must be resolved dynamically at runtime.
//!
//! ## How to obtain a `HyperlinkProxy`
//!
//! There are three idiomatic ways to obtain a `HyperlinkProxy`:
//!
//! ### 1. Resolved from a [`HypertextProxy`][ht] (Most Common)
//! Usually, you discover hyperlinks by querying a [`Hypertext`][ht] container.
//! Once you have a link's index, you can retrieve its object reference and resolve it:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! # use atspi_proxies::proxy_ext::ProxyExt;
//! # use atspi_proxies::accessible::ObjectRefExt;
//! # use atspi_common::{NonNullObjectRef, ObjectRefOwned};
//! use atspi_connection::{AccessibilityConnection, P2P};
//! use atspi_proxies::hyperlink::HyperlinkProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! // Establish an `AccessibleProxy` pointing to a node with hypertext
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//! let root_node = obj_ref.into_accessible_proxy(&conn).await?;
//! let proxies = root_node.proxies().await?;
//! let hypertext = proxies.hypertext().await?;
//!
//! // Get the reference of the first link in the hypertext container:
//! let link_ref = hypertext.get_link(0).await?;
//! let link_ref: NonNullObjectRef = link_ref.try_into().expect("No null reference expected");
//!
//! // Resolve it directly (P2P-aware):
//! let _hyperlink = a11y.object_as_accessible(&link_ref).await?;
//!
//! // Or manually instantiate using the builder:
//! let _hyperlink = HyperlinkProxy::builder(&conn)
//!     .destination(link_ref.name().clone())?
//!     .path(link_ref.path().clone())?
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! ### 2. Safe conversion via [`ProxyExt`][pe]
//! If you already have an [`AccessibleProxy`][ap] for a link node, you can safely
//! convert it using the [`ProxyExt`][pe] trait:
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
//! // Establish an `AccessibleProxy` for the link node
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//! let accessible_node = obj_ref.into_accessible_proxy(&conn).await?;
//!
//! let proxies = accessible_node.proxies().await?;
//! let hyperlink = proxies.hyperlink().await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! ### 3. Manual construction using the `builder`
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::hyperlink::HyperlinkProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! let bus_name = ":1.1001";
//! let object_path = "/org/a11y/atspi/accessible/root";
//!
//! let hyperlink = HyperlinkProxy::builder(&conn)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [`get_uri`]: HyperlinkProxy#method.get_uri
//! [`start_index`]: HyperlinkProxy#method.start_index
//! [`end_index`]: HyperlinkProxy#method.end_index
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy
//! [ht]: crate::hypertext::HypertextProxy

use atspi_common::object_ref::ObjectRefOwned;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
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
