pub trait ActionExtError: crate::action::Action {
	type Error: std::error::Error;
}

pub trait ActionExt {
}

impl<T: ActionExtError + crate::action::Action> ActionExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	action_ext::ActionExt,
	action::{ActionProxy,
	ActionProxyBlocking}};	fn implements_action_ext<T: ActionExt>() {}
	#[test]
	fn check_action_implements_action_ext() {
		implements_action_ext::<ActionProxy<'static>>();
	}
	#[test]
	fn check_blocking_action_implements_action_ext() {
		implements_action_ext::<ActionProxyBlocking<'static>>();
	}
}