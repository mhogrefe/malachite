use malachite_test::common::DemoBenchRegistry;

pub mod comparison;
pub mod num;
pub mod rounding_modes;
pub mod slices;
pub mod strings;
pub mod vecs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    comparison::register(registry);
    num::register(registry);
    rounding_modes::register(registry);
    slices::register(registry);
    strings::register(registry);
    vecs::register(registry);
}
