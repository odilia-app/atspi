pub trait SocketExtError: crate::socket::Socket {
	type Error: std::error::Error;
}
pub trait SocketBlockingExtError: crate::socket::SocketBlocking {
	type Error: std::error::Error;
}

pub trait SocketExt {
}
pub trait SocketBlockingExt {
}

impl<T: SocketExtError + crate::socket::Socket> SocketExt for T {
}
impl<T: SocketBlockingExtError + crate::socket::SocketBlocking> SocketBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    socket_ext::{
      SocketExt,
      SocketBlockingExt,
    },
    socket::{
      SocketProxy,
      SocketProxyBlocking,
    },
  };
  fn implements_socket_ext<T: SocketExt>() {}
  fn implements_socket_blocking_ext<T: SocketBlockingExt>() {}
	#[test]
	fn check_socket_implements_socket_ext() {
		implements_socket_ext::<SocketProxy<'static>>();
	}
	#[test]
	fn check_blocking_socket_implements_socket_ext() {
		implements_socket_blocking_ext::<SocketProxyBlocking<'static>>();
	}
}
