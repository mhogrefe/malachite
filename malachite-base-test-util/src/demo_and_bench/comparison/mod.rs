use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    macros::register(runner);
}

mod macros;
