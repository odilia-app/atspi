//! # `DBus` interface proxy for: `org.a11y.atspi.Socket`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Socket.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use crate::atspi_proxy;
use crate::common::ObjectRef;

#[atspi_proxy(
	interface = "org.a11y.atspi.Socket",
	default_path = "/org/a11y/atspi/accessible/root",
	default_service = "org.a11y.atspi.Registry"
)]
trait Socket {
	/// Embed method
	fn embed(&self, plug: &(&str, zbus::zvariant::ObjectPath<'_>)) -> zbus::Result<ObjectRef>;

	/// Unembed method
	fn unembed(&self, plug: &(&str, zbus::zvariant::ObjectPath<'_>)) -> zbus::Result<()>;
}
