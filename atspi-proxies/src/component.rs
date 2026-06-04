//! # `ComponentProxy`
//!
//! `org.a11y.atspi.Component` provides methods to obtain information about a
//! UI component.
//!
//! ## Defaults
//!
//! If "org.a11y.atspi.Component" is implemented, it is implemented per individual
//! node in the application's UI-tree. This means the path will vary, so no default
//! path is applicable for this proxy.
//!
//! Path and service need to be provided by the builder or inherited from the
//! [`zbus::Proxy`] this `ComponentProxy` is derived from.
//!
//! Since no default path or service make sense for this proxy, the macro is
//! instructed explicitly not to generate these fields.

use crate::common::{CoordType, Layer, ScrollType};
use atspi_common::object_ref::ObjectRefOwned;

#[zbus::proxy(interface = "org.a11y.atspi.Component", assume_defaults = false)]
pub trait Component {
	/// `Contains` method
	fn contains(&self, x: i32, y: i32, coord_type: CoordType) -> zbus::Result<bool>;

	/// `GetAccessibleAtPoint` method
	/// To get an accessible at a point inside a frame of a particular app, you must use `CoordType::Window`
	fn get_accessible_at_point(
		&self,
		x: i32,
		y: i32,
		coord_type: CoordType,
	) -> zbus::Result<ObjectRefOwned>;

	/// `GetAlpha` method
	fn get_alpha(&self) -> zbus::Result<f64>;

	/// `GetExtents` method
	fn get_extents(&self, coord_type: CoordType) -> zbus::Result<(i32, i32, i32, i32)>;

	/// `GetLayer` method
	fn get_layer(&self) -> zbus::Result<Layer>;

	/// `GetMDIZOrder` method
	#[zbus(name = "GetMDIZOrder")]
	fn get_mdiz_order(&self) -> zbus::Result<i16>;

	/// `GetPosition` method
	/// To get the position of a frame of a particular app, you must use `CoordType::Screen`
	fn get_position(&self, coord_type: CoordType) -> zbus::Result<(i32, i32)>;

	/// `GetSize` method
	fn get_size(&self) -> zbus::Result<(i32, i32)>;

	/// `GrabFocus` method
	fn grab_focus(&self) -> zbus::Result<bool>;

	/// `ScrollTo` method
	fn scroll_to(&self, type_: ScrollType) -> zbus::Result<bool>;

	/// `ScrollToPoint` method
	fn scroll_to_point(&self, coord_type: CoordType, x: i32, y: i32) -> zbus::Result<bool>;

	/// `SetExtents` method
	fn set_extents(
		&self,
		x: i32,
		y: i32,
		width: i32,
		height: i32,
		coord_type: CoordType,
	) -> zbus::Result<bool>;

	/// `SetPosition` method
	fn set_position(&self, x: i32, y: i32, coord_type: CoordType) -> zbus::Result<bool>;

	/// `SetSize` method
	fn set_size(&self, width: i32, height: i32) -> zbus::Result<bool>;
}
