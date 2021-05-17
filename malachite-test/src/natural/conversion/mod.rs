use malachite_test::common::DemoBenchRegistry;

pub mod clone;
pub mod from_limbs;
pub mod primitive_int_from_natural;
pub mod to_limbs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    clone::register(registry);
    from_limbs::register(registry);
    primitive_int_from_natural::register(registry);
    to_limbs::register(registry);
}
