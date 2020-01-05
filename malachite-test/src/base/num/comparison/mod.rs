use common::DemoBenchRegistry;

pub mod cmp_abs_and_partial_cmp_abs;
pub mod comparators;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    cmp_abs_and_partial_cmp_abs::register(registry);
    comparators::register(registry);
}
