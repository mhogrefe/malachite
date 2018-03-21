use common::DemoBenchRegistry;

pub mod eq;
pub mod hash;
pub mod ord;
pub mod ord_abs;
pub mod partial_ord_abs_i32;
pub mod partial_ord_abs_natural;
pub mod partial_ord_abs_u32;
pub mod partial_ord_i32;
pub mod partial_ord_natural;
pub mod partial_ord_u32;
pub mod partial_eq_i32;
pub mod partial_eq_natural;
pub mod partial_eq_u32;
pub mod sign;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    eq::register(registry);
    hash::register(registry);
    ord::register(registry);
    ord_abs::register(registry);
    partial_ord_abs_i32::register(registry);
    partial_ord_abs_natural::register(registry);
    partial_ord_abs_u32::register(registry);
    partial_ord_i32::register(registry);
    partial_ord_natural::register(registry);
    partial_ord_u32::register(registry);
    partial_eq_i32::register(registry);
    partial_eq_natural::register(registry);
    partial_eq_u32::register(registry);
    sign::register(registry);
}
