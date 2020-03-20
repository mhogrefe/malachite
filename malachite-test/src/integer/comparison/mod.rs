use common::DemoBenchRegistry;

pub mod eq;
pub mod hash;
pub mod ord;
pub mod ord_abs;
pub mod partial_eq_natural;
pub mod partial_eq_primitive_integer;
pub mod partial_ord_abs_natural;
pub mod partial_ord_abs_natural_comparators;
pub mod partial_ord_abs_primitive_integer;
pub mod partial_ord_abs_primitive_integer_comparators;
pub mod partial_ord_natural;
pub mod partial_ord_primitive_integer;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    eq::register(registry);
    hash::register(registry);
    ord::register(registry);
    ord_abs::register(registry);
    partial_ord_abs_natural::register(registry);
    partial_ord_abs_natural_comparators::register(registry);
    partial_ord_abs_primitive_integer::register(registry);
    partial_ord_natural::register(registry);
    partial_ord_primitive_integer::register(registry);
    partial_ord_abs_primitive_integer_comparators::register(registry);
    partial_eq_natural::register(registry);
    partial_eq_primitive_integer::register(registry);
}
