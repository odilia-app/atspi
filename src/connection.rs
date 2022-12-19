use std::ops::Deref;

use futures::stream::{Stream, StreamExt};
use zbus::{
    names::InterfaceName, zvariant::Signature, Address, Message, MessageStream, MessageType,
};

use crate::{bus::BusProxy, events::Event, registry::RegistryProxy};

// Event body signatures: These outline the event specific deserialized event type.
// Safety: These are evaluated at compile time, the downstream user will not unwrap at runtime.
// ----
// The signal signature "(so)" (an Accessible) is ambiguous, because it is used in:
// -  Cache : RemoveAccessible
// -  Socket: Available
//  Both specify an `Accessible`, however their purpose is different.
//
// These two are separated streams, grouping these two semantically little sense.
//
// ATSPI- and QSPI both describe generic events. These can be converted into
// specific signal typs with TryFrom implementations.
//  AVAILABLE signals the availability of the `Registry` daeomon.
//  EVENT_LISTENER is a type signature used to notify when events are registered or deregistered.
//  CACHE_ADD and *_REMOVE have very different types
//  DEVICE_EVENT marks a type for both registerering and deregistering device events (? citation needed)
const ATSPI_EVENT: Signature = Signature::from_static_str_unchecked("siiva{sv}");
const QSPI_EVENT: Signature = Signature::from_static_str_unchecked("siiv(so)");
const AVAILABLE: Signature = Signature::from_static_str_unchecked("(so)");
const EVENT_LISTENER: Signature = Signature::from_static_str_unchecked("(ss)");
const CACHE_ADD: Signature = Signature::from_static_str_unchecked("((so)(so)(so)iiassusau)");
const CACHE_REM: Signature = Signature::from_static_str_unchecked("(so)");
const DEVICE_EVENT: Signature = Signature::from_static_str_unchecked("(souua(iisi)u(bbb))");

