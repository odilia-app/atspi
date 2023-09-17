//! Common types for `org.a11y.atspi.Cache` events.
//!

use crate::{Accessible, InterfaceSet, Role, StateSet};
use serde::{Deserialize, Serialize};
use zvariant::Type;

/// The item type provided by `Cache:Add` signals
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct CacheItem {
	/// The accessible object (within the application)   (so)
	pub object: Accessible,
	/// The application (root object(?)    (so)
	pub app: Accessible,
	/// The parent object.  (so)
	pub parent: Accessible,
	/// The accessbile index in parent.  i
	pub index: i32,
	/// Child count of the accessible  i
	pub children: i32,
	/// The exposed interface(s) set.  as
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
impl Default for CacheItem {
	fn default() -> Self {
		Self {
			object: Accessible {
				name: ":0.0".into(),
				path: "/org/a11y/atspi/accessible/object".try_into().unwrap(),
			},
			app: Accessible {
				name: ":0.0".into(),
				path: "/org/a11y/atspi/accessible/application".try_into().unwrap(),
			},
			parent: Accessible {
				name: ":0.0".into(),
				path: "/org/a11y/atspi/accessible/parent".try_into().unwrap(),
			},
			index: 0,
			children: 0,
			ifaces: InterfaceSet::empty(),
			short_name: String::default(),
			role: Role::Invalid,
			name: String::default(),
			states: StateSet::empty(),
		}
	}
}

#[test]
fn zvariant_type_signature_of_cache_item() {
	assert_eq!(
		CacheItem::signature(),
		zbus::zvariant::Signature::from_static_str("((so)(so)(so)iiassusau)").unwrap()
	);
}

/// The item type provided by `Cache:Add` signals
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct LegacyCacheItem {
	/// The accessible object (within the application)   (so)
	pub object: Accessible,
	/// The application (root object(?)    (so)
	pub app: Accessible,
	/// The parent object.  (so)
	pub parent: Accessible,
	/// List of references to the accessible's children.  a(so)
	pub children: Vec<Accessible>,
	/// The exposed interface(s) set.  as
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
impl Default for LegacyCacheItem {
	fn default() -> Self {
		Self {
			object: Accessible {
				name: ":0.0".into(),
				path: "/org/a11y/atspi/accessible/object".try_into().unwrap(),
			},
			app: Accessible {
				name: ":0.0".into(),
				path: "/org/a11y/atspi/accessible/application".try_into().unwrap(),
			},
			parent: Accessible {
				name: ":0.0".into(),
				path: "/org/a11y/atspi/accessible/parent".try_into().unwrap(),
			},
			children: Vec::new(),
			ifaces: InterfaceSet::empty(),
			short_name: String::default(),
			role: Role::Invalid,
			name: String::default(),
			states: StateSet::empty(),
		}
	}
}

#[test]
fn zvariant_type_signature_of_legacy_cache_item() {
	assert_eq!(
		LegacyCacheItem::signature(),
		zbus::zvariant::Signature::from_static_str("((so)(so)(so)a(so)assusau)").unwrap()
	);
}
