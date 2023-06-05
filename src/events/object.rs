use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Event.Object", assume_defaults = true)]
trait Object {}
