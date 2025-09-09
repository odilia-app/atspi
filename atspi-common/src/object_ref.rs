use crate::AtspiError;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use zbus_lockstep_macros::validate;
use zbus_names::{BusName, UniqueName};
use zvariant::{ObjectPath, OwnedValue, Structure, Type, Value};

const NULL_PATH_STR: &str = "/org/a11y/atspi/null";
const NULL_OBJECT_PATH: &ObjectPath<'static> =
	&ObjectPath::from_static_str_unchecked(NULL_PATH_STR);

#[cfg(test)]
pub(crate) const TEST_OBJECT_BUS_NAME: &str = ":0.0";
#[cfg(test)]
pub(crate) const TEST_OBJECT_PATH_STR: &str = "/org/a11y/atspi/test/default";
#[cfg(test)]
pub(crate) const TEST_DEFAULT_OBJECT_REF: ObjectRef<'static> =
	ObjectRef::from_static_str_unchecked(TEST_OBJECT_BUS_NAME, TEST_OBJECT_PATH_STR);

// Cannot derive `zvariant::Value` or `zvariant::OwnedValue` on non-unit variants in enums.	20250903

/// A unique *non-null* object reference.
/// An identifier for an object in the accessibility tree.
///
/// In AT-SPI2, objects in the applications' UI object tree are uniquely identified
/// using an application's bus name and object path. "(so)"
///
/// Emitted by `RemoveAccessible` and `Available`
#[validate(signal: "Available")]
#[derive(Clone, Debug, Eq, Type)]
#[zvariant(signature = "(so)")]
pub enum NonNullObjectRef<'o> {
	Owned { name: UniqueName<'static>, path: ObjectPath<'static> },
	Borrowed { name: UniqueName<'o>, path: ObjectPath<'o> },
}

impl<'o> NonNullObjectRef<'o> {
	/// Create a new `ObjectRef::Borrowed` from a `UniqueName` and `ObjectPath`.
	#[must_use]
	pub fn new(name: UniqueName<'o>, path: ObjectPath<'o>) -> Self {
		Self::new_borrowed(name, path)
	}

	/// Create a new, borrowed `ObjectRef`.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::NonNullObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	///
	/// let object_ref = NonNullObjectRef::new_borrowed(name, path);
	/// # assert_eq!(object_ref.name_as_str(), ":1.23");
	/// # assert_eq!(object_ref.path_as_str(), "/org/a11y/example/path/007");
	/// ```
	pub fn new_borrowed<N, P>(name: N, path: P) -> NonNullObjectRef<'o>
	where
		N: Into<UniqueName<'o>>,
		P: Into<ObjectPath<'o>>,
	{
		let name: UniqueName<'o> = name.into();
		let path: ObjectPath<'o> = path.into();

		Self::Borrowed { name, path }
	}

	/// Create a new, owned `NonNullObjectRef`.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::NonNullObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	///
	/// let object_ref = NonNullObjectRef::new_owned(name, path);
	/// # assert_eq!(object_ref.name_as_str(), ":1.23");
	/// # assert_eq!(object_ref.path_as_str(), "/org/a11y/example/path/007");
	/// ```
	pub fn new_owned<N, P>(name: N, path: P) -> NonNullObjectRef<'static>
	where
		N: Into<UniqueName<'static>>,
		P: Into<ObjectPath<'static>>,
	{
		let name: UniqueName<'static> = name.into();
		let path: ObjectPath<'static> = path.into();

		NonNullObjectRef::Owned { name, path }
	}

	/// Returns the name of the object reference.
	#[must_use]
	#[allow(clippy::match_same_arms)] // Arms differ by lifetime
	pub fn name(&self) -> &UniqueName<'_> {
		match self {
			Self::Owned { name, .. } => name,
			Self::Borrowed { name, .. } => name,
		}
	}

	/// Returns the path of the object reference.
	#[must_use]
	#[allow(clippy::match_same_arms)] // Arms differ by lifetime
	pub fn path(&self) -> &ObjectPath<'_> {
		match self {
			Self::Owned { path, .. } => path,
			Self::Borrowed { path, .. } => path,
		}
	}

	/// Create a new `NonNullObjectRef`, from `BusName` and `ObjectPath`.
	///
	/// # Errors
	/// Will fail if the `sender` is not a `UniqueName`.
	pub fn try_from_bus_name_and_path(
		sender: BusName<'o>,
		path: ObjectPath<'o>,
	) -> Result<Self, AtspiError> {
		// Check whether `BusName` matches `UniqueName`
		if let BusName::Unique(name) = sender {
			Ok(NonNullObjectRef::Borrowed { name, path })
		} else {
			Err(AtspiError::ParseError("Expected UniqueName"))
		}
	}

	/// Create a new `NonNullObjectRef`, unchecked.
	///
	/// # Safety
	/// The caller must ensure that the strings are valid for `UniqueName` and `ObjectPath`.
	#[must_use]
	pub const fn from_static_str_unchecked(name: &'static str, path: &'static str) -> Self {
		let name = UniqueName::from_static_str_unchecked(name);
		let path = ObjectPath::from_static_str_unchecked(path);

		NonNullObjectRef::Owned { name, path }
	}

	/// Converts the `NonNullObjectRef` into it's owned variant, consuming `self`.\
	/// If the object reference is `Owned`, it returns the same `NonNullObjectRef::Owned`.\
	/// If the object reference is `Borrowed`, it converts the name and path to owned versions\
	///  and returns `NonNullObjectRef::Owned`.
	///
	/// # Lifetime extension 'magic' (from 'o -> 'static')
	///
	/// `NonNullObjectRef<'_>` leans on the implementation of `UniqueName` and `ObjectPath` to
	/// convert the inner types to `'static`.\
	/// These types have an `Inner` enum that can contain an `Owned`, `Borrowed`, or `Static` [`Str` type.][docs.rs/zvariant/latest/zvariant/struct.Str]\
	/// The `Str` type is either a `&'static str` (static), `&str` (borrowed), or an `Arc<str>` (owned).
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::NonNullObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = NonNullObjectRef::new_borrowed(name, path);
	///
	/// let object_ref = object_ref.into_owned();
	/// assert!(matches!(object_ref, NonNullObjectRef::Owned { .. }));
	/// ```
	#[must_use]
	pub fn into_owned(self) -> NonNullObjectRef<'static> {
		match self {
			Self::Owned { name, path } => NonNullObjectRef::Owned { name, path },
			Self::Borrowed { name, path } => {
				NonNullObjectRef::Owned { name: name.to_owned(), path: path.to_owned() }
			}
		}
	}

	/// Returns the name of the object reference as a string slice.
	#[must_use]
	pub fn name_as_str(&self) -> &str {
		match self {
			NonNullObjectRef::Owned { name, .. } | NonNullObjectRef::Borrowed { name, .. } => {
				name.as_str()
			}
		}
	}

	/// Returns the path of the object reference as a string slice.
	#[must_use]
	pub fn path_as_str(&self) -> &str {
		match self {
			NonNullObjectRef::Owned { path, .. } | NonNullObjectRef::Borrowed { path, .. } => {
				path.as_str()
			}
		}
	}
}

