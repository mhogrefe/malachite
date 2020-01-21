extern crate malachite_base;
extern crate malachite_nz;
extern crate malachite_test;
extern crate num;
extern crate rand;
extern crate rug;
extern crate rust_wheels;

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
        pub mod arithmetic {
            pub mod log_two;
        }

        pub mod basic {
            pub mod crement;
        }

        pub mod conversion {
            pub mod checked_from_and_exact_from;
            pub mod convertible_from;
            pub mod from_other_type_slice;
            pub mod join_halves;
            pub mod lower_half;
            pub mod overflowing_from;
            pub mod saturating_from;
            pub mod split_in_half;
            pub mod upper_half;
            pub mod vec_from_other_type;
            pub mod vec_from_other_type_slice;
            pub mod wrapping_from;
        }

        pub mod comparison {
            pub mod ord_abs_partial_ord_abs_and_comparators;
        }

        pub mod logic {
            pub mod bit_access;
            pub mod bit_block_access;
            pub mod bit_scan;
            pub mod get_highest_bit;
            pub mod hamming_distance;
            pub mod not_assign;
            pub mod significant_bits;
        }
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
        pub mod add_mul;
        pub mod div;
        pub mod div_exact;
        pub mod div_mod;
        pub mod div_round;
        pub mod divisible_by;
        pub mod divisible_by_power_of_two;
        pub mod eq_mod;
        pub mod eq_mod_power_of_two;
        pub mod mod_op;
        pub mod mod_power_of_two;
        pub mod mul;
        pub mod neg;
        pub mod parity;
        pub mod shl_i;
        pub mod shl_u;
        pub mod shr_i;
        pub mod shr_u;
        pub mod sub;
        pub mod sub_mul;
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
        pub mod partial_eq_natural;
        pub mod partial_eq_primitive_integer;
        pub mod partial_ord_abs_natural_and_comparators;
        pub mod partial_ord_abs_primitive_integer_and_comparators;
        pub mod partial_ord_natural;
        pub mod partial_ord_primitive_integer;
        pub mod sign;
    }

    pub mod conversion {
        pub mod clone;
        pub mod floating_point_from_integer;
        pub mod from_floating_point;
        pub mod from_natural;
        pub mod from_primitive_integer;
        pub mod from_sign_and_limbs;
        pub mod from_twos_complement_bits;
        pub mod from_twos_complement_limbs;
        pub mod natural_from_integer;
        pub mod primitive_integer_from_integer;
        pub mod serde;
        pub mod to_sign_and_limbs;
        pub mod to_twos_complement_bits;
        pub mod to_twos_complement_limbs;
    }

    pub mod logic {
        pub mod and;
        pub mod assign_bit;
        pub mod checked_count_ones;
        pub mod checked_count_zeros;
        pub mod checked_hamming_distance;
        pub mod clear_bit;
        pub mod flip_bit;
        pub mod get_bit;
        pub mod index_of_next_false_bit;
        pub mod index_of_next_true_bit;
        pub mod not;
        pub mod or;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod trailing_zeros;
        pub mod xor;
    }
}

pub mod natural {
    pub mod arithmetic {
        pub mod add;
        pub mod add_mul;
        pub mod checked_sub;
        pub mod checked_sub_mul;
        pub mod div;
        pub mod div_exact;
        pub mod div_mod;
        pub mod div_round;
        pub mod divisible_by;
        pub mod divisible_by_power_of_two;
        pub mod eq_mod;
        pub mod eq_mod_power_of_two;
        pub mod is_power_of_two;
        pub mod log_two;
        pub mod mod_op;
        pub mod mod_power_of_two;
        pub mod mul;
        pub mod neg;
        pub mod next_power_of_two;
        pub mod parity;
        pub mod saturating_sub;
        pub mod saturating_sub_mul;
        pub mod shl_i;
        pub mod shl_u;
        pub mod shr_i;
        pub mod shr_u;
        pub mod sub;
        pub mod sub_mul;
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
        pub mod partial_eq_primitive_integer;
        pub mod partial_ord_abs_primitive_integer_and_comparators;
        pub mod partial_ord_primitive_integer;
    }

    pub mod conversion {
        pub mod clone;
        pub mod floating_point_from_natural;
        pub mod from_bits;
        pub mod from_floating_point;
        pub mod from_limbs;
        pub mod from_primitive_integer;
        pub mod primitive_integer_from_natural;
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
