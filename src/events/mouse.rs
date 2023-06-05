use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Event.Mouse", assume_defaults = true)]
trait Mouse {}
