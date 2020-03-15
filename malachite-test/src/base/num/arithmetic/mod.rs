use common::DemoBenchRegistry;

pub mod log_two;
pub mod mod_is_reduced;
pub mod mod_power_of_two_is_reduced;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    log_two::register(registry);
    mod_is_reduced::register(registry);
    mod_power_of_two_is_reduced::register(registry);
}
