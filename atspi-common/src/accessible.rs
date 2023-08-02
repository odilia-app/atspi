use crate::events::signatures_are_eq;
use serde::{Deserialize, Serialize};
use zvariant::{ObjectPath, OwnedObjectPath, Signature, Type};

pub const ACCESSIBLE_PAIR_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(so)");

// TODO: Try to make borrowed versions work,
// check where the lifetimes of the borrow are tied to, see also: comment on `interface()` method
// in `DefaultEvent` impl
// then rename into Owned for this one.
/// Owned Accessible type
/// Emitted by `CacheRemove` and `Available`
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct Accessible {
	pub name: String,
	pub path: OwnedObjectPath,
}

impl Default for Accessible {
	fn default() -> Self {
		Accessible {
			name: ":0.0".into(),
			path: ObjectPath::from_static_str("/org/a11y/atspi/accessible/null")
				.unwrap()
				.into(),
		}
	}
}

#[test]
fn test_accessible_signature() {
	assert_eq!(
		Accessible::signature(),
		ACCESSIBLE_PAIR_SIGNATURE,
		"Accessible does not have the correct type."
	);
}

#[test]
fn test_accessible_from_dbus_ctxt_to_accessible() {
	use zvariant::{from_slice, to_bytes, EncodingContext as Context, Value};

	let acc = Accessible::default();
	let ctxt = Context::<byteorder::LE>::new_dbus(0);
	let acc_value: Value<'_> = acc.try_into().unwrap();
	let encoded = to_bytes(ctxt, &acc_value).unwrap();
	let decoded: Value = from_slice(&encoded, ctxt).unwrap();
	let accessible: Accessible = decoded.try_into().unwrap();

	assert_eq!(accessible.name.as_str(), ":0.0");
	assert_eq!(accessible.path.as_str(), "/org/a11y/atspi/accessible/null");
}

#[test]
fn test_accessible_value_wrapped_from_dbus_ctxt_to_accessible() {
	use zvariant::{from_slice, to_bytes, EncodingContext as Context, Value};

	let acc = Accessible::default();
	let value: zvariant::Value = acc.into();
	let ctxt = Context::<byteorder::LE>::new_dbus(0);
	let encoded = to_bytes(ctxt, &value).unwrap();
	let decoded: Value = from_slice(&encoded, ctxt).unwrap();
	let accessible: Accessible = decoded.try_into().unwrap();

	assert_eq!(accessible.name.as_str(), ":0.0");
	assert_eq!(accessible.path.as_str(), "/org/a11y/atspi/accessible/null");
}

impl<'a> TryFrom<zvariant::Value<'a>> for Accessible {
	type Error = zvariant::Error;
	fn try_from(value: zvariant::Value<'a>) -> Result<Self, Self::Error> {
		value.to_owned().try_into()
	}
}

impl TryFrom<zvariant::OwnedValue> for Accessible {
	type Error = zvariant::Error;
	fn try_from<'a>(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
		match &*value {
			zvariant::Value::Structure(s) => {
				if !signatures_are_eq(&s.signature(), &ACCESSIBLE_PAIR_SIGNATURE) {
					return Err(zvariant::Error::SignatureMismatch(s.signature(), format!("To turn a zvariant::Value into an atspi::Accessible, it must be of type {}", ACCESSIBLE_PAIR_SIGNATURE.as_str())));
				}
				let fields = s.fields();
				let name: String =
					fields.get(0).ok_or(zvariant::Error::IncorrectType)?.try_into()?;
				let path_value: ObjectPath<'_> =
					fields.get(1).ok_or(zvariant::Error::IncorrectType)?.try_into()?;
				Ok(Accessible { name, path: path_value.into() })
			}
			_ => Err(zvariant::Error::IncorrectType),
		}
	}
}

impl From<Accessible> for zvariant::Structure<'_> {
	fn from(accessible: Accessible) -> Self {
		(accessible.name.as_str().to_string(), accessible.path).into()
	}
}
