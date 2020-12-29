use malachite_test::common::DemoBenchRegistry;

pub mod slice_move_left;
pub mod split_into_chunks;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    slice_move_left::register(registry);
    split_into_chunks::register(registry);
}
