use common::DemoBenchRegistry;

pub mod wrapping_from;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    wrapping_from::register(registry);
}
