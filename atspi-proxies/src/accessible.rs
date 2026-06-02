//! # `AccessibleProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Accessible`
//! interface.
//!
//! `Accessible` is the base interface implemented by all accessible objects in the
//! AT-SPI2 UI-tree.
//!
//! ## D-Bus Addressing
//!
//! Because `Accessible` is implemented on every individual node within an
//! application's UI-tree, the D-Bus object path varies dynamically per node, except
//! for the root node.
//!
//! The root node's path is fixed and can be resolved using [`atspi_common::ACCESSIBLE_ROOT_PATH`].
//!
//! ## How to obtain an `AccessibleProxy`
//!
//! There are three idiomatic ways to obtain an `AccessibleProxy`:
//!
//! ### 1. From an [`ObjectRef`] using [`ObjectRefExt`] (Recommended)
//! If you have an [`ObjectRef`] pointing to a node, you can resolve it to an
//! [`AccessibleProxy`] using the [`ObjectRefExt`] trait:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! # use atspi_connection::AccessibilityConnection;
//! use atspi_common::ObjectRefOwned;
//! use atspi_proxies::accessible::ObjectRefExt;
//!
//! # block_on( async {
//! # let a11y = AccessibilityConnection::new().await?;
//! # let conn = a11y.connection();
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//!
//! let _accessible = obj_ref.as_accessible_proxy(conn).await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! ### 2. Transitioning to other interface proxies via [`ProxyExt`][pe]
//! Since all accessible objects implement the `Accessible` omterface,
//! you can use [`ProxyExt`][pe] on an [`AccessibleProxy`] to safely obtain
//! any other interface proxy (such as [`TextProxy`][tp]) that the node
//! implements:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::proxy_ext::ProxyExt;
//! use atspi_proxies::accessible::ObjectRefExt;
//! use atspi_common::ObjectRefOwned;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! // Convert an example object into `AccessibleProxy`.
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked("1:1000", "/org/a11y/atspi/accessible/root");
//! let accessible_proxy = obj_ref.into_accessible_proxy(&conn).await?;
//!
//! // Convert the `AccesssibleProxy` to [`Proxies`][prxs], a safe conversion type:
//! let proxies = accessible_proxy.proxies().await?;
//!
//! // Convert a to `TextProxy`, this will return an error if `TextProxy` is not implemented.
//! let text_proxy = proxies.text().await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! Note that all proxies obtained through [`ProxyExt`][pe] share their underlying
//! [`zbus::Connection`], inheriting the same P2P configuration if applicable.
//!
//! ### 3. Manual construction using the `builder`
//! If you know the exact D-Bus service destination and object path, you can
//! construct the proxy manually:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::accessible::AccessibleProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! let bus_name = ":1.1001";
//! let object_path = "/org/a11y/atspi/accesible/root";
//!
//! let accessible = AccessibleProxy::builder(&conn)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [pe]: crate::proxy_ext::ProxyExt
//! [tp]: crate::text::TextProxy

use crate::common::{InterfaceSet, ObjectRef, RelationType, Role, StateSet};
use crate::AtspiError;
use atspi_common::object_ref::{NonNullObjectRef, ObjectRefOwned};
use zbus::names::BusName;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>

/// # `AccessibleProxy`
///
/// A handle for a remote object implementing the `org.a11y.atspi.Accessible`
/// interface.
///
/// Accessible is the interface which is implemented by all accessible objects.
///
#[zbus::proxy(interface = "org.a11y.atspi.Accessible", assume_defaults = false)]
pub trait Accessible {
	/// Returns an [`ObjectRef`] which refers to the `Application` object of the application.
	/// This object will have [`Application`] interface implemented.
	///
	/// The application object is the root of the accessibility hierarchy for the application.
	/// It is the only object in the hierarchy that does not have a parent.
	///
	/// ## Notes
	/// The application object is the only object in the accessibility hierarchy that is
	/// guaranteed to be persistent for the lifetime of the application.
	/// All other objects in the accessibility hierarchy may be created and destroyed dynamically.
	///
	/// [`ObjectRef`]: [`crate::common::events::ObjectRef`]
	/// [`Application`]: [`crate::application::ApplicationProxy`]
	fn get_application(&self) -> zbus::Result<ObjectRefOwned>;

