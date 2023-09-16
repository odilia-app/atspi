use crate::{
	accessible::{Accessible, AccessibleBlocking, AccessibleProxy, AccessibleProxyBlocking},
	action::{Action, ActionBlocking, ActionProxy, ActionProxyBlocking},
	application::{Application, ApplicationBlocking, ApplicationProxy, ApplicationProxyBlocking},
	collection::{Collection, CollectionBlocking, CollectionProxy, CollectionProxyBlocking},
	component::{Component, ComponentBlocking, ComponentProxy, ComponentProxyBlocking},
	document::{Document, DocumentBlocking, DocumentProxy, DocumentProxyBlocking},
	editable_text::{
		EditableText, EditableTextBlocking, EditableTextProxy, EditableTextProxyBlocking,
	},
	hyperlink::{Hyperlink, HyperlinkBlocking, HyperlinkProxy, HyperlinkProxyBlocking},
	hypertext::{Hypertext, HypertextBlocking, HypertextProxy, HypertextProxyBlocking},
	image::{Image, ImageBlocking, ImageProxy, ImageProxyBlocking},
	selection::{Selection, SelectionBlocking, SelectionProxy, SelectionProxyBlocking},
	table::{Table, TableBlocking, TableProxy, TableProxyBlocking},
	table_cell::{TableCell, TableCellBlocking, TableCellProxy, TableCellProxyBlocking},
	text::{Text, TextBlocking, TextProxy, TextProxyBlocking},
	value::{Value, ValueBlocking, ValueProxy, ValueProxyBlocking},
	AtspiProxy,
};
use async_trait::async_trait;
use std::ops::Deref;
use zbus::{
	blocking::Proxy as ProxyBlocking, blocking::ProxyBuilder as ProxyBuilderBlocking,
	CacheProperties, Error, Proxy, ProxyBuilder, ProxyDefault,
};

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait Convertable {
	type Error: std::error::Error;
	type Accessible: Accessible + Send + Sync;
	type Action: Action + Send + Sync;
	type Application: Application + Send + Sync;
	type Collection: Collection + Send + Sync;
	type Component: Component + Send + Sync;
	type Document: Document + Send + Sync;
	type Hypertext: Hypertext + Send + Sync;
	type Hyperlink: Hyperlink + Send + Sync;
	type Image: Image + Send + Sync;
	type Selection: Selection + Send + Sync;
	type Table: Table + Send + Sync;
	type TableCell: TableCell + Send + Sync;
	type Text: Text + Send + Sync;
	type EditableText: EditableText + Send + Sync;
	type Value: Value + Send + Sync;

	/// Creates an [`Self::Accessible`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation of.
	/// Generally, it fails if the accessible item does not implement to accessible interface.
	/// This shouldn't be possible, but this function may fail for other reasons.
	/// For example, to convert a [`zbus::Proxy`] into a [`Self::Accessible`], it may fail to create the new [`crate::accessible::AccessibleProxy`].
	async fn to_accessible(&self) -> Result<Self::Accessible, Self::Error>;
	/// Creates an [`Self::Action`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to action interface.
	async fn to_action(&self) -> Result<Self::Action, Self::Error>;
	/// Creates an [`Self::Application`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to application interface.
	async fn to_application(&self) -> Result<Self::Application, Self::Error>;
	/// Creates an [`Self::Collection`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to collection interface.
	async fn to_collection(&self) -> Result<Self::Collection, Self::Error>;
	/// Creates an [`Self::Component`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to component interface.
	async fn to_component(&self) -> Result<Self::Component, Self::Error>;
	async fn to_document(&self) -> Result<Self::Document, Self::Error>;
	async fn to_hypertext(&self) -> Result<Self::Hypertext, Self::Error>;
	async fn to_hyperlink(&self) -> Result<Self::Hyperlink, Self::Error>;
	async fn to_image(&self) -> Result<Self::Image, Self::Error>;
	async fn to_selection(&self) -> Result<Self::Selection, Self::Error>;
	async fn to_table(&self) -> Result<Self::Table, Self::Error>;
	async fn to_table_cell(&self) -> Result<Self::TableCell, Self::Error>;
	async fn to_text(&self) -> Result<Self::Text, Self::Error>;
	async fn to_editable_text(&self) -> Result<Self::EditableText, Self::Error>;
	async fn to_value(&self) -> Result<Self::Value, Self::Error>;
}

