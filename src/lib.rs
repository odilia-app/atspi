#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code)]
// #![deny(clippy::missing_docs)]
#![allow(clippy::multiple_crate_versions)]

#[macro_use]
#[cfg(feature = "unstable_traits")]
extern crate static_assertions;

#[cfg(feature = "unstable_traits")]
use atspi_macros::atspi_proxy;

#[cfg(not(feature = "unstable_traits"))]
use zbus::dbus_proxy as atspi_proxy;

pub mod accessible;
#[cfg(feature = "unstable_traits")]
pub mod accessible_ext;
pub mod accessible_id;
pub use accessible_id::*;

pub mod action;
#[cfg(feature = "unstable_traits")]
pub mod action_ext;
pub mod application;
#[cfg(feature = "unstable_traits")]
pub mod application_ext;
pub mod bus;
pub mod cache;
#[cfg(feature = "unstable_traits")]
pub mod cache_ext;
pub mod collection;
#[cfg(feature = "unstable_traits")]
pub mod collection_ext;
pub mod component;
#[cfg(feature = "unstable_traits")]
pub mod component_ext;
#[cfg(feature = "unstable_traits")]
pub mod convertable;
pub mod device_event_controller;
#[cfg(feature = "unstable_traits")]
pub mod device_event_controller_ext;
pub mod device_event_listener;
#[cfg(feature = "unstable_traits")]
pub mod device_event_listener_ext;
pub mod document;
#[cfg(feature = "unstable_traits")]
pub mod document_ext;
pub mod editable_text;
#[cfg(feature = "unstable_traits")]
pub mod editable_text_ext;
pub mod events;
pub mod identify;
pub mod signify;
pub use events::{Event, EventBody};
pub mod hyperlink;
#[cfg(feature = "unstable_traits")]
pub mod hyperlink_ext;
pub mod hypertext;
#[cfg(feature = "unstable_traits")]
pub mod hypertext_ext;
pub mod image;
#[cfg(feature = "unstable_traits")]
pub mod image_ext;
pub mod registry;
#[cfg(feature = "unstable_traits")]
pub mod registry_ext;
pub mod selection;
#[cfg(feature = "unstable_traits")]
pub mod selection_ext;
pub mod socket;
#[cfg(feature = "unstable_traits")]
pub mod socket_ext;
pub mod table_cell;
#[cfg(feature = "unstable_traits")]
pub mod table_cell_ext;
pub mod table;
#[cfg(feature = "unstable_traits")]
pub mod table_ext;
pub mod text;
#[cfg(feature = "unstable_traits")]
pub mod text_ext;
pub mod value;
#[cfg(feature = "unstable_traits")]
pub mod value_ext;

// Hand-written connection module
mod connection;
pub use connection::*;

mod interfaces;
pub use interfaces::*;

mod state;
pub use state::*;

pub mod error;
pub use error::AtspiError;

pub use zbus;
use zbus::zvariant::Type;

use serde::{Deserialize, Serialize};

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

pub trait AtspiProxy {
	const INTERFACE: Interface;
}
