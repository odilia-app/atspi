use atspi_proxies::socket::{Socket, SocketBlocking, SocketProxy, SocketProxyBlocking};

impl_extended_errors!(SocketProxy<'_>, SocketExtError);
impl_extended_errors!(SocketProxyBlocking<'_>, SocketBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait SocketExtError: Socket {
	type Error: std::error::Error;
}
pub trait SocketBlockingExtError: SocketBlocking {
	type Error: std::error::Error;
}

pub trait SocketExt {}
pub trait SocketBlockingExt {}

impl<T: SocketExtError + Socket> SocketExt for T {}
impl<T: SocketBlockingExtError + SocketBlocking> SocketBlockingExt for T {}

