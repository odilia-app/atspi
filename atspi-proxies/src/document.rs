//! # `DocumentProxy`
//!
//! `org.a11y.atspi.Document` provides access to the text content of a document.
//!
//! ## Defaults
//!
//! "org.a11y.atspi.Document" may be implemented for individual nodes in the
//! application's UI-tree.
//!
//! Service and path are either provided by the builder or inherited from the
//! [`zbus::Proxy`] this `DocumentProxy` is derived from.
//!
//! No default service or default path makes sense for this proxy, thus
//! the macro is instructed explicitly not to generate the defaults.
//!

use crate::common::TextSelection;

#[zbus::proxy(interface = "org.a11y.atspi.Document", assume_defaults = false)]
pub trait Document {
	/// `GetTextSelections` method
	fn get_text_selections(&self) -> zbus::Result<Vec<TextSelection>>;

	/// `SetTextSelections` method
	fn set_text_selections(&self, selections: Vec<TextSelection>) -> zbus::Result<bool>;

	/// `GetAttributeValue` method
	fn get_attribute_value(&self, attributename: &str) -> zbus::Result<String>;

	/// `GetAttributes` method
	fn get_attributes(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

	/// `GetLocale` method
	fn get_locale(&self) -> zbus::Result<String>;

	/// `CurrentPageNumber` property
	#[zbus(property)]
	fn current_page_number(&self) -> zbus::Result<i32>;

	/// `PageCount` property
	#[zbus(property)]
	fn page_count(&self) -> zbus::Result<i32>;
}
