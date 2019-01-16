use common::DemoBenchRegistry;

#[cfg(feature = "32_bit_limbs")]
pub mod assign_double_limb;
pub mod assign_limb;
pub mod assign_natural;
#[cfg(feature = "32_bit_limbs")]
pub mod assign_signed_double_limb;
pub mod assign_signed_limb;
pub mod clone_and_assign;
pub mod double_limb_from_integer;
pub mod from_double_limb;
pub mod from_limb;
pub mod from_natural;
pub mod from_sign_and_limbs;
pub mod from_signed_double_limb;
pub mod from_signed_limb;
pub mod from_twos_complement_bits;
pub mod from_twos_complement_limbs;
pub mod limb_from_integer;
pub mod natural_assign_integer;
pub mod natural_from_integer;
pub mod serde;
pub mod signed_double_limb_from_integer;
pub mod signed_limb_from_integer;
pub mod to_sign_and_limbs;
pub mod to_twos_complement_bits;
pub mod to_twos_complement_limbs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    assign_signed_limb::register(registry);
    #[cfg(feature = "32_bit_limbs")]
    assign_signed_double_limb::register(registry);
    assign_natural::register(registry);
    assign_limb::register(registry);
    #[cfg(feature = "32_bit_limbs")]
    assign_double_limb::register(registry);
    clone_and_assign::register(registry);
    from_signed_limb::register(registry);
    from_signed_double_limb::register(registry);
    from_natural::register(registry);
    from_sign_and_limbs::register(registry);
    from_twos_complement_bits::register(registry);
    from_twos_complement_limbs::register(registry);
    from_limb::register(registry);
    from_double_limb::register(registry);
    signed_limb_from_integer::register(registry);
    signed_double_limb_from_integer::register(registry);
    natural_assign_integer::register(registry);
    natural_from_integer::register(registry);
    serde::register(registry);
    to_sign_and_limbs::register(registry);
    to_twos_complement_bits::register(registry);
    to_twos_complement_limbs::register(registry);
    limb_from_integer::register(registry);
    double_limb_from_integer::register(registry);
}
