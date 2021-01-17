use malachite_test::common::DemoBenchRegistry;

pub mod num;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    num::register(registry);
}
