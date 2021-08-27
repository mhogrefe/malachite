#![allow(unstable_name_collisions)]

extern crate itertools;
extern crate malachite_base;
extern crate malachite_base_test_util;
extern crate malachite_nz;
extern crate malachite_nz_test_util;
extern crate malachite_test;
extern crate num;
extern crate rand;
extern crate rug;
extern crate rust_wheels;

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
        pub mod divisible_by_power_of_2;
        pub mod eq_mod;
        pub mod eq_mod_power_of_2;
        pub mod mod_op;
        pub mod mod_power_of_2;
        pub mod mul;
        pub mod parity;
        pub mod pow;
        pub mod power_of_2;
        pub mod round_to_multiple;
        pub mod round_to_multiple_of_power_of_2;
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
        pub mod divisible_by_power_of_2;
        pub mod eq_mod;
        pub mod eq_mod_power_of_2;
        pub mod is_power_of_2;
        pub mod mod_add;
        pub mod mod_is_reduced;
        pub mod mod_mul;
        pub mod mod_neg;
        pub mod mod_op;
        pub mod mod_pow;
        pub mod mod_power_of_2;
        pub mod mod_power_of_2_add;
        pub mod mod_power_of_2_is_reduced;
        pub mod mod_power_of_2_mul;
        pub mod mod_power_of_2_neg;
        pub mod mod_power_of_2_pow;
        pub mod mod_power_of_2_shl;
        pub mod mod_power_of_2_shr;
        pub mod mod_power_of_2_square;
        pub mod mod_power_of_2_sub;
        pub mod mod_shl;
        pub mod mod_shr;
        pub mod mod_square;
        pub mod mod_sub;
        pub mod next_power_of_2;
        pub mod parity;
        pub mod pow;
        pub mod power_of_2;
        pub mod round_to_multiple;
        pub mod round_to_multiple_of_power_of_2;
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
}
