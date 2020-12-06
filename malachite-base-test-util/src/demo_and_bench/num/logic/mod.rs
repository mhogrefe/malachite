use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    bit_access::register(runner);
    bit_convertible::register(runner);
    bit_scan::register(runner);
    get_highest_bit::register(runner);
    significant_bits::register(runner);
}

mod bit_access;
mod bit_convertible;
mod bit_scan;
mod get_highest_bit;
mod significant_bits;
