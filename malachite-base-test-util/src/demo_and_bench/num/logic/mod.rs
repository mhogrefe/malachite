use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    bit_access::register(runner);
    get_highest_bit::register(runner);
    significant_bits::register(runner);
}

mod bit_access;
mod get_highest_bit;
mod significant_bits;
