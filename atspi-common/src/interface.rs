//! Conversion functions and types representing a set of [`Interface`]s.
//!
//! Each `AccessibleProxy` will implement some set of these interfaces,
//! represented by a [`InterfaceSet`].

use enumflags2::{bitflags, BitFlag, BitFlags};
use serde::{
	de::{self, Deserialize, Deserializer, Visitor},
	ser::{Serialize, Serializer},
};
use std::fmt;
use zvariant::{Signature, Type};

const ACCESSIBLE_INTERFACE_NAME: &str = "org.a11y.atspi.Accessible";
const ACTION_INTERFACE_NAME: &str = "org.a11y.atspi.Action";
const APPLICATION_INTERFACE_NAME: &str = "org.a11y.atspi.Application";
const CACHE_INTERFACE_NAME: &str = "org.a11y.atspi.Cache";
const COLLECTION_INTERFACE_NAME: &str = "org.a11y.atspi.Collection";
const COMPONENT_INTERFACE_NAME: &str = "org.a11y.atspi.Component";
const DOCUMENT_INTERFACE_NAME: &str = "org.a11y.atspi.Document";
const DEVICE_EVENT_CONTROLLER_INTERFACE_NAME: &str = "org.a11y.atspi.DeviceEventController";
const DEVICE_EVENT_LISTENER_INTERFACE_NAME: &str = "org.a11y.atspi.DeviceEventListener";
const EDITABLE_TEXT_INTERFACE_NAME: &str = "org.a11y.atspi.EditableText";
const HYPERLINK_INTERFACE_NAME: &str = "org.a11y.atspi.Hyperlink";
const HYPERTEXT_INTERFACE_NAME: &str = "org.a11y.atspi.Hypertext";
const IMAGE_INTERFACE_NAME: &str = "org.a11y.atspi.Image";
const REGISTRY_INTERFACE_NAME: &str = "org.a11y.atspi.Registry";
const SELECTION_INTERFACE_NAME: &str = "org.a11y.atspi.Selection";
const SOCKET_INTERFACE_NAME: &str = "org.a11y.atspi.Socket";
const TABLE_INTERFACE_NAME: &str = "org.a11y.atspi.Table";
const TABLE_CELL_INTERFACE_NAME: &str = "org.a11y.atspi.TableCell";
const TEXT_INTERFACE_NAME: &str = "org.a11y.atspi.Text";
const VALUE_INTERFACE_NAME: &str = "org.a11y.atspi.Value";

/// AT-SPI interfaces an accessible object can implement.
#[bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
	/// Interface to indicate implementation of `AccessibleProxy`.
	Accessible,
	/// Interface to indicate implementation of `ActionProxy`.
	Action,
	/// Interface to indicate implementation of `ApplicationProxy`.
	Application,
	/// Interface to indicate implementation of `CacheProxy`.
	Cache,
	/// Interface to indicate implementation of `CollectionProxy`.
	Collection,
	/// Interface to indicate implementation of `ComponentProxy`.
	Component,
	/// Interface to indicate implementation of `DocumentProxy`.
	Document,
	/// Interface to indicate implementation of `DeviceEventControllerProxy`.
	DeviceEventController,
	/// Interface to indicate implementation of `DeviceEventListenerProxy`.
	DeviceEventListener,
	/// Interface to indicate implementation of `EditableTextProxy`.
	EditableText,
	/// Interface to indicate implementation of `HyperlinkProxy`.
	Hyperlink,
	/// Interface to indicate implementation of `HypertextProxy`.
	Hypertext,
	/// Interface to indicate implementation of `ImageProxy`.
	Image,
	/// Interface to indicate implementation of `RegistryProxy`.
	Registry,
	/// Interface to indicate implementation of `SelectionProxy`.
	Selection,
	/// Interface to indicate implementation of `SocketProxy`.
	Socket,
	/// Interface to indicate implementation of `TableProxy`.
	Table,
	/// Interface to indicate implementation of `TableCellProxy`.
	TableCell,
	/// Interface to indicate implementation of `TextProxy`.
	Text,
	/// Interface to indicate implementation of `ValueProxy`.
	Value,
}

impl<'de> Deserialize<'de> for Interface {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct InterfaceVisitor;

