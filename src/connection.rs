use std::ops::Deref;

use futures::stream::{Stream, StreamExt};
use zbus::{Address, MessageStream};

use crate::{bus::BusProxy, events::Event, registry::RegistryProxy};

/// A connection to the at-spi bus
pub struct Connection {
    registry: RegistryProxy<'static>,
}

impl Connection {
    /// Open a new connection to the bus
    #[tracing::instrument]
    pub async fn open() -> zbus::Result<Self> {
        // Grab the a11y bus address from the session bus
        let a11y_bus_addr = {
            tracing::debug!("Connecting to session bus");
            let session_bus = zbus::Connection::session().await?;
            tracing::debug!(
                name = session_bus.unique_name().map(|n| n.as_str()),
                "Connected to session bus"
            );
            let proxy = BusProxy::new(&session_bus).await?;
            tracing::debug!("Getting a11y bus address from session bus");
            proxy.get_address().await?
        };
        tracing::debug!(address = %a11y_bus_addr, "Got a11y bus address");
        let addr: Address = a11y_bus_addr.parse()?;
        Self::connect(addr).await
    }

    pub async fn connect(bus_addr: Address) -> zbus::Result<Self> {
        tracing::debug!("Connecting to a11y bus");
        let bus = zbus::ConnectionBuilder::address(bus_addr)?.build().await?;
        tracing::debug!(name = bus.unique_name().map(|n| n.as_str()), "Connected to a11y bus");
        // The Proxy holds a strong reference to a Connection, so we only need to store the proxy
        let registry = RegistryProxy::new(&bus).await?;

        Ok(Self { registry })
    }

    pub fn event_stream(&self) -> impl Stream<Item = zbus::Result<Event>> {
        MessageStream::from(self.registry.connection()).filter_map(|res| async move {
            let msg = match res {
                Ok(m) => m,
                Err(e) => return Some(Err(e)),
            };
            if msg.interface()?.starts_with("org.a11y.atspi.Event.") {
                Some(Event::try_from(msg))
            } else {
                None
            }
        })
    }
}

impl Deref for Connection {
    type Target = RegistryProxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.registry
    }
}

/// Set the `IsEnabled` property in the session bus.
/// 
/// Assistive Technology provider applications (ATs) should set the accessibility
/// `IsEnabled` status on the users session bus on startup as applications may monitor this property
/// to  enable their accessibility support dynamically.
/// 
/// See: The [freedesktop - AT-SPI2 wiki](https://www.freedesktop.org/wiki/Accessibility/AT-SPI2/) 
/// 
///  ## Example
/// ```rust
///     use futures_lite::future::block_on;
/// 
///     let result =  block_on( atspi::set_session_accessibility(true) );
///     assert!(result.is_ok());
/// ```
///  ## Errors
/// * when no connection with the session bus can be established,
/// * if creation of a [`crate::bus::StatusProxy`] fails
/// * if the `IsEnabled` property cannot be read
/// * the `IsEnabled` property cannot be set.
pub async fn set_session_accessibility(
    status: bool,
) -> std::result::Result<(), zbus::Error> {
    // Get a connection to the session bus.
    let session = zbus::Connection::session().await?;
    
    // Aqcuire a `StatusProxy` for the session bus.
    let status_proxy = crate::bus::StatusProxy::new(&session).await?;

    if status_proxy.is_enabled().await? != status {
        status_proxy.set_is_enabled(status).await?;
    }
    Ok(())
}
