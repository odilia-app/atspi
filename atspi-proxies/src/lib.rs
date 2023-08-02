#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code)]
#![allow(clippy::multiple_crate_versions)]

#[cfg(all(not(feature = "async-std"), not(feature = "tokio")))]
compile_error!("You must specify at least one of the `async-std` or `tokio` features.");

pub use atspi_common as common;
use zbus::dbus_proxy as atspi_proxy;

pub mod accessible;
pub mod action;
pub mod application;
pub mod bus;
pub mod cache;
pub mod collection;
pub mod component;
pub mod device_event_controller;
pub mod device_event_listener;
pub mod document;
pub mod editable_text;
pub use common::{events, AtspiError, CoordType, Interface, InterfaceSet};

pub mod hyperlink;
pub mod hypertext;
pub mod image;
pub mod registry;
pub mod selection;
pub mod socket;
pub mod table;
pub mod table_cell;
pub mod text;
pub mod value;

pub use zbus;

pub trait AtspiProxy {
	const INTERFACE: Interface;
}
