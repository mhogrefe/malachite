use malachite_test::common::DemoBenchRegistry;

pub mod arithmetic;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    arithmetic::register(registry);
}
