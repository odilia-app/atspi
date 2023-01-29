use crate::{
    accessible::{AccessibleProxy, Accessible},
		action::{Action, ActionProxy},
		application::{Application, ApplicationProxy},
    cache::{Cache, CacheProxy},
		collection::{Collection, CollectionProxy},
		component::{Component, ComponentProxy},
    device_event_controller::{DeviceEventController, DeviceEventControllerProxy},
    device_event_listener::{DeviceEventListener, DeviceEventListenerProxy},
		document::{Document, DocumentProxy},
    editable_text::{EditableText, EditableTextProxy},
		hyperlink::{Hyperlink, HyperlinkProxy},
		hypertext::{Hypertext, HypertextProxy},
    image::{Image, ImageProxy},
		registry::{Registry, RegistryProxy},
		selection::{Selection, SelectionProxy},
		table::{Table, TableProxy},
    table_cell::{TableCell, TableCellProxy},
		text::{Text, TextProxy},
		value::{Value, ValueProxy},
		AtspiProxy,
};
use async_trait::async_trait;
use std::ops::Deref;
use zbus::{CacheProperties, Error, Proxy, ProxyBuilder, ProxyDefault};

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
	type Cache: Cache + Send + Sync;
	type Value: Value + Send + Sync;
	type Registry: Registry + Send + Sync;
	type DeviceEventController: DeviceEventController + Send + Sync;
	type DeviceEventListener: DeviceEventListener + Send + Sync;

	async fn to_accessible(&self) -> Result<Self::Accessible, Self::Error>;
	async fn to_action(&self) -> Result<Self::Action, Self::Error>;
	async fn to_application(&self) -> Result<Self::Application, Self::Error>;
	async fn to_collection(&self) -> Result<Self::Collection, Self::Error>;
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
	async fn to_cache(&self) -> Result<Self::Cache, Self::Error>;
	async fn to_value(&self) -> Result<Self::Value, Self::Error>;
	async fn to_registry(&self) -> Result<Self::Registry, Self::Error>;
	async fn to_device_event_controller(&self) -> Result<Self::DeviceEventController, Self::Error>;
	async fn to_device_event_listener(&self) -> Result<Self::DeviceEventListener, Self::Error>;
}

#[inline]
async fn convert_to_new_type<
    'a,
    'b,
    T: From<Proxy<'b>> + ProxyDefault,
    U: Deref<Target = Proxy<'a>> + ProxyDefault + AtspiProxy,
>(
    from: &U,
) -> zbus::Result<T> {
    // first thing is first, we need to creat an accessible to query the interfaces.
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
        .contains(<U as AtspiProxy>::INTERFACE)
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
		type Cache = CacheProxy<'a>;
		type Value = ValueProxy<'a>;
		type Registry = RegistryProxy<'a>;
		type DeviceEventController = DeviceEventControllerProxy<'a>;
		type DeviceEventListener = DeviceEventListenerProxy<'a>;
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
    async fn to_cache(&self) -> zbus::Result<Self::Cache> {
        convert_to_new_type(self).await
    }
    async fn to_value(&self) -> zbus::Result<Self::Value> {
        convert_to_new_type(self).await
    }
    async fn to_registry(&self) -> zbus::Result<Self::Registry> {
        convert_to_new_type(self).await
    }
    async fn to_device_event_controller(&self) -> zbus::Result<Self::DeviceEventController> {
        convert_to_new_type(self).await
    }
    async fn to_device_event_listener(&self) -> zbus::Result<Self::DeviceEventListener> {
        convert_to_new_type(self).await
    }
}
