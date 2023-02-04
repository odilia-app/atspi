#[allow(clippy::module_name_repetitions)]
pub trait ActionExtError: crate::action::Action {
	type Error: std::error::Error;
}
pub trait ActionBlockingExtError: crate::action::ActionBlocking {
	type Error: std::error::Error;
}

pub trait ActionExt {
}
pub trait ActionBlockingExt {
}

impl<T: ActionExtError + crate::action::Action> ActionExt for T {
}
impl<T: ActionBlockingExtError + crate::action::ActionBlocking> ActionBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    action_ext::{
      ActionExt,
      ActionBlockingExt,
    },
    action::{
      ActionProxy,
      ActionProxyBlocking,
    },
  };
  fn implements_action_ext<T: ActionExt>() {}
  fn implements_action_blocking_ext<T: ActionBlockingExt>() {}
	#[test]
	fn check_action_implements_action_ext() {
		implements_action_ext::<ActionProxy<'static>>();
	}
	#[test]
	fn check_blocking_action_implements_action_ext() {
		implements_action_blocking_ext::<ActionProxyBlocking<'static>>();
	}
}
