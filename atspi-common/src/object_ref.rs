use serde::{Deserialize, Serialize};
use zbus_lockstep_macros::validate;
use zbus_names::{OwnedUniqueName, UniqueName};
use zvariant::{ObjectPath, OwnedObjectPath, Signature, Type, Value};

// Equiv to "(so)"
pub const OBJECT_REF_SIGNATURE: &Signature =
	&Signature::static_structure(&[&Signature::Str, &Signature::ObjectPath]);

/// A unique identifier for an object in the accessibility tree.
///
/// A ubiquitous type used to refer to an object in the accessibility tree.
///
/// In AT-SPI2, objects in the applications' UI object tree are uniquely identified
/// using a server name and object path. "(so)"
///
/// Emitted by `RemoveAccessible` and `Available`
#[validate(signal: "Available")]
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct ObjectRef {
	pub name: OwnedUniqueName,
	pub path: OwnedObjectPath,
}

impl Default for ObjectRef {
	fn default() -> Self {
		ObjectRef {
			name: UniqueName::from_static_str(":0.0").unwrap().into(),
			path: ObjectPath::from_static_str("/org/a11y/atspi/accessible/null")
				.unwrap()
				.into(),
		}
	}
}

/// A unique identifier for an object in the accessibility tree.
///
/// This is a borrowed version of [`ObjectRef`].
///
/// A ubiquitous type used to refer to an object in the accessibility tree.
///
/// In AT-SPI2, objects in the applications' UI object tree are uniquely identified
/// using a server name and object path. "(so)"
///
/// Emitted by `RemoveAccessible` and `Available`
#[validate(signal: "Available")]
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct ObjectRefBorrow<'a> {
	#[serde(borrow)]
	pub name: UniqueName<'a>,
	#[serde(borrow)]
	pub path: ObjectPath<'a>,
}

impl ObjectRef {
	/// Create a new `ObjectRef`
	#[must_use]
	pub fn new<'a>(sender: UniqueName<'a>, path: ObjectPath<'a>) -> Self {
		Self { name: sender.into(), path: path.into() }
	}
}

impl<'a> ObjectRefBorrow<'a> {
	/// Create a new `ObjectRefBorrow`
	#[must_use]
	pub fn new(name: UniqueName<'a>, path: ObjectPath<'a>) -> Self {
		Self { name, path }
	}

	/// Convert a partially borrowed `ObjectRefBorrow` into a fully owned `ObjectRef`
	// A derived clone would clone the owned fields and create new borrows for the borrowed fields.
	// Whereas sometimes we want to convert the borrowed fields into owned fields.
	#[must_use]
	pub fn to_fully_owned(&self) -> ObjectRef {
		let name = OwnedUniqueName::from(self.name.clone());
		let path = OwnedObjectPath::from(self.path.clone());
		ObjectRef { name, path }
	}
}

impl Default for ObjectRefBorrow<'_> {
	fn default() -> Self {
		ObjectRefBorrow {
			name: UniqueName::from_static_str(":0.0").unwrap(),
			path: ObjectPath::from_static_str("/org/a11y/atspi/accessible/null").unwrap(),
		}
	}
}

impl<'a> TryFrom<zvariant::Value<'a>> for ObjectRef {
	type Error = zvariant::Error;
	fn try_from(value: zvariant::Value<'a>) -> Result<Self, Self::Error> {
		// Relies on `TryFrom<OwnedValue> for (T0, T1)` implementation
		let (name, path): (OwnedUniqueName, OwnedObjectPath) = value.try_to_owned()?.try_into()?;
		Ok(ObjectRef { name, path })
	}
}

impl TryFrom<zvariant::OwnedValue> for ObjectRef {
	type Error = zvariant::Error;
	fn try_from(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
		let (name, path): (OwnedUniqueName, OwnedObjectPath) = value.try_into()?;
		Ok(ObjectRef { name, path })
	}
}

impl<'b, 'a: 'b> TryFrom<Value<'a>> for ObjectRefBorrow<'b> {
	type Error = zvariant::Error;
	fn try_from(value: zvariant::Value<'a>) -> Result<Self, Self::Error> {
		let (name, path): (UniqueName, ObjectPath) = value.try_into()?;
		Ok(ObjectRefBorrow { name, path })
	}
}

impl From<ObjectRef> for zvariant::Structure<'_> {
	fn from(obj: ObjectRef) -> Self {
		(obj.name, obj.path).into()
	}
}

#[cfg(test)]
mod test {
	use crate::{object_ref::ObjectRefBorrow, ObjectRef};

	#[test]
	fn test_accessible_from_dbus_ctxt_to_object_ref() {
		use zvariant::serialized::Context;
		use zvariant::{to_bytes, Value, LE};

		let acc = ObjectRef::default();
		let ctxt = Context::new_dbus(LE, 0);
		let acc_value: Value<'_> = acc.into();
		let data = to_bytes(ctxt, &acc_value).unwrap();
		let (value, _) = data.deserialize::<Value>().unwrap();
		let accessible: ObjectRef = value.try_into().unwrap();

		assert_eq!(accessible.name.as_str(), ":0.0");
		assert_eq!(accessible.path.as_str(), "/org/a11y/atspi/accessible/null");
	}

	#[test]
	fn test_accessible_value_wrapped_from_dbus_ctxt_to_object_ref() {
		use zvariant::serialized::Context;
		use zvariant::{to_bytes, Value, LE};

		let acc = ObjectRef::default();
		let value: zvariant::Value = acc.into();
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &value).unwrap();
		let (value, _) = encoded.deserialize::<Value>().unwrap();
		let accessible: ObjectRef = value.try_into().unwrap();

		assert_eq!(accessible.name.as_str(), ":0.0");
		assert_eq!(accessible.path.as_str(), "/org/a11y/atspi/accessible/null");
	}

	#[test]
	fn test_try_from_value_for_object_ref() {
		use zvariant::Value;

		let oref = ObjectRef::default();
		let value: Value = oref.into();
		let obj: ObjectRef = value.try_into().unwrap();

		assert_eq!(obj.name.as_str(), ":0.0");
		assert_eq!(obj.path.as_str(), "/org/a11y/atspi/accessible/null");
	}

	#[test]
	fn test_try_from_owned_value_for_object_ref() {
		use zvariant::OwnedValue;
		use zvariant::Value;

		let oref = ObjectRef::default();
		let value: Value = oref.into();
		let value: OwnedValue = value.try_into().unwrap();
		let obj: ObjectRef = value.try_into().unwrap();

		assert_eq!(obj.name.as_str(), ":0.0");
		assert_eq!(obj.path.as_str(), "/org/a11y/atspi/accessible/null");
	}

	#[test]
	fn must_fail_test_try_from_invalid_value_for_object_ref() {
		let value = zvariant::Value::from(42);
		let obj: Result<ObjectRef, _> = value.try_into();
		assert!(obj.is_err());
	}

	#[test]
	fn test_try_from_value_for_object_ref_borrow() {
		use zvariant::Value;

		let oref = ObjectRef::default();
		let value: Value = oref.into();
		let obj_borrow: ObjectRefBorrow = value.try_into().unwrap();

		assert_eq!(obj_borrow.name.as_str(), ":0.0");
		assert_eq!(obj_borrow.path.as_str(), "/org/a11y/atspi/accessible/null");
	}

	#[test]
	fn must_fail_test_try_from_invalid_value_for_object_ref_borrow() {
		let value = zvariant::Value::from((42, true));
		let obj: Result<ObjectRefBorrow, _> = value.try_into();
		assert!(obj.is_err());
	}
}
