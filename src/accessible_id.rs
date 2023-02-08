use zbus::zvariant::{
  Error as ZvariantError,
  Value,
  Type,
	OwnedValue,
	ObjectPath,
	OwnedObjectPath,
	Signature,
};
use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Unexpected, self}};
use crate::{
  error::ObjectPathConversionError,
};

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Type)]
#[zvariant(signature = "o")]
pub enum AccessibleId {
    Null,
    Root,
    Number(i64),
}
impl TryFrom<Value<'static>> for AccessibleId {
  type Error = ZvariantError;
  fn try_from(value: Value<'static>) -> Result<AccessibleId, Self::Error> {
    match value {
      Value::ObjectPath(path_id) =>
        match path_id.as_str().try_into() {
          Ok(id) => Ok(id),
          Err(_) => Err(ZvariantError::Message("Incorrect string format; it must be in the format: /org/a11y/atspi/accessible/ID, where ID is between 0 and `i64::MAX`.".to_string())),
        },
      _ => Err(ZvariantError::Message("Invalid type to convert into an AccessibleId. It must be a Value::Str type".to_string())),
    }
  }
}
impl Serialize for AccessibleId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where S: Serializer {
    serializer.serialize_str(&self.to_string())
  }
}
impl<'de> Deserialize<'de> for AccessibleId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
  where
    D: Deserializer<'de>,
  {
    let string_id = String::deserialize(deserializer)?;
    match string_id.clone().try_into() {
      Ok(id) => Ok(id),
      Err(_e) => Err(de::Error::invalid_value(
        Unexpected::Str(&string_id),
        &"Format must be like /org/a11y/atspi/accessible/ID, where ID is some positive value from `i64::ZERO` to `i64::MAX`.",
      ))
    }
  }
}
impl ToString for AccessibleId {
    fn to_string(&self) -> String {
        let ending = match self {
            Self::Null => "null".to_string(),
            Self::Root => "root".to_string(),
            Self::Number(int) => int.to_string(),
        };
        format!("/org/a11y/atspi/accessible/{ending}")
    }
}
impl TryFrom<AccessibleId> for OwnedObjectPath {
  type Error = zbus::zvariant::Error;
  fn try_from(id: AccessibleId) -> Result<OwnedObjectPath, Self::Error> {
    OwnedObjectPath::try_from(id.to_string())
  }
}
impl<'a> TryInto<ObjectPath<'a>> for AccessibleId {
    type Error = zbus::zvariant::Error;

    fn try_into(self) -> Result<ObjectPath<'a>, Self::Error> {
        ObjectPath::try_from(self.to_string())
    }
}

impl TryFrom<OwnedObjectPath> for AccessibleId {
    type Error = ObjectPathConversionError;

    fn try_from(path: OwnedObjectPath) -> Result<Self, Self::Error> {
        match path.split('/').next_back() {
            Some("null") => Ok(AccessibleId::Null),
            Some("root") => Ok(AccessibleId::Root),
            Some(id) => match id.parse::<i64>() {
                Ok(uid) => Ok(AccessibleId::Number(uid)),
                Err(e) => Err(Self::Error::ParseError(e)),
            },
            None => Err(Self::Error::NoIdAvailable),
        }
    }
}
impl TryFrom<&str> for AccessibleId {
	type Error = zbus::zvariant::Error;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        match path.split('/').next_back() {
            Some("null") => Ok(AccessibleId::Null),
            Some("root") => Ok(AccessibleId::Root),
            Some(id) => match id.parse::<i64>() {
                Ok(uid) => Ok(AccessibleId::Number(uid)),
                Err(_) => Err(Self::Error::Message("Unable to parse the ID as part of a conversion from a String to an AccessibleId.".to_string())),
            },
            None => Err(Self::Error::Message("No ID in attempted conversion from a String to an AccessibleId".to_string())),
        }
    }
}
impl TryFrom<String> for AccessibleId {
	type Error = zbus::zvariant::Error;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        match path.split('/').next_back() {
            Some("null") => Ok(AccessibleId::Null),
            Some("root") => Ok(AccessibleId::Root),
            Some(id) => match id.parse::<i64>() {
                Ok(uid) => Ok(AccessibleId::Number(uid)),
                Err(_) => Err(Self::Error::Message("Unable to parse the ID as part of a conversion from a String to an AccessibleId.".to_string())),
            },
            None => Err(Self::Error::Message("No ID in attempted conversion from a String to an AccessibleId".to_string())),
        }
    }
}

impl<'a> TryFrom<&'a ObjectPath<'a>> for AccessibleId {
    type Error = zbus::zvariant::Error;

    fn try_from(path: &'a ObjectPath<'a>) -> Result<Self, Self::Error> {
        match path.split('/').next_back() {
            Some("null") => Ok(AccessibleId::Null),
            Some("root") => Ok(AccessibleId::Root),
            Some(id) => match id.parse::<i64>() {
                Ok(uid) => Ok(AccessibleId::Number(uid)),
                Err(e) => Err(Self::Error::Message(format!("{e}"))),
            },
            None => Err(Self::Error::Message("No ID in attempted conversion from a String to an AccessibleId".to_string())),
				}
		}
}
impl<'a> TryFrom<ObjectPath<'a>> for AccessibleId {
    type Error = zbus::zvariant::Error;