/// A unique identifier for an object in the accessibility tree that can also be null.
/// A ubiquitous type used to refer to an object in the accessibility tree.
///
/// In AT-SPI2, objects in the applications' UI object tree are uniquely identified
/// using an application's bus name and object path. "(so)"
///
/// # null variant
/// A null-reference may be used either in the accessibility tree or
/// in method return messages to indicate that there is no object.
///
/// Emitted by `RemoveAccessible` and `Available`
#[validate(signal: "Available")]
#[derive(Clone, Debug, Eq, Type)]
#[zvariant(signature = "(so)")]
pub enum ObjectRef<'o> {
	Null,
	NonNull(NonNullObjectRef<'o>),
}

impl<'o> ObjectRef<'o> {
	/// Create a new `ObjectRef::Borrowed` from a `UniqueName` and `ObjectPath`.
	#[must_use]
	pub fn new(name: UniqueName<'o>, path: ObjectPath<'o>) -> Self {
		let non_null = NonNullObjectRef::new_borrowed(name, path);
		Self::NonNull(non_null)
	}

	/// Create a new, owned `ObjectRef`.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	///
	/// let object_ref = ObjectRef::new_owned(name, path);
	/// # assert_eq!(object_ref.name_as_str(), Some(":1.23"));
	/// # assert_eq!(object_ref.path_as_str(), "/org/a11y/example/path/007");
	/// ```
	pub fn new_owned<N, P>(name: N, path: P) -> ObjectRef<'static>
	where
		N: Into<UniqueName<'static>>,
		P: Into<ObjectPath<'static>>,
	{
		let name: UniqueName<'static> = name.into();
		let path: ObjectPath<'static> = path.into();

		let non_null = NonNullObjectRef::Owned { name, path };
		ObjectRef::NonNull(non_null)
	}

	/// Create a new, borrowed `ObjectRef`.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	///
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	/// # assert_eq!(object_ref.name_as_str(), Some(":1.23"));
	/// # assert_eq!(object_ref.path_as_str(), "/org/a11y/example/path/007");
	/// ```
	pub fn new_borrowed<N, P>(name: N, path: P) -> ObjectRef<'o>
	where
		N: Into<UniqueName<'o>>,
		P: Into<ObjectPath<'o>>,
	{
		let name: UniqueName<'o> = name.into();
		let path: ObjectPath<'o> = path.into();

		let non_null = NonNullObjectRef::Borrowed { name, path };
		Self::NonNull(non_null)
	}

	/// Create a new `ObjectRef`, from `BusName` and `ObjectPath`.
	///
	/// # Errors
	/// Will fail if the `sender` is not a `UniqueName`.
	pub fn try_from_bus_name_and_path(
		sender: BusName<'o>,
		path: ObjectPath<'o>,
	) -> Result<Self, AtspiError> {
		// Check whether `BusName` matches `UniqueName`
		if let BusName::Unique(name) = sender {
			let non_null = NonNullObjectRef::Borrowed { name, path };
			Ok(ObjectRef::NonNull(non_null))
		} else {
			Err(AtspiError::ParseError("Expected UniqueName"))
		}
	}

	/// Create a new `ObjectRef`, unchecked.
	///
	/// # Safety
	/// The caller must ensure that the strings are valid.
	#[must_use]
	pub const fn from_static_str_unchecked(name: &'static str, path: &'static str) -> Self {
		let non_null = NonNullObjectRef::from_static_str_unchecked(name, path);
		ObjectRef::NonNull(non_null)
	}

