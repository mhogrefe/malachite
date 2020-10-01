use malachite_test::common::DemoBenchRegistry;

pub mod string_is_subset;
pub mod string_sort;
pub mod string_unique;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    string_is_subset::register(registry);
    string_sort::register(registry);
    string_unique::register(registry);
}
