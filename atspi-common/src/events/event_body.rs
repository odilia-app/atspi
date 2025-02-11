use serde::{ser::SerializeMap, Deserialize, Serialize};
use zbus::zvariant::Type;
use zbus_lockstep_macros::validate;
use zvariant::{Array, OwnedValue, Signature, Value};

use crate::{object_ref::ObjectRefBorrow, AtspiError, ObjectRef};

/// Event body as used exclusively by 'Qt' toolkit.
///
/// Signature:  "siiv(so)"
#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
pub struct EventBodyQT {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	// #[serde(rename = "type")]
	pub kind: String,

	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,

	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,

	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	pub any_data: OwnedValue,

	/// A tuple of properties.
	/// Not in use.
	pub properties: ObjectRef,
}

impl Default for EventBodyQT {
	fn default() -> Self {
		Self {
			kind: String::new(),
			detail1: 0,
			detail2: 0,
			any_data: 0_u8.into(),
			properties: ObjectRef::default(),
		}
	}
}

/// Empty struct for properties field.
///
/// AT-SPI2 does not read or write properties.
/// This is a placeholder with implementations for `SerializeDict` and `DeserializeDict`.
/// To make it serialize as an empty dictionary adn deserialize as placeholder.
#[derive(Debug, Clone, Type)]
#[zvariant(signature = "a{sv}")]
pub struct Properties;

impl Default for Properties {
	fn default() -> Self {
		Self
	}
}

impl Serialize for Properties {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		let map = serializer.serialize_map(Some(0))?.end()?;
		Ok(map)
	}
}

impl<'de> Deserialize<'de> for Properties {
	fn deserialize<D>(_deserializer: D) -> Result<Properties, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		Ok(Properties)
	}
}

/// AT-SPI2 protocol native event body type.
///
/// All of the various event-group signals in the AT-SPI2 protocol share this shape.
/// Most toolkits and implementors emit this type, except for `Qt`, which has has its
/// own type: [`EventBodyQT`].
///
/// Signature `(siiva{sv})`,
#[validate(signal: "PropertyChange")]
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct EventBodyOwned {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	#[serde(rename = "type")]
	pub kind: String,

	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,

	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,

	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	///
	pub any_data: OwnedValue,

	/// A map of properties.
	/// Not in use.
	/// See: [`Properties`].
	pub properties: Properties,
}

impl PartialEq for EventBodyOwned {
	fn eq(&self, other: &Self) -> bool {
		self.kind == other.kind
			&& self.detail1 == other.detail1
			&& self.detail2 == other.detail2
			&& self.any_data == other.any_data
	}
}

impl Default for EventBodyOwned {
	fn default() -> Self {
		Self {
			kind: String::new(),
			detail1: 0,
			detail2: 0,
			any_data: 0_u32.into(),
			properties: Properties,
		}
	}
}

/// Safety: This implementation of [`Clone`] *can panic!* Although the chance is extremely remote.
///
/// If:
/// 1. the `any_data` or `properties` field contain an [`std::os::fd::OwnedFd`] type, and
/// 2. the maximum number of open files for the process is exceeded.
///
/// Then, and only then, will this function panic.
/// None of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
/// Events on the AT-SPI bus *could, theoretically* send a file descriptor, but nothing in the
/// specification allows that.
///
/// See [`zvariant::Value::try_clone`] for more information.
impl Clone for EventBodyOwned {
	fn clone(&self) -> Self {
		let cloned_any_data = self.any_data.try_clone().unwrap_or_else(|err| {
			panic!("Failure cloning 'any_data' field: {err:?}");
		});

		Self {
			kind: self.kind.clone(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: cloned_any_data,
			properties: Properties,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct EventBodyBorrow<'a> {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	#[serde(rename = "type")]
	#[serde(borrow)]
	pub kind: &'a str,

	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,

	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,

	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	#[serde(borrow)]
	pub any_data: Value<'a>,

	/// A map of properties.
	/// Not in use.
	pub properties: Properties,
}

impl PartialEq for EventBodyBorrow<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.kind == other.kind
			&& self.detail1 == other.detail1
			&& self.detail2 == other.detail2
			&& self.any_data == other.any_data
	}
}

impl Default for EventBodyBorrow<'_> {
	fn default() -> Self {
		Self {
			kind: "",
			detail1: 0,
			detail2: 0,
			any_data: Value::new(Array::new(&Signature::U8)),
			properties: Properties,
		}
	}
}

impl EventBodyBorrow<'_> {
	/// Convert this borrowed event body to an owned event body.
	///
	/// # Errors
	///
	/// This will error if the following conditions are met:
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	///
	/// Chances are slim because none of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
	/// See [`zvariant::Value::try_clone`] for more information.
	pub fn to_fully_owned(&self) -> Result<EventBodyOwned, AtspiError> {
		let owned_any_data = self.any_data.try_to_owned()?;

		Ok(EventBodyOwned {
			kind: self.kind.into(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: owned_any_data,
			properties: Properties,
		})
	}
}

impl EventBodyBorrow<'_> {
	/// Convert partially borrowed event body to an owned event body.
	///
	/// # Errors
	///
	/// This will error if the following conditions are met:
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	pub fn try_to_owned(self) -> Result<EventBodyOwned, AtspiError> {
		let any_data = self.any_data.try_to_owned()?;

		Ok(EventBodyOwned {
			kind: self.kind.to_owned(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data,
			properties: Properties,
		})
	}
}

