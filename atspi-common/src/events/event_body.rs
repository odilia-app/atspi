use serde::{
	ser::{SerializeMap, SerializeTuple as _},
	Deserialize, Serialize,
};
use zbus::zvariant::Type;
use zbus_lockstep_macros::validate;
use zvariant::{Array, ObjectPath, OwnedValue, Signature, Value};

use crate::AtspiError;

use super::{ATSPI_EVENT_SIGNATURE, QSPI_EVENT_SIGNATURE};

/// Event body as used exclusively by 'Qt' toolkit.
///
/// Signature:  "siiv(so)"
#[derive(Debug, Serialize, Deserialize, PartialEq, Type)]
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
	/// See: [`QtProperties`].
	pub(crate) properties: QtProperties,
}

impl Clone for EventBodyQT {
	/// # Safety  
	///
	/// This implementation of [`Clone`] *can panic!* although chances are slim.
	///
	/// If the following conditions are met:
	/// 1. the `any_data` or `properties` field contain an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	///
	/// Then this function panic.  
	/// None of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
	/// Events on the AT-SPI bus *could, theoretically* send a file descriptor, but nothing in the current
	/// specification describes that.  
	/// See [`zvariant::Value::try_clone`] for more information.
	fn clone(&self) -> Self {
		let cloned_any_data = self.any_data.try_clone().unwrap_or_else(|err| {
			panic!("Failure cloning 'any_data' field: {err:?}");
		});

		Self {
			kind: self.kind.clone(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: cloned_any_data,
			properties: QtProperties,
		}
	}
}

/// Unit struct placeholder for `EventBodyQT.properties`
///
/// AT-SPI2 never reads or writes to `EventBodyQT.properties`.  
/// `QtProperties` has the appropriate implementations for `Serialize` and `Deserialize`  
/// to make it serialize as an a valid tuple and valid bytes deserialize as placeholder.
#[derive(Debug, Copy, Clone, Type, PartialEq)]
#[zvariant(signature = "(so)")]
pub(crate) struct QtProperties;

impl Serialize for QtProperties {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		let mut tuple = serializer.serialize_tuple(2)?;
		// shortest valid name & path
		tuple.serialize_element(&":0.0")?;
		tuple.serialize_element(&ObjectPath::from_static_str_unchecked("/"))?;
		tuple.end()
	}
}

impl<'de> Deserialize<'de> for QtProperties {
	fn deserialize<D>(deserializer: D) -> Result<QtProperties, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		struct ObjectRefVisitor;

		impl<'de> serde::de::Visitor<'de> for ObjectRefVisitor {
			type Value = QtProperties;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("a D-Bus tuple of (so)")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				seq.next_element::<&str>()?;
				seq.next_element::<ObjectPath>()?;
				Ok(QtProperties)
			}
		}

		deserializer.deserialize_tuple(2, ObjectRefVisitor)
	}
}

impl Default for EventBodyQT {
	fn default() -> Self {
		Self {
			kind: String::new(),
			detail1: 0,
			detail2: 0,
			any_data: 0_u8.into(),
			properties: QtProperties,
		}
	}
}

/// Unit struct placeholder for `EventBody.properties`
///
/// AT-SPI2 never reads or writes to `EventBody.properties`.  
/// `Properties` has the appropriate implementations for `Serialize` and `Deserialize`  
/// to make it serialize as an a valid dictionary and valid bytes deserialize as placeholder.
#[derive(Debug, Copy, Clone, Type, PartialEq)]
#[zvariant(signature = "a{sv}")]
pub(crate) struct Properties;

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
	fn deserialize<D>(deserializer: D) -> Result<Properties, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		struct MapVisitor;

		impl<'de> serde::de::Visitor<'de> for MapVisitor {
			type Value = Properties;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("a D-Bus dictionary of type a{sv}")
			}

			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::MapAccess<'de>,
			{
				while map.next_entry::<&str, Value>()?.is_some() {}
				Ok(Properties)
			}
		}

		deserializer.deserialize_map(MapVisitor)
	}
}

