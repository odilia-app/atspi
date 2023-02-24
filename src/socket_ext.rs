use crate::socket::{Socket, SocketBlocking, SocketProxy, SocketProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait SocketExtError: crate::socket::Socket {
	type Error: std::error::Error;
}
pub trait SocketBlockingExtError: crate::socket::SocketBlocking {
	type Error: std::error::Error;
}

pub trait SocketExt {}
pub trait SocketBlockingExt {}

impl<T: SocketExtError + crate::socket::Socket> SocketExt for T {}
impl<T: SocketBlockingExtError + crate::socket::SocketBlocking> SocketBlockingExt for T {}

assert_impl_all!(SocketProxy: Socket, SocketExt);
assert_impl_all!(SocketProxyBlocking: SocketBlocking, SocketBlockingExt);