    fn try_from(path: ObjectPath<'a>) -> Result<Self, Self::Error> {
        match path.split('/').next_back() {
            Some("null") => Ok(AccessibleId::Null),
            Some("root") => Ok(AccessibleId::Root),
            Some(id) => match id.parse::<i64>() {
                Ok(uid) => Ok(AccessibleId::Number(uid)),
                Err(e) => Err(Self::Error::Message(format!("{e}"))),
            },
            None => Err(Self::Error::Message("No ID in attempted conversion from a String to an AccessibleId".to_string())),
				}
		}
}

impl TryFrom<OwnedValue> for AccessibleId {
	type Error = zbus::zvariant::Error;

	fn try_from(path: OwnedValue) -> Result<Self, Self::Error> {
		let string_sig = Signature::try_from("s")?;
		if path.value_signature() != string_sig {
			return Err(Self::Error::SignatureMismatch(path.value_signature().to_owned(), "The value type which attempted to be transformed into an AccessibleId is not valid. It should be \"s\".".to_string()));
		}
		let path_string = String::try_from(path)?;
		path_string.try_into()
	}
}

#[allow(clippy::module_name_repetitions)]
pub trait HasAccessibleId {
	type Error: std::error::Error;

	/// Gets the accessible ID of an item.
	/// This must be implemented for any type which implements the [`crate::accessible::Accessible`] trait.
	/// But it is separated since it should never by async... in theory.
	///
	/// # Errors
	/// * Will return an error if either: the field is not readable (may be behind an `RWLock`),
	/// * or if a conversion from some other type was not able to be parsed into the `AccessibleId`.
	fn id(&self) -> Result<AccessibleId, Self::Error>;
}

#[cfg(test)]
mod tests {
  use serde_plain;
  use crate::AccessibleId;

  #[test]
  fn deserialize_root_object_path() {
    let root_str = "/org/a11y/atspi/accessible/root";
    let id: AccessibleId = serde_plain::from_str(root_str).expect("Can not deserialize {root_str}");
    assert_eq!(id, AccessibleId::Root);
  }
  #[test]
  fn deserialize_null_object_path() {
    let root_str = "/org/a11y/atspi/accessible/null";
    let id = AccessibleId::try_from(root_str).expect("Can not deserialize {root_str}");
    assert_eq!(id, AccessibleId::Null);
  }
  #[test]
  fn deserialize_zero_object_path() {
    let root_str = "/org/a11y/atspi/accessible/0";
    let id = AccessibleId::try_from(root_str).expect("Can not deserialize {root_str}");
    assert_eq!(id, AccessibleId::Number(0));
  }
  #[test]
  fn deserialize_1337_object_path() {
    let root_str = "/org/a11y/atspi/accessible/1337";
    let id = AccessibleId::try_from(root_str).expect("Can not deserialize {root_str}");
    assert_eq!(id, AccessibleId::Number(1337));
  }
  // this test is specifically because we nned to check for i64-sized numbers.
  #[test]
  fn deserialize_large_num_object_path() {
    let root_str = "/org/a11y/atspi/accessible/123923283733455";
    let id = AccessibleId::try_from(root_str).expect("Can not deserialize {root_str}");
    assert_eq!(id, AccessibleId::Number(123923283733455));
  }
  #[test]
  fn serialize_root_object_path() {
    let id = AccessibleId::Root;
    let root_str = serde_plain::to_string(&id).expect("Could not deserialize {id}");
    assert_eq!(root_str, "/org/a11y/atspi/accessible/root".to_string());
  }
  #[test]
  fn serialize_null_object_path() {
    let id = AccessibleId::Null;
    let null_str = serde_plain::to_string(&id).expect("Could not deserialize {id}");
    assert_eq!(null_str, "/org/a11y/atspi/accessible/null".to_string());
  }
  #[test]
  fn serialize_zero_object_path() {
    let id = AccessibleId::Number(0);
    let zero_str = serde_plain::to_string(&id).expect("Could not deserialize {id}");
    assert_eq!(zero_str, "/org/a11y/atspi/accessible/0");
  }
  #[test]
  fn serialize_1337_object_path() {
    let id = AccessibleId::Number(1337);
    let one_one_three_one_str = serde_plain::to_string(&id).expect("Could not deserialize {id}");
    assert_eq!(one_one_three_one_str, "/org/a11y/atspi/accessible/1337".to_string());
  }
  // this test is specifically because we nned to check for i64-sized numbers.
  #[test]
  fn serialize_large_num_object_path() {
    let id = AccessibleId::Number(123923283733455);
    let large_str = serde_plain::to_string(&id).expect("Could not deserialize {id}");
    assert_eq!(large_str, "/org/a11y/atspi/accessible/123923283733455".to_string());
  }
}
