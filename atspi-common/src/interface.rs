//! Conversion functions and types representing a set of [`Interface`]s.
//!
//! Each `AccessibleProxy` will implement some set of these interfaces,
//! represented by a [`InterfaceSet`].

use enumflags2::{bitflags, BitFlag, BitFlags};
use serde::{
	de::{self, Deserializer, Visitor},
	ser::{self, Serializer},
	Deserialize, Serialize,
};
use std::{fmt, str::FromStr};
use strum::{Display, IntoStaticStr};
use zvariant::{Signature, Type};

use crate::AtspiError;

/// AT-SPI interfaces an accessible object can implement.
#[bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Display, IntoStaticStr, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interface {
	/// Interface to indicate implementation of `AccessibleProxy`.  
	#[serde(rename = "org.a11y.atspi.Accessible")]
	#[strum(serialize = "org.a11y.atspi.Accessible")]
	Accessible,

	/// Interface to indicate implementation of `ActionProxy`.
	#[serde(rename = "org.a11y.atspi.Action")]
	#[strum(serialize = "org.a11y.atspi.Action")]
	Action,

	/// Interface to indicate implementation of `ApplicationProxy`.
	#[serde(rename = "org.a11y.atspi.Application")]
	#[strum(serialize = "org.a11y.atspi.Application")]
	Application,

	/// Interface to indicate implementation of `CacheProxy`.
	#[serde(rename = "org.a11y.atspi.Cache")]
	#[strum(serialize = "org.a11y.atspi.Cache")]
	Cache,

	/// Interface to indicate implementation of `CollectionProxy`.
	#[serde(rename = "org.a11y.atspi.Collection")]
	#[strum(serialize = "org.a11y.atspi.Collection")]
	Collection,

	/// Interface to indicate implementation of `ComponentProxy`.
	#[serde(rename = "org.a11y.atspi.Component")]
	#[strum(serialize = "org.a11y.atspi.Component")]
	Component,

	/// Interface to indicate implementation of `DocumentProxy`.
	#[serde(rename = "org.a11y.atspi.Document")]
	#[strum(serialize = "org.a11y.atspi.Document")]
	Document,

	/// Interface to indicate implementation of `DeviceEventControllerProxy`.
	#[serde(rename = "org.a11y.atspi.DeviceEventController")]
	#[strum(serialize = "org.a11y.atspi.DeviceEventController")]
	DeviceEventController,

	/// Interface to indicate implementation of `DeviceEventListenerProxy`.
	#[serde(rename = "org.a11y.atspi.DeviceEventListener")]
	#[strum(serialize = "org.a11y.atspi.DeviceEventListener")]
	DeviceEventListener,

	/// Interface to indicate implementation of `EditableTextProxy`.
	#[serde(rename = "org.a11y.atspi.EditableText")]
	#[strum(serialize = "org.a11y.atspi.EditableText")]
	EditableText,

	/// Interface to indicate implementation of `HyperlinkProxy`.
	#[serde(rename = "org.a11y.atspi.Hyperlink")]
	#[strum(serialize = "org.a11y.atspi.Hyperlink")]
	Hyperlink,

	/// Interface to indicate implementation of `HypertextProxy`.
	#[serde(rename = "org.a11y.atspi.Hypertext")]
	#[strum(serialize = "org.a11y.atspi.Hypertext")]
	Hypertext,

	/// Interface to indicate implementation of `ImageProxy`.
	#[serde(rename = "org.a11y.atspi.Image")]
	#[strum(serialize = "org.a11y.atspi.Image")]
	Image,

	/// Interface to indicate implementation of `RegistryProxy`.
	#[serde(rename = "org.a11y.atspi.Registry")]
	#[strum(serialize = "org.a11y.atspi.Registry")]
	Registry,

	/// Interface to indicate implementation of `SelectionProxy`.
	#[serde(rename = "org.a11y.atspi.Selection")]
	#[strum(serialize = "org.a11y.atspi.Selection")]
	Selection,

	/// Interface to indicate implementation of `SocketProxy`.
	#[serde(rename = "org.a11y.atspi.Socket")]
	#[strum(serialize = "org.a11y.atspi.Socket")]
	Socket,

	/// Interface to indicate implementation of `TableProxy`.
	#[serde(rename = "org.a11y.atspi.Table")]
	#[strum(serialize = "org.a11y.atspi.Table")]
	Table,

	/// Interface to indicate implementation of `TableCellProxy`.
	#[serde(rename = "org.a11y.atspi.TableCell")]
	#[strum(serialize = "org.a11y.atspi.TableCell")]
	TableCell,

	/// Interface to indicate implementation of `TextProxy`.
	#[serde(rename = "org.a11y.atspi.Text")]
	#[strum(serialize = "org.a11y.atspi.Text")]
	Text,

	/// Interface to indicate implementation of `ValueProxy`.
	#[serde(rename = "org.a11y.atspi.Value")]
	#[strum(serialize = "org.a11y.atspi.Value")]
	Value,
}

