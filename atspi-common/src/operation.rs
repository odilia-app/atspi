/// An operation can either be [`Self::Insert`] or [`Self::Delete`].
/// These correspond to methods available on [`Vec`].
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub enum Operation {
	#[default]
	#[serde(rename = "add")]
	#[serde(alias = "add/system")]
	Insert,
	#[serde(rename = "delete")]
	#[serde(alias = "delete/system")]
	Delete,
}

impl TryFrom<&str> for Operation {
	type Error = crate::AtspiError;
	fn try_from(s: &str) -> Result<Operation, Self::Error> {
		match s {
			"add" | "add/system" => Ok(Operation::Insert),
			"delete" | "delete/system" => Ok(Operation::Delete),
			_ => Err(crate::AtspiError::KindMatch(format!("\"{s}\" is not a type of Operation"))),
		}
	}
}

impl From<Operation> for String {
	fn from(op: Operation) -> String {
		match op {
			Operation::Insert => "add",
			Operation::Delete => "delete",
		}
		.to_string()
	}
}