		impl<'de> Visitor<'de> for InterfaceVisitor {
			type Value = Interface;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("an AT-SPI interface name")
			}

			fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				match value {
					ACCESSIBLE_INTERFACE_NAME => Ok(Interface::Accessible),
					ACTION_INTERFACE_NAME => Ok(Interface::Action),
					APPLICATION_INTERFACE_NAME => Ok(Interface::Application),
					CACHE_INTERFACE_NAME => Ok(Interface::Cache),
					COLLECTION_INTERFACE_NAME => Ok(Interface::Collection),
					COMPONENT_INTERFACE_NAME => Ok(Interface::Component),
					DEVICE_EVENT_CONTROLLER_INTERFACE_NAME => Ok(Interface::DeviceEventController),
					DEVICE_EVENT_LISTENER_INTERFACE_NAME => Ok(Interface::DeviceEventListener),
					DOCUMENT_INTERFACE_NAME => Ok(Interface::Document),
					EDITABLE_TEXT_INTERFACE_NAME => Ok(Interface::EditableText),
					HYPERLINK_INTERFACE_NAME => Ok(Interface::Hyperlink),
					HYPERTEXT_INTERFACE_NAME => Ok(Interface::Hypertext),
					IMAGE_INTERFACE_NAME => Ok(Interface::Image),
					REGISTRY_INTERFACE_NAME => Ok(Interface::Registry),
					SELECTION_INTERFACE_NAME => Ok(Interface::Selection),
					SOCKET_INTERFACE_NAME => Ok(Interface::Socket),
					TABLE_INTERFACE_NAME => Ok(Interface::Table),
					TABLE_CELL_INTERFACE_NAME => Ok(Interface::TableCell),
					TEXT_INTERFACE_NAME => Ok(Interface::Text),
					VALUE_INTERFACE_NAME => Ok(Interface::Value),
					_ => Err(de::Error::custom("unknown interface")),
				}
			}
		}

		deserializer.deserialize_identifier(InterfaceVisitor)
	}
}

impl Serialize for Interface {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(match self {
			Interface::Accessible => ACCESSIBLE_INTERFACE_NAME,
			Interface::Action => ACTION_INTERFACE_NAME,
			Interface::Application => APPLICATION_INTERFACE_NAME,
			Interface::Cache => CACHE_INTERFACE_NAME,
			Interface::Collection => COLLECTION_INTERFACE_NAME,
			Interface::Component => COMPONENT_INTERFACE_NAME,
			Interface::DeviceEventController => DEVICE_EVENT_CONTROLLER_INTERFACE_NAME,
			Interface::DeviceEventListener => DEVICE_EVENT_LISTENER_INTERFACE_NAME,
			Interface::Document => DOCUMENT_INTERFACE_NAME,
			Interface::EditableText => EDITABLE_TEXT_INTERFACE_NAME,
			Interface::Hyperlink => HYPERLINK_INTERFACE_NAME,
			Interface::Hypertext => HYPERTEXT_INTERFACE_NAME,
			Interface::Image => IMAGE_INTERFACE_NAME,
			Interface::Registry => REGISTRY_INTERFACE_NAME,
			Interface::Selection => SELECTION_INTERFACE_NAME,
			Interface::Socket => SOCKET_INTERFACE_NAME,
			Interface::Table => TABLE_INTERFACE_NAME,
			Interface::TableCell => TABLE_CELL_INTERFACE_NAME,
			Interface::Text => TEXT_INTERFACE_NAME,
			Interface::Value => VALUE_INTERFACE_NAME,
		})
	}
}

/// A collection type which encodes the AT-SPI interfaces an accessible object has implemented.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InterfaceSet(BitFlags<Interface>);

impl InterfaceSet {
	pub fn new<B: Into<BitFlags<Interface>>>(value: B) -> Self {
		Self(value.into())
	}

	#[must_use]
	pub fn empty() -> InterfaceSet {
		InterfaceSet(Interface::empty())
	}

	#[must_use]
	pub fn bits(&self) -> u32 {
		self.0.bits()
	}

	#[must_use]
	pub fn all() -> InterfaceSet {
		InterfaceSet(Interface::all())
	}

	pub fn contains<B: Into<BitFlags<Interface>>>(self, other: B) -> bool {
		self.0.contains(other)
	}

