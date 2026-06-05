//! # `DeviceEventControllerProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.DeviceEventController`
//! interface.
//!
//! Historically, the `DeviceEventController` (DEC) interface, hosted by the central
//! AT-SPI registry daemon, allowed assistive technologies (like screen readers) to
//! intercept global hardware input events (keystrokes, mouse buttons, pointer moves)
//! and synthesize input events (keyboard and mouse macros).
//!
//! ## ⚠️ Deprecation & Wayland Warning (Legacy Only)
//!
//! **This interface is legacy and largely obsolete in modern Linux desktop environments.**
//!
//! 1. **Wayland Incompatibility**: Under Wayland, security protocols strictly prevent ordinary
//!    D-Bus clients from listening globally to user keystrokes (keylogging) or injecting
//!    synthetic input. Therefore, this interface is **non-functional on Wayland**.
//! 2. **Disabled in Core**: In modern versions of `at-spi2-core` (version 2.46+), general
//!    device event listeners (`register_device_event_listener`) have been **completely
//!    removed and disabled** due to security concerns (see GNOME `at-spi2-core` issue #94).
//! 3. **What Orca does**: While Orca historically used `RegisterKeystrokeListener` on X11 to
//!    intercept its modifier shortcuts, on modern Wayland systems it relies on compositor-native
//!    key-grabbing protocols (such as GNOME Mutter's key-binding interfaces) rather than AT-SPI.
//!
//! For modern input synthesis, use secure alternatives like **`libei`** (Emulated Input).
//! For global shortcuts, use compositor-native portal APIs.
//!
//! ## Defaults
//!
//! Since the Device Event Controller is a single global daemon hosted by the AT-SPI Registry,
//! it has a fixed D-Bus address on the accessibility bus:
//!
//! * **Service Destination**: `org.a11y.atspi.Registry`
//! * **Object Path**: `/org/a11y/atspi/registry/deviceeventcontroller`
//!
//! Due to these fixed defaults, the proxy can be constructed directly using `new` or `builder`.
//!
//! ## How to instantiate the proxy (X11 only)
//!
//! ```rust,ignore
//! // Note: This will only work on legacy X11 sessions where the registry daemon supports it.
//! let dec = DeviceEventControllerProxy::new(&connection).await?;
//! ```

use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
pub enum EventType {
	KeyPressed,
	KeyReleased,
	ButtonPressed,
	ButtonReleased,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
pub enum KeySynthType {
	Press,
	Release,
	Pressrelease,
	Sym,
	String,
	Lockmodifiers,
	Unlockmodifiers,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct DeviceEvent<'a> {
	pub event_type: EventType,
	pub id: i32,
	pub hw_code: i32,
	pub modifiers: i32,
	pub timestamp: i32,
	pub event_string: &'a str,
	pub is_text: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct EventListenerMode {
	/// Whether events are delivered synchronously, before the currently focused application sees them.
	/// If `false`, events may be delivered asynchronously, which means in some
	/// cases they may already have been delivered to the
	/// application before the AT client receives the notification.
	pub synchronous: bool,
	/// Whether events may be consumed by the AT client.
	/// Requires [`EventListenerMode::synchronous`] to be set to `true`.
	pub preemptive: bool,
	/// If `true`, indicates that events are received not from the application toolkit layer,
	/// but from the device driver or windowing system subsystem.
	pub global: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct KeyDefinition<'a> {
	pub keycode: i32,
	pub keysym: i32,
	pub keystring: &'a str,
	pub unused: i32,
}

#[zbus::proxy(
	interface = "org.a11y.atspi.DeviceEventController",
	default_path = "/org/a11y/atspi/registry/deviceeventcontroller",
	default_service = "org.a11y.atspi.Registry",
	assume_defaults = false
)]
pub trait DeviceEventController {
	/// `DeregisterDeviceEventListener` method
	fn deregister_device_event_listener(
		&self,
		listener: &zbus::zvariant::ObjectPath<'_>,
		types: EventType,
	) -> zbus::Result<()>;

	/// `DeregisterKeystrokeListener` method
	fn deregister_keystroke_listener(
		&self,
		listener: &zbus::zvariant::ObjectPath<'_>,
		keys: &[KeyDefinition<'_>],
		mask: u32,
		type_: EventType,
	) -> zbus::Result<()>;

	/// `GenerateKeyboardEvent` method
	fn generate_keyboard_event(
		&self,
		keycode: i32,
		keystring: &str,
		type_: KeySynthType,
	) -> zbus::Result<()>;

	/// `GenerateMouseEvent` method
	fn generate_mouse_event(&self, x: i32, y: i32, event_name: &str) -> zbus::Result<()>;

	/// `NotifyListenersAsync` method
	fn notify_listeners_async(&self, event: &DeviceEvent<'_>) -> zbus::Result<()>;

	/// `NotifyListenersSync` method
	fn notify_listeners_sync(&self, event: &DeviceEvent<'_>) -> zbus::Result<bool>;

	/// `RegisterDeviceEventListener` method
	fn register_device_event_listener(
		&self,
		listener: &zbus::zvariant::ObjectPath<'_>,
		types: EventType,
	) -> zbus::Result<bool>;

	/// `RegisterKeystrokeListener` method
	fn register_keystroke_listener(
		&self,
		listener: &zbus::zvariant::ObjectPath<'_>,
		keys: &[KeyDefinition<'_>],
		mask: u32,
		type_: &[EventType],
		mode: &EventListenerMode,
	) -> zbus::Result<bool>;
}