	/// Gets a list of name/value pairs of attributes or annotations for this object.
	///
	/// ## Disambiguation
	/// For	typographic, textual, or textually-semantic attributes,
	/// see [`TextProxy`]'s [`get_attributes`] method instead.
	///
	/// [`TextProxy`]: [`crate::text::TextProxy`]
	/// [`get_attributes`]: [`crate::text::TextProxy#method.get_attributes`]
	fn get_attributes(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

	/// Retrieve child by index (starting from 0),
	///
	/// Queries the N-th accessible child of `self`. It is expected that this
	/// will correspond to the order that the [`get_children`] method would return.
	///
	/// ## Notes
	/// Implementations vary in their behavior when the index is out of range.
	/// GTK4 returns an error, while atk-adaptor (e.g. Gtk3) returns the
	/// null object path "/org/a11y/atspi/null".
	///
	/// Documentation advises implementors to return a `DBus` Error when the index is
	/// out of range, to "keep the type system gods happy".
	///
	/// [`get_children`]: #method.get_children
	fn get_child_at_index(&self, index: i32) -> zbus::Result<ObjectRefOwned>;

	/// Retrieves a list of the object's accessible children.
	///
	/// Each array element is an [`Accessible`] representing the accessible child object.
	///
	/// ## Registry
	///
	/// On the [`Accessible`] interface of `org.a11y.atspi.Registry`, the registry daemon, this method retrieves a list
	/// of all accessible applications' root objects on the bus.
	///
	/// [`Accessible`]: [`crate::accessible::AccessibleProxy`]
	fn get_children(&self) -> zbus::Result<Vec<ObjectRefOwned>>;

	/// This object resides in its parent's list of children.
	/// This returns its position in this list of children, starting from 0.
	///
	/// The function returns -1 if the object does not have a parent or
	/// if an exception occurs.
	fn get_index_in_parent(&self) -> zbus::Result<i32>;

	/// Returns an [`InterfaceSet`] accessible interface names supported by the `self` object.
	/// [`InterfaceSet`]: [`crate::common::InterfaceSet`]
	fn get_interfaces(&self) -> zbus::Result<InterfaceSet>;

	/// Gets a `String` corresponding to the name of the role played by an object,
	/// translated to the current locale.
	///
	/// ## Notes
	///
	/// This method will return useful values for roles that fall outside the
	/// enumeration used in the [`get_role`] method.
	///
	/// For applications, implementing this method is optional, and it may be removed
	/// in a future version of the API.
	///
	/// For example, [`libatspi`] will only call it in the event of an unknown role.
	///
	/// [`libatspi`]: <https://gitlab.gnome.org/GNOME/at-spi2-core/main/atspi>
	/// [`get_role`]: #method.get_role
	fn get_localized_role_name(&self) -> zbus::Result<String>;

	/// Returns a set of relationships between the this `self` object and others.
	///
	/// This vector of tuples contains a [`RelationType`] and a vector of [`Accessible`]'s to which that
	/// relationship applies.
	/// These relationships allow for better identification of how objects are associated with one another.
	///
	/// For example, the relationship [`RelationType::LabelledBy`] can be used to identify labeling information
	/// that should accompany the accessible [`name`] property when presenting an object's content or identity
	/// to the end user.
	///
	/// Similarly, [`RelationType::ControllerFor`] can be used to specify the context in which a valuator is useful
	/// and/or the other UI components that are directly affected by user interactions with the valuator.
	/// Common examples include the association of scrollbars with the viewport or panel that they control.
	///
	/// [`RelationType`]: [`crate::common::RelationType`]
	/// [`RelationType::LabelledBy`]: [`crate::common::RelationType::LabelledBy`]
	/// [`RelationType::ControllerFor`]: [`crate::common::RelationType::ControllerFor`]
	/// [`name`]: #method.name
	/// [`Accessible`]: [`crate::common::events::Accessible`]
	fn get_relation_set(&self) -> zbus::Result<Vec<(RelationType, Vec<ObjectRefOwned>)>>;

	/// Gets the [`Role`] that the current accessible object represents.
	///
	/// Roles make it possible for various UI toolkits to expose their controls to
	/// assistive technologies (ATs) with a standard interface, regardless of toolkit.
	///
	/// For example, a widget that acts like a conventional push button
	/// (appears unpressed; presses	when acted upon; invokes a certain action
	/// when pressed) can expose an	[`Role::Button`] role.
	///
	/// [`Role::Button`]: [`crate::common::Role::Button`]
	/// [`Role`]: [`crate::common::Role`]
	fn get_role(&self) -> zbus::Result<Role>;

	/// Gets a `String` corresponding to the name of the role played by an object,
	/// translated to the current locale.
	///
	/// ## Notes
	///
	/// This method will return useful values for roles that fall outside the
	/// enumeration used in the `get_role` method.
	///
	/// For applications, implementing this method is optional, and it may be removed
	/// in a future version of the API.
	///
	/// [`libatspi`]: <https://gitlab.gnome.org/GNOME/at-spi2-core/main/atspi>
	/// [`libatspi`]: <https://gitlab.gnome.org/GNOME/at-spi2-core/>
	fn get_role_name(&self) -> zbus::Result<String>;

	/// Method to retrieve the [`StateSet`] of states currently held by `self`.
	/// [`StateSet`]: [`crate::common::StateSet`]
	fn get_state(&self) -> zbus::Result<StateSet>;

	/// Application-specific identifier for the current object.
	///
	/// A special id given to an object.
	/// Accessible application developers can use this to give a special id to an object
	/// to use in tests, for example, "`my_widget`".
	///
	/// Note that there is no way to directly find an object by its id;
	/// a test program may have to recursively get the children to find a specific id.
	/// This is because accessible objects can be created dynamically, and they do not always
	/// correspond to a static view of an application's data.
	#[zbus(property)]
	fn accessible_id(&self) -> zbus::Result<String>;

	/// Number of accessible children for the current object.
	#[zbus(property)]
	fn child_count(&self) -> zbus::Result<i32>;

	/// Human-readable, localized description of `self` in more detail.
	///
	/// This is a longer description than the [`Name`][name] property.
	///
	/// For example, a button might have a name of "OK", but a description of "OK button".
	///
	/// While the Name property is meant to be a short string that screen readers say
	/// during normal navigation, the Description property is for when the user asks for
	/// more detail.
	///
	/// [name]: #method.name
	#[zbus(property)]
	fn description(&self) -> zbus::Result<String>;

	/// Unix locale for the current object.
	///
	/// This is a string in the form of "`language_territory.codeset`".
	/// For example, "en_US.UTF-8" or "de_DE.UTF-8".
	///
	/// For an application, this may be the locale for the language that the application
	/// shows in its user interface.
	///
	/// For a document being shown in an application, or a paragraph within a document,
	/// the locale may refer to that object exclusively. For example:
	/// an application may be showing itself in English ("en"), but it may be used to
	/// display a document in Spanish ("es").
	/// In the latter case, a screen reader will want to know that it should switch to
	/// Spanish while reading the document.
	#[zbus(property)]
	fn locale(&self) -> zbus::Result<String>;

	/// Human-readable, localized, short name for the object.
	///
	/// Applications should have this set for objects which do not
	/// have a [`RelationType::LabelledBy`] relation.
	///
	/// Consider a widget to select RGB colors by setting three sliders.
	/// The	names for the sliders would be "Red", "Green", "Blue", respectively, or
	/// their translations to application's locale.  The names would be unnecessary if each
	/// slider had a `LabeledBy` relation to corresponding labels visible in the user
	/// interface.
	///
	/// [`RelationType::LabelledBy`]: [`crate::common::RelationType::LabelledBy`]
	#[zbus(property)]
	fn name(&self) -> zbus::Result<String>;

	/// `ObjectRef` parent object of the current object.
	///
	/// Null parent:
	/// If the object has no parent (e.g. the application's root object is being queried),
	/// The application should return "" for the application name name and "/org/a11y/atspi/null"
	/// for the object path.
	///
	/// Root object:
	/// An application must have a single root object, called "/org/a11y/atspi/accessible/root".
	/// All other objects should have that one as their highest-level ancestor.
	#[zbus(property)]
	fn parent(&self) -> zbus::Result<ObjectRefOwned>;

	/// Help text for the current object.
	#[zbus(property)]
	fn help_text(&self) -> zbus::Result<String>;
}

impl TryFrom<AccessibleProxy<'_>> for ObjectRefOwned {
	type Error = AtspiError;
	fn try_from(proxy: AccessibleProxy<'_>) -> Result<ObjectRefOwned, Self::Error> {
		let sender = proxy.inner().destination();
		let path = proxy.inner().path();
		let object_ref = ObjectRef::try_from_bus_name_and_path(sender, path)?;
		Ok(ObjectRefOwned::from(object_ref))
	}
}

