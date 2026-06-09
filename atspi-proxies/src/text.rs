//! # `TextProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Text`
//! interface.
//!
//! The `Text` interface is one of the most widely used AT-SPI2 interfaces. It provides
//! methods to query and manipulate accessible text content, including reading text
//! blocks or segments ([`get_text`], [`get_string_at_offset`]), managing text selections,
//! resolving character/range bounding box coordinates ([`get_character_extents`], [`get_range_extents`]),
//! manipulating the cursor ([`caret_offset`], [`set_caret_offset`]), and retrieving rich formatting
//! and style properties ([`get_attributes`]).
//!
//! ## D-Bus Addressing
//!
//! Since this interface is implemented dynamically on individual nodes within an
//! application's UI-tree, its D-Bus addressing (the unique bus name and object path)
//! varies per node. There is no static, well-known service destination or object path
//! applicable; address details must be resolved dynamically at runtime.
//!
//! ## How to obtain a `TextProxy`
//!
//! There are two idiomatic ways to obtain a `TextProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] pointing to a node containing text,
//! you can safely query and convert it using the [`ProxyExt`][pe] trait:
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
//! // Establish an `AccessibleProxy` for the text node
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//! let accessible_node = obj_ref.into_accessible_proxy(&conn).await?;
//!
//! let proxies = accessible_node.proxies().await?;
//! let text = proxies.text().await?;
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
//! use atspi_proxies::text::TextProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! let bus_name = ":1.1001";
//! let object_path = "/org/a11y/atspi/accessible/root";
//!
//! let text = TextProxy::builder(&conn)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .cache_properties(CacheProperties::No) // Disable property caching
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [`get_text`]: TextProxy#method.get_text
//! [`get_string_at_offset`]: TextProxy#method.get_string_at_offset
//! [`get_character_extents`]: TextProxy#method.get_character_extents
//! [`get_range_extents`]: TextProxy#method.get_range_extents
//! [`caret_offset`]: TextProxy#method.caret_offset
//! [`set_caret_offset`]: TextProxy#method.set_caret_offset
//! [`get_attributes`]: TextProxy#method.get_attributes
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy

#![allow(clippy::too_many_arguments)]
// this is to silence clippy due to zbus expanding parameter expressions

use crate::common::{ClipType, CoordType, Granularity};

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
#[zbus::proxy(interface = "org.a11y.atspi.Text", assume_defaults = false)]
pub trait Text {
	/// `AddSelection` method
	fn add_selection(&self, start_offset: i32, end_offset: i32) -> zbus::Result<bool>;

	/// `GetAttributeRun` method
	fn get_attribute_run(
		&self,
		offset: i32,
		include_defaults: bool,
	) -> zbus::Result<(std::collections::HashMap<String, String>, i32, i32)>;

	/// `GetAttributeValue` method
	fn get_attribute_value(&self, offset: i32, attribute_name: &str) -> zbus::Result<String>;

	/// `GetAttributes` method
	fn get_attributes(
		&self,
		offset: i32,
	) -> zbus::Result<(std::collections::HashMap<String, String>, i32, i32)>;

	/// `GetBoundedRanges` method
	fn get_bounded_ranges(
		&self,
		x: i32,
		y: i32,
		width: i32,
		height: i32,
		coord_type: CoordType,
		x_clip_type: ClipType,
		y_clip_type: ClipType,
	) -> zbus::Result<Vec<(i32, i32, String, zbus::zvariant::OwnedValue)>>;

	/// `GetCharacterAtOffset` method
	fn get_character_at_offset(&self, offset: i32) -> zbus::Result<i32>;

	/// `GetCharacterExtents` method
	fn get_character_extents(
		&self,
		offset: i32,
		coord_type: CoordType,
	) -> zbus::Result<(i32, i32, i32, i32)>;

	/// `GetDefaultAttributeSet` method
	fn get_default_attribute_set(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

	/// `GetDefaultAttributes` method
	fn get_default_attributes(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

	/// `GetNSelections` method
	#[zbus(name = "GetNSelections")]
	fn get_n_selections(&self) -> zbus::Result<i32>;

	/// `GetOffsetAtPoint` method
	fn get_offset_at_point(&self, x: i32, y: i32, coord_type: CoordType) -> zbus::Result<i32>;

	/// `GetRangeExtents` method
	fn get_range_extents(
		&self,
		start_offset: i32,
		end_offset: i32,
		coord_type: CoordType,
	) -> zbus::Result<(i32, i32, i32, i32)>;

	/// `GetSelection` method
	fn get_selection(&self, selection_num: i32) -> zbus::Result<(i32, i32)>;

	/// `GetStringAtOffset` method
	fn get_string_at_offset(
		&self,
		offset: i32,
		granularity: Granularity,
	) -> zbus::Result<(String, i32, i32)>;

	/// `GetText` method
	/// This should be called with explicitly known offsets. Calling with an arbitrary
	/// large offset can cause undefined behavior or no text to be returned.
	fn get_text(&self, start_offset: i32, end_offset: i32) -> zbus::Result<String>;

	/// `GetTextAfterOffset` method
	fn get_text_after_offset(&self, offset: i32, type_: u32) -> zbus::Result<(String, i32, i32)>;

	/// `GetTextAtOffset` method
	fn get_text_at_offset(&self, offset: i32, type_: u32) -> zbus::Result<(String, i32, i32)>;

	/// `GetTextBeforeOffset` method
	fn get_text_before_offset(&self, offset: i32, type_: u32) -> zbus::Result<(String, i32, i32)>;

	/// `RemoveSelection` method
	fn remove_selection(&self, selection_num: i32) -> zbus::Result<bool>;

	/// `ScrollSubstringTo` method
	fn scroll_substring_to(
		&self,
		start_offset: i32,
		end_offset: i32,
		type_: u32,
	) -> zbus::Result<bool>;

	/// `ScrollSubstringToPoint` method
	fn scroll_substring_to_point(
		&self,
		start_offset: i32,
		end_offset: i32,
		type_: u32,
		x: i32,
		y: i32,
	) -> zbus::Result<bool>;

	/// `SetCaretOffset` method
	fn set_caret_offset(&self, offset: i32) -> zbus::Result<bool>;

	/// `SetSelection` method
	fn set_selection(
		&self,
		selection_num: i32,
		start_offset: i32,
		end_offset: i32,
	) -> zbus::Result<bool>;

	/// `CaretOffset` property
	#[zbus(property)]
	fn caret_offset(&self) -> zbus::Result<i32>;

	/// `CharacterCount` property
	#[zbus(property)]
	fn character_count(&self) -> zbus::Result<i32>;
}
