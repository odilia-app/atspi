//! # `DeviceEventListenerProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.DeviceEventListener`
//! interface.
//!
//! The `DeviceEventListener` is the interface implemented **by the AT client itself**
//! (such as a screen reader) and registered with the `DeviceEventController`. When a
//! registered key or mouse event occurs, the registry calls [`notify_event`] on the
//! client's listener proxy.
//!
//! ## ⚠️ Legacy Status Warning
//!
//! **This interface is legacy and deprecated.**
//!
//! Because D-Bus-based global keylogging is a severe security vulnerability, and Wayland
//! strictly isolates input between applications, registering global device event listeners
//! is **no longer supported** on modern Linux systems.
//!
//! For modern application development, do not attempt to implement or call this interface.
//!
//! ## How to instantiate the proxy
//!
//! In rare legacy X11 testing environments, you would implement this trait on a local
//! D-Bus object to receive notifications. The generated proxy allows calling the listener
//! remotely:
//!
//! ```rust,ignore
//! let listener = DeviceEventListenerProxy::new(&connection, destination, path).await?;
//! ```
//!
//! [`notify_event`]: DeviceEventListenerProxy#method.notify_event

use crate::device_event_controller::DeviceEvent;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
#[zbus::proxy(interface = "org.a11y.atspi.DeviceEventListener", assume_defaults = false)]
pub trait DeviceEventListener {
	/// `NotifyEvent` method
	fn notify_event(&self, event: &DeviceEvent<'_>) -> zbus::Result<bool>;
}
