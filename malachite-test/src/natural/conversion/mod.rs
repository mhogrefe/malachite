use common::DemoBenchRegistry;

pub mod assign_u32;
pub mod assign_u64;
pub mod clone_and_assign;
pub mod from_bits;
pub mod from_integer;
pub mod from_limbs;
pub mod from_u32;
pub mod from_u64;
pub mod serde;
pub mod to_bits;
pub mod to_limbs;
pub mod u32_from_natural;
pub mod u64_from_natural;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    assign_u32::register(registry);
    assign_u64::register(registry);
    clone_and_assign::register(registry);
    from_bits::register(registry);
    from_integer::register(registry);
    from_limbs::register(registry);
    from_u32::register(registry);
    from_u64::register(registry);
    serde::register(registry);
    to_bits::register(registry);
    to_limbs::register(registry);
    u32_from_natural::register(registry);
    u64_from_natural::register(registry);
}
