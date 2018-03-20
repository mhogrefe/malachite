use common::DemoBenchRegistry;

pub mod decrement;
pub mod increment;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    decrement::register(registry);
    increment::register(registry);
}
