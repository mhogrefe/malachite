use malachite_test::common::DemoBenchRegistry;

pub mod macros;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    macros::register(registry);
}