impl TryFrom<&AccessibleProxy<'_>> for ObjectRefOwned {
	type Error = AtspiError;
	fn try_from(proxy: &AccessibleProxy<'_>) -> Result<ObjectRefOwned, Self::Error> {
		let sender = proxy.inner().destination().clone();
		let path = proxy.inner().path().clone();
		let object_ref = ObjectRef::try_from_bus_name_and_path(sender, path)?;
		Ok(ObjectRefOwned::from(object_ref))
	}
}

pub trait ObjectRefExt {
	fn as_accessible_proxy(
		&self,
		conn: &zbus::Connection,
	) -> impl std::future::Future<Output = Result<AccessibleProxy<'_>, AtspiError>> + Send;

	fn into_accessible_proxy(
		self,
		conn: &zbus::Connection,
	) -> impl std::future::Future<Output = Result<AccessibleProxy<'_>, AtspiError>> + Send;
}

impl ObjectRefExt for NonNullObjectRef<'_> {
	/// Returns an [`AccessibleProxy`], borrowing the destination and path from `self`.
	///
	/// The returned proxy borrows the underlying string data of the object reference,
	/// meaning the proxy **cannot outlive** `self`. This avoids memory allocations
	/// for the object path.
	///
	/// # Errors
	/// Because `NonNullObjectRef` contains valid [`UniqueName`] and [`ObjectPath`],
	/// and because atspi proxies opt-out of zbus' property caching,
	/// it is highly unlikely that this method will return an error.
	async fn as_accessible_proxy(
		&self,
		conn: &zbus::Connection,
	) -> Result<AccessibleProxy<'_>, AtspiError> {
		let name: BusName = self.name().clone().into();
		let path = self.path();

		AccessibleProxy::builder(conn)
			.destination(name)?
			.path(path)?
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await
			.map_err(AtspiError::from)
	}

	/// Converts to [`AccessibleProxy`], consuming `self` and producing a fully owned, `'static` proxy.
	/// Use this if you need to store the proxy or send it to another thread.
	///
	/// # Errors
	/// Because `NonNullObjectRef` contains valid [`UniqueName`] and [`ObjectPath`],
	/// and because atspi proxies opt-out of zbus' property caching,
	/// it is highly unlikely that this method will return an error.
	async fn into_accessible_proxy(
		self,
		conn: &zbus::Connection,
	) -> Result<AccessibleProxy<'_>, AtspiError> {
		// Consume and deconstruct self
		let (name, path) = match self {
			NonNullObjectRef::Owned { name, path } => (name, path),
			NonNullObjectRef::Borrowed { name, path } => (name.to_owned(), path.to_owned()),
		};

		AccessibleProxy::builder(conn)
			.destination(name)?
			.path(path)?
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await
			.map_err(AtspiError::from)
	}
}