	/// Returns `true` if the object reference is `Null`, otherwise returns `false`.
	///
	/// Toolkits may use the `Null` object reference to indicate that an object is not available or does not exist.
	/// For example, when calling `Accessible::get_parent` on an object that has no parent,
	/// it may return a `Null` object reference.
	#[must_use]
	pub fn is_null(&self) -> bool {
		matches!(self, Self::Null)
	}

	/// Returns the name of the object reference.
	/// If the object reference is `Null`, it returns `None`.
	/// If the object reference is non-null, either `Owned` or `Borrowed`, it returns the name.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	///
	/// // Check the name of the object reference
	/// assert!(object_ref.name().is_some());
	/// assert_eq!(object_ref.name().unwrap().as_str(), ":1.23");
	/// ```
	#[must_use]
	pub fn name(&self) -> Option<&UniqueName<'_>> {
		match self {
			Self::NonNull(non_null) => Some(non_null.name()),
			Self::Null => None,
		}
	}

	/// Returns the path of the object reference.\
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	///
	/// // Check the path of the object reference
	/// assert_eq!(object_ref.path().as_str(), "/org/a11y/example/path/007");
	/// ```
	#[must_use]
	pub fn path(&self) -> &ObjectPath<'_> {
		match self {
			Self::NonNull(non_null) => non_null.path(),
			Self::Null => NULL_OBJECT_PATH,
		}
	}

	/// Converts the `ObjectRef` into an owned instance, consuming `self`.\
	/// If the object reference is `Null`, it returns `ObjectRef::Null`.\
	/// If the object reference is `Owned`, it returns the same `ObjectRef::Owned`.\
	/// If the object reference is `Borrowed`, it converts the name and path to owned versions and returns `ObjectRef::Owned`.
	///
	/// # Extending lifetime 'magic' (from 'o -> 'static')
	///
	/// `ObjectRef<'_>` leans on the implementation of `UniqueName` and `ObjectPath` to
	/// convert the inner types to `'static`.
	/// These types have an `Inner` enum that can contain an `Owned`, `Borrowed`, or `Static` `Str` type.
	/// The `Str`type is either a `&'static str` (static), `&str` (borrowed), or an `Arc<str>` (owned).
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::{ObjectRef, NonNullObjectRef};
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	///
	/// // Check whether the object reference can be converted to an owned version
	/// assert!(!object_ref.is_null());
	/// let object_ref = object_ref.into_owned();
	/// assert!(matches!(object_ref, ObjectRef::NonNull(NonNullObjectRef::Owned { .. })));
	/// ```
	#[must_use]
	pub fn into_owned(self) -> ObjectRef<'static> {
		match self {
			Self::Null => ObjectRef::Null,
			Self::NonNull(non_null) => ObjectRef::NonNull(non_null.into_owned()),
		}
	}

	/// Returns the name of the object reference as a string slice.
	#[must_use]
	pub fn name_as_str(&self) -> Option<&str> {
		match self {
			ObjectRef::Null => None,
			ObjectRef::NonNull(non_null) => Some(non_null.name_as_str()),
		}
	}

	/// Returns the path of the object reference as a string slice.
	#[must_use]
	pub fn path_as_str(&self) -> &str {
		match self {
			ObjectRef::Null => NULL_PATH_STR,
			ObjectRef::NonNull(non_null) => non_null.path_as_str(),
		}
	}
}

// Event tests lean on the `Default` implementation of `ObjectRef`.
// This is a workaround for the fact that `ObjectRef::Null` in
// `#[cfg(test)]` context is inconvenient.
// Events are guaranteed to have a non-null `ObjectRef` on their `item` field, because we receive signals over
// regular (non-p2p) DBus. Which means the `Message` `Header` has valid `Sender` and `Path` fields which
// are used to construct the `ObjectRef` from a `Message`.
#[cfg(test)]
impl Default for ObjectRef<'_> {
	/// Returns a non-Null object reference. (test implementation)
	fn default() -> Self {
		TEST_DEFAULT_OBJECT_REF
	}
}

#[cfg(not(test))]
impl Default for ObjectRef<'_> {
	/// Returns a `Null` object reference.
	fn default() -> Self {
		ObjectRef::Null
	}
}

/// A wrapper around the static variant of `ObjectRef`.
#[validate(signal: "Available")]
#[derive(Clone, Debug, Default, Eq, Type)]
pub struct ObjectRefOwned(pub(crate) ObjectRef<'static>);

impl From<ObjectRef<'_>> for ObjectRefOwned {
	/// Convert an `ObjectRef<'_>` into an `ObjectRefOwned`.
	///
	/// # Extending lifetime 'magic' (from 'o -> 'static')
	///
	/// `ObjectRef<'_>` leans on the implementation of `UniqueName` and `ObjectPath` to
	/// convert the inner types to `'static`.
	/// These types have an `Inner` enum that can contain an `Owned`, `Borrowed`, or `Static` `Str` type.
	/// The `Str`type is either a `&'static str` (static), `&str` (borrowed), or an `Arc<str>` (owned).
	fn from(object_ref: ObjectRef<'_>) -> Self {
		ObjectRefOwned(object_ref.into_owned())
	}
}

impl ObjectRefOwned {
	/// Create a new `ObjectRefOwned` from a static `ObjectRef`.
	#[must_use]
	pub const fn new(object_ref: ObjectRef<'static>) -> Self {
		ObjectRefOwned(object_ref)
	}