/// A connection to the at-spi bus
///
/// A number of types identified on ther bus:
///
pub struct Connection {
    registry: RegistryProxy<'static>,
}
pub fn valid_msg(msg: Message, sig: Signature) -> bool {
    msg.body_signature() == Ok(sig) && msg.primary_header().msg_type() == MessageType::Signal
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

    /// Stream yielding `AtspiEvent` types.
    ///
    /// Monitor this stream to be notified and receive `QspiEvent` events.
    ///
    /// # Example
    /// ```
    /// todo!()
    /// ```
    pub fn atspi_events(&self) -> impl Stream<Item = zbus::Result<Event>> {
        MessageStream::from(self.registry.connection()).filter_map(|res| async move {
            let msg = match res {
                Ok(m) => m,
                Err(e) => return Some(Err(e)),
            };
            let valid = || {
                (*msg).body_signature() == Ok(ATSPI_EVENT)
                    && msg.primary_header().msg_type() == MessageType::Signal
            };
            if valid() {
                Some(crate::events::Event::try_from(msg))
            } else {
                None
            }
        })
    }

    /// Stream yielding `QspiEvent` types.
    ///
    /// Monitor this stream to be notified and receive `QspiEvent` events.
    ///
    /// # Example
    /// ```
    /// todo!()
    /// ```
    pub fn qtspi_events(&self) -> impl Stream<Item = zbus::Result<Event>> {
        MessageStream::from(self.registry.connection()).filter_map(|res| async move {
            let msg = match res {
                Ok(m) => m,
                Err(e) => return Some(Err(e)),
            };
            let valid = || {
                (*msg).body_signature() == Ok(QSPI_EVENT)
                    && msg.primary_header().msg_type() == MessageType::Signal
            };
            if valid() {
                Some(crate::events::Event::try_from(msg))
            } else {
                None
            }
        })
    }

    /// Stream yielding `CacheAdd` event types.
    ///
    /// Monitor this stream to be notified of `CacheAdd` events.
    ///
    /// # Example
    /// ```
    /// todo!()
    /// ```
    pub fn cache_added_events(&self) -> impl Stream<Item = zbus::Result<Event>> {
        MessageStream::from(self.registry.connection()).filter_map(|res| async move {
            let msg = match res {
                Ok(m) => m,
                Err(e) => return Some(Err(e)),
            };
            let valid = || {
                (*msg).body_signature() == Ok(CACHE_ADD)
                    && msg.primary_header().msg_type() == MessageType::Signal
            };
            if valid() {
                Some(crate::events::Event::try_from(msg))
            } else {
                None
            }
        })
    }

    /// Stream yielding `CacheRemove` events.
    ///
    /// You want to monitor this stream if you want to be notified of `CacheRemove` events.
    ///
    /// # Example
    /// ```
    /// todo!()
    /// ```
    pub fn cache_removed_events(&self) -> impl Stream<Item = zbus::Result<Event>> {
        MessageStream::from(self.registry.connection()).filter_map(|res| async move {
            let msg = match res {
                Ok(m) => m,
                Err(e) => return Some(Err(e)),
            };
            let msg_header = match msg.header() {
                Ok(hdr) => hdr,
                Err(e) => return Some(Err(e)),
            };
            if msg_header.interface()
                != Ok(Some(&InterfaceName::from_static_str("org.a11y.atspi.Socket").unwrap()))
            {
                return None;
            }
            let valid = || {
                (*msg).body_signature() == Ok(CACHE_REM)
                    && msg.primary_header().msg_type() == MessageType::Signal
            };
            if valid() {
                Some(crate::events::Event::try_from(msg))
            } else {
                None
            }
        })
    }

    /// Stream yielding `DeviceEvent`s.
    ///
    /// This yields events of both `Register` and `Deregister` kinds.
    /// You want to monitor this stream if you want to be notified of these events.
    ///
    /// # Example
    /// ```
    /// todo!()
    /// ```
    pub fn device_events(&self) -> impl Stream<Item = zbus::Result<Event>> {
        MessageStream::from(self.registry.connection()).filter_map(|res| async move {
            let msg = match res {
                Ok(m) => m,
                Err(e) => return Some(Err(e)),
            };
            let valid = || {
                (*msg).body_signature() == Ok(DEVICE_EVENT)
                    && msg.primary_header().msg_type() == MessageType::Signal
            };
            if valid() {
                Some(crate::events::Event::try_from(msg))
            } else {
                None
            }
        })
    }

    /// Stream yielding the `Available` bus types.
    ///
    ///  The `Registry` interface,provided by the registry daemon,
    /// becomes available on the a11y bus.
    /// The registry daemon emits this signal upon startup.
    ///
    /// Monitor this stream to be notified of bus registry availability
    /// and receive corresponding `Available` events.
    ///
    /// # Example
    /// ```
    /// todo!()
    /// ```
    pub fn registry_available(&self) -> impl Stream<Item = zbus::Result<Event>> {
        MessageStream::from(self.registry.connection()).filter_map(|res| async move {
            let msg = match res {
                Ok(m) => m,
                Err(e) => return Some(Err(e)),
            };
            let msg_header = match msg.header() {
                Ok(hdr) => hdr,
                Err(e) => return Some(Err(e)),
            };
            if msg_header.interface()
                != Ok(Some(&InterfaceName::from_static_str("org.a11y.atspi.Socket").unwrap()))
            // TODO: Static InterfaceName to avoid unwrap please
            {
                return None;
            }
            let valid = || {
                (*msg).body_signature() == Ok(AVAILABLE)
                    && msg.primary_header().msg_type() == MessageType::Signal
            };
            if valid() {
                // TODO: Create Deserialized type that fits this stream (so)
                Some(crate::events::Event::try_from(msg))
            } else {
                None
            }
        })
    }

    /// Stream yielding the `EventListenerRegister` and `EventListenerDeregister` bus types.
    ///
    /// Monitor this stream to be notified of bus `EventListenerRegister` and
    /// `EventListenerDeregister` tyoes.
    ///
    /// # Example
    /// ```
    /// todo!()
    /// ```
    pub fn event_listener_events(&self) -> impl Stream<Item = zbus::Result<Event>> {
        MessageStream::from(self.registry.connection()).filter_map(|res| async move {
            let msg = match res {
                Ok(m) => m,
                Err(e) => return Some(Err(e)),
            };
            let __msg_header = match msg.header() {
                Ok(hdr) => hdr,
                Err(e) => return Some(Err(e)),
            };
            let valid = || {
                (*msg).body_signature() == Ok(EVENT_LISTENER)
                    && msg.primary_header().msg_type() == MessageType::Signal
            };
            if valid() {
                // TODO: Create Deserialized type that fits this stream (ss)
                Some(crate::events::Event::try_from(msg))
            } else {
                None
            }
        })
    }

    // pub fn compound_event_stream(&self) -> impl Stream<Item = zbus::Result<AtspiEvent>> {
    //     MessageStream::from(self.registry.connection()).filter_map(|res| async move {
    //         let msg = match res {
    //             Ok(m) => m,
    //             Err(e) => return Some(Err(e)),
    //         };
    //         if msg.header().ok()?.primary().msg_type() == MessageType::Signal {
    //             Some(AtspiEvent::try_from(msg))
    //         } else {
    //             None
    //         }
    //     })
    // }
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
