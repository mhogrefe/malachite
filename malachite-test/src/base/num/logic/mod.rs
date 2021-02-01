use malachite_test::common::DemoBenchRegistry;

pub mod bit_convertible;
pub mod bit_iterable;
pub mod hamming_distance;
pub mod low_mask;
pub mod not;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    bit_convertible::register(registry);
    bit_iterable::register(registry);
    hamming_distance::register(registry);
    low_mask::register(registry);
    not::register(registry);
}
