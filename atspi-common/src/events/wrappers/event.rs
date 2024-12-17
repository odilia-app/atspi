use zbus_names::UniqueName;
use zvariant::ObjectPath;
use crate::{
	EventTypeProperties,
	EventProperties,
};
use crate::events::{
	EventListenerEvents,
	wrappers::ObjectEvents,
	CacheEvents,
	AvailableEvent,
	WindowEvents,
	TerminalEvents,
	MouseEvents,
	KeyboardEvents,
	FocusEvents,
	DocumentEvents,
};
use serde::{Serialize, Deserialize};

/// Encapsulates the various different accessibility bus signal types.
///
/// Assumes being non exhaustive to allow for future- or custom signals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Event {
	/// See: [`DocumentEvents`].
	Document(DocumentEvents),
	/// See: [`FocusEvents`].
	Focus(FocusEvents),
	/// See: [`KeyboardEvents`].
	Keyboard(KeyboardEvents),
	/// See: [`MouseEvents`].
	Mouse(MouseEvents),
	/// See: [`ObjectEvents`].
	Object(ObjectEvents),
	/// See: [`TerminalEvents`].
	Terminal(TerminalEvents),
	/// See: [`WindowEvents`].
	Window(WindowEvents),
	/// See: [`AvailableEvent`].
	Available(AvailableEvent),
	/// See: [`CacheEvents`].
	Cache(CacheEvents),
	/// See: [`EventListenerEvents`].
	Listener(EventListenerEvents),
}

impl EventTypeProperties for Event {
  fn member(&self) -> &'static str {
    match self {
      Self::Document(inner) => inner.member(),
      Self::Focus(inner) => inner.member(),
      Self::Keyboard(inner) => inner.member(),
      Self::Mouse(inner) => inner.member(),
      Self::Object(inner) => inner.member(),
      Self::Terminal(inner) => inner.member(),
      Self::Window(inner) => inner.member(),
      Self::Available(inner) => inner.member(),
      Self::Cache(inner) => inner.member(),
      Self::Listener(inner) => inner.member(),
    }
  }
  fn interface(&self) -> &'static str {
    match self {
      Self::Document(inner) => inner.interface(),
      Self::Focus(inner) => inner.interface(),
      Self::Keyboard(inner) => inner.interface(),
      Self::Mouse(inner) => inner.interface(),
      Self::Object(inner) => inner.interface(),
      Self::Terminal(inner) => inner.interface(),
      Self::Window(inner) => inner.interface(),
      Self::Available(inner) => inner.interface(),
      Self::Cache(inner) => inner.interface(),
      Self::Listener(inner) => inner.interface(),
    }
  }
  fn match_rule(&self) -> &'static str {
    match self {
      Self::Document(inner) => inner.match_rule(),
      Self::Focus(inner) => inner.match_rule(),
      Self::Keyboard(inner) => inner.match_rule(),
      Self::Mouse(inner) => inner.match_rule(),
      Self::Object(inner) => inner.match_rule(),
      Self::Terminal(inner) => inner.match_rule(),
      Self::Window(inner) => inner.match_rule(),
      Self::Available(inner) => inner.match_rule(),
      Self::Cache(inner) => inner.match_rule(),
      Self::Listener(inner) => inner.match_rule(),
    }
  }
  fn registry_string(&self) -> &'static str {
    match self {
      Self::Document(inner) => inner.registry_string(),
      Self::Focus(inner) => inner.registry_string(),
      Self::Keyboard(inner) => inner.registry_string(),
      Self::Mouse(inner) => inner.registry_string(),
      Self::Object(inner) => inner.registry_string(),
      Self::Terminal(inner) => inner.registry_string(),
      Self::Window(inner) => inner.registry_string(),
      Self::Available(inner) => inner.registry_string(),
      Self::Cache(inner) => inner.registry_string(),
      Self::Listener(inner) => inner.registry_string(),
    }
  }
}

impl EventProperties for Event {
  fn path(&self) -> ObjectPath<'_> {
    match self {
      Self::Document(inner) => inner.path(),
      Self::Focus(inner) => inner.path(),
      Self::Keyboard(inner) => inner.path(),
      Self::Mouse(inner) => inner.path(),
      Self::Object(inner) => inner.path(),
      Self::Terminal(inner) => inner.path(),
      Self::Window(inner) => inner.path(),
      Self::Available(inner) => inner.path(),
      Self::Cache(inner) => inner.path(),
      Self::Listener(inner) => inner.path(),
    }
  }
  fn sender(&self) -> UniqueName<'_> {
    match self {
      Self::Document(inner) => inner.sender(),
      Self::Focus(inner) => inner.sender(),
      Self::Keyboard(inner) => inner.sender(),
      Self::Mouse(inner) => inner.sender(),
      Self::Object(inner) => inner.sender(),
      Self::Terminal(inner) => inner.sender(),
      Self::Window(inner) => inner.sender(),
      Self::Available(inner) => inner.sender(),
      Self::Cache(inner) => inner.sender(),
      Self::Listener(inner) => inner.sender(),
    }
  }
}
