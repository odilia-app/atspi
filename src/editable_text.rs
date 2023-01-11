//! # `DBus` interface proxy for: `org.a11y.atspi.EditableText`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `EditableText.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.EditableText", assume_defaults = true)]
trait EditableText {
    /// CopyText method
    fn copy_text(&self, start_pos: i32, end_pos: i32) -> zbus::Result<()>;

    /// CutText method
    fn cut_text(&self, start_pos: i32, end_pos: i32) -> zbus::Result<bool>;

    /// DeleteText method
    fn delete_text(&self, start_pos: i32, end_pos: i32) -> zbus::Result<bool>;

    /// InsertText method
    fn insert_text(&self, position: i32, text: &str, length: i32) -> zbus::Result<bool>;

    /// PasteText method
    fn paste_text(&self, position: i32) -> zbus::Result<bool>;

    /// SetTextContents method
    fn set_text_contents(&self, new_contents: &str) -> zbus::Result<bool>;
}
use crate::{AtspiProxy, Interface};
impl<'a> AtspiProxy for EditableTextProxy<'a> {
    const INTERFACE: Interface = Interface::EditableText;
}
