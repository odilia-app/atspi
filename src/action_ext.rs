use crate::action::{Action, ActionBlocking, ActionProxy, ActionProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait ActionExtError: crate::action::Action {
	type Error: std::error::Error;
}
pub trait ActionBlockingExtError: crate::action::ActionBlocking {
	type Error: std::error::Error;
}

pub trait ActionExt {}
pub trait ActionBlockingExt {}

impl<T: ActionExtError + crate::action::Action> ActionExt for T {}
impl<T: ActionBlockingExtError + crate::action::ActionBlocking> ActionBlockingExt for T {}

assert_impl_all!(ActionProxy: Action, ActionExt);
assert_impl_all!(ActionProxyBlocking: ActionBlocking, ActionBlockingExt);
