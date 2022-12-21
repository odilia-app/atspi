#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
#[non_exhaustive]
/// The aggregate error type for atspi and `std` and different `zbus` errors.
pub enum AtspiError {
    /// Converting one type into another failure
    Conversion(&'static str),

    /// Add Atspi Errors as we identify them, rather than (ab)using 'Other'.

    /// Other errors.
    Owned(String),

    /// A `zbus` error. variant.
    Zbus(zbus::Error),

    /// A `zbus_names` error variant
    ZBusNames(zbus::names::Error),

    /// The `D-Bus` standard interfaces `zbus` error variant.
    /// as defined in ` zbus::fdo`.
    ZbusFdo(Box<dyn std::error::Error>),

    /// Failed to parse a string into an enum variant
    ParseError(&'static str),

    /// Std i/o error variant.
    IO(std::io::Error),
}

impl std::error::Error for AtspiError {}

impl std::fmt::Display for AtspiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtspiError::Conversion(e) => f.write_str(&format!("atspi: conversion failure: {e}")),
            AtspiError::Owned(e) => f.write_str(&format!("atspi: other error: {e}")),
            AtspiError::Zbus(e) => f.write_str(&format!("ZBus Error: {e}")),
            AtspiError::ZBusNames(e) => f.write_str(&format!("ZBus_names Error: {e}")),
            AtspiError::ZbusFdo(e) => f.write_str(&format!("D-Bus standard interfaces Error: {e}")),
            AtspiError::ParseError(e) => f.write_str(e),
            AtspiError::IO(e) => f.write_str(&format!("std IO Error: {e}")),
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
