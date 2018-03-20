use common::DemoBenchRegistry;

pub mod arithmetic {
    pub mod add;
    pub mod add_u32;
    pub mod add_mul;
    pub mod add_mul_u32;
    pub mod divisible_by_power_of_two;
    pub mod even_odd;
    pub mod is_power_of_two;
    pub mod log_two;
    pub mod mod_power_of_two;
    pub mod mul;
    pub mod mul_u32;
    pub mod neg;
    pub mod shl_i32;
    pub mod shl_u32;
    pub mod shr_i32;
    pub mod shr_u32;
    pub mod sub;
    pub mod sub_u32;
    pub mod sub_mul;
    pub mod sub_mul_u32;
}
pub mod basic {
    pub mod decrement;
    pub mod increment;
}
pub mod comparison {
    pub mod eq;
    pub mod hash;
    pub mod ord;
    pub mod partial_ord_u32;
    pub mod partial_eq_u32;
}
pub mod conversion {
    pub mod assign_u32;
    pub mod assign_u64;
    pub mod clone_and_assign;
    pub mod from_bits;
    pub mod from_limbs;
    pub mod from_u32;
    pub mod from_u64;
    pub mod serde;
    pub mod to_bits;
    pub mod to_integer;
    pub mod to_limbs;
    pub mod to_u32;
    pub mod to_u64;
}
pub mod logic {
    pub mod assign_bit;
    pub mod clear_bit;
    pub mod flip_bit;
    pub mod get_bit;
    pub mod limb_count;
    pub mod not;
    pub mod set_bit;
    pub mod significant_bits;
    pub mod trailing_zeros;
}
pub mod random;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    random::register(registry);
}
