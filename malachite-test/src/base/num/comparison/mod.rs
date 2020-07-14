use malachite_test::common::DemoBenchRegistry;

pub mod abs_comparators;
pub mod cmp_abs_and_partial_cmp_abs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    abs_comparators::register(registry);
    cmp_abs_and_partial_cmp_abs::register(registry);
}
