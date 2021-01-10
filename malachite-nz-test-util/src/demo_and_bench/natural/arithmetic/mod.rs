use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    neg::register(runner);
}

mod neg;
