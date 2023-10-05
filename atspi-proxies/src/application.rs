//! # [`ApplicationProxy`]
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Application`
//! interface.
//!
//! `Application` is the interface which is implemented by each accessible application.
//! It is implemented for the root object of an application.
//!
//! It provides information about the application itself.
//!
//! ## Status
//!
//! A number of methods and properties of this interface have fallen in disuse or
//! are / may be deprecated in the future.
//!
//! * [`id`]
//! * [`set_id`]
//! * [`atspi_version`]
//! * [`get_locale`]
//!  
//! [`toolkit_name`] and [`version`] are still in use.
//!
//! See the documentation of the individual methods and properties for details.
//!
//! [`ApplicationProxy`]: crate::application::ApplicationProxy
//! [`id`]: ApplicationProxy#method.id
//! [`set_id`]: ApplicationProxy#method.set_id
//! [`atspi_version`]: ApplicationProxy#method.atspi_version
//! [`get_locale`]: ApplicationProxy#method.get_locale
//! [`toolkit_name`]: ApplicationProxy#method.toolkit_name
//! [`version`]: ApplicationProxy#method.version
//!

use crate::atspi_proxy;

/// `Application` is the interface which is implemented by each accessible application.
/// It is implemented for the root object of an application.
///
/// It provides information about the application itself.
///
/// ## Status
///
/// A number of methods and properties of this interface have fallen in disuse or
/// are / may be deprecated in the future.
///
/// * [`id`]
/// * [`set_id`]
/// * [`atspi_version`]
/// * [`get_locale`]
///  
/// [`toolkit_name`] and [`version`] are still in use.
///
/// See the documentation of the individual methods and properties for details.
///
/// [`id`]: ApplicationProxy#method.id
/// [`set_id`]: ApplicationProxy#method.set_id
/// [`atspi_version`]: ApplicationProxy#method.atspi_version
/// [`get_locale`]: ApplicationProxy#method.get_locale
/// [`toolkit_name`]: ApplicationProxy#method.toolkit_name
/// [`version`]: ApplicationProxy#method.version
///
#[atspi_proxy(interface = "org.a11y.atspi.Application", assume_defaults = true)]
trait Application {
	/// GetLocale method
	fn get_locale(&self, lctype: u32) -> zbus::Result<String>;

	/// retrieves AT-SPI2 version.
	///
	/// Applications are advised to return "2.1" here, thus that is what is what
	/// users should expect.
	///
	/// This was intended to be the version of the atspi interfaces
	/// that the application supports, but atspi will probably move to
	/// using versioned interface names instead.
	///
	/// member: "AtspiVersion", type: property
	#[dbus_proxy(property)]
	fn atspi_version(&self) -> zbus::Result<String>;

	/// Id property
	#[dbus_proxy(property)]
	fn id(&self) -> zbus::Result<i32>;

	/// Set ID property
	#[dbus_proxy(property)]
	fn set_id(&self, value: i32) -> zbus::Result<()>;

	/// ToolkitName property
	#[dbus_proxy(property)]
	fn toolkit_name(&self) -> zbus::Result<String>;

	/// Version property
	#[dbus_proxy(property)]
	fn version(&self) -> zbus::Result<String>;
}
