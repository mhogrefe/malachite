use common::DemoBenchRegistry;

pub mod clone;
pub mod double_limb_from_natural;
pub mod floating_point_from_natural;
pub mod from_bits;
pub mod from_floating_point;
pub mod from_limbs;
pub mod from_primitive_integer;
pub mod limb_from_natural;
pub mod serde;
pub mod to_bits;
pub mod to_limbs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    clone::register(registry);
    double_limb_from_natural::register(registry);
    floating_point_from_natural::register(registry);
    from_bits::register(registry);
    from_floating_point::register(registry);
    from_limbs::register(registry);
    from_primitive_integer::register(registry);
    limb_from_natural::register(registry);
    serde::register(registry);
    to_bits::register(registry);
    to_limbs::register(registry);
}
