use malachite_test::common::DemoBenchRegistry;

pub mod crement;
pub mod iverson;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    crement::register(registry);
    iverson::register(registry);
}
