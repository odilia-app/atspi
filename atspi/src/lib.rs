#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code)]
// #![deny(clippy::missing_docs)]
#![allow(clippy::multiple_crate_versions)]

#[macro_use]
#[cfg(feature = "unstable-traits")]
extern crate static_assertions;

#[cfg(feature = "unstable-traits")]
use atspi_macros::atspi_proxy;

#[cfg(not(feature = "unstable-traits"))]
use zbus::dbus_proxy as atspi_proxy;

pub mod accessible;
#[cfg(feature = "unstable-traits")]
pub mod accessible_ext;

pub mod action;
#[cfg(feature = "unstable-traits")]
pub mod action_ext;
pub mod application;
#[cfg(feature = "unstable-traits")]
pub mod application_ext;
pub mod bus;
pub mod cache;
#[cfg(feature = "unstable-traits")]
pub mod cache_ext;
pub mod collection;
#[cfg(feature = "unstable-traits")]
pub mod collection_ext;
pub mod component;
#[cfg(feature = "unstable-traits")]
pub mod component_ext;
#[cfg(feature = "unstable-traits")]
pub mod convertable;
pub mod device_event_controller;
#[cfg(feature = "unstable-traits")]
pub mod device_event_controller_ext;
pub mod device_event_listener;
#[cfg(feature = "unstable-traits")]
pub mod device_event_listener_ext;
pub mod document;
#[cfg(feature = "unstable-traits")]
pub mod document_ext;
pub mod editable_text;
#[cfg(feature = "unstable-traits")]
pub mod editable_text_ext;
pub mod events;
pub mod hyperlink;
#[cfg(feature = "unstable-traits")]
pub mod hyperlink_ext;
pub mod hypertext;
#[cfg(feature = "unstable-traits")]
pub mod hypertext_ext;
pub mod image;
#[cfg(feature = "unstable-traits")]
pub mod image_ext;
pub mod registry;
#[cfg(feature = "unstable-traits")]
pub mod registry_ext;
pub mod selection;
#[cfg(feature = "unstable-traits")]
pub mod selection_ext;
pub mod socket;
#[cfg(feature = "unstable-traits")]
pub mod socket_ext;
pub mod table;
pub mod table_cell;
#[cfg(feature = "unstable-traits")]
pub mod table_cell_ext;
#[cfg(feature = "unstable-traits")]
pub mod table_ext;
pub mod text;
#[cfg(feature = "unstable-traits")]
pub mod text_ext;
pub mod value;
#[cfg(feature = "unstable-traits")]
pub mod value_ext;

// Hand-written connection module
#[cfg(feature = "client-accessories")]
mod accessibility_connection;
#[cfg(feature = "client-accessories")]
pub use accessibility_connection::*;

pub use atspi_types::*;
pub use interfaces::*;
pub use state::*;
pub use errors::AtspiError;

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
