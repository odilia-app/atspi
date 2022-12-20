use std::{error::Error, ops::Deref};

use futures::stream::{Stream, StreamExt};
use zbus::{
    names::InterfaceName, zvariant::Signature, Address, Message, MessageStream, MessageType,
};

use crate::{bus::BusProxy, events::Event, registry::RegistryProxy};

// Event body signatures: These outline the event specific deserialized event types.
// Safety: These are evaluated at compile time.
// ----
// The signal signature "(so)" (an Accessible) is ambiguous, because it is used in:
// -  Cache : RemoveAccessible
// -  Socket: Available
//
// ATSPI- and QSPI both describe the generic events. These can be converted into
// specific signal types with TryFrom implementations. See crate::[`identify`]
//  AVAILABLE signals the availability of the `Registry` daeomon.
//  EVENT_LISTENER is a type signature used to notify when events are registered or deregistered.
//  CACHE_ADD and *_REMOVE have very different types
//  DEVICE_EVENT marks a type for both registerering and deregistering device events (? citation needed)
pub const ATSPI_EVENT: Signature<'_> = Signature::from_static_str_unchecked("siiva{sv}");
pub const QSPI_EVENT: Signature<'_> = Signature::from_static_str_unchecked("siiv(so)");
pub const AVAILABLE: Signature<'_> = Signature::from_static_str_unchecked("(so)");
pub const EVENT_LISTENER: Signature<'_> = Signature::from_static_str_unchecked("(ss)");
pub const CACHE_ADD: Signature<'_> =
    Signature::from_static_str_unchecked("((so)(so)(so)iiassusau)");
pub const CACHE_REM: Signature<'_> = Signature::from_static_str_unchecked("(so)");
pub const DEVICE_EVENT: Signature<'_> = Signature::from_static_str_unchecked("(souua(iisi)u(bbb))");

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

    /// Stream yielding all `Event` types.
    ///
    /// Monitor this stream to be notified and receive events on the a11y bus.
    ///
    /// # Example
    /// ```
    /// todo!()
    /// ```
    pub fn event_stream(&self) -> impl Stream<Item = Result<Event, Box<dyn Error>>> {
        MessageStream::from(self.registry.connection()).filter_map(|res| async move {
            let msg = res
                .map_err(|e| return Some(Box::new(Err::<std::sync::Arc<Message>, zbus::Error>(e))))
                .ok()?;
            match msg.message_type() {
                // TOCO: Checkme: dot-ok is not-ok
                MessageType::Signal => Some(Event::try_from(msg)),
                _ => None,
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
pub async fn set_session_accessibility(status: bool) -> std::result::Result<(), zbus::Error> {
    // Get a connection to the session bus.
    let session = zbus::Connection::session().await?;

    // Aqcuire a `StatusProxy` for the session bus.
    let status_proxy = crate::bus::StatusProxy::new(&session).await?;

    if status_proxy.is_enabled().await? != status {
        status_proxy.set_is_enabled(status).await?;
    }
    Ok(())
}
