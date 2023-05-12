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
