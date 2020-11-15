use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    checked_from_and_exact_from::register(runner);
    convertible_from::register(runner);
    join_halves::register(runner);
    overflowing_from::register(runner);
    saturating_from::register(runner);
    wrapping_from::register(runner);
}

mod checked_from_and_exact_from;
mod convertible_from;
mod join_halves;
mod overflowing_from;
mod saturating_from;
mod wrapping_from;
