use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    log_base_2::register(runner);
    mul::register(runner);
    neg::register(runner);
}

mod log_base_2;
mod mul;
mod neg;