	pub fn insert<B: Into<BitFlags<Interface>>>(&mut self, other: B) {
		self.0.insert(other);
	}

	pub fn iter(self) -> impl Iterator<Item = Interface> {
		self.0.iter()
	}
}

impl<'de> Deserialize<'de> for InterfaceSet {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct InterfaceSetVisitor;

		impl<'de> Visitor<'de> for InterfaceSetVisitor {
			type Value = InterfaceSet;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a sequence comprised of valid AT-SPI interface names")
			}

			fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
			where
				D: Deserializer<'de>,
			{
				match <Vec<Interface> as Deserialize>::deserialize(deserializer) {
					Ok(interfaces) => Ok(InterfaceSet(BitFlags::from_iter(interfaces))),
					Err(e) => Err(e),
				}
			}
		}

		deserializer.deserialize_newtype_struct("InterfaceSet", InterfaceSetVisitor)
	}
}

impl Serialize for InterfaceSet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer
			.serialize_newtype_struct("InterfaceSet", &self.0.iter().collect::<Vec<Interface>>())
	}
}

impl Type for InterfaceSet {
	fn signature() -> Signature<'static> {
		<Vec<String> as Type>::signature()
	}
}

impl From<Interface> for InterfaceSet {
	fn from(value: Interface) -> Self {
		Self(value.into())
	}
}

impl std::ops::BitAnd for InterfaceSet {
	type Output = InterfaceSet;

	fn bitand(self, other: Self) -> Self::Output {
		InterfaceSet(self.0 & other.0)
	}
}

impl std::ops::BitXor for InterfaceSet {
	type Output = InterfaceSet;

	fn bitxor(self, other: Self) -> Self::Output {
		InterfaceSet(self.0 ^ other.0)
	}
}

impl std::ops::BitOr for InterfaceSet {
	type Output = InterfaceSet;

