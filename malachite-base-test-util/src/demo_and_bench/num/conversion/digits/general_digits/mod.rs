use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    to_digits::register(runner);
}

mod to_digits;
