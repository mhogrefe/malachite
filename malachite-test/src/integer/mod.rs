use common::DemoBenchRegistry;

pub mod arithmetic;
pub mod basic;
pub mod comparison {
    pub mod eq;
    pub mod hash;
    pub mod ord;
    pub mod ord_abs;
    pub mod partial_ord_abs_i32;
    pub mod partial_ord_abs_natural;
    pub mod partial_ord_abs_u32;
    pub mod partial_ord_i32;
    pub mod partial_ord_natural;
    pub mod partial_ord_u32;
    pub mod partial_eq_i32;
    pub mod partial_eq_natural;
    pub mod partial_eq_u32;
    pub mod sign;
}
pub mod conversion {
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
    pub mod natural_assign_integer;
    pub mod serde;
    pub mod to_i32;
    pub mod to_i64;
    pub mod to_natural;
    pub mod to_sign_and_limbs;
    pub mod to_u32;
    pub mod to_u64;
}
pub mod logic {
    pub mod assign_bit;
    pub mod clear_bit;
    pub mod flip_bit;
    pub mod from_twos_complement_limbs;
    pub mod get_bit;
    pub mod not;
    pub mod set_bit;
    pub mod significant_bits;
    pub mod trailing_zeros;
    pub mod twos_complement_limbs;
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    arithmetic::register(registry);
    basic::register(registry);
}