#[allow(clippy::module_name_repetitions)]
pub trait ConvertableBlocking {
	type Error: std::error::Error;
	type Accessible: AccessibleBlocking;
	type Action: ActionBlocking;
	type Application: ApplicationBlocking;
	type Collection: CollectionBlocking;
	type Component: ComponentBlocking;
	type Document: DocumentBlocking;
	type Hypertext: HypertextBlocking;
	type Hyperlink: HyperlinkBlocking;
	type Image: ImageBlocking;
	type Selection: SelectionBlocking;
	type Table: TableBlocking;
	type TableCell: TableCellBlocking;
	type Text: TextBlocking;
	type EditableText: EditableTextBlocking;
	type Value: ValueBlocking;

	/// Creates an [`Self::Accessible`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation of.
	/// Generally, it fails if the accessible item does not implement to accessible interface.
	/// This shouldn't be possible, but this function may fail for other reasons.
	/// For example, to convert a [`zbus::Proxy`] into a [`Self::Accessible`], it may fail to create the new [`crate::accessible::AccessibleProxyBlocking`].
	fn to_accessible(&self) -> Result<Self::Accessible, Self::Error>;
	/// Creates an [`Self::Action`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to action interface.
	fn to_action(&self) -> Result<Self::Action, Self::Error>;
	/// Creates an [`Self::Application`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to application interface.
	fn to_application(&self) -> Result<Self::Application, Self::Error>;
	/// Creates an [`Self::Collection`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to collection interface.
	fn to_collection(&self) -> Result<Self::Collection, Self::Error>;
	/// Creates an [`Self::Component`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to component interface.
	fn to_component(&self) -> Result<Self::Component, Self::Error>;
	/// Creates an [`Self::Document`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to document interface.
	fn to_document(&self) -> Result<Self::Document, Self::Error>;
	/// Creates an [`Self::Hypertext`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to hypertext interface.
	fn to_hypertext(&self) -> Result<Self::Hypertext, Self::Error>;
	/// Creates an [`Self::Hyperlink`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to hyperlink interface.
	fn to_hyperlink(&self) -> Result<Self::Hyperlink, Self::Error>;
	/// Creates an [`Self::Image`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to image interface.
	fn to_image(&self) -> Result<Self::Image, Self::Error>;
	/// Creates an [`Self::Selection`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to selection interface.
	fn to_selection(&self) -> Result<Self::Selection, Self::Error>;
	/// Creates an [`Self::Table`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to table interface.
	fn to_table(&self) -> Result<Self::Table, Self::Error>;
	/// Creates an [`Self::TableCell`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to table cell interface.
	fn to_table_cell(&self) -> Result<Self::TableCell, Self::Error>;
	/// Creates an [`Self::Text`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to text interface.
	fn to_text(&self) -> Result<Self::Text, Self::Error>;
	/// Creates an [`Self::EditableText`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to editable text interface.
	fn to_editable_text(&self) -> Result<Self::EditableText, Self::Error>;
	/// Creates an [`Self::Value`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to value interface.
	fn to_value(&self) -> Result<Self::Value, Self::Error>;
}

#[inline]
async fn convert_to_new_type<
	'a,
	'b,
	T: From<Proxy<'b>> + ProxyDefault + AtspiProxy,
	U: Deref<Target = Proxy<'a>> + ProxyDefault + AtspiProxy,
