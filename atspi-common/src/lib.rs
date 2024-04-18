#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code, rustdoc::all)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]

//! # atspi-common
//!
//! Defines all common types, events, and data structures for `atspi-proxies` and `atspi-connection`.
//! Since `atspi-proxies` and `atspi-connection` are downstream crates, the documentation can not link to it directly.
//! Any type ending in `*Proxy` is in `atspi-proxies`.
//!

#[macro_use]
extern crate static_assertions;
#[macro_use]
pub(crate) mod macros;

pub mod object_ref;
pub use object_ref::ObjectRef;
pub mod interface;
pub use interface::{Interface, InterfaceSet};
pub mod state;
pub use state::{State, StateSet};
pub mod cache;
pub use cache::{CacheItem, LegacyCacheItem};
pub mod error;
pub use error::AtspiError;
pub mod events;
pub use events::{BusProperties, Event, EventProperties, EventTypeProperties};
mod role;
pub use role::Role;
mod relation_type;
pub use relation_type::RelationType;

use serde::{Deserialize, Serialize};
#[cfg(feature = "strum")]
use strum::{Display, FromRepr, IntoStaticStr};
use zvariant::Type;

pub type Result<T> = std::result::Result<T, AtspiError>;

pub type MatchArgs<'a> = (
	&'a [i32],
	MatchType,
	std::collections::HashMap<&'a str, &'a str>,
	MatchType,
	&'a [i32],
	MatchType,
	&'a [&'a str],
	MatchType,
	bool,
);

/// Enumeration used by interface `CollectionProxy` to specify the way [`ObjectRef`]
/// objects should be sorted.
///
/// [`ObjectRef`]: crate::object_ref::ObjectRef
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Type)]
#[repr(u32)]
#[cfg_attr(feature = "strum", derive(Display, FromRepr, IntoStaticStr))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum SortOrder {
	/// Invalid sort order
	Invalid,
	/// Canonical sort order
	Canonical,
	/// Flow sort order
	Flow,
	/// Tab sort order
	Tab,
	/// Reverse canonical sort order
	ReverseCanonical,
	/// Reverse flow sort order
	ReverseFlow,
	/// Reverse tab sort order
	ReverseTab,
}

/// Method of traversing a tree in the `CollectionProxy`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Type)]
#[repr(u32)]
#[cfg_attr(feature = "strum", derive(Display, FromRepr, IntoStaticStr))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum TreeTraversalType {
	/// Restrict children tree traversal
	RestrictChildren,
	/// Restrict sibling tree traversal
	RestrictSibling,
	/// In-order tree traversal.
	Inorder,
}

/// Enumeration used by [`MatchArgs`] to specify how to interpret [`ObjectRef`] objects.
///
/// [`ObjectRef`]: crate::object_ref::ObjectRef
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Type)]
#[repr(i32)]
#[cfg_attr(feature = "strum", derive(Display, FromRepr, IntoStaticStr))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum MatchType {
	/// Invalid match type
	Invalid,
	/// true if all of the criteria are met.
	All,
	/// true if any of the criteria are met.
	Any,
	/// true if none of the criteria are met.
	NA,
	/// Same as [`Self::All`] if the criteria is non-empty;
	/// for empty criteria this rule requires returned value to also have empty set.
	Empty,
}

/// The coordinate type encodes the frame of reference.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Type)]
#[repr(u32)]
#[cfg_attr(feature = "strum", derive(Display, FromRepr, IntoStaticStr))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum CoordType {
	/// In relation to the entire screen.
	Screen,
	/// In relation to only the window.
	Window,
	/// In relation to the parent of the element being checked.
	Parent,
}

/// Enumeration used by `TextProxy` to indicate how to treat characters intersecting bounding boxes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Type)]
#[repr(u32)]
#[cfg_attr(feature = "strum", derive(Display, FromRepr, IntoStaticStr))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum ClipType {
	/// No characters/glyphs are omitted.
	Neither,
	/// Characters/glyphs clipped by the minimum coordinate are omitted.
	Min,
	/// Characters/glyphs which intersect the maximum coordinate are omitted.
	Max,
	/// Only glyphs falling entirely within the region bounded by min and max are retained.
	Both,
}

/// Level of granularity to get text of, in relation to a cursor position.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Type)]
#[repr(u32)]
#[cfg_attr(feature = "strum", derive(Display, FromRepr, IntoStaticStr))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum Granularity {
	/// Gives the character at the index of the cursor. With a line-style cursor (which is standard) this will get the character that appears after the cursor.
	Char,
	/// Gives the entire word in front of, or which contains, the cursor. TODO: confirm that it always chooses the word in front of the cursor.
	Word,
	/// Gives entire sentence in front of, or which contains, the cursor. TODO: confirm that it always chooses the sentence after the cursor.
	Sentence,
	/// Gives the line, as seen visually of which the cursor is situated within.
	Line,
	/// Gives the entire block of text, regardless of where the cursor lies within it.
	Paragraph,
}