/// AT-SPI2 protocol native event body type.
///
/// All of the various signals in the AT-SPI2 protocol share this shape.
/// Most toolkits and implementors emit this type, except for `Qt`, which has has its
/// own type: [`EventBodyQT`].
///
/// Signature `(siiva{sv})`,
#[validate(signal: "PropertyChange")]
#[derive(Debug, Serialize, Deserialize, PartialEq, Type)]
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
	pub(crate) properties: Properties,
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

impl Clone for EventBodyOwned {
	/// # Safety  
	///
	/// This implementation of [`Clone`] *can panic!* although chances are slim.
	///
	/// If the following conditions are met:
	/// 1. the `any_data` or `properties` field contain an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	///
	/// Then this function panic.  
	/// None of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
	/// Events on the AT-SPI bus *could, theoretically* send a file descriptor, but nothing in the current
	/// specification describes that.  
	/// See [`zvariant::Value::try_clone`] for more information.
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Type)]
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
	pub(crate) properties: Properties,
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
	pub(crate) properties: QtProperties,
}

impl Default for EventBodyQTBorrow<'_> {
	fn default() -> Self {
		Self {
			kind: "",
			detail1: 0,
			detail2: 0,
			any_data: Value::new(Array::new(&Signature::U8)),
			properties: QtProperties,
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
		let any_data = self.any_data.try_to_owned()?;

		Ok(EventBodyQT {
			kind: self.kind.to_owned(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data,
			properties: self.properties,
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
	fn from(body: EventBodyOwned) -> Self {
		EventBodyQT {
			kind: body.kind,
			detail1: body.detail1,
			detail2: body.detail2,
			any_data: body.any_data,
			properties: QtProperties,
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

/// Common event body that can be either owned or borrowed.
///
/// This is useful for APIs that can return either owned or borrowed event bodies.  
/// Having this type allows to be generic over the event body type.
#[derive(Debug)]
pub enum EventBody<'a> {
	Owned(EventBodyOwned),
	Borrowed(EventBodyBorrow<'a>),
}

impl EventBody<'_> {
	/// Non-consuming conversion to an owned event body.
	///
	/// Does cloning.
	///
	/// # Errors
	/// The borrowed variant will error if the following conditions are met:  
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and  
	/// 2. the maximum number of open files for the process is exceeded.
	pub fn as_owned(&self) -> Result<EventBodyOwned, AtspiError> {
		match self {
			EventBody::Owned(owned) => Ok(owned.clone()),
			EventBody::Borrowed(borrowed) => borrowed.to_fully_owned(),
		}
	}

	/// Consuming conversion to an owned event body.
	///
	/// Does cloning.
	///
	/// # Errors
	/// The borrowed variant will error if the following conditions are met:  
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and  
	/// 2. the maximum number of open files for the process is exceeded.
	pub fn into_owned(self) -> Result<EventBodyOwned, AtspiError> {
		match self {
			EventBody::Owned(owned) => Ok(owned),
			EventBody::Borrowed(borrowed) => borrowed.to_fully_owned(),
		}
	}
}

impl Type for EventBody<'_> {
	const SIGNATURE: &'static zvariant::Signature = ATSPI_EVENT_SIGNATURE;
}

impl<'de: 'a, 'a> Deserialize<'de> for EventBody<'a> {
	fn deserialize<D>(deserializer: D) -> Result<EventBody<'a>, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		let borrowed = EventBodyBorrow::deserialize(deserializer)?;
		Ok(EventBody::Borrowed(borrowed))
	}
}

/// Qt event body that can be either owned or borrowed.
///
/// This is useful for APIs that can return either owned or borrowed event bodies.  
/// Having this type allows to be generic over the event body type.
#[derive(Debug)]
pub enum EventBodyQt<'a> {
	Owned(EventBodyQT),
	Borrowed(EventBodyQTBorrow<'a>),
}

impl Type for EventBodyQt<'_> {
	const SIGNATURE: &'static zvariant::Signature = QSPI_EVENT_SIGNATURE;
}

impl<'de: 'a, 'a> Deserialize<'de> for EventBodyQt<'a> {
	fn deserialize<D>(deserializer: D) -> Result<EventBodyQt<'a>, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		let borrowed = EventBodyQTBorrow::deserialize(deserializer)?;
		Ok(EventBodyQt::Borrowed(borrowed))
	}
}