	/// Create a new `ObjectRefOwned` from `&'static str` unchecked.
	///
	/// # Safety
	/// The caller must ensure that the strings are valid.
	#[must_use]
	pub const fn from_static_str_unchecked(name: &'static str, path: &'static str) -> Self {
		ObjectRefOwned(ObjectRef::from_static_str_unchecked(name, path))
	}

	/// Returns `true` if the object reference is `Null`, otherwise returns `false`.
	#[must_use]
	pub fn is_null(&self) -> bool {
		matches!(self.0, ObjectRef::Null)
	}

	/// Returns the inner `ObjectRef`, consuming `self`.
	#[must_use]
	pub fn into_inner(self) -> ObjectRef<'static> {
		self.0
	}

	/// Returns the name of the object reference.
	/// If the object reference is `Null`, it returns `None`.
	/// If the object reference is `Owned` or `Borrowed`, it returns the name.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	///
	/// // Check the name of the object reference
	/// assert!(object_ref.name().is_some());
	/// assert_eq!(object_ref.name_as_str().unwrap(), ":1.23");
	/// ```
	#[must_use]
	pub fn name(&self) -> Option<&UniqueName<'static>> {
		match &self.0 {
			ObjectRef::NonNull(non_null) => match non_null {
				NonNullObjectRef::Owned { name, .. } | NonNullObjectRef::Borrowed { name, .. } => {
					Some(name)
				}
			},
			ObjectRef::Null => None,
		}
	}

	/// Returns the path of the object reference.\
	/// If the object reference is `Null`, it returns the null-path.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	///
	/// assert_eq!(object_ref.path_as_str(), "/org/a11y/example/path/007");
	/// ```
	#[must_use]
	pub fn path(&self) -> &ObjectPath<'static> {
		match &self.0 {
			ObjectRef::NonNull(non_null) => match non_null {
				NonNullObjectRef::Owned { path, .. } | NonNullObjectRef::Borrowed { path, .. } => {
					path
				}
			},
			ObjectRef::Null => NULL_OBJECT_PATH,
		}
	}

	/// Returns the name of the object reference as a string slice.
	#[must_use]
	pub fn name_as_str(&self) -> Option<&str> {
		match &self.0 {
			ObjectRef::Null => None,
			ObjectRef::NonNull(non_null) => Some(non_null.name_as_str()),
		}
	}

	/// Returns the path of the object reference as a string slice.
	#[must_use]
	pub fn path_as_str(&self) -> &str {
		match &self.0 {
			ObjectRef::Null => NULL_PATH_STR,
			ObjectRef::NonNull(non_null) => non_null.path_as_str(),
		}
	}
}

impl<'o> From<NonNullObjectRef<'o>> for ObjectRef<'o> {
	/// Convert a `NonNullObjectRef<'o>` into an `ObjectRef<'o>`.
	fn from(non_null: NonNullObjectRef<'o>) -> Self {
		ObjectRef::NonNull(non_null)
	}
}

impl From<NonNullObjectRef<'_>> for ObjectRefOwned {
	/// Convert a `NonNullObjectRef<'_>` into an `ObjectRefOwned`.
	fn from(non_null: NonNullObjectRef<'_>) -> Self {
		match non_null {
			// Somehow the compiler does not see that if we match on Owned, non_null must be owned.
			NonNullObjectRef::Owned { .. } => ObjectRefOwned(non_null.into_owned().into()),
			NonNullObjectRef::Borrowed { .. } => {
				let non_null = non_null.into_owned();
				ObjectRefOwned(non_null.into())
			}
		}
	}
}

impl<'o> TryFrom<ObjectRef<'o>> for NonNullObjectRef<'o> {
	type Error = AtspiError;

	/// Convert an `ObjectRef<'o>` into a `NonNullObjectRef<'o>`.
	///
	/// # Errors
	/// Will return an `AtspiError::ParseError` if the `ObjectRef` is `Null`.
	fn try_from(object_ref: ObjectRef<'o>) -> Result<Self, Self::Error> {
		match object_ref {
			ObjectRef::NonNull(non_null) => Ok(non_null),
			ObjectRef::Null => Err(AtspiError::ParseError("Expected NonNullObjectRef")),
		}
	}
}

impl TryFrom<ObjectRefOwned> for NonNullObjectRef<'static> {
	type Error = AtspiError;

	/// Convert an `ObjectRefOwned` into a `NonNullObjectRef<'static>`.
	///
	/// # Errors
	/// Will return an `AtspiError::ParseError` if the inner `ObjectRef` is `Null`.
	fn try_from(object_ref: ObjectRefOwned) -> Result<Self, Self::Error> {
		NonNullObjectRef::try_from(object_ref.0)
	}
}

impl Serialize for NonNullObjectRef<'_> {
	/// `NonNullObjectRef`'s wire format is `(&str, ObjectPath)`.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match &self {
			NonNullObjectRef::Owned { name, path } | NonNullObjectRef::Borrowed { name, path } => {
				(name.as_str(), path).serialize(serializer)
			}
		}
	}
}

impl Serialize for ObjectRef<'_> {
	/// `ObjectRef`'s wire format is `(&str, ObjectPath)`.
	/// The `Null` variant, the "Null object", is serialized as `("", ObjectPath("/org/a11y/atspi/null"))`.
	/// Both `Owned` and `Borrowed` variants are serialized as `(&str, ObjectPath)` with the object's\
	/// unique name and path.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match &self {
			ObjectRef::Null => ("", NULL_OBJECT_PATH).serialize(serializer),
			ObjectRef::NonNull(non_null) => non_null.serialize(serializer),
		}
	}
}

