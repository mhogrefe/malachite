use malachite_test::common::DemoBenchRegistry;

pub mod split_into_chunks;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    split_into_chunks::register(registry);
}