impl EventBodyQt<'_> {
	/// Non-consuming conversion to an owned event body.
	///
	/// Does cloning.
	///
	/// # Errors
	/// The borrowed variant will error if the following conditions are met:  
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and  
	/// 2. the maximum number of open files for the process is exceeded.
	pub fn as_owned(&self) -> Result<EventBodyQT, AtspiError> {
		match self {
			EventBodyQt::Owned(owned) => Ok(owned.clone()),
			EventBodyQt::Borrowed(borrowed) => borrowed.try_to_owned(),
		}
	}

	/// Consuming conversion to an owned event body.
	///
	/// Does cloning.
	///
	/// # Errors
	/// The borrowed variant will error if the following conditions are met:  
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and  
	/// 2. the maximum number of open files for the process is exceeded.
	pub fn into_owned(self) -> Result<EventBodyQT, AtspiError> {
		match self {
			EventBodyQt::Owned(owned) => Ok(owned),
			EventBodyQt::Borrowed(borrowed) => borrowed.try_to_owned(),
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
	use crate::object_ref::ObjectRefBorrow;
	use crate::ObjectRef;
	use std::collections::HashMap;
	use zvariant::{serialized::Context, LE};

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
		assert_eq!(event.properties, QtProperties);
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

	#[test]
	fn borrowed_to_qt() {
		let borrowed: EventBodyBorrow = EventBodyQTBorrow::default().into();

		assert_eq!(borrowed, EventBodyBorrow::default());
	}

	#[test]
	fn test_object_ref_borrow_deserializes_as_qt_properties() {
		let object_ref_borrow = ObjectRefBorrow::default();

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<ObjectRefBorrow>(ctxt, &object_ref_borrow).unwrap();

		let (properties, _) = bytes.deserialize::<QtProperties>().unwrap();

		assert_eq!(properties, QtProperties);
	}

	#[test]
	fn test_valid_hashmap_of_string_value_deserializes_as_properties() {
		let val = Value::from(0_u32);
		let key = "test";
		let map = HashMap::from([(key, val)]);

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<HashMap<&str, Value>>(ctxt, &map).unwrap();

		let (properties, _) = bytes.deserialize::<Properties>().unwrap();

		assert_eq!(properties, Properties);
	}

	#[test]
	fn test_object_ref_deserializes_as_qt_properties() {
		let object_ref = ObjectRef::default();

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<ObjectRef>(ctxt, &object_ref).unwrap();

		let (qt_props, _) = bytes.deserialize::<QtProperties>().unwrap();

		assert_eq!(qt_props, QtProperties);
	}

	#[test]
	fn test_deserializing_arbitrary_data_into_properties() {
		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<&str>(ctxt, &"test").unwrap();

		assert!(bytes.deserialize::<Properties>().is_err());
	}

	#[test]
	fn test_deserialize_arbitrary_data_as_into_qt_properties() {
		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<&str>(ctxt, &"ola").unwrap();

		assert!(bytes.deserialize::<QtProperties>().is_err());
	}

	#[test]
	fn test_properties_serializes_as_valid_hashmap() {
		let properties = Properties;
		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<Properties>(ctxt, &properties).unwrap();

		let (map, _) = bytes.deserialize::<HashMap<&str, Value>>().unwrap();

		assert_eq!(map, HashMap::new());
	}

	#[test]
	fn test_qt_properties_serializes_as_valid_string_objpath_tuple() {
		let qt_properties = QtProperties;
		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<QtProperties>(ctxt, &qt_properties).unwrap();

		let (tuple, _) = bytes.deserialize::<(&str, ObjectPath)>().unwrap();

		assert_eq!(tuple, (":0.0", ObjectPath::from_static_str_unchecked("/")));
	}

	#[test]
	fn test_qt_properties_serializes_as_valid_object_ref() {
		let qt_properties = QtProperties;
		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<QtProperties>(ctxt, &qt_properties).unwrap();

		let (objectref, _) = bytes.deserialize::<ObjectRef>().unwrap();

		assert_eq!(objectref.name, ":0.0");
		assert_eq!(objectref.path, ObjectPath::from_static_str_unchecked("/").into());
	}
}