impl Serialize for ObjectRefOwned {
	/// `ObjectRefOwned` is serialized as the inner `ObjectRef`.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.serialize(serializer)
	}
}

// Preferably deserialize to `ObjectRef` to deserialize references from the bus.
// The NonNullObjectRef will error on the Null object reference.
impl<'de: 'o, 'o> Deserialize<'de> for NonNullObjectRef<'o> {
	/// `NonNullObjectRef`'s wire format is `(&str, ObjectPath)`.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct NonNullObjectRefVisitor;

		impl<'de> serde::de::Visitor<'de> for NonNullObjectRefVisitor {
			type Value = NonNullObjectRef<'de>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("a tuple of (&str, ObjectPath)")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				let name: &str = seq
					.next_element()?
					.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
				let path: ObjectPath<'de> = seq
					.next_element()?
					.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

				Ok(NonNullObjectRef::Borrowed {
					name: UniqueName::try_from(name).map_err(serde::de::Error::custom)?,
					path,
				})
			}
		}

		deserializer.deserialize_tuple(2, NonNullObjectRefVisitor)
	}
}

impl<'de: 'o, 'o> Deserialize<'de> for ObjectRef<'o> {
	/// `ObjectRef`'s wire format is `(&str, ObjectPath)`.
	/// An empty `&str` with a "/org/a11y/atspi/null" path is considered a `Null` object,
	/// this is deserialized as `ObjectRef::Null`.\
	/// Any other valid `(&str, ObjectPath)`  will deserialize into `ObjectRef::Borrowed`.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct ObjectRefVisitor;

		impl<'de> serde::de::Visitor<'de> for ObjectRefVisitor {
			type Value = ObjectRef<'de>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("a tuple of (&str, ObjectPath)")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				let name: &str = seq
					.next_element()?
					.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
				let path: ObjectPath<'de> = seq
					.next_element()?
					.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

				// Even though the specifications state that a null-reference is defined as: ("", ObjectPath),
				// some implementations use (valid bus name, null path) to indicate a null object,
				// We consider the sequance null if the path is null.
				// After this arm, empty names are a reason to panic.
				if path == *NULL_OBJECT_PATH {
					Ok(ObjectRef::Null)
				} else {
					assert!(
						!name.is_empty(),
						"A non-null ObjectRef requires a name and a path but got: (\"\", {path})"
					);
					Ok(ObjectRef::NonNull(NonNullObjectRef::Borrowed {
						name: UniqueName::try_from(name).map_err(serde::de::Error::custom)?,
						path,
					}))
				}
			}
		}

		deserializer.deserialize_tuple(2, ObjectRefVisitor)
	}
}

impl<'de> Deserialize<'de> for ObjectRefOwned {
	/// `ObjectRefOwned` is deserialized as "Owned" variant `ObjectRef<'static>`.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let object_ref: ObjectRef<'_> = Deserialize::deserialize(deserializer)?;
		Ok(object_ref.into())
	}
}

impl PartialEq for NonNullObjectRef<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name() && self.path() == other.path()
	}
}

impl PartialEq for ObjectRef<'_> {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			// Match order is relevant here. Null == Null, but Null != any other object.
			(ObjectRef::Null, ObjectRef::Null) => true,
			(ObjectRef::Null, _) | (_, ObjectRef::Null) => false,
			_ => self.name() == other.name() && self.path() == other.path(),
		}
	}
}

// NonNullObjectRef's hash must not consider the variant (Owned/Borrowed),
// because PartialEq does not consider it either.
//
// This to uphold the contract that if a == b, then a.hash() == b.hash() must hold true.
impl Hash for NonNullObjectRef<'_> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name().hash(state);
		self.path().hash(state);
	}
}

// ObjectRef's hash must not consider the variant (Null / Borrowed),
// because PartialEq does not consider it either. We say a borrowed and owned
// object reference with the same name and path are equal.
//
// This to uphold the contract that if a == b, then a.hash() == b.hash() must hold true.
impl Hash for ObjectRef<'_> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			ObjectRef::Null => {
				// Hashing a Null object reference.
				"Null".hash(state);
			}
			ObjectRef::NonNull(non_null) => {
				non_null.hash(state);
			}
		}
	}
}

// ObjectRefOwned's hash must not consider the variant (Owned/Borrowed),
// because PartialEq does not consider it either.
//
// This to uphold the contract that if a == b, then a.hash() == b.hash() must hold true.
impl Hash for ObjectRefOwned {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.hash(state);
	}
}

impl PartialEq for ObjectRefOwned {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl PartialEq<ObjectRef<'_>> for ObjectRefOwned {
	fn eq(&self, other: &ObjectRef<'_>) -> bool {
		self.0 == *other
	}
}

impl PartialEq<ObjectRefOwned> for ObjectRef<'_> {
	fn eq(&self, other: &ObjectRefOwned) -> bool {
		*self == other.0
	}
}

#[cfg(feature = "zbus")]
impl<'m: 'o, 'o> TryFrom<&'m zbus::message::Header<'_>> for ObjectRef<'o> {
	type Error = crate::AtspiError;

