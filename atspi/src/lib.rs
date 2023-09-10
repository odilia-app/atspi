//! `atspi` root crate, which may bring in various subcrates:
//! * `atspi_common` (required)
//! * `atspi_proxies` (with use of `proxies` feature flag)
//! * `atspi_connection` (with use of `connection` feature flag)
//! * `zbus` re-export (with use of `zbus` feature flag)

#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code, rustdoc::all)]

#[cfg(all(not(feature = "async-std"), not(feature = "tokio")))]
compile_error!("You must specify at least one of the `async-std` or `tokio` features.");

pub use atspi_common::*;

#[cfg(feature = "proxies")]
pub use atspi_proxies as proxy;

#[cfg(feature = "connection")]
pub use atspi_connection as connection;

#[cfg(feature = "zbus")]
pub use zbus;
