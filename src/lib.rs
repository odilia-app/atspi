#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code)]
// #![deny(clippy::missing_docs)]

#[cfg(feature = "accessible")]
pub mod accessible;
#[cfg(feature = "accessible")]
pub mod accessible_ext;

#[cfg(feature = "action")]
pub mod action;

#[cfg(feature = "application")]
pub mod application;

#[cfg(feature = "bus")]
pub mod bus;

#[cfg(feature = "cache")]
pub mod cache;

#[cfg(feature = "collection")]
pub mod collection;

#[cfg(feature = "component")]
pub mod component;

#[cfg(feature = "convertable")]
pub mod convertable;

#[cfg(feature = "device_event_controller")]
pub mod device_event_controller;

#[cfg(feature = "device_event_listener")]
pub mod device_event_listener;

#[cfg(feature = "document")]
pub mod document;

#[cfg(feature = "editable_text")]
pub mod editable_text;

#[cfg(feature = "events")]
pub mod events;
#[cfg(feature = "events")]
pub use events::EventBody;

#[cfg(feature = "hyperlink")]
pub mod hyperlink;

#[cfg(feature = "hypertext")]
pub mod hypertext;

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "processed")]
pub mod processed;

#[cfg(feature = "registry")]
pub mod registry;

#[cfg(feature = "selection")]
pub mod selection;

#[cfg(feature = "socket")]
pub mod socket;

#[cfg(feature = "table")]
pub mod table;

#[cfg(feature = "table_cell")]
pub mod table_cell;

#[cfg(feature = "text")]
pub mod text;

#[cfg(feature = "value")]
pub mod value;

// Hand-written connection module
#[cfg(feature = "connection")]
mod connection;
#[cfg(feature = "connection")]
pub use connection::*;

#[cfg(feature = "interfaces")]
mod interfaces;
#[cfg(feature = "interfaces")]
pub use interfaces::*;

#[cfg(feature = "state")]
mod state;
#[cfg(feature = "state")]
pub use state::*;

pub use zbus;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use zbus::zvariant::Type;

#[cfg(feature = "serde")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[cfg(not(feature = "serde"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Type)]
#[repr(u32)]
/// The relative coordinate type.
pub enum CoordType {
    /// In relation to the entire screen.
    Screen,
    /// In relation to only the window.
    Window,
    /// In relation to the parent of the element being checked.
    Parent,
}
