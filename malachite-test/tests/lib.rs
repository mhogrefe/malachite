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
        pub mod get_bit;
        pub mod increment;
        pub mod join_halves;
        pub mod log_two;
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
        pub mod size;
    }
}
pub mod integer {
    pub mod arithmetic {
        pub mod abs;
        pub mod add;
        pub mod add_i32;
        pub mod add_mul;
        pub mod add_mul_i32;
        pub mod add_mul_u32;
        pub mod add_u32;
        pub mod div_exact_u32;
        pub mod div_i32;
        pub mod div_mod_i32;
        pub mod div_mod_u32;
        pub mod div_round_i32;
        pub mod div_round_u32;
        pub mod div_u32;
        pub mod divisible_by_power_of_two;
        pub mod divisible_by_u32;
        pub mod eq_i32_mod_power_of_two;
        pub mod eq_mod_power_of_two;
        pub mod eq_u32_mod_power_of_two;
        pub mod eq_u32_mod_u32;
        pub mod mod_i32;
        pub mod mod_power_of_two;
        pub mod mod_u32;
        pub mod mul;
        pub mod mul_i32;
        pub mod mul_u32;
        pub mod neg;
        pub mod parity;
        pub mod shl_i;
        pub mod shl_u;
        pub mod shr_i;
        pub mod shr_u;
        pub mod sub;
        pub mod sub_i32;
        pub mod sub_mul;
        pub mod sub_mul_i32;
        pub mod sub_mul_u32;
        pub mod sub_u32;
    }
    pub mod basic {
        pub mod constants;
        pub mod decrement;
        pub mod increment;
        pub mod size;
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
        pub mod from_natural;
        pub mod from_sign_and_limbs;
        pub mod from_twos_complement_bits;
        pub mod from_twos_complement_limbs;
        pub mod from_u32;
        pub mod from_u64;
        pub mod i32_from_integer;
        pub mod i64_from_integer;
        pub mod natural_assign_integer;
        pub mod natural_from_integer;
        pub mod serde;
        pub mod to_sign_and_limbs;
        pub mod to_twos_complement_bits;
        pub mod to_twos_complement_limbs;
        pub mod u32_from_integer;
        pub mod u64_from_integer;
    }
    pub mod logic {
        pub mod and;
        pub mod and_i32;
        pub mod and_u32;
        pub mod assign_bit;
        pub mod checked_count_ones;
        pub mod checked_count_zeros;
        pub mod checked_hamming_distance;
        pub mod checked_hamming_distance_i32;
        pub mod checked_hamming_distance_u32;
        pub mod clear_bit;
        pub mod flip_bit;
        pub mod get_bit;
        pub mod index_of_next_false_bit;
        pub mod index_of_next_true_bit;
        pub mod not;
        pub mod or;
        pub mod or_i32;
        pub mod or_u32;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod trailing_zeros;
        pub mod xor;
        pub mod xor_i32;
        pub mod xor_u32;
    }
}
pub mod natural {
    pub mod arithmetic {
        pub mod add;
        pub mod add_mul;
        pub mod add_mul_u32;
        pub mod add_u32;
        pub mod checked_sub;
        pub mod checked_sub_u32;
        pub mod div_exact_u32;
        pub mod div_mod_u32;
        pub mod div_round_u32;
        pub mod div_u32;
        pub mod divisible_by_power_of_two;
        pub mod divisible_by_u32;
        pub mod eq_mod_power_of_two;
        pub mod eq_u32_mod_power_of_two;
        pub mod eq_u32_mod_u32;
        pub mod is_power_of_two;
        pub mod log_two;
        pub mod mod_power_of_two;
        pub mod mod_u32;
        pub mod mul;
        pub mod mul_u32;
        pub mod neg;
        pub mod next_power_of_two;
        pub mod parity;
        pub mod saturating_sub;
        pub mod saturating_sub_u32;
        pub mod shl_i;
        pub mod shl_u;
        pub mod shr_i;
        pub mod shr_u;
        pub mod sub;
        pub mod sub_mul;
        pub mod sub_mul_u32;
        pub mod sub_u32;
    }
    pub mod basic {
        pub mod constants;
        pub mod decrement;
        pub mod increment;
        pub mod size;
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
        pub mod from_bits;
        pub mod from_limbs;
        pub mod from_u32;
        pub mod from_u64;
        pub mod serde;
        pub mod to_bits;
        pub mod to_limbs;
        pub mod u32_from_natural;
        pub mod u64_from_natural;
    }
    pub mod logic {
        pub mod and;
        pub mod and_u32;
        pub mod assign_bit;
        pub mod clear_bit;
        pub mod count_ones;
        pub mod flip_bit;
        pub mod get_bit;
        pub mod hamming_distance;
        pub mod hamming_distance_u32;
        pub mod index_of_next_false_bit;
        pub mod index_of_next_true_bit;
        pub mod limb_count;
        pub mod not;
        pub mod or;
        pub mod or_u32;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod trailing_zeros;
        pub mod xor;
        pub mod xor_u32;
    }
    pub mod random {
        pub mod random_natural_below;
        pub mod random_natural_up_to_bits;
        pub mod random_natural_with_bits;
        pub mod special_random_natural_below;
        pub mod special_random_natural_up_to_bits;
        pub mod special_random_natural_with_bits;
    }
}