>(
	from: &U,
) -> zbus::Result<T> {
	// first thing is first, we need to create an accessible to query the interfaces.
	let accessible = AccessibleProxy::builder(from.connection())
		.destination(from.destination())?
		.cache_properties(CacheProperties::No)
		.path(from.path())?
		.build()
		.await?;
	// if the interface we're trying to convert to is not available as an interface; this can be problematic because the interface we're passing in could potentially be different from what we're converting to.
	if !accessible
		.get_interfaces()
		.await?
		.contains(<T as AtspiProxy>::INTERFACE)
	{
		return Err(Error::InterfaceNotFound);
	}
	// otherwise, make a new Proxy with the related type.
	let path = from.path().to_owned();
	let dest = from.destination().to_owned();
	ProxyBuilder::<'b, T>::new_bare(from.connection())
		.interface(<T as ProxyDefault>::INTERFACE)?
		.destination(dest)?
		.cache_properties(CacheProperties::No)
		.path(path)?
		.build()
		.await
}

#[inline]
fn convert_to_new_type_blocking<
	'a,
	'b,
	T: From<Proxy<'b>> + ProxyDefault + AtspiProxy,
	U: Deref<Target = ProxyBlocking<'a>> + ProxyDefault,
>(
	from: &U,
) -> zbus::Result<T> {
	// first thing is first, we need to create an accessible to query the interfaces.
	let accessible = AccessibleProxyBlocking::builder(from.connection())
		.destination(from.destination())?
		.cache_properties(CacheProperties::No)
		.path(from.path())?
		.build()?;
	// if the interface we're trying to convert to is not available as an interface; this can be problematic because the interface we're passing in could potentially be different from what we're converting to.
	if !accessible.get_interfaces()?.contains(<T as AtspiProxy>::INTERFACE) {
		return Err(Error::InterfaceNotFound);
	}
	// otherwise, make a new Proxy with the related type.
	let path = from.path().to_owned();
	let dest = from.destination().to_owned();
	ProxyBuilderBlocking::<'b, T>::new_bare(from.connection())
		.interface(<T as ProxyDefault>::INTERFACE)?
		.destination(dest)?
		.cache_properties(CacheProperties::No)
		.path(path)?
		.build()
}

#[async_trait]
impl<'a, T: Deref<Target = Proxy<'a>> + ProxyDefault + AtspiProxy + Sync> Convertable for T {
	type Error = zbus::Error;
	type Accessible = AccessibleProxy<'a>;
	type Action = ActionProxy<'a>;
	type Application = ApplicationProxy<'a>;
	type Collection = CollectionProxy<'a>;
	type Component = ComponentProxy<'a>;
	type Document = DocumentProxy<'a>;
	type Hypertext = HypertextProxy<'a>;
	type Hyperlink = HyperlinkProxy<'a>;
	type Image = ImageProxy<'a>;
	type Selection = SelectionProxy<'a>;
	type Table = TableProxy<'a>;
	type TableCell = TableCellProxy<'a>;
	type Text = TextProxy<'a>;
	type EditableText = EditableTextProxy<'a>;
	type Value = ValueProxy<'a>;
	/* no guard due to assumption it is always possible */
	async fn to_accessible(&self) -> zbus::Result<Self::Accessible> {
		convert_to_new_type(self).await
	}
	async fn to_action(&self) -> zbus::Result<Self::Action> {
		convert_to_new_type(self).await
	}
	async fn to_application(&self) -> zbus::Result<Self::Application> {
		convert_to_new_type(self).await
	}
	async fn to_collection(&self) -> zbus::Result<Self::Collection> {
		convert_to_new_type(self).await
	}
	async fn to_component(&self) -> zbus::Result<Self::Component> {
		convert_to_new_type(self).await
	}
	async fn to_document(&self) -> zbus::Result<Self::Document> {
		convert_to_new_type(self).await
	}
	async fn to_hypertext(&self) -> zbus::Result<Self::Hypertext> {
		convert_to_new_type(self).await
	}
	async fn to_hyperlink(&self) -> zbus::Result<Self::Hyperlink> {
		convert_to_new_type(self).await
	}
	async fn to_image(&self) -> zbus::Result<Self::Image> {
		convert_to_new_type(self).await
	}
	async fn to_selection(&self) -> zbus::Result<Self::Selection> {
		convert_to_new_type(self).await
	}
	async fn to_table(&self) -> zbus::Result<Self::Table> {
		convert_to_new_type(self).await
	}
	async fn to_table_cell(&self) -> zbus::Result<Self::TableCell> {
		convert_to_new_type(self).await
	}
	async fn to_text(&self) -> zbus::Result<Self::Text> {
		convert_to_new_type(self).await
	}
	async fn to_editable_text(&self) -> zbus::Result<Self::EditableText> {
		convert_to_new_type(self).await
	}
	async fn to_value(&self) -> zbus::Result<Self::Value> {
		convert_to_new_type(self).await
	}
}

