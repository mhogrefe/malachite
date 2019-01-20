use common::DemoBenchRegistry;

pub mod eq;
pub mod hash;
pub mod ord;
pub mod ord_abs;
pub mod partial_eq_limb;
pub mod partial_eq_natural;
pub mod partial_eq_signed_limb;
pub mod partial_ord_abs_limb;
pub mod partial_ord_abs_natural;
pub mod partial_ord_abs_signed_limb;
pub mod partial_ord_limb;
pub mod partial_ord_natural;
pub mod partial_ord_signed_limb;
pub mod sign;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    eq::register(registry);
    hash::register(registry);
    ord::register(registry);
    ord_abs::register(registry);
    partial_ord_abs_limb::register(registry);
    partial_ord_abs_natural::register(registry);
    partial_ord_abs_signed_limb::register(registry);
    partial_ord_limb::register(registry);
    partial_ord_natural::register(registry);
    partial_ord_signed_limb::register(registry);
    partial_eq_limb::register(registry);
    partial_eq_natural::register(registry);
    partial_eq_signed_limb::register(registry);
    sign::register(registry);
}
