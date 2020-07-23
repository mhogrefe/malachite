use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    get_highest_bit::register(runner);
    significant_bits::register(runner);
}

pub mod get_highest_bit;
pub mod significant_bits;
