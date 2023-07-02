use enumflags2::{bitflags, BitFlag, BitFlags};
use serde::{
	de::{self, Deserialize, Deserializer, Visitor},
	ser::{Serialize, Serializer},
};
use std::fmt;
use zvariant::{Signature, Type};

/// AT-SPI interfaces an accessible object can implement.
#[bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
	Accessible,
	Action,
	Application,
	Cache,
	Collection,
	Component,
	Document,
	DeviceEventController,
	DeviceEventListener,
	EditableText,
	Hyperlink,
	Hypertext,
	Image,
	Registry,
	Selection,
	Socket,
	Table,
	TableCell,
	Text,
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
					"org.a11y.atspi.Accessible" => Ok(Interface::Accessible),
					"org.a11y.atspi.Action" => Ok(Interface::Action),
					"org.a11y.atspi.Application" => Ok(Interface::Application),
					"org.a11y.atspi.Cache" => Ok(Interface::Cache),
					"org.a11y.atspi.Collection" => Ok(Interface::Collection),
					"org.a11y.atspi.Component" => Ok(Interface::Component),
					"org.a11y.atspi.DeviceEventController" => Ok(Interface::DeviceEventController),
					"org.a11y.atspi.DeviceEventListener" => Ok(Interface::DeviceEventListener),
					"org.a11y.atspi.Document" => Ok(Interface::Document),
					"org.a11y.atspi.EditableText" => Ok(Interface::EditableText),
					"org.a11y.atspi.Hyperlink" => Ok(Interface::Hyperlink),
					"org.a11y.atspi.Hypertext" => Ok(Interface::Hypertext),
					"org.a11y.atspi.Image" => Ok(Interface::Image),
					"org.a11y.atspi.Registry" => Ok(Interface::Registry),
					"org.a11y.atspi.Selection" => Ok(Interface::Selection),
					"org.a11y.atspi.Socket" => Ok(Interface::Socket),
					"org.a11y.atspi.Table" => Ok(Interface::Table),
					"org.a11y.atspi.TableCell" => Ok(Interface::TableCell),
					"org.a11y.atspi.Text" => Ok(Interface::Text),
					"org.a11y.atspi.Value" => Ok(Interface::Value),
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
			Interface::Accessible => "org.a11y.atspi.Accessible",
			Interface::Action => "org.a11y.atspi.Action",
			Interface::Application => "org.a11y.atspi.Application",
			Interface::Cache => "org.a11y.atspi.Cache",
			Interface::Collection => "org.a11y.atspi.Collection",
			Interface::Component => "org.a11y.atspi.Component",
			Interface::DeviceEventController => "org.a11y.atspi.DeviceEventController",
			Interface::DeviceEventListener => "org.a11y.atspi.DeviceEventListener",
			Interface::Document => "org.a11y.atspi.Document",
			Interface::EditableText => "org.a11y.atspi.EditableText",
			Interface::Hyperlink => "org.a11y.atspi.Hyperlink",
			Interface::Hypertext => "org.a11y.atspi.Hypertext",
			Interface::Image => "org.a11y.atspi.Image",
			Interface::Registry => "org.a11y.atspi.Registry",
			Interface::Selection => "org.a11y.atspi.Selection",
			Interface::Socket => "org.a11y.atspi.Socket",
			Interface::Table => "org.a11y.atspi.Table",
			Interface::TableCell => "org.a11y.atspi.TableCell",
			Interface::Text => "org.a11y.atspi.Text",
			Interface::Value => "org.a11y.atspi.Value",
		})
	}
}

/// A collection type which encodes the AT-SPI interfaces an accellible object has implemented.
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
			let displayed = format!("{}", iface);
			let serde_val = serde_plain::to_string(&iface).expect("Unable to serialize {iface}");
			// this is not *necessary* if Display wants to be implemented for some other reason.
			// as of when this test is written, it should be the same.
			// but if you've made a concious decision as a developer that there is a better use for Display, go ahead and remove this
			assert_eq!(displayed, serde_val, "Serde's serialization does not match the Display trait implementation.");
			let from_str = Interface::try_from(&*displayed).unwrap();
			assert_eq!(iface, from_str, "The display trait for {} became \"{}\", but was re-serialized as {} via TryFrom<&str>", iface, displayed, from_str);
			let serde_from_str: Interface = serde_plain::from_str(&serde_val).unwrap();
			assert_eq!(serde_from_str, iface, "Serde's deserialization does not match its serialization. {} was serialized to \"{}\", but deserialized into {}", iface, serde_val, serde_from_str);
		}
	}
}
impl TryFrom<&str> for Interface {
	type Error = &'static str;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		match s {
			"org.a11y.atspi.Accessible" => Ok(Interface::Accessible),
			"org.a11y.atspi.Action" => Ok(Interface::Action),
			"org.a11y.atspi.Application" => Ok(Interface::Application),
			"org.a11y.atspi.Collection" => Ok(Interface::Collection),
			"org.a11y.atspi.Component" => Ok(Interface::Component),
			"org.a11y.atspi.Document" => Ok(Interface::Document),
			"org.a11y.atspi.Hypertext" => Ok(Interface::Hypertext),
			"org.a11y.atspi.Hyperlink" => Ok(Interface::Hyperlink),
			"org.a11y.atspi.Image" => Ok(Interface::Image),
			"org.a11y.atspi.Selection" => Ok(Interface::Selection),
			"org.a11y.atspi.Socket" => Ok(Interface::Socket),
			"org.a11y.atspi.Table" => Ok(Interface::Table),
			"org.a11y.atspi.TableCell" => Ok(Interface::TableCell),
			"org.a11y.atspi.Text" => Ok(Interface::Text),
			"org.a11y.atspi.EditableText" => Ok(Interface::EditableText),
			"org.a11y.atspi.Cache" => Ok(Interface::Cache),
			"org.a11y.atspi.Value" => Ok(Interface::Value),
			"org.a11y.atspi.Registry" => Ok(Interface::Registry),
			"org.a11y.atspi.DeviceEventController" => Ok(Interface::DeviceEventController),
			"org.a11y.atspi.DeviceEventListener" => Ok(Interface::DeviceEventListener),
			_ => Err("No interface found for conversion."),
		}
	}
}
impl std::fmt::Display for Interface {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let interface_string = match self {
			Interface::Accessible => "org.a11y.atspi.Accessible",
			Interface::Action => "org.a11y.atspi.Action",
			Interface::Application => "org.a11y.atspi.Application",
			Interface::Collection => "org.a11y.atspi.Collection",
			Interface::Component => "org.a11y.atspi.Component",
			Interface::Document => "org.a11y.atspi.Document",
			Interface::Hypertext => "org.a11y.atspi.Hypertext",
			Interface::Hyperlink => "org.a11y.atspi.Hyperlink",
			Interface::Image => "org.a11y.atspi.Image",
			Interface::Socket => "org.a11y.atspi.Socket",
			Interface::Selection => "org.a11y.atspi.Selection",
			Interface::Table => "org.a11y.atspi.Table",
			Interface::TableCell => "org.a11y.atspi.TableCell",
			Interface::Text => "org.a11y.atspi.Text",
			Interface::EditableText => "org.a11y.atspi.EditableText",
			Interface::Cache => "org.a11y.atspi.Cache",
			Interface::Value => "org.a11y.atspi.Value",
			Interface::Registry => "org.a11y.atspi.Registry",
			Interface::DeviceEventController => "org.a11y.atspi.DeviceEventController",
			Interface::DeviceEventListener => "org.a11y.atspi.DeviceEventListener",
		}
		.to_string();
		write!(f, "{interface_string}")
	}
}
