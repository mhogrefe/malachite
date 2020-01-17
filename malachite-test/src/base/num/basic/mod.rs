use common::DemoBenchRegistry;

pub mod crement;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    crement::register(registry);
}
