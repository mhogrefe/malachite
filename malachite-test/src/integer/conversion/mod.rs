use common::DemoBenchRegistry;

pub mod clone;
pub mod floating_point_from_integer;
pub mod from_floating_point;
pub mod from_natural;
pub mod from_primitive_integer;
pub mod from_twos_complement_limbs;
pub mod natural_from_integer;
pub mod primitive_integer_from_integer;
pub mod serde;
pub mod to_twos_complement_limbs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    clone::register(registry);
    floating_point_from_integer::register(registry);
    from_floating_point::register(registry);
    from_natural::register(registry);
    from_primitive_integer::register(registry);
    from_twos_complement_limbs::register(registry);
    natural_from_integer::register(registry);
    primitive_integer_from_integer::register(registry);
    serde::register(registry);
    to_twos_complement_limbs::register(registry);
}
