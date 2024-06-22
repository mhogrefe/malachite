// Copyright © 2024 Mikhail Hogrefe
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
    clippy::type_complexity
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
extern crate malachite_q;
extern crate num;
extern crate rug;

pub mod arithmetic {
    pub mod abs;
    pub mod add;
    pub mod approximate;
    pub mod ceiling;
    pub mod denominators_in_closed_interval;
    pub mod div;
    pub mod floor;
    pub mod is_power_of_2;
    pub mod log_base;
    pub mod log_base_2;
    pub mod log_base_power_of_2;
    pub mod mul;
    pub mod neg;
    pub mod next_power_of_2;
    pub mod pow;
    pub mod power_of_2;
    pub mod reciprocal;
    pub mod root;
    pub mod round_to_multiple;
    pub mod round_to_multiple_of_power_of_2;
    pub mod shl;
    pub mod shr;
    pub mod sign;
    pub mod simplest_rational_in_interval;
    pub mod sqrt;
    pub mod square;
    pub mod sub;
}
pub mod basic {
    pub mod constants;
    pub mod default;
    pub mod named;
    pub mod significant_bits;
    pub mod size;
}
pub mod comparison {
    pub mod cmp;
    pub mod cmp_abs;
    pub mod eq;
    pub mod hash;
    pub mod partial_cmp_abs_integer;
    pub mod partial_cmp_abs_natural;
    pub mod partial_cmp_abs_primitive_float;
    pub mod partial_cmp_abs_primitive_int;
    pub mod partial_cmp_integer;
    pub mod partial_cmp_natural;
    pub mod partial_cmp_primitive_float;
    pub mod partial_cmp_primitive_int;
    pub mod partial_eq_integer;
    pub mod partial_eq_natural;
    pub mod partial_eq_primitive_float;
    pub mod partial_eq_primitive_int;
}
pub mod conversion {
    pub mod clone;
    pub mod continued_fraction {
        pub mod convergents;
        pub mod from_continued_fraction;
        pub mod to_continued_fraction;
    }
    pub mod digits {
        #[allow(clippy::module_inception)]
        pub mod digits;
        pub mod from_digits;
        pub mod from_power_of_2_digits;
        pub mod power_of_2_digits;
        pub mod to_digits;
        pub mod to_power_of_2_digits;
    }
    pub mod from_bool;
    pub mod from_float_simplest;
    pub mod from_integer;
    pub mod from_natural;
    pub mod from_numerator_and_denominator;
    pub mod from_primitive_float;
    pub mod from_primitive_int;
    pub mod integer_from_rational;
    pub mod is_integer;
    pub mod mutate_numerator_or_denominator;
    pub mod natural_from_rational;
    pub mod primitive_float_from_rational;
    pub mod primitive_int_from_rational;
    pub mod sci_mantissa_and_exponent;
    pub mod serde;
    pub mod string {
        pub mod from_sci_string;
        pub mod from_string;
        pub mod to_sci;
        pub mod to_string;
    }
    pub mod to_numerator_or_denominator;
}
pub mod exhaustive {
    pub mod exhaustive_negative_rationals;
    pub mod exhaustive_non_negative_rationals;
    pub mod exhaustive_nonzero_rationals;
    pub mod exhaustive_positive_rationals;
    pub mod exhaustive_rational_inclusive_range;
    pub mod exhaustive_rational_range;
    pub mod exhaustive_rational_range_to_infinity;
    pub mod exhaustive_rational_range_to_negative_infinity;
    pub mod exhaustive_rationals;
    pub mod exhaustive_rationals_with_denominator_inclusive_range;
    pub mod exhaustive_rationals_with_denominator_range;
    pub mod exhaustive_rationals_with_denominator_range_to_infinity;
    pub mod exhaustive_rationals_with_denominator_range_to_negative_infinity;
}
pub mod random {
    pub mod random_negative_rationals;
    pub mod random_non_negative_rationals;
    pub mod random_nonzero_rationals;
    pub mod random_positive_rationals;
    pub mod random_rational_inclusive_range;
    pub mod random_rational_range;
    pub mod random_rational_range_to_infinity;
    pub mod random_rational_range_to_negative_infinity;
    pub mod random_rational_with_denominator_inclusive_range;
    pub mod random_rational_with_denominator_range;
    pub mod random_rational_with_denominator_range_to_infinity;
    pub mod random_rational_with_denominator_range_to_negative_infinity;
    pub mod random_rationals;
    pub mod striped_random_negative_rationals;
    pub mod striped_random_non_negative_rationals;
    pub mod striped_random_nonzero_rationals;
    pub mod striped_random_positive_rationals;
    pub mod striped_random_rationals;
}