impl ObjectRefExt for ObjectRef<'_> {
	/// Returns an [`AccessibleProxy`], borrowing the destination and path from `self`.
	///
	/// The returned proxy borrows the underlying string data of the object reference,
	/// meaning the proxy **cannot outlive** `self`. This avoids memory allocations
	/// for the object path.
	///
	/// # Errors
	/// If `self` is [`ObjectRef::Null`], this method will return [`AtspiError::ParseError`].
	///
	/// Otherwise, because `ObjectRef` contains valid [`UniqueName`] and [`ObjectPath`],
	/// and atspi proxies opt-out of zbus' property caching, this method is highly unlikely to return an error.
	async fn as_accessible_proxy(
		&self,
		conn: &zbus::Connection,
	) -> Result<AccessibleProxy<'_>, AtspiError> {
		match self {
			ObjectRef::NonNull(non_null) => non_null.as_accessible_proxy(conn).await,
			ObjectRef::Null => Err(AtspiError::ParseError("Expected NonNullObjectRef, found Null")),
		}
	}

	/// Converts to an [`AccessibleProxy`], consuming `self` and producing a fully owned, `'static` proxy.
	/// Use this if you need to store the proxy or send it to another thread.
	///
	/// # Errors
	/// If `self` is [`ObjectRef::Null`], this method will return [`AtspiError::ParseError`].
	///
	/// Because `ObjectRef` contains valid [`UniqueName`] and [`ObjectPath`],
	/// and because atspi proxies opt-out of zbus' property caching,
	/// this method is otherwise highly unlikely to return an error.
	async fn into_accessible_proxy(
		self,
		conn: &zbus::Connection,
	) -> Result<AccessibleProxy<'_>, AtspiError> {
		// ObjectRef can be Null
		let non_null: NonNullObjectRef<'_> = self.try_into()?;
		non_null.into_accessible_proxy(conn).await
	}
}

