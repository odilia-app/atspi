use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Event.Terminal", assume_defaults = true)]
trait Terminal {}
