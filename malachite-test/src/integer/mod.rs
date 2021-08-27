use malachite_test::common::DemoBenchRegistry;

pub mod arithmetic;
pub mod comparison;
pub mod logic;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    arithmetic::register(registry);
    comparison::register(registry);
    logic::register(registry);
}
