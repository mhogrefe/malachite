use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_power_of_two_digits::register(runner);
    power_of_two_digit_iterable::register(runner);
    to_power_of_two_digits::register(runner);
}

mod from_power_of_two_digits;
mod power_of_two_digit_iterable;
mod to_power_of_two_digits;
