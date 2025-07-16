use serde::{Deserialize, Serialize};
use zbus_lockstep_macros::validate;
use zbus_names::{OwnedUniqueName, UniqueName};
use zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Type, Value};

/// An object path used in respones from accessible applications indicating that the path does not
/// exist.
///
/// "Why not just use `None`?"
///
/// `DBus` (which the AT-SPI2 protocol runs on) does not have optional types, so this path indicates the sentinal, `None` value.
///
/// Also see: [`IsNullExt`]
pub const NULL_OBJECT_PATH: ObjectPath<'static> =
	ObjectPath::from_static_str_unchecked("/org/atspi/atspi/null");

/// An [extention trait](http://xion.io/post/code/rust-extension-traits.html) which adds a method to check if an object path matches the
/// [`NULL_OBJECT_PATH`].
trait IsNullExt {
	fn is_null(&self) -> bool;
}

impl IsNullExt for ObjectPath<'_> {
	fn is_null(&self) -> bool {
		*self == NULL_OBJECT_PATH
	}
}

/// A unique identifier for an object in the accessibility tree.
///
/// A ubiquitous type used to refer to an object in the accessibility tree.
///
/// In AT-SPI2, objects in the applications' UI object tree are uniquely identified
/// using a server name and object path. "(so)"
///
/// Emitted by `RemoveAccessible` and `Available`
#[validate(signal: "Available")]
#[derive(Debug, Clone, Serialize, Type, PartialEq, Eq, Hash)]
pub struct ObjectRef {
	pub name: OwnedUniqueName,
	pub path: OwnedObjectPath,
}

// This addresses the issue of `ObjectRef` not being deserializable (#271).
// when the name is empty. Root nodes have no parent, so the name is empty.
impl<'de> Deserialize<'de> for ObjectRef {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let (name, path): (String, OwnedObjectPath) = Deserialize::deserialize(deserializer)?;

		// Check if the name is empty, which is a special case.
		if name.is_empty() && path.is_null() {
			return Ok(ObjectRef::default());
		}

		let name = UniqueName::try_from(name).map_err(serde::de::Error::custom)?;
		let name = OwnedUniqueName::from(name);
		Ok(ObjectRef { name, path })
	}
}

impl Default for ObjectRef {
	fn default() -> Self {
		ObjectRef {
			name: UniqueName::from_static_str_unchecked(":0.0").into(),
			path: NULL_OBJECT_PATH.into(),
		}
	}
}

impl ObjectRef {
	/// Create a new `ObjectRef`
	#[must_use]
	pub fn new<'a>(sender: UniqueName<'a>, path: ObjectPath<'a>) -> Self {
		Self { name: sender.into(), path: path.into() }
	}

	/// Create a new `ObjectRef`, unchecked, with the static string values.
	///
	/// # Safety
	/// The caller must ensure that the strings are valid.
	#[must_use]
	pub fn from_static_str_unchecked(sender: &'static str, path: &'static str) -> Self {
		Self {
			name: UniqueName::from_static_str_unchecked(sender).into(),
			path: ObjectPath::from_static_str_unchecked(path).into(),
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
#[derive(Debug, Clone, Serialize, Type, PartialEq, Eq, Hash)]
pub struct ObjectRefBorrowed<'a> {
	#[serde(borrow)]
	pub name: UniqueName<'a>,
	#[serde(borrow)]
	pub path: ObjectPath<'a>,
}

impl<'de> Deserialize<'de> for ObjectRefBorrowed<'de> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let (name, path): (&'de str, ObjectPath<'_>) = Deserialize::deserialize(deserializer)?;
		// Check if the name is empty, which is a special case to indicate there is no parent object.
		if name.is_empty() && path.is_null() {
			// If name is empty and path is the null path, return the default ObjectRefBorrowed.
			return Ok(ObjectRefBorrowed::default());
		}
		let name = UniqueName::try_from(name).map_err(serde::de::Error::custom)?;
		Ok(ObjectRefBorrowed { name, path })
	}
}

impl ObjectRefBorrowed<'_> {
	/// Convert a partially borrowed `ObjectRefBorrowed` into a fully owned `ObjectRef`
	// A derived clone would clone the owned fields and create new borrows for the borrowed fields.
	// Whereas sometimes we want to convert the borrowed fields into owned fields.
	#[must_use]
	pub fn to_fully_owned(&self) -> ObjectRef {
		let name = OwnedUniqueName::from(self.name.clone());
		let path = OwnedObjectPath::from(self.path.clone());
		ObjectRef { name, path }
	}
}

