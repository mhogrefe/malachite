// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#![allow(
    unstable_name_collisions,
    clippy::bool_assert_comparison,
    clippy::assertions_on_constants,
    clippy::cognitive_complexity,
    clippy::excessive_precision,
    clippy::many_single_char_names,
    clippy::range_plus_one,
    clippy::suspicious_arithmetic_impl,
    clippy::suspicious_op_assign_impl,
    clippy::too_many_arguments,
    clippy::float_cmp,
    clippy::type_complexity,
    clippy::multiple_bound_locations
)]
#![warn(
    clippy::cast_lossless,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map_next,
    clippy::large_digit_groups,
    clippy::manual_filter_map,
    clippy::manual_find_map,
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
    clippy::uninlined_format_args,
    clippy::unused_self,
    clippy::if_not_else,
    clippy::manual_assert,
    clippy::range_plus_one,
    clippy::redundant_else,
    clippy::semicolon_if_nothing_returned,
    clippy::cloned_instead_of_copied,
    clippy::flat_map_option,
    clippy::unnecessary_wraps,
    clippy::unnested_or_patterns,
    clippy::trivially_copy_pass_by_ref
)]

extern crate itertools;
#[macro_use]
extern crate malachite_base;
extern crate malachite_nz;
extern crate num;
extern crate rug;

