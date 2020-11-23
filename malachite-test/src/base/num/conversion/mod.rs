use malachite_test::common::DemoBenchRegistry;

pub mod from_other_type_slice;
pub mod vec_from_other_type_slice;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    from_other_type_slice::register(registry);
    vec_from_other_type_slice::register(registry);
}