#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
pub struct EventBodyQTBorrow<'m> {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	#[serde(rename = "type")]
	pub kind: &'m str,

	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,

	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,

	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	#[serde(borrow)]
	pub any_data: Value<'m>,

	/// A tuple of properties.
	#[serde(borrow)]
	pub properties: ObjectRefBorrow<'m>,
}

impl Default for EventBodyQTBorrow<'_> {
	fn default() -> Self {
		Self {
			kind: "",
			detail1: 0,
			detail2: 0,
			any_data: Value::new(Array::new(&Signature::U8)),
			properties: ObjectRefBorrow::default(),
		}
	}
}

impl EventBodyQTBorrow<'_> {
	/// Convert partially borrowed Qt event body to an owned event body.
	///
	/// # Errors
	///
	/// This will error if the following conditions are met:
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	pub fn try_to_owned(&self) -> Result<EventBodyQT, AtspiError> {
		let properties = self.properties.to_fully_owned();
		let any_data = self.any_data.try_to_owned()?;

		Ok(EventBodyQT {
			kind: self.kind.to_owned(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data,
			properties,
		})
	}
}

impl<'de> From<EventBodyQTBorrow<'de>> for EventBodyBorrow<'de> {
	fn from(borrow: EventBodyQTBorrow<'de>) -> Self {
		let EventBodyQTBorrow { kind, detail1, detail2, any_data, properties: _ } = borrow;

		Self { kind, detail1, detail2, any_data, properties: Properties }
	}
}

impl From<EventBodyOwned> for EventBodyQT {
	fn from(ev: EventBodyOwned) -> Self {
		EventBodyQT {
			kind: ev.kind,
			detail1: ev.detail1,
			detail2: ev.detail2,
			any_data: ev.any_data,
			properties: ObjectRef::default(),
		}
	}
}

impl From<EventBodyQT> for EventBodyOwned {
	fn from(body: EventBodyQT) -> Self {
		Self {
			kind: body.kind,
			detail1: body.detail1,
			detail2: body.detail2,
			any_data: body.any_data,
			properties: Properties,
		}
	}
}

impl PartialEq<EventBodyOwned> for EventBodyQT {
	fn eq(&self, other: &EventBodyOwned) -> bool {
		self.kind == other.kind
			&& self.detail1 == other.detail1
			&& self.detail2 == other.detail2
			&& self.any_data == other.any_data
	}
}

impl PartialEq<EventBodyQT> for EventBodyOwned {
	fn eq(&self, other: &EventBodyQT) -> bool {
		self.kind == other.kind
			&& self.detail1 == other.detail1
			&& self.detail2 == other.detail2
			&& self.any_data == other.any_data
	}
}

impl PartialEq<EventBodyBorrow<'_>> for EventBodyQTBorrow<'_> {
	fn eq(&self, other: &EventBodyBorrow<'_>) -> bool {
		self.kind == other.kind
			&& self.detail1 == other.detail1
			&& self.detail2 == other.detail2
			&& self.any_data == other.any_data
	}
}

impl PartialEq<EventBodyQTBorrow<'_>> for EventBodyBorrow<'_> {
	fn eq(&self, other: &EventBodyQTBorrow<'_>) -> bool {
		self.kind == other.kind
			&& self.detail1 == other.detail1
			&& self.detail2 == other.detail2
			&& self.any_data == other.any_data
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[cfg(test)]
	#[test]
	fn owned_event_body_clone() {
		let event = EventBodyOwned::default();
		let cloned = event.clone();

		assert_eq!(event, cloned);
	}

	#[test]
	fn owned_event_body_default() {
		let event = EventBodyOwned::default();

		assert_eq!(event.kind, "");
		assert_eq!(event.detail1, 0);
		assert_eq!(event.detail2, 0);
		assert_eq!(event.any_data, 0_u32.into());
	}

	#[test]
	fn qt_event_body_default() {
		let event = EventBodyQT::default();

		assert_eq!(event.kind, "");
		assert_eq!(event.detail1, 0);
		assert_eq!(event.detail2, 0);
		assert_eq!(event.any_data, 0_u8.into());
		assert_eq!(event.properties, ObjectRef::default());
	}

	#[test]
	fn qt_to_owned() {
		let qt = EventBodyQT::default();
		let owned: EventBodyOwned = EventBodyQT::default().into();

		assert_eq!(owned, qt);
	}

	#[cfg(test)]
	mod owned_to_qt {
		use crate::events::{EventBodyOwned, EventBodyQT};

		#[test]
		fn owned_to_qt() {
			let owned = EventBodyOwned::default();
			let qt: EventBodyQT = owned.into();

			assert_eq!(qt, EventBodyOwned::default());
		}
	}

	#[cfg(test)]
	mod borrowed_to_qt {
		use crate::events::event_body::{EventBodyBorrow, EventBodyQTBorrow};

		#[test]
		fn borrowed_to_qt() {
			let borrowed: EventBodyBorrow = EventBodyQTBorrow::default().into();

			assert_eq!(borrowed, EventBodyBorrow::default());
		}
	}
}
