use malachite_test::common::DemoBenchRegistry;

pub mod arithmetic;
pub mod logic;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    arithmetic::register(registry);
    logic::register(registry);
}
