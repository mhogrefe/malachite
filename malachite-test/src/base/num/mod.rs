use malachite_test::common::DemoBenchRegistry;

pub mod arithmetic;
pub mod conversion;
pub mod logic;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    arithmetic::register(registry);
    conversion::register(registry);
    logic::register(registry);
}
