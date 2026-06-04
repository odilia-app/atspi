//! # `DeviceEventListener`
//!
//! `org.a11y.atspi.DeviceEventListener` provides ... TODO
//!
//! ## Defaults
//!
//! TODO

use crate::device_event_controller::DeviceEvent;

#[zbus::proxy(interface = "org.a11y.atspi.DeviceEventListener", assume_defaults = false)]
pub trait DeviceEventListener {
	/// `NotifyEvent` method
	fn notify_event(&self, event: &DeviceEvent<'_>) -> zbus::Result<bool>;
}
