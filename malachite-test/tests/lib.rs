extern crate malachite_base;
extern crate malachite_nz;
extern crate malachite_test;
extern crate num;
extern crate rand;
extern crate rug;
extern crate rust_wheels;

pub mod common;

pub mod base {
    pub mod bools {
        pub mod not_assign;
    }

    pub mod chars {
        pub mod char_to_contiguous_range;
        pub mod contiguous_range_to_char;
        pub mod decrement;
        pub mod increment;
    }

    pub mod limbs {
        pub mod limbs_delete_left;
        pub mod limbs_leading_zero_limbs;
        pub mod limbs_move_left;
        pub mod limbs_pad_left;
        pub mod limbs_set_zero;
        pub mod limbs_test_zero;
        pub mod limbs_trailing_zero_limbs;
    }

    pub mod num {
        pub mod conversion {
            pub mod assign;
            pub mod checked_from;
            pub mod convertible_from;
            pub mod overflowing_from;
            pub mod saturating_from;
            pub mod wrapping_from;
        }

        pub mod logic {
            pub mod get_highest_bit;
        }

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

    pub mod rounding_modes {
        pub mod clone;
        pub mod display;
        pub mod eq;
        pub mod from_str;
        pub mod hash;
        pub mod neg;
        pub mod size;
    }

    pub mod strings {
        pub mod string_is_subset;
        pub mod string_nub;
        pub mod string_sort;
    }
}

pub mod integer {
    pub mod arithmetic {
        pub mod abs;
        pub mod add;
        pub mod add_limb;
        pub mod add_mul;
        pub mod add_mul_limb;
        pub mod add_mul_signed_limb;
        pub mod add_natural;
        pub mod add_signed_limb;
        pub mod div_exact_limb;
        pub mod div_exact_signed_limb;
        pub mod div_limb;
        pub mod div_mod_limb;
        pub mod div_mod_signed_limb;
        pub mod div_round_limb;
        pub mod div_round_signed_limb;
        pub mod div_signed_limb;
        pub mod divisible_by_limb;
        pub mod divisible_by_power_of_two;
        pub mod divisible_by_signed_limb;
        pub mod eq_limb_mod_limb;
        pub mod eq_limb_mod_power_of_two;
        pub mod eq_mod_power_of_two;
        pub mod eq_natural_mod_power_of_two;
        pub mod eq_signed_limb_mod_power_of_two;
        pub mod eq_signed_limb_mod_signed_limb;
        pub mod mod_limb;
        pub mod mod_power_of_two;
        pub mod mod_signed_limb;
        pub mod mul;
        pub mod mul_natural;
        pub mod neg;
        pub mod parity;
        pub mod shl_i;
        pub mod shl_u;
        pub mod shr_i;
        pub mod shr_u;
        pub mod sub;
        pub mod sub_limb;
        pub mod sub_mul;
        pub mod sub_mul_limb;
        pub mod sub_mul_signed_limb;
        pub mod sub_natural;
        pub mod sub_signed_limb;
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
        pub mod partial_eq_limb;
        pub mod partial_eq_natural;
        pub mod partial_eq_signed_limb;
        pub mod partial_ord_abs_limb;
        pub mod partial_ord_abs_natural;
        pub mod partial_ord_abs_signed_limb;
        pub mod partial_ord_limb;
        pub mod partial_ord_natural;
        pub mod partial_ord_signed_limb;
        pub mod sign;
    }

    pub mod conversion {
        pub mod assign_double_limb;
        pub mod assign_limb;
        pub mod assign_natural;
        pub mod assign_signed_double_limb;
        pub mod assign_signed_limb;
        pub mod clone_and_assign;
        pub mod double_limb_from_integer;
        pub mod floating_point_from_integer;
        pub mod from_double_limb;
        pub mod from_floating_point;
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
    }

    pub mod logic {
        pub mod and;
        pub mod and_natural;
        pub mod assign_bit;
        pub mod checked_count_ones;
        pub mod checked_count_zeros;
        pub mod checked_hamming_distance;
        pub mod checked_hamming_distance_limb;
        pub mod checked_hamming_distance_natural;
        pub mod checked_hamming_distance_signed_limb;
        pub mod clear_bit;
        pub mod flip_bit;
        pub mod get_bit;
        pub mod index_of_next_false_bit;
        pub mod index_of_next_true_bit;
        pub mod not;
        pub mod or;
        pub mod or_natural;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod trailing_zeros;
        pub mod xor;
        pub mod xor_natural;
    }
}

pub mod natural {
    pub mod arithmetic {
        pub mod add;
        pub mod add_limb;
        pub mod add_mul;
        pub mod add_mul_limb;
        pub mod checked_sub;
        pub mod checked_sub_limb;
        pub mod checked_sub_mul;
        pub mod checked_sub_mul_limb;
        pub mod div;
        pub mod div_exact;
        pub mod div_exact_limb;
        pub mod div_limb;
        pub mod div_mod;
        pub mod div_mod_limb;
        pub mod div_round;
        pub mod div_round_limb;
        pub mod divisible_by;
        pub mod divisible_by_limb;
        pub mod divisible_by_power_of_two;
        pub mod eq_limb_mod_limb;
        pub mod eq_limb_mod_power_of_two;
        pub mod eq_mod;
        pub mod eq_mod_power_of_two;
        pub mod is_power_of_two;
        pub mod log_two;
        pub mod mod_limb;
        pub mod mod_op;
        pub mod mod_power_of_two;
        pub mod mul;
        pub mod neg;
        pub mod next_power_of_two;
        pub mod parity;
        pub mod saturating_sub;
        pub mod saturating_sub_limb;
        pub mod saturating_sub_mul;
        pub mod saturating_sub_mul_limb;
        pub mod shl_i;
        pub mod shl_u;
        pub mod shr_i;
        pub mod shr_u;
        pub mod sub;
        pub mod sub_limb;
        pub mod sub_mul;
        pub mod sub_mul_limb;
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
        pub mod partial_eq_limb;
        pub mod partial_ord_limb;
    }

    pub mod conversion {
        pub mod assign_double_limb;
        pub mod assign_limb;
        pub mod clone_and_assign;
        pub mod double_limb_from_natural;
        pub mod floating_point_from_natural;
        pub mod from_bits;
        pub mod from_double_limb;
        pub mod from_floating_point;
        pub mod from_limb;
        pub mod from_limbs;
        pub mod limb_from_natural;
        pub mod serde;
        pub mod to_bits;
        pub mod to_limbs;
    }

    pub mod logic {
        pub mod and;
        pub mod assign_bit;
        pub mod clear_bit;
        pub mod count_ones;
        pub mod flip_bit;
        pub mod get_bit;
        pub mod hamming_distance;
        pub mod hamming_distance_limb;
        pub mod index_of_next_false_bit;
        pub mod index_of_next_true_bit;
        pub mod limb_count;
        pub mod not;
        pub mod or;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod trailing_zeros;
        pub mod xor;
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
