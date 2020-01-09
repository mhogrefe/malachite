use common::DemoBenchRegistry;

pub mod eq;
pub mod hash;
pub mod ord;
pub mod partial_eq_primitive_integer;
pub mod partial_ord_limb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    eq::register(registry);
    hash::register(registry);
    ord::register(registry);
    partial_eq_primitive_integer::register(registry);
    partial_ord_limb::register(registry);
}
