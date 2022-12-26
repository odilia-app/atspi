//! A module to convert "yes"/"no" from a string to a bool.
//! This is exceptionally strange behaviour from an API. Just use `bool`.
//! But anyway, since Lunanode does indeed use this system, a custom module to serialize and
//! deserialize their "yes"/"no" responses was necessary.

use zvariant::Signature;
use serde::{
  de::{
    self,
    Unexpected,
  },
  Serializer,
  Deserializer,
  Deserialize,
}; 

/// Seiralize bool into "yes"/"no", just like the LunaNode API does.
pub fn serialize<S>(signature: &Signature<'static>, serializer: S) -> Result<S::Ok, S::Error> 
where
  S: Serializer {
  serializer.serialize_str(signature.as_str())
}

/// Deserialize bool from String with custom value mapping "yes" => true, "no" => false
pub fn deserialize<'de, D>(deserializer: D) -> Result<Signature<'static>, D::Error>
where
    D: Deserializer<'de>,
{
		let string_deserializer = String::deserialize(deserializer)?;
    let zbus_signature = string_deserializer.as_ref();
		match Signature::try_from(zbus_signature) {
			Ok(sig) => Ok(sig.to_owned()),
			Err(_) => Err(de::Error::invalid_value(
				Unexpected::Str(zbus_signature),
				&"Cannot create zbus signature from string",
			)),
		}
}
