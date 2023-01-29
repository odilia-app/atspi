use zbus::zvariant::{
	OwnedValue,
	ObjectPath,
	OwnedObjectPath,
	Signature,
};
use serde::{Serialize, Deserialize};
use crate::error::ObjectPathConversionError;

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AccessibleId {
    Null,
    Root,
    Number(i64),
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
