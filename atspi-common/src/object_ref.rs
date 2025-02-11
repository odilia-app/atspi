use serde::{Deserialize, Serialize};
use zbus_lockstep_macros::validate;
use zbus_names::{OwnedUniqueName, UniqueName};
use zvariant::{ObjectPath, OwnedObjectPath, Signature, Type};

// Equiv to "(so)"
pub const OBJECT_REF_SIGNATURE: &Signature =
	&Signature::static_structure(&[&Signature::Str, &Signature::ObjectPath]);

/// `ObjectRef` type
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

#[validate(signal: "Available")]
#[derive(Debug, Clone, Serialize, Type, PartialEq, Eq, Hash)]
pub struct ObjectRefBorrow<'a> {
	#[serde(borrow)]
	pub name: UniqueName<'a>,
	#[serde(borrow)]
	pub path: ObjectPath<'a>,
}

impl<'de: 'or, 'or> Deserialize<'de> for ObjectRefBorrow<'or> {
	fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let (name, path) = <(UniqueName<'or>, ObjectPath<'or>)>::deserialize(deserializer)?;
		Ok(Self { name, path })
	}
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

#[cfg(test)]
#[test]
fn test_accessible_from_dbus_ctxt_to_accessible() {
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

#[cfg(test)]
#[test]
fn test_accessible_value_wrapped_from_dbus_ctxt_to_accessible() {
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

impl<'a> TryFrom<zvariant::Value<'a>> for ObjectRef {
	type Error = zvariant::Error;
	fn try_from(value: zvariant::Value<'a>) -> Result<Self, Self::Error> {
		value.try_to_owned()?.try_into()
	}
}

impl TryFrom<zvariant::OwnedValue> for ObjectRef {
	type Error = zvariant::Error;
	fn try_from<'a>(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
		match &*value {
			zvariant::Value::Structure(s) => {
				if s.signature() != OBJECT_REF_SIGNATURE {
					return Err(zvariant::Error::SignatureMismatch(s.signature().clone(), format!("To turn a zvariant::Value into an atspi::ObjectRef, it must be of type {OBJECT_REF_SIGNATURE}")));
				}
				let fields = s.fields();
				let name: String =
					fields.first().ok_or(zvariant::Error::IncorrectType)?.try_into()?;
				let path_value: ObjectPath<'_> =
					fields.last().ok_or(zvariant::Error::IncorrectType)?.try_into()?;
				Ok(ObjectRef {
					name: name.try_into().map_err(|_| zvariant::Error::IncorrectType)?,
					path: path_value.into(),
				})
			}
			_ => Err(zvariant::Error::IncorrectType),
		}
	}
}

impl From<ObjectRef> for zvariant::Structure<'_> {
	fn from(obj: ObjectRef) -> Self {
		(obj.name, obj.path).into()
	}
}
