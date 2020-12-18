#![allow(
    unstable_name_collisions,
    clippy::assertions_on_constants,
    clippy::cognitive_complexity,
    clippy::many_single_char_names,
    clippy::range_plus_one,
    clippy::suspicious_arithmetic_impl,
    clippy::suspicious_op_assign_impl,
    clippy::too_many_arguments,
    clippy::float_cmp
)]
#![warn(
    clippy::cast_lossless,
    clippy::decimal_literal_representation,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map,
    clippy::filter_map_next,
    clippy::find_map,
    clippy::large_digit_groups,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::missing_const_for_fn,
    clippy::mut_mut,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::print_stdout,
    clippy::redundant_closure_for_method_calls,
    clippy::single_match_else,
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::unused_self
)]

#[macro_use]
extern crate malachite_base;
#[macro_use]
extern crate malachite_base_test_util;
extern crate malachite_nz;
extern crate malachite_nz_test_util;
extern crate num;
extern crate rug;

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
    pub mod basic {
        pub mod constants;
        pub mod size;
    }
    pub mod comparison {
        pub mod eq;
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
    pub mod exhaustive {
        pub mod exhaustive_integer_inclusive_range;
        pub mod exhaustive_integer_range;
        pub mod exhaustive_integer_range_to_infinity;
        pub mod exhaustive_integer_range_to_negative_infinity;
        pub mod exhaustive_integers;
        pub mod exhaustive_natural_integers;
        pub mod exhaustive_negative_integers;
        pub mod exhaustive_nonzero_integers;
        pub mod exhaustive_positive_integers;
        pub mod integer_decreasing_range_to_negative_infinity;
        pub mod integer_increasing_inclusive_range;
        pub mod integer_increasing_range;
        pub mod integer_increasing_range_to_infinity;
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
        pub mod neg;
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
    pub mod basic {
        pub mod constants;
        pub mod size;
    }
    pub mod comparison {
        pub mod eq;
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
        pub mod from_primitive_int;
        pub mod primitive_int_from_natural;
        pub mod serde;
        pub mod to_limbs;
    }
    pub mod exhaustive {
        pub mod exhaustive_natural_inclusive_range;
        pub mod exhaustive_natural_range;
        pub mod exhaustive_natural_range_to_infinity;
        pub mod exhaustive_naturals;
        pub mod exhaustive_positive_naturals;
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