impl Type for Interface {
	fn signature() -> Signature<'static> {
		<String as Type>::signature()
	}
}

impl FromStr for Interface {
	type Err = AtspiError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let prefix = "org.a11y.atspi.";
		if s.starts_with(prefix) {
			match &s[prefix.len()..] {
				"Accessible" => Ok(Interface::Accessible),
				"Action" => Ok(Interface::Action),
				"Application" => Ok(Interface::Application),
				"Cache" => Ok(Interface::Cache),
				"Collection" => Ok(Interface::Collection),
				"Component" => Ok(Interface::Component),
				"Document" => Ok(Interface::Document),
				"DeviceEventController" => Ok(Interface::DeviceEventController),
				"DeviceEventListener" => Ok(Interface::DeviceEventListener),
				"EditableText" => Ok(Interface::EditableText),
				"Hyperlink" => Ok(Interface::Hyperlink),
				"Hypertext" => Ok(Interface::Hypertext),
				"Image" => Ok(Interface::Image),
				"Registry" => Ok(Interface::Registry),
				"Selection" => Ok(Interface::Selection),
				"Socket" => Ok(Interface::Socket),
				"Table" => Ok(Interface::Table),
				"TableCell" => Ok(Interface::TableCell),
				"Text" => Ok(Interface::Text),
				"Value" => Ok(Interface::Value),
				_ => Err(AtspiError::InterfaceMatch(format!(
					"No interface found for conversion: {s}"
				))),
			}
		} else {
			Err(AtspiError::InterfaceMatch(format!("No interface found for conversion: {s}")))
		}
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

impl<'de> de::Deserialize<'de> for InterfaceSet {
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

impl ser::Serialize for InterfaceSet {
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

impl std::fmt::Display for InterfaceSet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut iter = self.0.iter();
		if let Some(first) = iter.next() {
			write!(f, "{first}")?;
			for iface in iter {
				write!(f, ", {iface}")?;
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::{Interface, InterfaceSet};
	use std::str::FromStr;
	use zvariant::{
		serialized::{Context, Data},
		to_bytes, Type, LE,
	};

	#[test]
	fn interface_into_static_str_impl() {
		let iface: Interface = Interface::Accessible;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Accessible");

		let iface: Interface = Interface::Action;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Action");

		let iface: Interface = Interface::Application;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Application");

		let iface: Interface = Interface::Cache;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Cache");

		let iface: Interface = Interface::Collection;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Collection");

		let iface: Interface = Interface::Component;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Component");

		let iface: Interface = Interface::Document;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Document");

		let iface: Interface = Interface::DeviceEventController;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.DeviceEventController");

		let iface: Interface = Interface::DeviceEventListener;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.DeviceEventListener");

		let iface: Interface = Interface::EditableText;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.EditableText");

		let iface: Interface = Interface::Hyperlink;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Hyperlink");

		let iface: Interface = Interface::Hypertext;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Hypertext");

		let iface: Interface = Interface::Image;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Image");

		let iface: Interface = Interface::Registry;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Registry");

		let iface: Interface = Interface::Selection;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Selection");

		let iface: Interface = Interface::Socket;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Socket");

		let iface: Interface = Interface::Table;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Table");

		let iface: Interface = Interface::TableCell;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.TableCell");

		let iface: Interface = Interface::Text;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Text");

		let iface = Interface::Value;
		let iface_str: &'static str = iface.into();
		assert_eq!(iface_str, "org.a11y.atspi.Value");
	}

	#[test]
	fn interface_from_str_impl() {
		let iface = Interface::from_str("org.a11y.atspi.Accessible").unwrap();
		assert_eq!(iface, Interface::Accessible);

		let iface = Interface::from_str("org.a11y.atspi.Action").unwrap();
		assert_eq!(iface, Interface::Action);

		let iface = Interface::from_str("org.a11y.atspi.Application").unwrap();
		assert_eq!(iface, Interface::Application);

		let iface = Interface::from_str("org.a11y.atspi.Cache").unwrap();
		assert_eq!(iface, Interface::Cache);

		let iface = Interface::from_str("org.a11y.atspi.Collection").unwrap();
		assert_eq!(iface, Interface::Collection);

		let iface = Interface::from_str("org.a11y.atspi.Component").unwrap();
		assert_eq!(iface, Interface::Component);

		let iface = Interface::from_str("org.a11y.atspi.Document").unwrap();
		assert_eq!(iface, Interface::Document);

		let iface = Interface::from_str("org.a11y.atspi.DeviceEventController").unwrap();
		assert_eq!(iface, Interface::DeviceEventController);

		let iface = Interface::from_str("org.a11y.atspi.DeviceEventListener").unwrap();
		assert_eq!(iface, Interface::DeviceEventListener);

		let iface = Interface::from_str("org.a11y.atspi.EditableText").unwrap();
		assert_eq!(iface, Interface::EditableText);

		let iface = Interface::from_str("org.a11y.atspi.Hyperlink").unwrap();
		assert_eq!(iface, Interface::Hyperlink);

		let iface = Interface::from_str("org.a11y.atspi.Hypertext").unwrap();
		assert_eq!(iface, Interface::Hypertext);

		let iface = Interface::from_str("org.a11y.atspi.Image").unwrap();
		assert_eq!(iface, Interface::Image);

		let iface = Interface::from_str("org.a11y.atspi.Registry").unwrap();
		assert_eq!(iface, Interface::Registry);

		let iface = Interface::from_str("org.a11y.atspi.Selection").unwrap();
		assert_eq!(iface, Interface::Selection);

		let iface = Interface::from_str("org.a11y.atspi.Socket").unwrap();
		assert_eq!(iface, Interface::Socket);

		let iface = Interface::from_str("org.a11y.atspi.Table").unwrap();
		assert_eq!(iface, Interface::Table);

		let iface = Interface::from_str("org.a11y.atspi.TableCell").unwrap();
		assert_eq!(iface, Interface::TableCell);

		let iface = Interface::from_str("org.a11y.atspi.Text").unwrap();
		assert_eq!(iface, Interface::Text);

		let iface = Interface::from_str("org.a11y.atspi.Value").unwrap();
		assert_eq!(iface, Interface::Value);
	}

	#[test]
	fn interface_from_str_impl_no_match() {
		let res = Interface::from_str("org.a11y.atspi.Foo");
		assert!(res.is_err());

		let res = Interface::from_str("com.a11y.atspi.Accessible");
		assert!(res.is_err());
	}

	#[test]
	fn interface_display_impl() {
		assert_eq!(format!("{}", Interface::Accessible), "org.a11y.atspi.Accessible".to_owned());
		assert_eq!(format!("{}", Interface::Action), "org.a11y.atspi.Action".to_owned());
		assert_eq!(format!("{}", Interface::Application), "org.a11y.atspi.Application".to_owned());
		assert_eq!(format!("{}", Interface::Cache), "org.a11y.atspi.Cache".to_owned());
		assert_eq!(format!("{}", Interface::Collection), "org.a11y.atspi.Collection".to_owned());
		assert_eq!(format!("{}", Interface::Component), "org.a11y.atspi.Component".to_owned());
		assert_eq!(format!("{}", Interface::Document), "org.a11y.atspi.Document".to_owned());
		assert_eq!(
			format!("{}", Interface::DeviceEventController),
			"org.a11y.atspi.DeviceEventController".to_owned()
		);
		assert_eq!(
			format!("{}", Interface::DeviceEventListener),
			"org.a11y.atspi.DeviceEventListener".to_owned()
		);
		assert_eq!(
			format!("{}", Interface::EditableText),
			"org.a11y.atspi.EditableText".to_owned()
		);
		assert_eq!(format!("{}", Interface::Hyperlink), "org.a11y.atspi.Hyperlink".to_owned());
		assert_eq!(format!("{}", Interface::Hypertext), "org.a11y.atspi.Hypertext".to_owned());
		assert_eq!(format!("{}", Interface::Image), "org.a11y.atspi.Image".to_owned());
		assert_eq!(format!("{}", Interface::Registry), "org.a11y.atspi.Registry".to_owned());
		assert_eq!(format!("{}", Interface::Selection), "org.a11y.atspi.Selection".to_owned());
		assert_eq!(format!("{}", Interface::Socket), "org.a11y.atspi.Socket".to_owned());
		assert_eq!(format!("{}", Interface::Table), "org.a11y.atspi.Table".to_owned());
		assert_eq!(format!("{}", Interface::TableCell), "org.a11y.atspi.TableCell".to_owned());
		assert_eq!(format!("{}", Interface::Text), "org.a11y.atspi.Text".to_owned());
		assert_eq!(format!("{}", Interface::Value), "org.a11y.atspi.Value".to_owned());
	}

	#[test]
	fn interface_set_display_impl() {
		let ifaceset = InterfaceSet::new(Interface::Accessible);
		assert_eq!(format!("{}", ifaceset), "org.a11y.atspi.Accessible");

		let ifaceset = InterfaceSet::new(Interface::Accessible | Interface::Action);
		assert_eq!(format!("{}", ifaceset), "org.a11y.atspi.Accessible, org.a11y.atspi.Action");

		let ifaceset =
			InterfaceSet::new(Interface::Accessible | Interface::Action | Interface::Component);
		assert_eq!(
			format!("{}", ifaceset),
			"org.a11y.atspi.Accessible, org.a11y.atspi.Action, org.a11y.atspi.Component"
		);
	}

	#[test]
	fn interface_type_signature() {
		assert_eq!(Interface::signature().as_str(), "s");
	}

	#[test]
	fn interface_set_type_signature() {
		assert_eq!(InterfaceSet::signature().as_str(), "as");
	}

	#[test]
	fn serialize_and_deserialize_accessible_interface() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &Interface::Accessible).unwrap();
		assert_eq!(
			encoded.bytes(),
			&[
				25, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115, 112, 105, 46,
				65, 99, 99, 101, 115, 115, 105, 98, 108, 101, 0
			]
		);

		let (decoded, _) = encoded.deserialize::<Interface>().unwrap();
		assert_eq!(decoded, Interface::Accessible);
	}

