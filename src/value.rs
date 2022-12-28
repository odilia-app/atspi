//! # `DBus` interface proxy for: `org.a11y.atspi.Value`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Value.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Value", assume_defaults = true)]
trait Value {
    /// CurrentValue property
    #[dbus_proxy(property)]
    fn current_value(&self) -> zbus::Result<f64>;
    #[dbus_proxy(property)]
    fn set_current_value(&self, value: f64) -> zbus::Result<()>;

    /// MaximumValue property
    #[dbus_proxy(property)]
    fn maximum_value(&self) -> zbus::Result<f64>;

    /// MinimumIncrement property
    #[dbus_proxy(property)]
    fn minimum_increment(&self) -> zbus::Result<f64>;

    /// MinimumValue property
    #[dbus_proxy(property)]
    fn minimum_value(&self) -> zbus::Result<f64>;
}
