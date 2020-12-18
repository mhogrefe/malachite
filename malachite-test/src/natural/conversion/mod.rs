use malachite_test::common::DemoBenchRegistry;

pub mod clone;
pub mod digits;
pub mod floating_point_from_natural;
pub mod from_floating_point;
pub mod from_limbs;
pub mod primitive_int_from_natural;
pub mod serde;
pub mod to_limbs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    clone::register(registry);
    digits::register(registry);
    floating_point_from_natural::register(registry);
    from_floating_point::register(registry);
    from_limbs::register(registry);
    primitive_int_from_natural::register(registry);
    serde::register(registry);
    to_limbs::register(registry);
}
