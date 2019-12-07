use common::DemoBenchRegistry;

pub mod checked_from;
pub mod convertible_from;
pub mod overflowing_from;
pub mod saturating_from;
pub mod wrapping_from;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    checked_from::register(registry);
    convertible_from::register(registry);
    overflowing_from::register(registry);
    saturating_from::register(registry);
    wrapping_from::register(registry);
}
