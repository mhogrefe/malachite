use common::DemoBenchRegistry;

pub mod not_assign;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    not_assign::register(registry);
}
