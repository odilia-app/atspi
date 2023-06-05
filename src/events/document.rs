use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Event.Document", assume_defaults = true)]
trait Document {}
