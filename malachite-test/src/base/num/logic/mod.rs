use malachite_test::common::DemoBenchRegistry;

pub mod bit_access;
pub mod bit_block_access;
pub mod bit_convertible;
pub mod bit_iterable;
pub mod hamming_distance;
pub mod low_mask;
pub mod not;
pub mod rotate;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    bit_access::register(registry);
    bit_block_access::register(registry);
    bit_convertible::register(registry);
    bit_iterable::register(registry);
    hamming_distance::register(registry);
    low_mask::register(registry);
    not::register(registry);
    rotate::register(registry);
}
