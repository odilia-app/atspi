#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate static_assertions;
#[macro_use]
mod macros;

pub mod interface;
pub use interface::{Interface, InterfaceSet};
pub mod state;
pub use state::{State, StateSet};
pub mod cache;
pub use cache::CacheItem;
pub mod error;
pub use error::AtspiError;
pub mod events;
pub use events::{Event, GenericEvent};
mod role;
pub use role::Role;
mod relation_type;
pub use relation_type::RelationType;

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
