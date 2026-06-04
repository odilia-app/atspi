//! # `EditableText`
//!
//! `org.a11y.atspi.EditableText` provides access to the text content of an
//! editable text field.
//!
//! ## Defaults
//!
//! "org.a11y.atspi.EditableText" may be implemented for individual nodes
//! in the application's UI-tree.
//!
//! Service and path are either provided by the builder or inherited from the
//! [`zbus::Proxy`] this `DocumentProxy` is derived from.
//!
//! No default service or default path makes sense for this proxy, thus
//! the macro is instructed explicitly not to generate the defaults.

#[zbus::proxy(interface = "org.a11y.atspi.EditableText", assume_defaults = false)]
pub trait EditableText {
	/// `CopyText` method
	fn copy_text(&self, start_pos: i32, end_pos: i32) -> zbus::Result<()>;

	/// `CutText` method
	fn cut_text(&self, start_pos: i32, end_pos: i32) -> zbus::Result<bool>;

	/// `DeleteText` method
	fn delete_text(&self, start_pos: i32, end_pos: i32) -> zbus::Result<bool>;

	/// `InsertText` method
	fn insert_text(&self, position: i32, text: &str, length: i32) -> zbus::Result<bool>;

	/// `PasteText` method
	fn paste_text(&self, position: i32) -> zbus::Result<bool>;

	/// `SetTextContents` method
	fn set_text_contents(&self, new_contents: &str) -> zbus::Result<bool>;
}