	// Construct an ObjectRef<'o> by reborrowing from the Header’s data.
	// 'm: 'o, 'm outlives 'o, so the references returned by this function
	// are guaranteed to be valid for the lifetime of the header.

	/// Construct an `ObjectRef` from a `zbus::message::Header`.
	///
	/// # Header fields
	///
	/// `Path` is a mandatory field on method calls and signals,
	/// `Sender` is an optional field, see:
	/// [DBus specification - header fields](<https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-header-fields>)).,
	///
	/// ```quote
	///  On a message bus, this header field is controlled by the message bus,
	///  so it is as reliable and trustworthy as the message bus itself.
	///  Otherwise, (eg. P2P) this header field is controlled by the message sender,
	///  unless there is out-of-band information that indicates otherwise.
	/// ```
	///
	/// While unlikely, it is possible that `Sender` or `Path` are not set on the header.
	/// This could happen if the server implementation does not set these fields for any reason.
	///
	/// # Errors
	/// Will return an `AtspiError::ParseError` if the header does not contain a valid path or sender.
	fn try_from(header: &'m zbus::message::Header) -> Result<Self, Self::Error> {
		let path = header.path().ok_or(crate::AtspiError::MissingPath)?;
		let name = header.sender().ok_or(crate::AtspiError::MissingName)?;
		Ok(ObjectRef::new_borrowed(name, path))
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::message::Header<'_>> for ObjectRefOwned {
	type Error = crate::AtspiError;

	/// Construct an `ObjectRefOwned` from a `zbus::message::Header`.
	fn try_from(header: &zbus::message::Header) -> Result<Self, Self::Error> {
		let name = header.sender().ok_or(crate::AtspiError::MissingName)?.to_owned();
		let path = header.path().ok_or(crate::AtspiError::MissingPath)?.to_owned();
		let object_ref = ObjectRef::new_owned(name, path);
		Ok(ObjectRefOwned::new(object_ref))
	}
}

// Implementing TryFrom<Value> and not From<Value>.
//
// If we have a TryFrom<T> for U, we can no longer implement From<T> for U or Into<U> for T,
// Because std core would implement TryFrom in terms of Into:
// <https://doc.rust-lang.org/std/convert/trait.TryFrom.html#impl-TryFrom%3CU%3E-for-T>
//
// impl<T, U> TryFrom<U> for T
// where
//    U: Into<T>, (This includes From<U> for T)
//
// We cannot derive TryFrom with `Value` derive macro, for `NonNullObjectRef` because `NonNullObjectRef`
// contains non-unit variants.
// The derive macros `Value` and `OwnedValue` do not support struct-like variants.

impl<'v> TryFrom<Value<'v>> for NonNullObjectRef<'v> {
	type Error = zvariant::Error;

	fn try_from(value: Value<'v>) -> Result<Self, Self::Error> {
		// Relies on the generic `Value` to tuple conversion `(UniqueName, ObjectPath)`.
		let (name, path): (UniqueName, ObjectPath) = value.try_into()?;
		Ok(NonNullObjectRef::new_borrowed(name, path))
	}
}

impl TryFrom<OwnedValue> for NonNullObjectRef<'static> {
	type Error = zvariant::Error;

	fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
		// Relies on the generic `Value` to tuple conversion `(UniqueName, ObjectPath)`.
		let (name, path): (UniqueName<'static>, ObjectPath<'static>) = value.try_into()?;
		Ok(NonNullObjectRef::new_owned(name, path))
	}
}

impl<'v> TryFrom<Value<'v>> for ObjectRef<'v> {
	type Error = zvariant::Error;

	fn try_from(value: Value<'v>) -> Result<Self, Self::Error> {
		let (name, path): (UniqueName, ObjectPath) = value.try_into()?;
		// Like `Deserialize`, let's make all null-path combinations ObjectRef::Null
		if path == *NULL_OBJECT_PATH {
			Ok(ObjectRef::Null)
		} else {
			assert!(
				!name.as_str().is_empty(),
				"A non-null ObjectRef requires a name and a path but got: (\"\", {path})"
			);
			Ok(ObjectRef::new_borrowed(name, path))
		}
	}
}

impl TryFrom<OwnedValue> for ObjectRef<'static> {
	type Error = zvariant::Error;

	fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
		let (name, path): (UniqueName<'static>, ObjectPath<'static>) = value.try_into()?;
		// Like `Deserialize`, let's make all null-path combinations ObjectRef::Null
		if path == *NULL_OBJECT_PATH {
			Ok(ObjectRef::Null)
		} else {
			assert!(
				!name.as_str().is_empty(),
				"A non-null ObjectRef requires a name and a path but got: (\"\", {path})"
			);
			Ok(ObjectRef::new_owned(name, path))
		}
	}
}

impl TryFrom<Value<'_>> for ObjectRefOwned {
	type Error = zvariant::Error;

	fn try_from(value: Value<'_>) -> Result<Self, Self::Error> {
		let value = OwnedValue::try_from(value)?;
		let object_ref = value.try_into()?;
		Ok(ObjectRefOwned::new(object_ref))
	}
}

impl TryFrom<OwnedValue> for ObjectRefOwned {
	type Error = zvariant::Error;

	fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
		let object_ref: ObjectRef<'static> = value.try_into()?;
		Ok(ObjectRefOwned::new(object_ref))
	}
}