	fn bitor(self, other: Self) -> Self::Output {
		InterfaceSet(self.0 | other.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use byteorder::LE;
	use zbus::zvariant::{from_slice, to_bytes, EncodingContext as Context};

	#[test]
	fn serialize_empty_interface_set() {
		let ctxt = Context::<LE>::new_dbus(0);
		let encoded = to_bytes(ctxt, &InterfaceSet::empty()).unwrap();
		assert_eq!(encoded, &[0, 0, 0, 0]);
	}

	#[test]
	fn deserialize_empty_interface_set() {
		let ctxt = Context::<LE>::new_dbus(0);
		let decoded: InterfaceSet = from_slice(&[0, 0, 0, 0], ctxt).unwrap();
		assert_eq!(decoded, InterfaceSet::empty());
	}

	#[test]
	fn serialize_interface_set_accessible() {
		let ctxt = Context::<LE>::new_dbus(0);
		let encoded = to_bytes(ctxt, &InterfaceSet::new(Interface::Accessible)).unwrap();
		assert_eq!(
			encoded,
			&[
				30, 0, 0, 0, 25, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115,
				112, 105, 46, 65, 99, 99, 101, 115, 115, 105, 98, 108, 101, 0
			]
		);
	}

	#[test]
	fn deserialize_interface_set_accessible() {
		let ctxt = Context::<LE>::new_dbus(0);
		let decoded: InterfaceSet = from_slice(
			&[
				30, 0, 0, 0, 25, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115,
				112, 105, 46, 65, 99, 99, 101, 115, 115, 105, 98, 108, 101, 0,
			],
			ctxt,
		)
		.unwrap();
		assert_eq!(decoded, InterfaceSet::new(Interface::Accessible));
	}

	#[test]
	fn can_handle_multiple_interfaces() {
		let ctxt = Context::<LE>::new_dbus(0);
		let object =
			InterfaceSet::new(Interface::Accessible | Interface::Action | Interface::Component);
		let encoded = to_bytes(ctxt, &object).unwrap();
		let decoded: InterfaceSet = from_slice(&encoded, ctxt).unwrap();
		assert!(object == decoded);
	}
	#[test]
	fn match_various_de_serialization_methods() {
		for iface in InterfaceSet::all().iter() {
			let displayed = format!("{iface}");
			let serde_val = serde_plain::to_string(&iface).expect("Unable to serialize {iface}");
			// this is not *necessary* if Display wants to be implemented for some other reason.
			// as of when this test is written, it should be the same.
			// but if you've made a concious decision as a developer that there is a better use for Display, go ahead and remove this
			assert_eq!(
				displayed, serde_val,
				"Serde's serialization does not match the Display trait implementation."
			);
			let from_str = Interface::try_from(&*displayed).unwrap();
			assert_eq!(iface, from_str, "The display trait for {iface} became \"{displayed}\", but was re-serialized as {from_str} via TryFrom<&str>");
			let serde_from_str: Interface = serde_plain::from_str(&serde_val).unwrap();
			assert_eq!(serde_from_str, iface, "Serde's deserialization does not match its serialization. {iface} was serialized to \"{serde_val}\", but deserialized into {serde_from_str}");
		}
	}
}
impl TryFrom<&str> for Interface {
	type Error = &'static str;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		match s {
			ACCESSIBLE_INTERFACE_NAME => Ok(Interface::Accessible),
			ACTION_INTERFACE_NAME => Ok(Interface::Action),
			APPLICATION_INTERFACE_NAME => Ok(Interface::Application),
			COLLECTION_INTERFACE_NAME => Ok(Interface::Collection),
			COMPONENT_INTERFACE_NAME => Ok(Interface::Component),
			DOCUMENT_INTERFACE_NAME => Ok(Interface::Document),
			HYPERTEXT_INTERFACE_NAME => Ok(Interface::Hypertext),
			HYPERLINK_INTERFACE_NAME => Ok(Interface::Hyperlink),
			IMAGE_INTERFACE_NAME => Ok(Interface::Image),
			SELECTION_INTERFACE_NAME => Ok(Interface::Selection),
			SOCKET_INTERFACE_NAME => Ok(Interface::Socket),
			TABLE_INTERFACE_NAME => Ok(Interface::Table),
			TABLE_CELL_INTERFACE_NAME => Ok(Interface::TableCell),
			TEXT_INTERFACE_NAME => Ok(Interface::Text),
			EDITABLE_TEXT_INTERFACE_NAME => Ok(Interface::EditableText),
			CACHE_INTERFACE_NAME => Ok(Interface::Cache),
			VALUE_INTERFACE_NAME => Ok(Interface::Value),
			REGISTRY_INTERFACE_NAME => Ok(Interface::Registry),
			DEVICE_EVENT_CONTROLLER_INTERFACE_NAME => Ok(Interface::DeviceEventController),
			DEVICE_EVENT_LISTENER_INTERFACE_NAME => Ok(Interface::DeviceEventListener),
			_ => Err("No interface found for conversion."),
		}
	}
}
impl std::fmt::Display for Interface {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let interface_string = match self {
			Interface::Accessible => ACCESSIBLE_INTERFACE_NAME,
			Interface::Action => ACTION_INTERFACE_NAME,
			Interface::Application => APPLICATION_INTERFACE_NAME,
			Interface::Cache => CACHE_INTERFACE_NAME,
			Interface::Collection => COLLECTION_INTERFACE_NAME,
			Interface::Component => COMPONENT_INTERFACE_NAME,
			Interface::DeviceEventController => DEVICE_EVENT_CONTROLLER_INTERFACE_NAME,
			Interface::DeviceEventListener => DEVICE_EVENT_LISTENER_INTERFACE_NAME,
			Interface::Document => DOCUMENT_INTERFACE_NAME,
			Interface::EditableText => EDITABLE_TEXT_INTERFACE_NAME,
			Interface::Hypertext => HYPERTEXT_INTERFACE_NAME,
			Interface::Hyperlink => HYPERLINK_INTERFACE_NAME,
			Interface::Image => IMAGE_INTERFACE_NAME,
			Interface::Registry => REGISTRY_INTERFACE_NAME,
			Interface::Socket => SOCKET_INTERFACE_NAME,
			Interface::Selection => SELECTION_INTERFACE_NAME,
			Interface::Table => TABLE_INTERFACE_NAME,
			Interface::TableCell => TABLE_CELL_INTERFACE_NAME,
			Interface::Text => TEXT_INTERFACE_NAME,
			Interface::Value => VALUE_INTERFACE_NAME,
		}
		.to_string();
		write!(f, "{interface_string}")
	}
}
