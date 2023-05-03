#[macro_export]

macro_rules! impl_to_dbus_message {
	($type:ty) => {
		impl TryFrom<$type> for zbus::Message {
			type Error = AtspiError;
			fn try_from(event: $type) -> Result<Self, Self::Error> {
				Ok(zbus::MessageBuilder::signal(
					event.path(),
					<$type as GenericEvent>::DBUS_INTERFACE,
					<$type as GenericEvent>::DBUS_MEMBER,
				)?
				.sender(event.sender())?
				.build(&((event.body()),))?)
			}
		}
	}
}
macro_rules! impl_from_dbus_message {
	($type:ty) => {
		impl TryFrom<&zbus::Message> for $type {
			type Error = AtspiError;
			fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
				Ok(<$type>::build( msg.try_into()?, msg.body::<<$type as GenericEvent>::Body>()? ))
			}
		}
	}
}

pub(crate) use impl_from_dbus_message;
pub(crate) use impl_to_dbus_message;