impl<'a, T: Deref<Target = ProxyBlocking<'a>> + ProxyDefault + AtspiProxy> ConvertableBlocking
	for T
{
	type Error = zbus::Error;
	type Accessible = AccessibleProxyBlocking<'a>;
	type Action = ActionProxyBlocking<'a>;
	type Application = ApplicationProxyBlocking<'a>;
	type Collection = CollectionProxyBlocking<'a>;
	type Component = ComponentProxyBlocking<'a>;
	type Document = DocumentProxyBlocking<'a>;
	type Hypertext = HypertextProxyBlocking<'a>;
	type Hyperlink = HyperlinkProxyBlocking<'a>;
	type Image = ImageProxyBlocking<'a>;
	type Selection = SelectionProxyBlocking<'a>;
	type Table = TableProxyBlocking<'a>;
	type TableCell = TableCellProxyBlocking<'a>;
	type Text = TextProxyBlocking<'a>;
	type EditableText = EditableTextProxyBlocking<'a>;
	type Value = ValueProxyBlocking<'a>;
	/* no guard due to assumption it is always possible */
	fn to_accessible(&self) -> zbus::Result<Self::Accessible> {
		convert_to_new_type_blocking(self)
	}
	fn to_action(&self) -> zbus::Result<Self::Action> {
		convert_to_new_type_blocking(self)
	}
	fn to_application(&self) -> zbus::Result<Self::Application> {
		convert_to_new_type_blocking(self)
	}
	fn to_collection(&self) -> zbus::Result<Self::Collection> {
		convert_to_new_type_blocking(self)
	}
	fn to_component(&self) -> zbus::Result<Self::Component> {
		convert_to_new_type_blocking(self)
	}
	fn to_document(&self) -> zbus::Result<Self::Document> {
		convert_to_new_type_blocking(self)
	}
	fn to_hypertext(&self) -> zbus::Result<Self::Hypertext> {
		convert_to_new_type_blocking(self)
	}
	fn to_hyperlink(&self) -> zbus::Result<Self::Hyperlink> {
		convert_to_new_type_blocking(self)
	}
	fn to_image(&self) -> zbus::Result<Self::Image> {
		convert_to_new_type_blocking(self)
	}
	fn to_selection(&self) -> zbus::Result<Self::Selection> {
		convert_to_new_type_blocking(self)
	}
	fn to_table(&self) -> zbus::Result<Self::Table> {
		convert_to_new_type_blocking(self)
	}
	fn to_table_cell(&self) -> zbus::Result<Self::TableCell> {
		convert_to_new_type_blocking(self)
	}
	fn to_text(&self) -> zbus::Result<Self::Text> {
		convert_to_new_type_blocking(self)
	}
	fn to_editable_text(&self) -> zbus::Result<Self::EditableText> {
		convert_to_new_type_blocking(self)
	}
	fn to_value(&self) -> zbus::Result<Self::Value> {
		convert_to_new_type_blocking(self)
	}
}
