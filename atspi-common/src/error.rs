#[allow(clippy::module_name_repetitions)]
#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
/// An error type that can describe atspi and `std` and different `zbus` errors.
pub enum AtspiError {
	/// When testing on either variant, we might find the we are not interested in.
	CacheVariantMismatch,

	/// On specific types, if the event / message member does not match the Event's name.
	MemberMatch(String),

	/// On specific types, if the event / message member does not match the Event's name.
	InterfaceMatch(String),

	/// To indicate a match or equality test on a signa body signature failed.
	UnknownBusSignature(String),

	/// When matching on an unknown interface
	UnknownInterface(String),

	/// No interface on event.
	MissingInterface,

	/// No member on event.
	MissingMember,

	/// No name on bus.
	MissingName,

	/// No path on bus.
	MissingPath,

	/// The signal that was encountered is unknown.
	UnknownSignal(String),

	/// Other errors.
	Owned(String),

	/// A `zbus` or `zbus::Fdo` error. variant.
	Zbus(String),

	/// Failed to parse a string into an enum variant
	ParseError(&'static str),

	/// Failed to convert an integer into another type of integer (usually i32 -> usize).
	IntConversionError,

	/// An infallible error; this is just something to satisfy the compiler.
	Infallible,

	/// An error for when you attempt to extract the incorrect variant of a type.
	/// For example: [`crate::events::Event`] has many variants and sub-variants that can be extracted.
	/// If you attempt to convert to a variant which is not the variant contained within the `Event` enum, then you will get this error.
	InvalidType,
	
	/// An error type to hold innumerable errors. This includes implementation detail errors like internal zbus errors or serde serialization problems (again internal to zbus).
	/// Any generic error here *can* be moved into its own variant if requested.
	/// These errors in general should be fairly rare; if you get this error, something very bad has happened, and you may want to consider [filing a bug](https://github.com/odilia-app/atspi/issues/) with a way to reproduce the bug.
	Generic(String),
}

impl std::error::Error for AtspiError {}

impl std::fmt::Display for AtspiError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidType => {
				f.write_str("atspi: invalid type conversion")
			}
			Self::MemberMatch(e) => {
				f.write_str(format!("atspi: member mismatch in conversion: {e}").as_str())
			}
			Self::InterfaceMatch(e) => {
				f.write_str(format!("atspi: interface mismatch in conversion: {e}").as_str())
			}
			Self::UnknownBusSignature(signature) => {
				f.write_str(&format!("atspi: Unknown bus body signature: {signature}."))
			}
			Self::UnknownInterface(interface_name) => {
				f.write_str(&format!("Unknown interface: {interface_name}."))
			}
			Self::MissingInterface => f.write_str("Missing interface."),
			Self::MissingMember => f.write_str("Missing member."),
			Self::MissingPath => f.write_str("Missing path."),
			Self::UnknownSignal(signal_name) => {
				f.write_str(&format!("atspi: Unknown signal: {signal_name}"))
			}
			Self::CacheVariantMismatch => f.write_str("atspi: Cache variant mismatch"),
			Self::Owned(e) => f.write_str(&format!("atspi: other error: {e}")),
			Self::Zbus(e) => f.write_str(&format!("ZBus Error: {e}")),
			Self::ParseError(e) => f.write_str(e),
			Self::IntConversionError => f.write_str("Integer conversion error."),
			Self::MissingName => f.write_str("Missing name for a bus."),
			Self::Infallible => {
				f.write_str("Infallible; only to trick the compiler. This should never happen.")
			},
			Self::Generic(msg) => f.write_str(msg),
		}
	}
}

impl From<std::convert::Infallible> for AtspiError {
	fn from(_e: std::convert::Infallible) -> Self {
		Self::Infallible
	}
}
impl From<std::num::TryFromIntError> for AtspiError {
	fn from(_e: std::num::TryFromIntError) -> Self {
		Self::IntConversionError
	}
}

#[cfg(feature = "zbus")]
impl From<zbus::fdo::Error> for AtspiError {
	fn from(e: zbus::fdo::Error) -> Self {
		Self::Generic(e.to_string())
	}
}

#[cfg(feature = "zbus")]
impl From<zbus::Error> for AtspiError {
	fn from(e: zbus::Error) -> Self {
		Self::Generic(e.to_string())
	}
}

impl From<zbus_names::Error> for AtspiError {
	fn from(e: zbus_names::Error) -> Self {
		Self::Generic(e.to_string())
	}
}

impl From<zvariant::Error> for AtspiError {
	fn from(zv_err: zvariant::Error) -> Self {
		match zv_err {
			zvariant::Error::Message(generic_msg) => Self::Generic(generic_msg),
			e => Self::Generic(e.to_string()),
		}
	}
}

impl From<std::io::Error> for AtspiError {
	fn from(e: std::io::Error) -> Self {
		Self::Generic(e.to_string())
	}
}
