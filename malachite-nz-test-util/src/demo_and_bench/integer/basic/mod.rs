use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_sign_and_abs::register(runner);
}

mod from_sign_and_abs;
