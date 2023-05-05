use crate::{InterfaceSet, ObjectPair, Role, StateSet};
use serde::{Deserialize, Serialize};
use zvariant::Type;

/// The item type provided by `Cache:Add` signals
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct CacheItem {
	/// The accessible object (within the application)   (so)
	pub object: ObjectPair,
	/// The application (root object(?)    (so)
	pub app: ObjectPair,
	/// The parent object.  (so)
	pub parent: ObjectPair,
	/// The accessbile index in parent.  i
	pub index: i32,
	/// Child count of the accessible  i
	pub children: i32,
	/// The exposed interfece(s) set.  as
	pub ifaces: InterfaceSet,
	/// The short localized name.  s
	pub short_name: String,
	/// Accessible role. u
	pub role: Role,
	/// More detailed localized name.
	pub name: String,
	/// The states applicable to the accessible.  au
	pub states: StateSet,
}

#[test]
fn zvariant_type_signature_of_cache_item() {
	assert_eq!(
		CacheItem::signature(),
		zbus::zvariant::Signature::from_static_str("((so)(so)(so)iiassusau)").unwrap()
	);
}

// impl CacheItem {
//     fn accessible(&self, conn: &Connection) -> AccessibleProxy<'_> {
//         let conn = conn.inner().connection();
//         let (name, path) = (self.object.0, self.object.1);
//         ProxyBuilder::new(conn)
//     }
// }
//
