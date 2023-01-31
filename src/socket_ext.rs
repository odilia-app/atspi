pub trait SocketExtError: crate::socket::Socket {
	type Error: std::error::Error;
}

pub trait SocketExt {
}

impl<T: SocketExtError + crate::socket::Socket> SocketExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	socket_ext::SocketExt,
	socket::{SocketProxy,
	SocketProxyBlocking}};	fn implements_socket_ext<T: SocketExt>() {}
	#[test]
	fn check_socket_implements_socket_ext() {
		implements_socket_ext::<SocketProxy<'static>>();
	}
	#[test]
	fn check_blocking_socket_implements_socket_ext() {
		implements_socket_ext::<SocketProxyBlocking<'static>>();
	}
}