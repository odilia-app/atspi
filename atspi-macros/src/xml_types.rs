use crate::serde_signature;
use zvariant::Signature;
use serde::{
	Serialize, Deserialize, self,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum Access {
	Read,
	Write,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub struct Property {
	name: String,
	#[serde(rename="type", with="serde_signature")]
	dbus_type: Signature<'static>,
	/// the level of access a client has to this property, it may be read or write
	access: Access,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum Direction {
	In,
	Out,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub struct SignalArg {
	name: Option<String>,
	#[serde(rename="type", with="serde_signature")]
	dbus_type: Signature<'static>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub struct MethodArg {
	direction: Direction,
	name: Option<String>,
	#[serde(rename="type", with="serde_signature")]
	dbus_type: Signature<'static>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub struct Annotation {
	name: String,
	value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum MethodItem {
	Arg(MethodArg),
	Annotation(Annotation),
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum SignalItem {
	Arg(SignalArg),
	Annotation(Annotation),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub struct Method {
	name: String,
	#[serde(rename = "$value")]
	items: Vec<MethodItem>
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub struct Signal {
	name: String,
	#[serde(rename = "$value")]
	items: Vec<SignalItem>
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum Item {
	Property(Property),
	Method(Method),
	Signal(Signal),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub struct Interface {
	name: String,
	#[serde(rename = "$value")]
	properties: Vec<Item>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub struct Node {
	#[serde(rename = "$value")]
	interfaces: Vec<Interface>
}
