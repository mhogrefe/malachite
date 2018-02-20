extern crate malachite_base;
extern crate malachite_nz;
extern crate malachite_test;
extern crate num;
extern crate rand;
extern crate rug;
extern crate rust_wheels;

pub mod common;
pub mod base {
    pub mod chars {
        pub mod char_to_contiguous_range;
        pub mod contiguous_range_to_char;
        pub mod decrement;
        pub mod increment;
    }
    pub mod limbs {
        pub mod limbs_delete_left;
        pub mod limbs_pad_left;
        pub mod limbs_set_zero;
        pub mod limbs_test_zero;
    }
    pub mod num {
        pub mod assign_bit;
        pub mod clear_bit;
        pub mod decrement;
        pub mod flip_bit;
        pub mod increment;
        pub mod join_halves;
        pub mod get_bit;
        pub mod lower_half;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod split_in_half;
        pub mod upper_half;
    }
    pub mod rounding_mode {
        pub mod clone;
        pub mod eq;
        pub mod hash;
        pub mod neg;
    }
}
pub mod integer {
    pub mod arithmetic {
        pub mod abs;
        pub mod add;
        pub mod add_i32;
        pub mod add_u32;
        pub mod add_mul;
        pub mod add_mul_i32;
        pub mod add_mul_u32;
        pub mod divisible_by_power_of_two;
        pub mod even_odd;
        pub mod mod_power_of_two;
        pub mod mul;
        pub mod mul_i32;
        pub mod mul_u32;
        pub mod neg;
        pub mod shl_i32;
        pub mod shl_u32;
        pub mod shr_i32;
        pub mod shr_u32;
        pub mod sub;
        pub mod sub_i32;
        pub mod sub_u32;
        pub mod sub_mul;
        pub mod sub_mul_i32;
        pub mod sub_mul_u32;
    }
    pub mod basic {
        pub mod constants;
        pub mod decrement;
        pub mod increment;
    }
    pub mod comparison {
        pub mod eq;
        pub mod hash;
        pub mod ord;
        pub mod ord_abs;
        pub mod partial_eq_i32;
        pub mod partial_eq_natural;
        pub mod partial_eq_u32;
        pub mod partial_ord_abs_i32;
        pub mod partial_ord_abs_natural;
        pub mod partial_ord_abs_u32;
        pub mod partial_ord_i32;
        pub mod partial_ord_natural;
        pub mod partial_ord_u32;
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
        pub mod from_u32;
        pub mod from_u64;
        pub mod to_natural;
        pub mod natural_assign_integer;
        pub mod to_i32;
        pub mod to_i64;
        pub mod to_u32;
        pub mod to_u64;
    }
    pub mod logic {
        pub mod assign_bit;
        pub mod clear_bit;
        pub mod flip_bit;
        pub mod from_sign_and_limbs;
        pub mod from_twos_complement_limbs;
        pub mod get_bit;
        pub mod not;
        pub mod set_bit;
        pub mod sign_and_limbs;
        pub mod significant_bits;
        pub mod trailing_zeros;
        pub mod twos_complement_limbs;
    }
}
pub mod natural {
    pub mod arithmetic {
        pub mod add;
        pub mod add_u32;
        pub mod add_mul;
        pub mod add_mul_u32;
        pub mod divisible_by_power_of_two;
        pub mod even_odd;
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
        pub mod is_power_of_two;
    }
    pub mod basic {
        pub mod constants;
        pub mod decrement;
        pub mod increment;
    }
    pub mod comparison {
        pub mod eq;
        pub mod hash;
        pub mod ord;
        pub mod partial_eq_u32;
        pub mod partial_ord_u32;
    }
    pub mod conversion {
        pub mod assign_u32;
        pub mod assign_u64;
        pub mod clone_and_assign;
        pub mod from_u32;
        pub mod from_u64;
        pub mod to_integer;
        pub mod to_u32;
        pub mod to_u64;
    }
    pub mod logic {
        pub mod assign_bit;
        pub mod clear_bit;
        pub mod flip_bit;
        pub mod from_limbs;
        pub mod get_bit;
        pub mod limb_count;
        pub mod limbs;
        pub mod not;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod trailing_zeros;
    }
    pub mod random {
        pub mod random_natural_with_bits;
        pub mod random_natural_up_to_bits;
        pub mod random_natural_below;
        pub mod special_random_natural_with_bits;
        pub mod special_random_natural_up_to_bits;
        pub mod special_random_natural_below;
    }
}
