use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zbus::names::OwnedUniqueName;
use zbus::zvariant::Type;
use zbus_lockstep_macros::validate;
use zbus_names::UniqueName;
use zvariant::{OwnedValue, Value};

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

impl Default for EventBodyQT {
	fn default() -> Self {
		Self {
			kind: String::new(),
			detail1: 0,
			detail2: 0,
			any_data: 0u8.into(),
			properties: ObjectRef::default(),
		}
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
#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
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
	pub any_data: OwnedValue,
	/// A map of properties.
	/// Not in use.
	pub properties: HashMap<OwnedUniqueName, OwnedValue>,
}

impl From<EventBodyQT> for EventBodyOwned {
	fn from(body: EventBodyQT) -> Self {
		let mut props = HashMap::new();

		let name = body.properties.name;
		let path = body.properties.path;

		// We know `path` is a `OwnedObjectPath`, so the conversion to
		// `OwnedValue` is infallible at present.
		// Should this ever change, we need to know.
		let value = Value::ObjectPath(path.into()).try_to_owned().unwrap_or_else(|err| {
			panic!("Error occurred: {err:?}");
		});

		props.insert(name, value);
		Self {
			kind: body.kind,
			detail1: body.detail1,
			detail2: body.detail2,
			any_data: body.any_data,
			properties: props,
		}
	}
}

impl Default for EventBodyOwned {
	fn default() -> Self {
		Self {
			kind: String::new(),
			detail1: 0,
			detail2: 0,
			any_data: 0u8.into(),
			properties: HashMap::new(),
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

		let cloned_properties = {
			let mut map = HashMap::new();
			for (key, value) in &self.properties {
				let cloned_value = value.try_clone().unwrap_or_else(|err| {
					panic!("Failure cloning 'props' field: {err:?}");
				});
				map.insert(key.clone(), cloned_value);
			}
			map
		};

		Self {
			kind: self.kind.clone(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: cloned_any_data,
			properties: cloned_properties,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
pub struct EventBodyBorrow<'msg> {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	pub kind: &'msg str,

	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,

	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,

	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	#[serde(borrow)]
	pub any_data: Value<'msg>,

	/// A map of properties.
	/// Not in use.
	#[serde(borrow)]
	pub properties: HashMap<UniqueName<'msg>, Value<'msg>>,
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
impl Clone for EventBodyBorrow<'_> {
	fn clone(&self) -> Self {
		let cloned_any_data = self.any_data.try_clone().unwrap_or_else(|err| {
			panic!("Failure cloning 'any_data' field: {err:?}");
		});

		let cloned_properties = {
			let mut map = HashMap::new();
			for (key, value) in &self.properties {
				let cloned_value = value.try_clone().unwrap_or_else(|err| {
					panic!("Failure cloning 'props' field: {err:?}");
				});
				map.insert(key.clone(), cloned_value);
			}
			map
		};

		Self {
			kind: self.kind,
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: cloned_any_data,
			properties: cloned_properties,
		}
	}
}

impl<'msg> EventBodyBorrow<'msg> {
	pub fn try_to_owned(&self) -> Result<EventBodyOwned, AtspiError> {
		let mut properties = HashMap::new();
		for (key, value) in self.properties.iter() {
			let key: OwnedUniqueName = key.to_owned().into();
			let value: OwnedValue = value.try_to_owned()?;
			properties.insert(key, value);
		}

		let any_data = self.any_data.try_to_owned()?;

		Ok(EventBodyOwned {
			kind: self.kind.to_owned(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data,
			properties,
		})
	}
}

#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
pub struct EventBodyQTBorrow<'msg> {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	// #[serde(rename = "type")]
	pub kind: &'msg str,

	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,

	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,

	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	#[serde(borrow)]
	pub any_data: Value<'msg>,

	/// A tuple of properties.
	/// Not in use.
	#[serde(borrow)]
	pub properties: ObjectRefBorrow<'msg>,
}

impl<'msg> EventBodyQTBorrow<'msg> {
	pub fn try_to_owned(&self) -> Result<EventBodyOwned, AtspiError> {
		let ObjectRef { name, path } = self.properties.to_fully_owned();
		let path = Value::from(path).try_into()?;
		let properties = HashMap::from([(name, path)]);

		let any_data = self.any_data.try_to_owned()?;

		Ok(EventBodyOwned {
			kind: self.kind.to_owned(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data,
			properties,
		})
	}
}

impl<'msg> From<EventBodyQTBorrow<'msg>> for EventBodyBorrow<'msg> {
	fn from(borrowed: EventBodyQTBorrow<'msg>) -> Self {
		let ObjectRefBorrow { name, path } = borrowed.properties;
		let path = Value::from(path);
		let properties = HashMap::from([(name, path)]);

		Self {
			kind: borrowed.kind,
			detail1: borrowed.detail1,
			detail2: borrowed.detail2,
			any_data: borrowed.any_data,
			properties,
		}
	}
}
