use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    checked_from_and_exact_from::register(runner);
    convertible_from::register(runner);
    overflowing_from::register(runner);
    saturating_from::register(runner);
    wrapping_from::register(runner);
}

pub mod checked_from_and_exact_from;
pub mod convertible_from;
pub mod overflowing_from;
pub mod saturating_from;
pub mod wrapping_from;
