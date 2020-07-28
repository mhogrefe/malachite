use malachite_test::common::DemoBenchRegistry;

pub mod arithmetic;
pub mod comparison;
pub mod conversion;
pub mod logic;
pub mod random;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    arithmetic::register(registry);
    comparison::register(registry);
    conversion::register(registry);
    logic::register(registry);
    random::register(registry);
}
