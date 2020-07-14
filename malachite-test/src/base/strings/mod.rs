use malachite_test::common::DemoBenchRegistry;

pub mod string_is_subset;
pub mod string_nub;
pub mod string_sort;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    string_is_subset::register(registry);
    string_nub::register(registry);
    string_sort::register(registry);
}
