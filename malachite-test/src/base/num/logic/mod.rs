use common::DemoBenchRegistry;

pub mod get_highest_bit;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    get_highest_bit::register(registry);
}