// implemented by zvariant as blanket for T: Into<Structure>
// impl<'v> From<ObjectRef<'v>> for Value<'v>

impl<'o> From<NonNullObjectRef<'o>> for Structure<'o> {
	fn from(non_null: NonNullObjectRef<'o>) -> Self {
		match non_null {
			NonNullObjectRef::Owned { name, path } | NonNullObjectRef::Borrowed { name, path } => {
				Structure::from((name, path))
			}
		}
	}
}

impl<'o> From<ObjectRef<'o>> for Structure<'_> {
	fn from(object_ref: ObjectRef<'o>) -> Self {
		match object_ref {
			ObjectRef::Null => Structure::from(("", NULL_OBJECT_PATH)),
			ObjectRef::NonNull(non_null) => {
				Structure::from((non_null.name().to_owned(), non_null.path().to_owned()))
			}
		}
	}
}

impl From<ObjectRefOwned> for Value<'static> {
	fn from(object_ref_owned: ObjectRefOwned) -> Self {
		let object_ref: ObjectRef<'static> = object_ref_owned.into_inner();
		object_ref.into()
	}
}

#[cfg(test)]
mod tests {
	use crate::object_ref::{NULL_OBJECT_PATH, NULL_PATH_STR};
	use crate::{NonNullObjectRef, ObjectRef};
	use std::hash::{DefaultHasher, Hash, Hasher};
	use zbus::zvariant;
	use zbus::{names::UniqueName, zvariant::ObjectPath};
	use zvariant::{serialized::Context, to_bytes, OwnedValue, Value, LE};

	const TEST_OBJECT_PATH: &str = "/org/a11y/atspi/path/007";

	#[test]
	fn non_null_object_ref_owned_creation() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let non_null = super::NonNullObjectRef::new_owned(name, path);