/// Indicates relative stacking order of a `atspi_proxies::component::ComponentProxy` with respect to the
/// onscreen visual representation of the UI.
///
/// The layer index, in combination with the component's extents,
/// can be used to compute the visibility of all or part of a component.
/// This is important in programmatic determination of region-of-interest for magnification,
/// and in flat screen review models of the screen, as well as for other uses.
/// Objects residing in two of the `Layer` categories support further z-ordering information,
/// with respect to their peers in the same layer:
/// namely, [`Layer::Window`] and [`Layer::Mdi`].
/// Relative stacking order for other objects within the same layer is not available;
/// the recommended heuristic is first child paints first. In other words,
/// assume that the first siblings in the child list are subject to being
/// overpainted by later siblings if their bounds intersect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Type)]
#[repr(u32)]
#[cfg_attr(feature = "strum", derive(Display, FromRepr, IntoStaticStr))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum Layer {
	/// Indicates an error condition or uninitialized value.
	Invalid,
	/// Reserved for the desktop background; this is the bottom-most layer,
	/// over which everything else is painted.
	Background,
	/// The 'background' layer for most content renderers and
	/// UI `atspi_proxies::component::ComponentProxy` containers.
	Canvas,
	/// The layer in which the majority of ordinary 'foreground' widgets reside.
	Widget,
	/// A special layer between [`Layer::Canvas`] and [`Layer::Widget`], in which the
	/// 'pseudo windows' (e.g. the Multiple-Document Interface frames) reside.
	///
	/// See `atspi_proxies::component::ComponentProxy::get_mdizorder`.
	Mdi,
	/// A layer for popup window content, above [`Layer::Widget`].
	Popup,
	/// The topmost layer.
	Overlay,
	/// The layer in which a toplevel window background usually resides.
	Window,
}

/// Enumeration used by interface the [`crate::interface::Interface::Accessible`] to specify where an object should be placed on the screen when using `scroll_to`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Type)]
#[repr(u32)]
#[cfg_attr(feature = "strum", derive(Display, FromRepr, IntoStaticStr))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum ScrollType {
	/// Scroll the object to the top left corner of the window.
	TopLeft,
	/// Scroll the object to the bottom right corner of the window.
	BottomRight,
	/// Scroll the object to the top edge of the window.
	TopEdge,
	/// Scroll the object to the bottom edge of the window.
	BottomEdge,
	/// Scroll the object to the left edge of the window.
	LeftEdge,
	/// Scroll the object to the right edge of the window.
	RightEdge,
	/// Scroll the object to application-dependent position on the window.
	Anywhere,
}

/// Enumeration used to indicate a type of live region and how assertive it
/// should be in terms of speaking notifications. Currently, this is only used
/// for `Announcement` events, but it may be used for additional purposes
/// in the future.
/// The argument in the `Announcement` event is named `politeness`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, Type)]
#[repr(i32)]
#[cfg_attr(feature = "strum", derive(Display, FromRepr, IntoStaticStr))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum Live {
	/// No live region.
	#[default]
	None,
	/// This live region should be considered polite.
	Polite,
	/// This live region should be considered assertive.
	Assertive,
}

#[cfg(test)]
mod tests {
	use super::*;
	use zbus_lockstep::{
		method_args_signature, method_return_signature, signal_body_type_signature,
	};

	#[test]
	fn convert_i32_to_live() {
		assert_eq!(Live::None, Live::from_repr(0).unwrap());
		assert_eq!(Live::Polite, Live::from_repr(1).unwrap());
		assert_eq!(Live::Assertive, Live::from_repr(2).unwrap());
		assert!(Live::from_repr(3).is_none());
		assert!(Live::from_repr(-1).is_none());
	}

	#[test]
	fn validate_live_signature() {
		let signature = signal_body_type_signature!("Announcement");
		let politeness_signature = signature.slice(1..2);
		assert_eq!(Live::signature(), politeness_signature);
	}

	#[test]
	fn validate_scroll_type_signature() {
		let signature = method_args_signature!(member: "ScrollTo", interface: "org.a11y.atspi.Component", argument: "type");
		assert_eq!(ScrollType::signature(), signature);
	}

	#[test]
	fn validate_layer_signature() {
		let signature = method_return_signature!("GetLayer");
		assert_eq!(Layer::signature(), signature);
	}

	#[test]
	fn validate_granularity_signature() {
		let signature = method_args_signature!(member: "GetStringAtOffset", interface: "org.a11y.atspi.Text", argument: "granularity");
		assert_eq!(Granularity::signature(), signature);
	}

	#[test]
	fn validate_clip_type_signature() {
		let signature = method_args_signature!(member: "GetTextAtOffset", interface: "org.a11y.atspi.Text", argument: "type");
		assert_eq!(ClipType::signature(), signature);
	}

	#[test]
	fn validate_coord_type_signature() {
		let signature = method_args_signature!(member: "GetImagePosition", interface: "org.a11y.atspi.Image", argument: "coordType");
		assert_eq!(CoordType::signature(), signature);
	}

	#[test]
	fn validate_match_type_signature() {
		let rule_signature = method_args_signature!(member: "GetMatchesTo", interface: "org.a11y.atspi.Collection", argument: "rule");
		let match_type_signature = rule_signature.slice(3..4);
		assert_eq!(MatchType::signature(), match_type_signature);
	}

	#[test]
	fn validate_tree_traversal_type_signature() {
		let signature = method_args_signature!(member: "GetMatchesTo", interface: "org.a11y.atspi.Collection", argument: "tree");
		assert_eq!(TreeTraversalType::signature(), signature);
	}

	#[test]
	fn validate_sort_order_signature() {
		let signature = method_args_signature!(member: "GetMatches", interface: "org.a11y.atspi.Collection", argument: "sortby");
		assert_eq!(SortOrder::signature(), signature);
	}
}
