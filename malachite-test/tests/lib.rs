#![allow(unstable_name_collisions)]

extern crate itertools;
#[macro_use]
extern crate malachite_base;
extern crate malachite_base_test_util;
extern crate malachite_nz;
extern crate malachite_nz_test_util;
extern crate malachite_test;
extern crate num;
extern crate rand;
extern crate rug;
extern crate rust_wheels;

pub mod base {
    pub mod num {
        pub mod arithmetic {
            pub mod arithmetic_checked_shl;
            pub mod arithmetic_checked_shr;
            pub mod checked_add_mul;
            pub mod checked_sub_mul;
            pub mod div_exact;
            pub mod div_mod;
            pub mod div_round;
            pub mod divisible_by;
            pub mod divisible_by_power_of_two;
            pub mod eq_mod;
            pub mod eq_mod_power_of_two;
            pub mod mod_add;
            pub mod mod_is_reduced;
            pub mod mod_mul;
            pub mod mod_neg;
            pub mod mod_op;
            pub mod mod_pow;
            pub mod mod_power_of_two;
            pub mod mod_power_of_two_add;
            pub mod mod_power_of_two_is_reduced;
            pub mod mod_power_of_two_mul;
            pub mod mod_power_of_two_neg;
            pub mod mod_power_of_two_pow;
            pub mod mod_power_of_two_shl;
            pub mod mod_power_of_two_shr;
            pub mod mod_power_of_two_square;
            pub mod mod_power_of_two_sub;
            pub mod mod_shl;
            pub mod mod_shr;
            pub mod mod_square;
            pub mod mod_sub;
            pub mod neg;
            pub mod next_power_of_two;
            pub mod overflowing_abs;
            pub mod overflowing_add;
            pub mod overflowing_add_mul;
            pub mod overflowing_div;
            pub mod overflowing_mul;
            pub mod overflowing_neg;
            pub mod overflowing_pow;
            pub mod overflowing_square;
            pub mod overflowing_sub;
            pub mod overflowing_sub_mul;
            pub mod parity;
            pub mod pow;
            pub mod power_of_two;
            pub mod round_to_multiple;
            pub mod round_to_multiple_of_power_of_two;
            pub mod saturating_abs;
            pub mod saturating_add;
            pub mod saturating_add_mul;
            pub mod saturating_mul;
            pub mod saturating_neg;
            pub mod saturating_pow;
            pub mod saturating_square;
            pub mod saturating_sub;
            pub mod saturating_sub_mul;
            pub mod shl_round;
            pub mod shr_round;
            pub mod sign;
            pub mod square;
            pub mod wrapping_abs;
            pub mod wrapping_add;
            pub mod wrapping_add_mul;
            pub mod wrapping_div;
            pub mod wrapping_mul;
            pub mod wrapping_neg;
            pub mod wrapping_pow;
            pub mod wrapping_square;
            pub mod wrapping_sub;
            pub mod wrapping_sub_mul;
            pub mod xx_add_yy_is_zz;
            pub mod xx_div_mod_y_is_qr;
            pub mod xx_sub_yy_is_zz;
            pub mod xxx_add_yyy_is_zzz;
            pub mod xxx_sub_yyy_is_zzz;
            pub mod xxxx_add_yyyy_is_zzzz;
        }
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
        pub mod parity;
        pub mod pow;
        pub mod power_of_two;
        pub mod round_to_multiple;
        pub mod round_to_multiple_of_power_of_two;
        pub mod shl;
        pub mod shl_round;
        pub mod shr;
        pub mod shr_round;
        pub mod sign;
        pub mod square;
        pub mod sub;
        pub mod sub_mul;
    }

    pub mod comparison {
        pub mod eq;
        pub mod hash;
        pub mod ord;
        pub mod ord_abs;
        pub mod partial_eq_natural;
        pub mod partial_eq_primitive_int;
        pub mod partial_ord_abs_natural_and_comparators;
        pub mod partial_ord_abs_primitive_int_and_comparators;
        pub mod partial_ord_natural;
        pub mod partial_ord_primitive_int;
    }

    pub mod conversion {
        pub mod clone;
        pub mod floating_point_from_integer;
        pub mod from_floating_point;
        pub mod from_natural;
        pub mod from_primitive_int;
        pub mod from_twos_complement_limbs;
        pub mod natural_from_integer;
        pub mod primitive_int_from_integer;
        pub mod serde;
        pub mod to_twos_complement_limbs;
    }

    pub mod logic {
        pub mod and;
        pub mod assign_bit;
        pub mod assign_bits;
        pub mod bits;
        pub mod checked_count_ones;
        pub mod checked_count_zeros;
        pub mod checked_hamming_distance;
        pub mod clear_bit;
        pub mod flip_bit;
        pub mod from_bits;
        pub mod get_bit;
        pub mod get_bits;
        pub mod index_of_next_false_bit;
        pub mod index_of_next_true_bit;
        pub mod low_mask;
        pub mod not;
        pub mod or;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod to_bits;
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
        pub mod mod_add;
        pub mod mod_is_reduced;
        pub mod mod_mul;
        pub mod mod_neg;
        pub mod mod_op;
        pub mod mod_pow;
        pub mod mod_power_of_two;
        pub mod mod_power_of_two_add;
        pub mod mod_power_of_two_is_reduced;
        pub mod mod_power_of_two_mul;
        pub mod mod_power_of_two_neg;
        pub mod mod_power_of_two_pow;
        pub mod mod_power_of_two_shl;
        pub mod mod_power_of_two_shr;
        pub mod mod_power_of_two_square;
        pub mod mod_power_of_two_sub;
        pub mod mod_shl;
        pub mod mod_shr;
        pub mod mod_square;
        pub mod mod_sub;
        pub mod mul;
        pub mod next_power_of_two;
        pub mod parity;
        pub mod pow;
        pub mod power_of_two;
        pub mod round_to_multiple;
        pub mod round_to_multiple_of_power_of_two;
        pub mod saturating_sub;
        pub mod saturating_sub_mul;
        pub mod shl;
        pub mod shl_round;
        pub mod shr;
        pub mod shr_round;
        pub mod sign;
        pub mod square;
        pub mod sub;
        pub mod sub_mul;
    }

    pub mod comparison {
        pub mod eq;
        pub mod hash;
        pub mod ord;
        pub mod partial_eq_primitive_int;
        pub mod partial_ord_abs_primitive_int_and_comparators;
        pub mod partial_ord_primitive_int;
    }

    pub mod conversion {
        pub mod clone;
        pub mod digits {
            pub mod from_power_of_two_digits;
            pub mod power_of_two_digits;
            pub mod to_power_of_two_digits;
        }
        pub mod floating_point_from_natural;
        pub mod from_floating_point;
        pub mod from_limbs;
        pub mod primitive_int_from_natural;
        pub mod serde;
        pub mod to_limbs;
    }

    pub mod logic {
        pub mod and;
        pub mod assign_bit;
        pub mod assign_bits;
        pub mod bits;
        pub mod clear_bit;
        pub mod count_ones;
        pub mod flip_bit;
        pub mod from_bits;
        pub mod get_bit;
        pub mod get_bits;
        pub mod hamming_distance;
        pub mod index_of_next_false_bit;
        pub mod index_of_next_true_bit;
        pub mod limb_count;
        pub mod low_mask;
        pub mod not;
        pub mod or;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod to_bits;
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
