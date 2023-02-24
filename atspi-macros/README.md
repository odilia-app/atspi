# Atspi-macros

Crate that is home to proc-macros used in `atspi`.

## Update Log

* `0.2.0`
  * Add custom implementations using an `atspi_proxy` macro, replacing the `zbus::dbus_proxy` macro. Note that internally, `zbus::dbus_proxy` is called on the item being identified with the macro.
    * Add auto-definition of `*` trait matching any trait used to generate a `*Proxy`.
    * Add auto-definition of `*Blocking` trait matching any trait used to generate a `*ProxyBlocking`.
    * Add auto-implementation of `*` trait for `*Proxy`.
    * Add auto-implementation of `*Blocking` trait for `*ProxyBlocking`.
