use malachite_test::common::DemoBenchRegistry;

pub mod power_of_two_digit_iterable;
pub mod power_of_two_digits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    power_of_two_digit_iterable::register(registry);
    power_of_two_digits::register(registry);
}
