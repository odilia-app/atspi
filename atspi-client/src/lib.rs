#[cfg(all(feature = "async-std", feature = "tokio"))]
compile_error!("You may not mix the async-std and tokio features.");

#[cfg(all(not(feature = "async-std"), not(feature = "tokio")))]
compile_error!("You must specify either the async-std or tokio feature.");

#[macro_use]
extern crate static_assertions;

#[macro_use]
pub mod macros;

pub mod accessible_ext;
pub mod action_ext;
pub mod application_ext;
pub mod cache_ext;
pub mod collection_ext;
pub mod component_ext;
pub mod convertable;
pub mod device_event_controller_ext;
pub mod device_event_listener_ext;
pub mod document_ext;
pub mod editable_text_ext;
pub mod hyperlink_ext;
pub mod hypertext_ext;
pub mod image_ext;
pub mod registry_ext;
pub mod selection_ext;
pub mod socket_ext;
pub mod table_cell_ext;
pub mod table_ext;
pub mod text_ext;
pub mod value_ext;