		assert_eq!(non_null.name_as_str(), ":1.23");
		assert_eq!(non_null.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn owned_object_ref_creation() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let object_ref = ObjectRef::new_owned(name, path);

		assert_eq!(object_ref.name_as_str(), Some(":1.23"));
		assert_eq!(object_ref.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn non_null_object_ref_borrowed_creation() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let non_null = super::NonNullObjectRef::new_borrowed(name, path);

		assert_eq!(non_null.name_as_str(), ":1.23");
		assert_eq!(non_null.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn borrowed_object_ref_creation() {
		let object_ref = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		assert_eq!(object_ref.name_as_str(), Some(":1.23"));
		assert_eq!(object_ref.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn null_object_ref() {
		let null_object_ref: ObjectRef = ObjectRef::Null;
		assert!(null_object_ref.is_null());
		assert!(null_object_ref.name().is_none());
		assert_eq!(null_object_ref.path(), NULL_OBJECT_PATH);
	}

	#[test]
	fn non_null_object_ref_into_owned() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let non_null = super::NonNullObjectRef::new_borrowed(name, path);
		let owned_non_null = non_null.into_owned();

		assert_eq!(owned_non_null.name_as_str(), ":1.23");
		assert_eq!(owned_non_null.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn object_ref_into_owned() {
		let borrowed_object_ref = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		let owned_object_ref = borrowed_object_ref.into_owned();
		assert!(matches!(owned_object_ref, ObjectRef::NonNull(NonNullObjectRef::Owned { .. })));
		assert_eq!(owned_object_ref.name_as_str(), Some(":1.23"));
		assert_eq!(owned_object_ref.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn object_ref_into_name_and_path() {
		let object_ref = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		let name = object_ref.name().unwrap();
		let path = object_ref.path();
		assert_eq!(name.as_str(), ":1.23");
		assert_eq!(path.as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn serialization_null_object_ref() {
		let null_object_ref: ObjectRef = ObjectRef::Null;
		assert!(null_object_ref.is_null());

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &null_object_ref).unwrap();

		let (obj, _) = encoded.deserialize::<ObjectRef>().unwrap();

		assert!(obj.is_null());
		assert!(obj.name().is_none());
		assert_eq!(obj.path(), NULL_OBJECT_PATH);
	}

	#[test]
	fn serialize_non_null_object_ref() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let non_null = super::NonNullObjectRef::new_borrowed(name, path);

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &non_null).unwrap();

		let (obj, _) = encoded.deserialize::<super::NonNullObjectRef>().unwrap();

		assert_eq!(obj.name_as_str(), ":1.23");
		assert_eq!(obj.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn serialization_owned_object_ref() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let object_ref = ObjectRef::new_owned(name, path);

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &object_ref).unwrap();

		let (obj, _) = encoded.deserialize::<ObjectRef>().unwrap();

		// Deserialization always results in a borrowed object reference.
		// On the wire the distinction between owned and borrowed is not preserved.
		// As borrowed is the cheaper option, we always deserialize to that.
		assert!(matches!(obj, ObjectRef::NonNull(NonNullObjectRef::Borrowed { .. })));
		assert_eq!(obj.name().unwrap().as_str(), ":1.23");
		assert_eq!(obj.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn serialize_non_null_owned_object_ref() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let non_null = super::NonNullObjectRef::new_owned(name, path);

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &non_null).unwrap();

		let (obj, _) = encoded.deserialize::<super::NonNullObjectRef>().unwrap();

		assert!(matches!(obj, super::NonNullObjectRef::Borrowed { .. }));
		assert_eq!(obj.name_as_str(), ":1.23");
		assert_eq!(obj.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn serialization_borrowed_object_ref() {
		let object_ref = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &object_ref).unwrap();

		let (obj, _) = encoded.deserialize::<ObjectRef>().unwrap();
		assert!(matches!(obj, ObjectRef::NonNull(NonNullObjectRef::Borrowed { .. })));

		assert_eq!(obj.name().unwrap().as_str(), ":1.23");
		assert_eq!(obj.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn non_null_object_ref_equality() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let object_ref1 = NonNullObjectRef::new_borrowed(&name, &path);
		let object_ref2 = NonNullObjectRef::new_borrowed(&name, &path);

		assert_eq!(object_ref1, object_ref2);

		let name2 = UniqueName::from_static_str_unchecked(":1.24");
		let object_ref3 = NonNullObjectRef::new_borrowed(name2, &path);
		assert_ne!(object_ref1, object_ref3);

		let object_ref4 = NonNullObjectRef::new_owned(name, &path);
		assert_eq!(object_ref1, object_ref4);
	}

	#[test]
	fn object_ref_equality() {
		let object_ref1 = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		let object_ref2 = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		assert_eq!(object_ref1, object_ref2);

		let object_ref3 = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.24").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		assert_ne!(object_ref1, object_ref3);

		let object_ref4 = ObjectRef::new_owned(
			UniqueName::from_static_str_unchecked(":1.23"),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		assert_eq!(object_ref1, object_ref4);

		let null_object_ref: ObjectRef = ObjectRef::Null;
		assert_ne!(object_ref1, null_object_ref);
		assert_ne!(null_object_ref, object_ref1);

		let null_object_ref2: ObjectRef = ObjectRef::Null;
		assert_eq!(null_object_ref, null_object_ref2);
	}

	#[test]
	fn try_from_value_for_objectref() {
		let name = UniqueName::from_static_str_unchecked(":0.0");
		let path = ObjectPath::from_static_str_unchecked("/org/a11y/atspi/testpath");

		let objref = ObjectRef::new_borrowed(name, path);
		let value: Value = objref.into();

		let objref_2: ObjectRef = value.try_into().unwrap();

		assert_eq!(objref_2.name().unwrap().as_str(), ":0.0");
		assert_eq!(objref_2.path_as_str(), "/org/a11y/atspi/testpath");
	}

	#[test]
	fn try_from_owned_value_for_objectref() {
		let name = UniqueName::from_static_str_unchecked(":0.0");
		let path = ObjectPath::from_static_str_unchecked("/org/a11y/atspi/testpath");

		let objref = ObjectRef::new_borrowed(name, path);

		let value: Value = objref.into();
		let value: OwnedValue = value.try_into().unwrap();
		let objref_2: ObjectRef = value.try_into().unwrap();

		assert_eq!(objref_2.name_as_str(), Some(":0.0"));
		assert_eq!(objref_2.path_as_str(), "/org/a11y/atspi/testpath");
	}

	// Must fail test:
	#[test]
	fn must_fail_test_try_from_invalid_value_for_object_ref() {
		let value = zvariant::Value::from((42, true));
		let obj: Result<ObjectRef, _> = value.try_into();
		assert!(obj.is_err());
	}

	#[test]
	fn non_null_hash_and_object_coherence() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let object_ref1 = super::NonNullObjectRef::new_borrowed(&name, &path);
		let object_ref2 = super::NonNullObjectRef::new_borrowed(name, path);

		// If a == b then a.hash() == b.hash()

		let mut hasher1 = DefaultHasher::new();
		let mut hasher2 = DefaultHasher::new();
		assert_eq!(object_ref1, object_ref2);
		object_ref1.hash(&mut hasher1);
		object_ref2.hash(&mut hasher2);
		assert_eq!(hasher1.finish(), hasher2.finish());
	}

	#[test]
	fn hash_and_object_coherence() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let object_ref1 = ObjectRef::new_borrowed(&name, &path);
		let object_ref2 = ObjectRef::new_borrowed(name, path);

		let mut hasher1 = DefaultHasher::new();
		let mut hasher2 = DefaultHasher::new();
		assert_eq!(object_ref1, object_ref2);
		object_ref1.hash(&mut hasher1);
		object_ref2.hash(&mut hasher2);
		assert_eq!(hasher1.finish(), hasher2.finish());
	}

	#[test]
	#[should_panic(
		expected = "assertion failed: matches!(obj, ObjectRef::NonNull(NonNullObjectRef::Borrowed { .. }))"
	)]
	fn valid_name_null_path_object_ref() {
		let object_ref = ObjectRef::from_static_str_unchecked("1.23", NULL_PATH_STR);

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &object_ref).unwrap();

		let (obj, _) = encoded.deserialize::<ObjectRef>().unwrap();
		assert!(matches!(obj, ObjectRef::NonNull(NonNullObjectRef::Borrowed { .. })));
	}

	// Check that the Deserialize implementation correctly panics
	#[test]
	#[should_panic(
		expected = r#"A non-null ObjectRef requires a name and a path but got: ("", /org/a11y/atspi/path/007)"#
	)]
	fn empty_name_valid_path_object_ref() {
		let object_ref = ObjectRef::from_static_str_unchecked("", TEST_OBJECT_PATH);

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &object_ref).unwrap();

		let (_obj, _) = encoded.deserialize::<ObjectRef>().unwrap();
	}
}
