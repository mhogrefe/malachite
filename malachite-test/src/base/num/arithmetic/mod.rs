use common::DemoBenchRegistry;

pub mod log_two;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    log_two::register(registry);
}