impl Default for ObjectRefBorrowed<'_> {
	fn default() -> Self {
		ObjectRefBorrowed {
			name: UniqueName::from_static_str_unchecked(":0.0"),
			path: NULL_OBJECT_PATH,
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

impl<'a> TryFrom<Value<'a>> for ObjectRefBorrowed<'a> {
	type Error = zvariant::Error;
	fn try_from(value: zvariant::Value<'a>) -> Result<Self, Self::Error> {
		let (name, path): (UniqueName, ObjectPath) = value.try_into()?;
		Ok(ObjectRefBorrowed { name, path })
	}
}

impl From<ObjectRef> for zvariant::Structure<'_> {
	fn from(obj: ObjectRef) -> Self {
		(obj.name, obj.path).into()
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::message::Header<'_>> for ObjectRef {
	type Error = crate::AtspiError;
	fn try_from(header: &zbus::message::Header) -> Result<Self, Self::Error> {
		let path = header.path().ok_or(crate::AtspiError::MissingPath)?;
		let owned_path: OwnedObjectPath = path.clone().into();

		let sender: UniqueName<'_> = header.sender().ok_or(crate::AtspiError::MissingName)?.into();
		let name: OwnedUniqueName = sender.to_owned().into();

		Ok(ObjectRef { name, path: owned_path })
	}
}

#[validate(signal: "Available")]
#[derive(Debug, Default, Clone, Type, PartialEq, Eq, Hash)]
#[zvariant(signature = "(so)")]
pub enum ParentRef {
	/// A reference to a valid parent object.
	///
	/// This is used in the `Cache:Add` signal to indicate the parent of the accessible object.
	Some(ObjectRef),

	#[default]
	/// When the serialized `name` is empty, it means there is no parent object.
	None,
}

impl Serialize for ParentRef {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			ParentRef::Some(ref obj_ref) => obj_ref.serialize(serializer),
			ParentRef::None => ("", NULL_OBJECT_PATH).serialize(serializer),
		}
	}
}

// This addresses the issue of `ObjectRef` not being deserializable (#271)
// when the name string is empty to indicate there is no parent object.
impl<'de> Deserialize<'de> for ParentRef {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let (name, path): (String, zvariant::OwnedObjectPath) =
			Deserialize::deserialize(deserializer)?;
		// Check if the name is empty, which is a special case to indicate there is no parent object.
		if name.is_empty() && path.is_null() {
			return Ok(ParentRef::None);
		}
		let name = UniqueName::try_from(name).map_err(serde::de::Error::custom)?;
		let name = zbus_names::OwnedUniqueName::from(name);
		Ok(ParentRef::Some(ObjectRef { name, path }))
	}
}

// Users can choose their "limitations" of #271.
// Either convert to `Option<ObjectRef>` or `ObjectRef` directly:

impl From<ParentRef> for Option<ObjectRef> {
	/// Converts `ParentRef` to an `Option<ObjectRef>`.
	///
	/// If the `ParentRef` is `Some`, it returns `Some(ObjectRef)`.
	/// If the `ParentRef` is `None`, it returns `None`.
	fn from(parent_ref: ParentRef) -> Self {
		match parent_ref {
			ParentRef::Some(obj_ref) => Some(obj_ref),
			ParentRef::None => None,
		}
	}
}

impl From<ParentRef> for ObjectRef {
	/// # Warning: Null Object Conversion:
	///
	/// This conversion treats `ObjectRef::default()` as a NULL object.
	/// The `ObjectRef::default()` is constructed with an unused name ":0.0" and "null path".
	/// While the name is valid, the path is used to indicate 'no object'.
	fn from(parent_ref: ParentRef) -> Self {
		match parent_ref {
			ParentRef::Some(obj_ref) => obj_ref,
			ParentRef::None => ObjectRef::default(),
		}
	}
}