impl ObjectRefExt for ObjectRefOwned {
	/// Returns an [`AccessibleProxy`], borrowing the destination and path from `self`.
	///
	/// The returned proxy borrows the underlying string data of the object reference,
	/// meaning the proxy **cannot outlive** `self`. This avoids memory allocations
	/// for the object path.
	///
	/// # Errors
	/// If `self` is [`ObjectRefOwned::Null`], this method will return [`AtspiError::ParseError`].
	///
	/// Otherwise, because `ObjectRefOwned` contains valid [`UniqueName`] and [`ObjectPath`],
	/// and atspi proxies opt-out of zbus' property caching, this method is highly unlikely to return an error.
	async fn as_accessible_proxy(
		&self,
		conn: &zbus::Connection,
	) -> Result<AccessibleProxy<'_>, AtspiError> {
		// Match directly on the inner reference &self.0 to avoid temporary variables
		match self.as_inner() {
			ObjectRef::NonNull(non_null) => non_null.as_accessible_proxy(conn).await,
			ObjectRef::Null => Err(AtspiError::ParseError("Expected NonNullObjectRef, found Null")),
		}
	}

	/// Converts to an [`AccessibleProxy`], consuming `self` and producing a fully owned, `'static` proxy.
	/// Use this if you need to store the proxy or send it to another thread.
	///
	/// # Errors
	/// If `self` is [`ObjectRefOwned::Null`], this method will return [`AtspiError::ParseError`].
	///
	/// Because `ObjectRefOwned` contains valid [`UniqueName`] and [`ObjectPath`],
	/// and because atspi proxies opt-out of zbus' property caching,
	/// this method is otherwise highly unlikely to return an error.
	async fn into_accessible_proxy(
		self,
		conn: &zbus::Connection,
	) -> Result<AccessibleProxy<'_>, AtspiError> {
		// ObjectRefOwned can be Null
		let non_null: NonNullObjectRef = self.try_into()?;
		non_null.into_accessible_proxy(conn).await
	}
}

#[cfg(test)]
mod tests {
	use crate::accessible::ObjectRefExt;
	use crate::bus::BusProxy;
	use atspi_common::{NonNullObjectRef, ObjectRef, ObjectRefOwned, Role};
	use zbus::connection::Builder;
	use zbus::fdo::DBusProxy;
	use zbus::names::{BusName, OwnedUniqueName};
	use zbus::zvariant::ObjectPath;

	#[test]
	fn test_output_of_role_name() {
		assert_eq!(Role::Invalid.name(), "invalid");
		assert_eq!(Role::PushButtonMenu.name(), "push button menu");
	}

	async fn get_a11y_connection() -> Result<zbus::Connection, zbus::Error> {
		let session_bus = zbus::Connection::session().await?;
		let bus_proxy = BusProxy::new(&session_bus).await?;
		let a11y_bus_addr = bus_proxy.get_address().await?;
		let addr: zbus::Address = a11y_bus_addr.parse()?;
		Builder::address(addr)?.build().await
	}

	#[tokio::test]
	async fn test_object_ref_ext_integration() {
		let a11y_conn = match get_a11y_connection().await {
			Ok(conn) => conn,
			Err(e) => {
				eprintln!("Skipping integration test: Accessibility bus is not available: {e}");
				return;
			}
		};

		// Resolve the well-known name "org.a11y.atspi.Registry" to its actual unique name
		let Some(unique_name): Option<OwnedUniqueName> = (async {
			let dbus_proxy = DBusProxy::new(&a11y_conn).await.ok()?;
			dbus_proxy
				.get_name_owner(BusName::from_static_str("org.a11y.atspi.Registry").unwrap())
				.await
				.ok()
		})
		.await
		else {
			eprintln!("Skipping integration test: Could not resolve unique name for Registry.");
			return;
		};

		let path = "/org/a11y/atspi/accessible/root";
		let object_path = ObjectPath::from_static_str_unchecked(path);

		let non_null = NonNullObjectRef::try_new_owned(unique_name, object_path.clone()).unwrap();
		let non_null_owned = non_null.clone().into_owned();
		let proxy_borrowed_res = non_null_owned.as_accessible_proxy(&a11y_conn).await;
		assert!(
			proxy_borrowed_res.is_ok(),
			"Failed to build borrowed proxy from NonNullObjectRef: {:?}",
			proxy_borrowed_res.err()
		);
		let proxy = proxy_borrowed_res.unwrap();

		if let Ok(role) = proxy.get_role().await {
			assert_eq!(role, Role::DesktopFrame);
		}

		// Test B: Using ObjectRef (borrowing proxy builder)
		let object_ref = ObjectRef::from(non_null.clone());
		let proxy_from_ref = object_ref.as_accessible_proxy(&a11y_conn).await;
		assert!(proxy_from_ref.is_ok());

		// Test C: Using ObjectRefOwned (consuming owned proxy builder)
		let object_ref_owned = ObjectRefOwned::from(non_null.clone());
		let proxy_from_owned = object_ref_owned.into_accessible_proxy(&a11y_conn).await;
		assert!(proxy_from_owned.is_ok());
	}
}
