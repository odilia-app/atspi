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
	/// Method to retrieve the application's locale.
	///
	/// ## Deprecation
	///  
	/// This method is likely to be removed in the future.
	///
	/// There is no need to call this method because there is also
	/// [`locale`] which offers the same functionality
	/// at the accessible object level.
	///
	/// See also: [Orca issues: "Plans for per-object locale?"](<https://gitlab.gnome.org/GNOME/orca/-/issues/260>)
	///
	/// member: "GetLocale", type: method
	///
	/// [`locale`]: crate::accessible::AccessibleProxy#method.locale
	fn get_locale(&self, lctype: u32) -> zbus::Result<String>;

	/// retrieves AT-SPI2 version.
	///
	/// Applications are advised to return "2.1" here, thus that is what
	/// users should expect.
	///
	/// This was intended to be the version of the atspi interfaces
	/// that the application supports, but atspi will probably move to
	/// using versioned interface names instead.
	///
	/// member: "AtspiVersion", type: property
	#[dbus_proxy(property)]
	fn atspi_version(&self) -> zbus::Result<String>;

	/// Retrieve numerical id of the application.
	///
	/// The 'id' is set an arbitrary numerical id when
	/// an application registers with the registry.
	///
	/// When a freshly-started application uses the
	/// [`org.a11y.atspi.Socket`]'s [`embed`] method to
	/// register with the accessibility registry, the
	/// registry will set a numerical id on the application.
	///
	/// ## status
	///
	/// The property has fallen in disuse.
	///
	/// As per [AT-SPI2-CORE issue #82](<https://gitlab.gnome.org/GNOME/at-spi2-core/-/issues/82>)
	/// it may turn out that this id is not actually used subsequently.
	/// This is a remnant of the time when registryd actually had to
	/// make up identifiers for each application.
	/// With DBus, however,	it is the bus that assigns unique names to applications that
	/// connect to it.
	///
	/// Applications or toolkits can remember the `Id` passed when the accessibility
	/// registry sets this property, and return it back when the property is read.
	///
	/// member: "Id", type: property
	///
	/// [`embed`]: crate::socket::SocketProxy#method.embed
	/// [`org.a11y.atspi.Socket`]: crate::socket::SocketProxy
	#[dbus_proxy(property)]
	fn id(&self) -> zbus::Result<i32>;

	/// Set ID of the application.
	///
	/// This method is used by the accessibility registry to set the
	/// application's id.
	///
	/// ## status
	///
	/// The property has fallen in disuse.
	///
	/// See [`id`] for details.
	///
	/// member: "Id", type: property
	///
	/// [`id`]: crate::application::ApplicationProxy#method.id
	#[dbus_proxy(property)]
	fn set_id(&self, value: i32) -> zbus::Result<()>;

	/// Retrieves the name of the toolkit used to implement the application's
	/// user interface.
	///
	/// member: "ToolkitName", type: property
	#[dbus_proxy(property)]
	fn toolkit_name(&self) -> zbus::Result<String>;

	/// Returns the version of the toolkit used to implement the
	/// application's user interface.
	///
	/// member: "Version", type: property
	#[dbus_proxy(property)]
	fn version(&self) -> zbus::Result<String>;
}
