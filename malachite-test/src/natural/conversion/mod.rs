use common::DemoBenchRegistry;

#[cfg(feature = "32_bit_limbs")]
pub mod assign_double_limb;
pub mod assign_limb;
pub mod clone_and_assign;
pub mod double_limb_from_natural;
pub mod from_bits;
pub mod from_double_limb;
pub mod from_floating_point;
pub mod from_limb;
pub mod from_limbs;
pub mod limb_from_natural;
pub mod serde;
pub mod to_bits;
pub mod to_limbs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    #[cfg(feature = "32_bit_limbs")]
    assign_double_limb::register(registry);
    assign_limb::register(registry);
    clone_and_assign::register(registry);
    double_limb_from_natural::register(registry);
    from_bits::register(registry);
    from_double_limb::register(registry);
    from_floating_point::register(registry);
    from_limb::register(registry);
    from_limbs::register(registry);
    limb_from_natural::register(registry);
    serde::register(registry);
    to_bits::register(registry);
    to_limbs::register(registry);
}
