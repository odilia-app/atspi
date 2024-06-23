//! # [`CollectionProxy`]
//!
//! A handle to a remote object implementing the `org.a11y.atspi.Collection`
//! interface.
//!
//! `Collection` is the interface which is implemented by objects that contain
//! other objects, such as a window or a table.
//!
//! See the documentation on the individual methods for details.
//!
//! [`CollectionProxy`]: crate::collection::CollectionProxy

use crate::common::{ObjectRef, SortOrder, TreeTraversalType};

#[zbus::proxy(interface = "org.a11y.atspi.Collection", assume_defaults = true)]
trait Collection {
	// The active descendant of the given object.
	//
	// Looks like this is unimplemented.
	//
	// See [atspi/collection.c](https://gitlab.gnome.org/GNOME/at-spi2-core/-/blob/main/atspi/atspi-collection.c?ref_type=heads#L272)
	//
	// fn get_active_descendant(&self) -> zbus::Result<ObjectRef>;

	/// GetMatches method
	fn get_matches(
		&self,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRef>>;

	/// GetMatchesFrom method
	fn get_matches_from(
		&self,
		current_object: &zbus::zvariant::ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRef>>;

	/// GetMatchesTo method
	#[allow(clippy::too_many_arguments)]
	fn get_matches_to(
		&self,
		current_object: &zbus::zvariant::ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		limit_scope: bool,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRef>>;
}