	#[test]
	fn serialize_and_deserialize_editable_text_interface() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &Interface::EditableText).unwrap();
		assert_eq!(
			encoded.bytes(),
			&[
				27, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115, 112, 105, 46,
				69, 100, 105, 116, 97, 98, 108, 101, 84, 101, 120, 116, 0
			]
		);

		let (decoded, _) = encoded.deserialize::<Interface>().unwrap();
		assert_eq!(decoded, Interface::EditableText);
	}

	#[test]
	fn serialize_and_deserialize_hyperlink_interface() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &Interface::Hyperlink).unwrap();
		assert_eq!(
			encoded.bytes(),
			&[
				24, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115, 112, 105, 46,
				72, 121, 112, 101, 114, 108, 105, 110, 107, 0
			]
		);

		let (decoded, _) = encoded.deserialize::<Interface>().unwrap();
		assert_eq!(decoded, Interface::Hyperlink);
	}

	#[test]
	fn serialize_and_deserialize_value_interface() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &Interface::Value).unwrap();
		assert_eq!(
			encoded.bytes(),
			&[
				20, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115, 112, 105, 46,
				86, 97, 108, 117, 101, 0
			]
		);

		let (decoded, _) = encoded.deserialize::<Interface>().unwrap();
		assert_eq!(decoded, Interface::Value);
	}

	#[test]
	fn serialize_empty_interface_set() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &InterfaceSet::empty()).unwrap();
		assert_eq!(encoded.bytes(), &[0, 0, 0, 0]);
	}

	#[test]
	fn deserialize_empty_interface_set() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &InterfaceSet::empty()).unwrap();
		let (decoded, _) = encoded.deserialize::<InterfaceSet>().unwrap();
		assert_eq!(decoded, InterfaceSet::empty());
	}

	#[test]
	fn serialize_interface_set_accessible() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &InterfaceSet::new(Interface::Accessible)).unwrap();
		assert_eq!(
			encoded.bytes(),
			&[
				30, 0, 0, 0, 25, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115,
				112, 105, 46, 65, 99, 99, 101, 115, 115, 105, 98, 108, 101, 0
			]
		);
	}

	#[test]
	fn deserialize_interface_set_accessible() {
		let ctxt = Context::new_dbus(LE, 0);
		let data = Data::new::<&[u8]>(
			&[
				30, 0, 0, 0, 25, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115,
				112, 105, 46, 65, 99, 99, 101, 115, 115, 105, 98, 108, 101, 0,
			],
			ctxt,
		);

		let (ifaceset, _) = data.deserialize::<InterfaceSet>().unwrap();
		assert_eq!(ifaceset, InterfaceSet::new(Interface::Accessible));
	}

	#[test]
	fn can_handle_multiple_interfaces() {
		let ctxt = Context::new_dbus(LE, 0);
		let object =
			InterfaceSet::new(Interface::Accessible | Interface::Action | Interface::Component);
		let encoded = to_bytes(ctxt, &object).unwrap();
		let (decoded, _) = encoded.deserialize::<InterfaceSet>().unwrap();
		assert!(object == decoded);
	}

	#[test]
	fn match_various_de_serialization_methods() {
		for iface in InterfaceSet::all().iter() {
			let displayed = format!("{iface}");
			let serde_val = serde_plain::to_string(&iface)
				.unwrap_or_else(|_| panic!("Unable to serialize {iface}"));

			// Check that the Display trait and Serde's serialization match.
			assert_eq!(
				displayed, serde_val,
				"Serde's serialization does not match the Display trait implementation."
			);

			// Check that the Display trait and TryFrom<&str> match.
			let from_str = Interface::from_str(&displayed).unwrap();
			assert_eq!(iface, from_str, "The display trait for {iface} became \"{displayed}\", but was re-serialized as {from_str} via TryFrom<&str>");
			let serde_from_str: Interface = serde_plain::from_str(&serde_val).unwrap();
			assert_eq!(serde_from_str, iface, "Serde's deserialization does not match its serialization. {iface} was serialized to \"{serde_val}\", but deserialized into {serde_from_str}");
		}
	}
}
