use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    log_base::register(runner);
    log_base_2::register(runner);
    log_base_power_of_2::register(runner);
    mul::register(runner);
    neg::register(runner);
}

mod log_base;
mod log_base_2;
mod log_base_power_of_2;
mod mul;
mod neg;
