/// An operation can either be [`Self::Insert`] or [`Self::Delete`].
/// These correspond to methods available on [`Vec`].
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub enum Operation {
	#[default]
	#[serde(rename = "add")]
	#[serde(alias = "add/system")]
	#[serde(alias = "insert")]
	#[serde(alias = "insert/system")]
	Insert,
	#[serde(rename = "delete")]
	#[serde(alias = "delete/system")]
	#[serde(alias = "remove")]
	#[serde(alias = "remove/system")]
	Delete,
}

impl TryFrom<&str> for Operation {
	type Error = crate::AtspiError;
	fn try_from(s: &str) -> Result<Operation, Self::Error> {
		match s {
			"add" | "add/system" | "insert" | "insert/system" => Ok(Operation::Insert),
			"delete" | "delete/system" | "remove" | "remove/system" => Ok(Operation::Delete),
			_ => Err(crate::AtspiError::KindMatch(format!("{s} is not a type of Operation"))),
		}
	}
}

impl From<Operation> for String {
	fn from(op: Operation) -> String {
		match op {
			Operation::Insert => "insert",
			Operation::Delete => "remove",
		}
		.to_string()
	}
}
