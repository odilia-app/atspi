use crate::{
	accessible::AccessibleProxy, action::ActionProxy, application::ApplicationProxy,
	cache::CacheProxy, collection::CollectionProxy, component::ComponentProxy,
	document::DocumentProxy, editable_text::EditableTextProxy, hyperlink::HyperlinkProxy,
	hypertext::HypertextProxy, image::ImageProxy, selection::SelectionProxy, table::TableProxy,
	table_cell::TableCellProxy, text::TextProxy, value::ValueProxy, AtspiError,
};
use atspi_common::{Interface, InterfaceSet, Result};

/// Easily acquire the other interface proxies an object may have.
///
/// Equip objects with conversions to proxies of the objects' further implemented interfaces
/// by extending `AccessibleProxy`.
///
/// The `proxies` method returns a `Proxies` struct, which contains lazily loaded proxy accessors.
///
/// # Lazy initialization and cheap checks
///
/// Proxies are lazily initialized, so they are only created when requested.
/// Interface availability is checked before creating the proxy and is a cheap bitop.
pub trait ProxyExt<'a> {
	/// Get `Proxies` for the current object.
	fn proxies(&self) -> impl std::future::Future<Output = Result<Proxies<'a>>>;
}

/// An object for safe conversion to the related interface proxies.
#[derive(Clone, Debug)]
pub struct Proxies<'a> {
	interfaces: InterfaceSet,
	proxy: zbus::Proxy<'a>,
	inner: InnerProxies<'a>,
}

#[derive(Clone, Debug, Default)]
struct InnerProxies<'a> {
	action: Option<ActionProxy<'a>>,
	application: Option<ApplicationProxy<'a>>,
	cache: Option<CacheProxy<'a>>,
	collection: Option<CollectionProxy<'a>>,
	component: Option<ComponentProxy<'a>>,
	document: Option<DocumentProxy<'a>>,
	editable_text: Option<EditableTextProxy<'a>>,
	hyperlink: Option<HyperlinkProxy<'a>>,
	hypertext: Option<HypertextProxy<'a>>,
	image: Option<ImageProxy<'a>>,
	selection: Option<SelectionProxy<'a>>,
	table: Option<TableProxy<'a>>,
	table_cell: Option<TableCellProxy<'a>>,
	text: Option<TextProxy<'a>>,
	value: Option<ValueProxy<'a>>,
}

impl<'a> ProxyExt<'a> for AccessibleProxy<'a> {
	async fn proxies(&self) -> Result<Proxies<'a>> {
		let iface_set: InterfaceSet = self.get_interfaces().await?;
		let proxy = self.inner().clone();

		Ok(Proxies { interfaces: iface_set, proxy, inner: InnerProxies::default() })
	}
}

impl<'a> Proxies<'a> {
	/// Get the `Action` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn action(&mut self) -> Result<&mut ActionProxy<'a>> {
		if self.interfaces.contains(Interface::Action) {
			let proxy_ref = self
				.inner
				.action
				.get_or_insert_with(|| ActionProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Action"))
		}
	}

	/// Get the `Application` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn application(&mut self) -> Result<&mut ApplicationProxy<'a>> {
		if self.interfaces.contains(Interface::Application) {
			let proxy_ref = self
				.inner
				.application
				.get_or_insert_with(|| ApplicationProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Application"))
		}
	}

	/// Get the `Cache` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn cache(&mut self) -> Result<&mut CacheProxy<'a>> {
		if self.interfaces.contains(Interface::Cache) {
			let proxy_ref = self
				.inner
				.cache
				.get_or_insert_with(|| CacheProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Cache"))
		}
	}

	/// Get the `Collection` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn collection(&mut self) -> Result<&mut CollectionProxy<'a>> {
		if self.interfaces.contains(Interface::Collection) {
			let proxy_ref = self
				.inner
				.collection
				.get_or_insert_with(|| CollectionProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Collection"))
		}
	}

	/// Get the `Component` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn component(&mut self) -> Result<&mut ComponentProxy<'a>> {
		if self.interfaces.contains(Interface::Component) {
			let proxy_ref = self
				.inner
				.component
				.get_or_insert_with(|| ComponentProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Component"))
		}
	}

	/// Get the `Document` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn document(&mut self) -> Result<&mut DocumentProxy<'a>> {
		if self.interfaces.contains(Interface::Document) {
			let proxy_ref = self
				.inner
				.document
				.get_or_insert_with(|| DocumentProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Document"))
		}
	}

	/// Get the `EditableText` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn editable_text(&mut self) -> Result<&mut EditableTextProxy<'a>> {
		if self.interfaces.contains(Interface::EditableText) {
			let proxy_ref = self
				.inner
				.editable_text
				.get_or_insert_with(|| EditableTextProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("EditableText"))
		}
	}

	/// Get the `Hyperlink` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn hyperlink(&mut self) -> Result<&mut HyperlinkProxy<'a>> {
		if self.interfaces.contains(Interface::Hyperlink) {
			let proxy_ref = self
				.inner
				.hyperlink
				.get_or_insert_with(|| HyperlinkProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Hyperlink"))
		}
	}

	/// Get the `Hypertext` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn hypertext(&mut self) -> Result<&mut HypertextProxy<'a>> {
		if self.interfaces.contains(Interface::Hypertext) {
			let proxy_ref = self
				.inner
				.hypertext
				.get_or_insert_with(|| HypertextProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Hypertext"))
		}
	}

	/// Get the `Image` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn image(&mut self) -> Result<&mut ImageProxy<'a>> {
		if self.interfaces.contains(Interface::Image) {
			let proxy_ref = self
				.inner
				.image
				.get_or_insert_with(|| ImageProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Image"))
		}
	}

	/// Get the `Registry` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn selection(&mut self) -> Result<&mut SelectionProxy<'a>> {
		if self.interfaces.contains(Interface::Selection) {
			let proxy_ref = self
				.inner
				.selection
				.get_or_insert_with(|| SelectionProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Selection"))
		}
	}

	/// Get the `Table` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn table(&mut self) -> Result<&mut TableProxy<'a>> {
		if self.interfaces.contains(Interface::Table) {
			let proxy_ref = self
				.inner
				.table
				.get_or_insert_with(|| TableProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Table"))
		}
	}

	/// Get the `TableCell` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn table_cell(&mut self) -> Result<&mut TableCellProxy<'a>> {
		if self.interfaces.contains(Interface::TableCell) {
			let proxy_ref = self
				.inner
				.table_cell
				.get_or_insert_with(|| TableCellProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("TableCell"))
		}
	}

	/// Get the `Text` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn text(&mut self) -> Result<&mut TextProxy<'a>> {
		if self.interfaces.contains(Interface::Text) {
			let proxy_ref = self
				.inner
				.text
				.get_or_insert_with(|| TextProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Text"))
		}
	}

	/// Get the `Value` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub fn value(&mut self) -> Result<&mut ValueProxy<'a>> {
		if self.interfaces.contains(Interface::Value) {
			let proxy_ref = self
				.inner
				.value
				.get_or_insert_with(|| ValueProxy::from(self.proxy.clone()));
			Ok(proxy_ref)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Value"))
		}
	}
}
