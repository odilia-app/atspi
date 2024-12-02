use core::ops::Deref;
use zbus_names::{InterfaceName, MemberName};
use zvariant::Signature;
use zbus::Message;

#[repr(transparent)]
pub struct ValidInterfaceMessage<'a>(&'a Message);
impl<'a> Deref for ValidInterfaceMessage<'a> {
    type Target = Message;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> ValidInterfaceMessage<'a> {
    fn validate<T>(zbm: &'a Message, interface: T) -> Option<Self> 
    where for<'b> &'b InterfaceName<'b>: PartialEq<T> {
        let header = zbm.header();
        let Some(int) = header.interface() else {
            return None;
        };
        if int != interface {
            return None;
        }
        Some(ValidInterfaceMessage(zbm))
    }
}
#[repr(transparent)]
pub struct ValidMemberMessage<'a>(&'a Message);
impl<'a> Deref for ValidMemberMessage<'a> {
    type Target = Message;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> ValidMemberMessage<'a> {
    fn validate<T>(zbm: &'a ValidInterfaceMessage<'a>, member: T) -> Option<Self> 
    where for<'b> &'b MemberName<'b>: PartialEq<T> {
        let header = zbm.0.header();
        let Some(mem) = header.member() else {
            return None;
        };
        if mem != member {
            return None;
        }
        Some(ValidMemberMessage(zbm.0))
    }
}

#[repr(transparent)]
pub struct ValidBodySigMessage<'a>(&'a Message);
impl<'a> Deref for ValidBodySigMessage<'a> {
    type Target = Message;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> ValidBodySigMessage<'a> {
    fn validate<T>(zbm: &'a ValidMemberMessage<'a>, body_sig: T) -> Option<Self> 
    where for<'b> &'b Signature<'b>: PartialEq<T> {
        let header = zbm.0.header();
        let Some(sig) = header.signature() else {
            return None;
        };
        if sig != body_sig {
            return None;
        }
        Some(ValidBodySigMessage(zbm.0))
    }
}

