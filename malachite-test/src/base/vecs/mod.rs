use malachite_test::common::DemoBenchRegistry;

pub mod vec_delete_left;
pub mod vec_pad_left;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    vec_delete_left::register(registry);
    vec_pad_left::register(registry);
}
