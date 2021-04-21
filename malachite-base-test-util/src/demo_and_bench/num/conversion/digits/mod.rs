use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    general_digits::register(runner);
    power_of_2_digits::register(runner);
}

mod general_digits;
mod power_of_2_digits;
