#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate static_assertions;
#[macro_use]
pub(crate) mod macros;

pub mod interface;
pub use interface::{Interface, InterfaceSet};
pub mod state;
pub use state::{State, StateSet};
pub mod cache;
pub use cache::{CacheItem, LegacyCacheItem};
pub mod error;
pub use error::AtspiError;
pub mod events;
pub use events::{Event, GenericEvent};
mod role;
pub use role::Role;
mod relation_type;
pub use relation_type::RelationType;

use serde::{Deserialize, Serialize};
use zvariant::Type;

/// A pair of (`sender`, `object path with id`) which constitutes the fundemental parts of an Accessible object in `atspi`.
/// NOTE: If you update the name of this type alias, also update the constant in `atspi_macros::OBJECT_PAIR_NAME`.
pub type ObjectPair = (String, zvariant::OwnedObjectPath);

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
pub enum SortOrder {
	Invalid,
	Canonical,
	Flow,
	Tab,
	ReverseCanonical,
	ReverseFlow,
	ReverseTab,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
pub enum TreeTraversalType {
	RestrictChildren,
	RestrictSibling,
	Inorder,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum MatchType {
	Invalid,
	All,
	Any,
	NA,
	Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
/// The coordinate type encodes the frame of reference.
pub enum CoordType {
	/// In relation to the entire screen.
	Screen,
	/// In relation to only the window.
	Window,
	/// In relation to the parent of the element being checked.
	Parent,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
pub enum ClipType {
	Neither,
	Min,
	Max,
	Both,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
/// Level of granularity to get text of, in relation to a cursor position.
pub enum Granularity {
	/// Gives the character at the index of the cursor. With a line-style cursor (which is standard) this will get the chracter that appears after the cursor.
	Char,
	/// Gives the entire word in front of or which contains the cursor. TODO: confirm that it always chooses the word in front of the cursor.
	Word,
	/// Gives to entire sentence in fron of the or which contains the cursor. TODO: confirm that it always chooses the sentence after the cursor.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum ScrollType {
	TopLeft,
	BottomRight,
	TopEdge,
	BottomEdge,
	LeftEdge,
	RightEdge,
	Anywhere,
}

pub type MatcherArgs = (
	Vec<Role>,
	MatchType,
	std::collections::HashMap<String, String>,
	MatchType,
	InterfaceSet,
	MatchType,
);
