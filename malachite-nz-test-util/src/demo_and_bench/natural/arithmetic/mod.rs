use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    mul::register(runner);
    neg::register(runner);
}

mod mul;
mod neg;
