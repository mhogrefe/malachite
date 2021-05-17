use malachite_test::common::DemoBenchRegistry;

pub mod clone;
pub mod from_natural;
pub mod from_primitive_int;
pub mod from_twos_complement_limbs;
pub mod natural_from_integer;
pub mod primitive_int_from_integer;
pub mod to_twos_complement_limbs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    clone::register(registry);
    from_natural::register(registry);
    from_primitive_int::register(registry);
    from_twos_complement_limbs::register(registry);
    natural_from_integer::register(registry);
    primitive_int_from_integer::register(registry);
    to_twos_complement_limbs::register(registry);
}
