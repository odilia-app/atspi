use atspi_proxies::action::{Action, ActionBlocking, ActionProxy, ActionProxyBlocking};

impl_extended_errors!(ActionProxy<'_>, ActionExtError);
impl_extended_errors!(ActionProxyBlocking<'_>, ActionBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait ActionExtError: Action {
	type Error: std::error::Error;
}
pub trait ActionBlockingExtError: ActionBlocking {
	type Error: std::error::Error;
}

pub trait ActionExt {}
pub trait ActionBlockingExt {}

impl<T: ActionExtError + Action> ActionExt for T {}
impl<T: ActionBlockingExtError + ActionBlocking> ActionBlockingExt for T {}

assert_impl_all!(ActionProxy: Action, ActionExt);
assert_impl_all!(ActionProxyBlocking: ActionBlocking, ActionBlockingExt);
