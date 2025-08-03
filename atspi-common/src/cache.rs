//! Common types for `org.a11y.atspi.Cache` events.
//!

use crate::{InterfaceSet, ObjectRef, Role, StateSet};
use serde::{Deserialize, Serialize};
use zbus_lockstep_macros::validate;
use zvariant::Type;

/// The item type provided by `Cache:Add` signals
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
#[validate(signal: "AddAccessible")]
pub struct CacheItem {
	/// The accessible object (within the application)   (so)
	pub object: ObjectRef,
	/// The application (root object(?)    (so)
	pub app: ObjectRef,
	/// The parent object.  (so)
	pub parent: ObjectRef,
	/// The accessbile index in parent.  i
	pub index: i32,
	/// Child count of the accessible  i
	pub children: i32,
	/// The exposed interface(s) set.  as
	pub ifaces: InterfaceSet,
	/// The short localized name.  s
	pub short_name: String,
	/// `ObjectRef` role. u
	pub role: Role,
	/// More detailed localized name.
	pub name: String,
	/// The states applicable to the accessible.  au
	pub states: StateSet,
}

impl Default for CacheItem {
	fn default() -> Self {
		Self {
			object: ObjectRef::from_static_str_unchecked(
				":0.0",
				"/org/a11y/atspi/accessible/object",
			),
			app: ObjectRef::from_static_str_unchecked(
				":0.0",
				"/org/a11y/atspi/accessible/application",
			),
			parent: ObjectRef::from_static_str_unchecked(
				":0.0",
				"/org/a11y/atspi/accessible/parent",
			),
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

/// The item type provided by `Cache:Add` signals
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct LegacyCacheItem {
	/// The accessible object (within the application)   (so)
	pub object: ObjectRef,
	/// The application (root object(?)    (so)
	pub app: ObjectRef,
	/// The parent object.  (so)
	pub parent: ObjectRef,
	/// List of references to the accessible's children.  a(so)
	pub children: Vec<ObjectRef>,
	/// The exposed interface(s) set.  as
	pub ifaces: InterfaceSet,
	/// The short localized name.  s
	pub short_name: String,
	/// `ObjectRef` role. u
	pub role: Role,
	/// More detailed localized name.
	pub name: String,
	/// The states applicable to the accessible.  au
	pub states: StateSet,
}

impl Default for LegacyCacheItem {
	fn default() -> Self {
		Self {
			object: ObjectRef::from_static_str_unchecked(
				":0.0",
				"/org/a11y/atspi/accessible/object",
			),
			app: ObjectRef::from_static_str_unchecked(
				":0.0",
				"/org/a11y/atspi/accessible/application",
			),
			parent: ObjectRef::from_static_str_unchecked(
				":0.0",
				"/org/a11y/atspi/accessible/parent",
			),
			children: Vec::new(),
			ifaces: InterfaceSet::empty(),
			short_name: String::default(),
			role: Role::Invalid,
			name: String::default(),
			states: StateSet::empty(),
		}
	}
}

#[cfg(test)]
#[test]
fn zvariant_type_signature_of_legacy_cache_item() {
	use std::str::FromStr;
	assert_eq!(
		*<LegacyCacheItem as Type>::SIGNATURE,
		zbus::zvariant::Signature::from_str("((so)(so)(so)a(so)assusau)").unwrap()
	);
}
