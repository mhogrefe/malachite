use common::DemoBenchRegistry;

pub mod assign_i32;
pub mod assign_i64;
pub mod assign_natural;
pub mod assign_u32;
pub mod assign_u64;
pub mod clone_and_assign;
pub mod from_i32;
pub mod from_i64;
pub mod from_sign_and_limbs;
pub mod from_u32;
pub mod from_u64;
pub mod i32_from_integer;
pub mod i64_from_integer;
pub mod natural_assign_integer;
pub mod serde;
pub mod from_natural;
pub mod to_sign_and_limbs;
pub mod to_twos_complement_limbs;
pub mod u32_from_integer;
pub mod u64_from_integer;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    assign_i32::register(registry);
    assign_i64::register(registry);
    assign_natural::register(registry);
    assign_u32::register(registry);
    assign_u64::register(registry);
    clone_and_assign::register(registry);
    from_i32::register(registry);
    from_i64::register(registry);
    from_natural::register(registry);
    from_sign_and_limbs::register(registry);
    from_u32::register(registry);
    from_u64::register(registry);
    i32_from_integer::register(registry);
    i64_from_integer::register(registry);
    natural_assign_integer::register(registry);
    serde::register(registry);
    to_sign_and_limbs::register(registry);
    to_twos_complement_limbs::register(registry);
    u32_from_integer::register(registry);
    u64_from_integer::register(registry);
}
