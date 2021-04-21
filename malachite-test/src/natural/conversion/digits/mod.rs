use malachite_test::common::DemoBenchRegistry;

pub mod from_power_of_2_digits;
pub mod power_of_2_digits;
pub mod to_power_of_2_digits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    from_power_of_2_digits::register(registry);
    power_of_2_digits::register(registry);
    to_power_of_2_digits::register(registry);
}
