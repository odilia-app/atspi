#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code)]
// #![deny(clippy::missing_docs)]
#![allow(clippy::multiple_crate_versions)]

#[macro_use]
extern crate static_assertions;

pub mod accessible;
pub mod accessible_ext;
pub mod accessible_id;
pub use accessible_id::*;

pub mod action;
pub mod action_ext;
pub mod application;
pub mod application_ext;
pub mod bus;
pub mod cache;
pub mod cache_ext;
pub mod collection;
pub mod collection_ext;
pub mod component;
pub mod component_ext;
pub mod convertable;
pub mod device_event_controller;
pub mod device_event_controller_ext;
pub mod device_event_listener;
pub mod device_event_listener_ext;
pub mod document;
pub mod document_ext;
pub mod editable_text;
pub mod editable_text_ext;
pub mod events;
pub mod identify;
pub mod signify;
pub use events::{Event, EventBody};
pub mod hyperlink;
pub mod hyperlink_ext;
pub mod hypertext;
pub mod hypertext_ext;
pub mod image;
pub mod image_ext;
pub mod registry;
pub mod registry_ext;
pub mod selection;
pub mod selection_ext;
pub mod socket;
pub mod socket_ext;
pub mod table;
pub mod table_cell;
pub mod table_cell_ext;
pub mod table_ext;
pub mod text;
pub mod text_ext;
pub mod value;
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
