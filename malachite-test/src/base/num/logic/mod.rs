use common::DemoBenchRegistry;

pub mod bit_access;
pub mod bit_block_access;
pub mod bit_convertible;
pub mod bit_iterable;
pub mod bit_scan;
pub mod get_highest_bit;
pub mod hamming_distance;
pub mod not;
pub mod power_of_two_digit_iterable;
pub mod power_of_two_digits;
pub mod rotate;
pub mod significant_bits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    bit_access::register(registry);
    bit_block_access::register(registry);
    bit_convertible::register(registry);
    bit_iterable::register(registry);
    bit_scan::register(registry);
    get_highest_bit::register(registry);
    hamming_distance::register(registry);
    not::register(registry);
    power_of_two_digits::register(registry);
    power_of_two_digit_iterable::register(registry);
    rotate::register(registry);
    significant_bits::register(registry);
}