impl From<OwnedValue> for ParentRef {
	/// If the `OwnedValue` is an `ObjectRef`, it returns `ParentRef::Some(ObjectRef)`.
	/// If the `OwnedValue` is not an `ObjectRef`, it returns `ParentRef::None`.
	fn from(value: OwnedValue) -> Self {
		if let Ok(obj_ref) = value.try_into() {
			ParentRef::Some(obj_ref)
		} else {
			ParentRef::None
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{object_ref::ObjectRefBorrowed, ObjectRef, ParentRef};
	use zbus_names::UniqueName;
	use zvariant::{serialized::Context, to_bytes, ObjectPath, Value, LE};

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
		assert_eq!(accessible.path.as_str(), NULL_OBJECT_PATH);
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
		assert_eq!(accessible.path.as_str(), NULL_OBJECT_PATH);
	}

	#[test]
	fn test_try_from_value_for_object_ref() {
		use zvariant::Value;

		let oref = ObjectRef::default();
		let value: Value = oref.into();
		let obj: ObjectRef = value.try_into().unwrap();

		assert_eq!(obj.name.as_str(), ":0.0");
		assert_eq!(obj.path.as_str(), NULL_OBJECT_PATH);
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
		assert_eq!(obj.path.as_str(), NULL_OBJECT_PATH);
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
		let obj_borrow: ObjectRefBorrowed = value.try_into().unwrap();

		assert_eq!(obj_borrow.name.as_str(), ":0.0");
		assert_eq!(obj_borrow.path.as_str(), NULL_OBJECT_PATH);
	}

	#[test]
	fn must_fail_test_try_from_invalid_value_for_object_ref_borrow() {
		let value = zvariant::Value::from((42, true));
		let obj: Result<ObjectRefBorrowed, _> = value.try_into();
		assert!(obj.is_err());
	}

	#[test]
	fn test_objectref_default_doesnt_panic() {
		let objr = ObjectRef::default();
		assert_eq!(objr.name.as_str(), ":0.0");
		assert_eq!(objr.path.as_str(), NULL_OBJECT_PATH);
	}

	#[test]
	fn try_into_value() {
		let objr = ObjectRef::default();
		let value_struct = Value::from(objr);
		let Value::Structure(structure) = value_struct else {
			panic!("Unable to destructure a structure out of the Value.");
		};
		let vals = structure.into_fields();
		assert_eq!(vals.len(), 2);
		let Value::Str(bus_name) = vals.first().unwrap() else {
			panic!("Unable to destructure field value: {:?}", vals.first().unwrap());
		};
		assert_eq!(bus_name, ":0.0");
		let Value::ObjectPath(path) = vals.last().unwrap() else {
			panic!("Unable to destructure field value: {:?}", vals.get(1).unwrap());
		};
		assert_eq!(path.as_str(), NULL_OBJECT_PATH);
	}

	#[test]
	fn parent_ref_serialized_deserialized_to_obj_ref() {
		let test_obj = ObjectRef {
			name: UniqueName::from_static_str(":1.0").unwrap().into(),
			path: ObjectPath::from_static_str("/org/a11y/atspi/accessible/100010")
				.unwrap()
				.into(),
		};

		let parent_ref = ParentRef::Some(test_obj.clone());

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &parent_ref).unwrap();

		// Must be deserializable as an `ObjectRef`
		let (decoded, _) = encoded.deserialize::<ObjectRef>().unwrap();
		assert_eq!(decoded, test_obj);
	}

	#[test]
	fn parent_ref_serialized_deserialized_to_parent_ref() {
		let test_obj = ObjectRef {
			name: UniqueName::from_static_str(":1.0").unwrap().into(),
			path: ObjectPath::from_static_str("/org/a11y/atspi/accessible/100010")
				.unwrap()
				.into(),
		};

		let parent_ref = ParentRef::Some(test_obj.clone());

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &parent_ref).unwrap();

		// Must be deserializable as an `ParentRef`
		let (decoded, _) = encoded.deserialize::<ParentRef>().unwrap();
		assert_eq!(decoded, parent_ref);
	}

	#[test]
	fn default_object_ref_serialized_deserialized_to_parent_ref() {
		let test_obj = ObjectRef::default();

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &test_obj).unwrap();

		// Only empty names are deserializable as `ParentRef::None` as this
		// is only meant to patch that gap.

		let (decoded, _) = encoded.deserialize::<ParentRef>().unwrap();
		assert_eq!(decoded, ParentRef::Some(test_obj));
	}

	#[test]
	fn parent_ref_none_serialized_deserialized_to_object_ref() {
		let test_obj = ParentRef::default();

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &test_obj).unwrap();

		// Must be deserializable as an `ObjectRef`
		let (decoded, _) = encoded.deserialize::<ObjectRef>().unwrap();
		assert_eq!(decoded, ObjectRef::default());
	}

	#[test]
	fn parent_ref_none_into_object_ref() {
		let test_parent_obj = ParentRef::None;

		let obj_ref: ObjectRef = test_parent_obj.into();

		let name = UniqueName::from_static_str(":0.0").unwrap();
		let path = ObjectPath::from_static_str(NULL_OBJECT_PATH).unwrap();
		assert_eq!(obj_ref.name, name.to_owned());
		assert_eq!(obj_ref.path, path.into());
	}

	#[test]
	fn parent_ref_none_into_option_object_ref() {
		let test_parent_obj = ParentRef::None;

		let opt_obj_ref: Option<ObjectRef> = test_parent_obj.into();
		assert!(opt_obj_ref.is_none());
	}
}
