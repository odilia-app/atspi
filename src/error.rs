#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
#[non_exhaustive]
/// An error type that can describe atspi and `std` and different `zbus` errors.
pub enum AtspiError {
    /// Converting one type into another failure
    Conversion(&'static str),

    /// When testing on either variant, we might find the we are not interested in.
    CacheVariantMismatch,

    /// On specific types, if the event / message member does not match the Event's name.
    MemberMatch(String),

    /// To indicate a match or equality test on a signa body signature failed.
    UnknownBusSignature,

    /// The signal that was encountered is unknown.
    UnknownSignal,

    /// Other errors.
    Owned(String),

    /// A `zbus` error. variant.
    Zbus(zbus::Error),

    /// A `zbus_names` error variant
    ZBusNames(zbus::names::Error),

    /// The `D-Bus` standard interfaces `zbus` error variant.
    /// as defined in ` zbus::fdo`.
    ZbusFdo(Box<zbus::fdo::Error>),

    /// Failed to parse a string into an enum variant
    ParseError(&'static str),

    /// Std i/o error variant.
    IO(std::io::Error),
}

impl std::error::Error for AtspiError {}

impl std::fmt::Display for AtspiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conversion(e) => f.write_str(&format!("atspi: conversion failure: {e}")),
            Self::MemberMatch(e) => {
                f.write_str(format!("atspi: member mismatch in conversion: {e}").as_str())
            }
            Self::UnknownBusSignature => f.write_str("atspi: Unknown bus body signature."),
            Self::UnknownSignal => f.write_str("atspi: Unknown signal"),
            Self::CacheVariantMismatch => f.write_str("atspi: Cache variant mismatch"),
            Self::Owned(e) => f.write_str(&format!("atspi: other error: {e}")),
            Self::Zbus(e) => f.write_str(&format!("ZBus Error: {e}")),
            Self::ZBusNames(e) => f.write_str(&format!("ZBus_names Error: {e}")),
            Self::ZbusFdo(e) => f.write_str(&format!("D-Bus standard interfaces Error: {e}")),
            Self::ParseError(e) => f.write_str(e),
            Self::IO(e) => f.write_str(&format!("std IO Error: {e}")),
        }
    }
}

impl From<zbus::fdo::Error> for AtspiError {
    fn from(e: zbus::fdo::Error) -> Self {
        Self::ZbusFdo(Box::new(e))
    }
}

impl From<zbus::Error> for AtspiError {
    fn from(e: zbus::Error) -> Self {
        Self::Zbus(e)
    }
}

impl From<zbus::names::Error> for AtspiError {
    fn from(e: zbus::names::Error) -> Self {
        Self::ZBusNames(e)
    }
}

impl From<zbus::zvariant::Error> for AtspiError {
    fn from(e: zbus::zvariant::Error) -> Self {
        Self::Zbus(zbus::Error::Variant(e))
    }
}

impl From<std::io::Error> for AtspiError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
