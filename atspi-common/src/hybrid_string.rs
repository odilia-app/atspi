//! # `HybridString`
//!
//! A string that can be either allocated on the stack or the heap.

use heapless::String as HeapLessString;
use serde::Deserialize;
use serde::Serialize;
use zvariant::Signature;
use zvariant::Type;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum HybridString<const N: usize> {
	Stack(HeapLessString<N>),
	Heap(std::string::String),
}

impl<const N: usize> Default for HybridString<N> {
	fn default() -> Self {
		HybridString::new()
	}
}

impl<const N: usize> From<&str> for HybridString<N> {
	fn from(s: &str) -> Self {
		HybridString::from_str(s)
	}
}

impl<const N: usize> From<std::string::String> for HybridString<N> {
	fn from(s: std::string::String) -> Self {
		HybridString::from_string(s)
	}
}

impl<const N: usize> Serialize for HybridString<N> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.as_str().serialize(serializer)
	}
}

impl<'de, const N: usize> Deserialize<'de> for HybridString<N> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use serde::de::Visitor;
		use std::fmt;

		struct HybridStringVisitor<const M: usize>;

		impl<'de, const M: usize> Visitor<'de> for HybridStringVisitor<M> {
			type Value = HybridString<M>;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a string")
			}

			// Handle borrowed strings - most efficient
			fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				Ok(HybridString::from_str(value))
			}

			// Handle owned strings
			fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				Ok(HybridString::from_string(value))
			}

			// Handle string slices
			fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				Ok(HybridString::from_str(value))
			}
		}

		deserializer.deserialize_string(HybridStringVisitor::<N>)
	}
}

impl<const N: usize> std::fmt::Debug for HybridString<N> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.as_str().fmt(f)
	}
}

impl<const N: usize> std::fmt::Display for HybridString<N> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.as_str().fmt(f)
	}
}

impl<const N: usize> Type for HybridString<N> {
	const SIGNATURE: &Signature = &Signature::Str;
}

impl<const N: usize> HybridString<N> {
	/// Create a new empty `HybridString`
	pub fn new() -> Self {
		HybridString::Stack(HeapLessString::new())
	}

	/// Create a new `HybridString` from a `String`
	pub fn from_string(s: std::string::String) -> Self {
		HybridString::Heap(s)
	}

	/// Create a new `HybridString` from a `&str`        
	pub fn from_str(s: &str) -> Self {
		if s.len() <= N {
			let mut stack = HeapLessString::new();
			stack
				.push_str(s)
				.expect("strings smaller than N should fit in the stack");
			HybridString::Stack(stack)
		} else {
			HybridString::Heap(s.to_string())
		}
	}

	/// Create a new `HybridString` from a &str
	pub fn as_str(&self) -> &str {
		match self {
			HybridString::Stack(s) => s.as_str(),
			HybridString::Heap(s) => s.as_str(),
		}
	}

	/// As a `std::string::String`
	pub fn as_string(&self) -> std::string::String {
		match self {
			HybridString::Stack(s) => s.as_str().to_string(),
			HybridString::Heap(s) => s.clone(),
		}
	}

	/// Push a &str to the `HybridString`
	pub fn push_str(&mut self, s: &str) {
		match self {
			HybridString::Stack(stack) => {
				if stack.len() + s.len() <= N {
					stack
						.push_str(s)
						.expect("strings smaller than N should fit in the stack");
				} else {
					let mut heap = std::string::String::from(stack.as_str());
					heap.push_str(s);
					*self = HybridString::Heap(heap);
				}
			}
			HybridString::Heap(heap) => heap.push_str(s),
		}
	}

	/// Return the length of a `HybridString`.
	pub fn len(&self) -> usize {
		match self {
			HybridString::Stack(s) => s.len(),
			HybridString::Heap(s) => s.len(),
		}
	}

	/// Return true if the `HybridString` is empty.
	pub fn is_empty(&self) -> bool {
		match self {
			HybridString::Stack(s) => s.is_empty(),
			HybridString::Heap(s) => s.is_empty(),
		}
	}
}

impl PartialEq<str> for HybridString<32> {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}

impl PartialEq<str> for HybridString<64> {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}

impl PartialEq<str> for HybridString<128> {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}

impl PartialEq<str> for HybridString<256> {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}

impl PartialEq<str> for HybridString<512> {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}