pub mod integer {
    pub mod arithmetic {
        pub mod abs;
        pub mod abs_diff;
        pub mod add;
        pub mod add_mul;
        pub mod binomial_coefficient;
        pub mod div;
        pub mod div_exact;
        pub mod div_mod;
        pub mod div_round;
        pub mod divisible_by;
        pub mod divisible_by_power_of_2;
        pub mod eq_mod;
        pub mod eq_mod_power_of_2;
        pub mod extended_gcd;
        pub mod kronecker_symbol;
        pub mod mod_op;
        pub mod mod_power_of_2;
        pub mod mul;
        pub mod neg;
        pub mod parity;
        pub mod pow;
        pub mod power_of_2;
        pub mod root;
        pub mod round_to_multiple;
        pub mod round_to_multiple_of_power_of_2;
        pub mod shl;
        pub mod shl_round;
        pub mod shr;
        pub mod shr_round;
        pub mod sign;
        pub mod sqrt;
        pub mod square;
        pub mod sub;
        pub mod sub_mul;
    }
    pub mod basic {
        pub mod constants;
        pub mod default;
        pub mod from_sign_and_abs;
        pub mod named;
        pub mod size;
    }
    pub mod comparison {
        pub mod cmp;
        pub mod cmp_abs;
        pub mod eq;
        pub mod eq_abs;
        pub mod eq_abs_natural;
        pub mod eq_abs_primitive_float;
        pub mod eq_abs_primitive_int;
        pub mod hash;
        pub mod partial_cmp_abs_natural;
        pub mod partial_cmp_abs_primitive_float;
        pub mod partial_cmp_abs_primitive_int;
        pub mod partial_cmp_natural;
        pub mod partial_cmp_primitive_float;
        pub mod partial_cmp_primitive_int;
        pub mod partial_eq_natural;
        pub mod partial_eq_primitive_float;
        pub mod partial_eq_primitive_int;
    }
    pub mod conversion {
        pub mod clone;
        pub mod from_bool;
        pub mod from_natural;
        pub mod from_primitive_float;
        pub mod from_primitive_int;
        pub mod from_twos_complement_limbs;
        pub mod is_integer;
        pub mod natural_from_integer;
        pub mod primitive_float_from_integer;
        pub mod primitive_int_from_integer;
        #[cfg(feature = "serde")]
        pub mod serde;
        pub mod string {
            pub mod from_sci_string;
            pub mod from_string;
            pub mod to_sci;
            pub mod to_string;
        }
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
    pub mod random {
        pub mod get_random_integer_from_range_to_infinity;
        pub mod get_random_integer_from_range_to_negative_infinity;
        pub mod get_striped_random_integer_from_inclusive_range;
        pub mod get_striped_random_integer_from_range;
        pub mod get_striped_random_integer_from_range_to_infinity;
        pub mod get_striped_random_integer_from_range_to_negative_infinity;
        pub mod get_uniform_random_integer_from_inclusive_range;
        pub mod get_uniform_random_integer_from_range;
        pub mod random_integer_inclusive_range;
        pub mod random_integer_range;
        pub mod random_integer_range_to_infinity;
        pub mod random_integer_range_to_negative_infinity;
        pub mod random_integers;
        pub mod random_natural_integers;
        pub mod random_negative_integers;
        pub mod random_nonzero_integers;
        pub mod random_positive_integers;
        pub mod striped_random_integer_inclusive_range;
        pub mod striped_random_integer_range;
        pub mod striped_random_integer_range_to_infinity;
        pub mod striped_random_integer_range_to_negative_infinity;
        pub mod striped_random_integers;
        pub mod striped_random_natural_integers;
        pub mod striped_random_negative_integers;
        pub mod striped_random_nonzero_integers;
        pub mod striped_random_positive_integers;
        pub mod uniform_random_integer_inclusive_range;
        pub mod uniform_random_integer_range;
    }
}
pub mod natural {
    pub mod arithmetic {
        pub mod abs_diff;
        pub mod add;
        pub mod add_mul;
        pub mod binomial_coefficient;
        pub mod checked_sub;
        pub mod checked_sub_mul;
        pub mod coprime_with;
        pub mod div;
        pub mod div_exact;
        pub mod div_mod;
        pub mod div_round;
        pub mod divisible_by;
        pub mod divisible_by_power_of_2;
        pub mod eq_mod;
        pub mod eq_mod_power_of_2;
        pub mod extended_gcd;
        pub mod factorial;
        pub mod gcd;
        pub mod is_power_of_2;
        pub mod kronecker_symbol;
        pub mod lcm;
        pub mod log_base;
        pub mod log_base_2;
        pub mod log_base_power_of_2;
        pub mod mod_add;
        pub mod mod_inverse;
        pub mod mod_is_reduced;
        pub mod mod_mul;
        pub mod mod_neg;
        pub mod mod_op;
        pub mod mod_pow;
        pub mod mod_power_of_2;
        pub mod mod_power_of_2_add;
        pub mod mod_power_of_2_inverse;
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
        pub mod mul;
        pub mod neg;
        pub mod next_power_of_2;
        pub mod parity;
        pub mod pow;
        pub mod power_of_2;
        pub mod primorial;
        pub mod root;
        pub mod round_to_multiple;
        pub mod round_to_multiple_of_power_of_2;
        pub mod saturating_sub;
        pub mod saturating_sub_mul;
        pub mod shl;
        pub mod shl_round;
        pub mod shr;
        pub mod shr_round;
        pub mod sign;
        pub mod sqrt;
        pub mod square;
        pub mod sub;
        pub mod sub_mul;
    }
    pub mod basic {
        pub mod constants;
        pub mod default;
        pub mod named;
        pub mod size;
    }
    pub mod comparison {
        pub mod cmp;
        pub mod eq;
        pub mod eq_abs_primitive_float;
        pub mod eq_abs_primitive_int;
        pub mod hash;
        pub mod partial_cmp_abs_primitive_float;
        pub mod partial_cmp_abs_primitive_int;
        pub mod partial_cmp_primitive_float;
        pub mod partial_cmp_primitive_int;
        pub mod partial_eq_primitive_float;
        pub mod partial_eq_primitive_int;
    }
    pub mod conversion {
        pub mod clone;
        pub mod digits {
            pub mod from_digits;
            pub mod from_power_of_2_digits;
            pub mod power_of_2_digits;
            pub mod to_digits;
            pub mod to_power_of_2_digits;
        }
        pub mod from_bool;
        pub mod from_limbs;
        pub mod from_primitive_float;
        pub mod from_primitive_int;
        pub mod is_integer;
        pub mod mantissa_and_exponent {
            pub mod integer_mantissa_and_exponent;
            pub mod sci_mantissa_and_exponent;
        }
        pub mod primitive_float_from_natural;
        pub mod primitive_int_from_natural;
        #[cfg(feature = "serde")]
        pub mod serde;
        pub mod string {
            pub mod from_sci_string;
            pub mod from_string;
            pub mod to_sci;
            pub mod to_string;
        }
        pub mod to_limbs;
    }
    pub mod exhaustive {
        pub mod exhaustive_natural_inclusive_range;
        pub mod exhaustive_natural_range;
        pub mod exhaustive_natural_range_to_infinity;
        pub mod exhaustive_naturals;
        pub mod exhaustive_positive_naturals;
    }
    pub mod factorization {
        pub mod primes;
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
        pub mod get_random_natural_less_than;
        pub mod get_random_natural_with_bits;
        pub mod get_random_natural_with_up_to_bits;
        pub mod get_striped_random_natural_from_inclusive_range;
        pub mod get_striped_random_natural_from_range;
        pub mod get_striped_random_natural_with_bits;
        pub mod get_striped_random_natural_with_up_to_bits;
        pub mod random_natural_inclusive_range;
        pub mod random_natural_range;
        pub mod random_natural_range_to_infinity;
        pub mod random_naturals;
        pub mod random_naturals_less_than;
        pub mod random_positive_naturals;
        pub mod striped_random_natural_inclusive_range;
        pub mod striped_random_natural_range;
        pub mod striped_random_natural_range_to_infinity;
        pub mod striped_random_naturals;
        pub mod striped_random_positive_naturals;
        pub mod uniform_random_natural_inclusive_range;
        pub mod uniform_random_natural_range;
    }
}
