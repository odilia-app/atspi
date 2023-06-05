use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Event.Window", assume_defaults = true)]
trait Window {}
