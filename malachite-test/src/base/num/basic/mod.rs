use malachite_test::common::DemoBenchRegistry;

pub mod iverson;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    iverson::register(registry);
}
