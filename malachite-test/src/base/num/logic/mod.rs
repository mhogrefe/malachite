use malachite_test::common::DemoBenchRegistry;

pub mod hamming_distance;
pub mod low_mask;
pub mod not;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    hamming_distance::register(registry);
    low_mask::register(registry);
    not::register(registry);
}
