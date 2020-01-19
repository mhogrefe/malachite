use common::DemoBenchRegistry;

pub mod bit_access;
pub mod bit_scan;
pub mod get_highest_bit;
pub mod hamming_distance;
pub mod not_assign;
pub mod significant_bits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    bit_access::register(registry);
    bit_scan::register(registry);
    get_highest_bit::register(registry);
    hamming_distance::register(registry);
    not_assign::register(registry);
    significant_bits::register(registry);
}
