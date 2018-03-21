use common::DemoBenchRegistry;

pub mod arithmetic;
pub mod basic;
pub mod comparison;
pub mod conversion;
pub mod logic;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    arithmetic::register(registry);
    basic::register(registry);
    comparison::register(registry);
    conversion::register(registry);
    logic::register(registry);
}
