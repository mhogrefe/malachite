use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_digits::register(runner);
    from_power_of_2_digits::register(runner);
    to_digits::register(runner);
    to_power_of_2_digits::register(runner);
}

mod from_digits;
mod from_power_of_2_digits;
mod to_digits;
mod to_power_of_2_digits;
