use malachite_test::common::DemoBenchRegistry;

pub mod num;
pub mod slices;
pub mod vecs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    num::register(registry);
    slices::register(registry);
    vecs::register(registry);
}
