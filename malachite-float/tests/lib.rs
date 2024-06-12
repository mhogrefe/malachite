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
    clippy::borrow_as_ptr,
    clippy::cloned_instead_of_copied,
    clippy::flat_map_option,
    clippy::unnecessary_wraps,
    clippy::unnested_or_patterns
)]

#[macro_use]
extern crate malachite_base;

pub mod arithmetic {
    pub mod abs;
    pub mod add;
    pub mod is_power_of_2;
    pub mod mul;
    pub mod neg;
    pub mod power_of_2;
    pub mod shl;
    pub mod shr;
    pub mod sign;
    pub mod square;
    pub mod sub;
}

pub mod basic {
    pub mod classification;
    pub mod complexity;
    pub mod constants;
    pub mod get_and_set;
    pub mod named;
    pub mod size;
    pub mod ulp;
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
    pub mod partial_cmp_abs_rational;
    pub mod partial_cmp_integer;
    pub mod partial_cmp_natural;
    pub mod partial_cmp_primitive_float;
    pub mod partial_cmp_primitive_int;
    pub mod partial_cmp_rational;
    pub mod partial_eq_integer;
    pub mod partial_eq_natural;
    pub mod partial_eq_primitive_float;
    pub mod partial_eq_primitive_int;
    pub mod partial_eq_rational;
}
pub mod conversion {
    pub mod clone;
    pub mod from_integer;
    pub mod from_natural;
    pub mod from_primitive_float;
    pub mod from_primitive_int;
    pub mod from_rational;
    pub mod integer_from_float;
    pub mod mantissa_and_exponent;
    pub mod natural_from_float;
    pub mod primitive_float_from_float;
    pub mod primitive_int_from_float;
    pub mod rational_from_float;
}
pub mod exhaustive {
    pub mod exhaustive_finite_floats;
    pub mod exhaustive_floats;
    pub mod exhaustive_negative_finite_floats;
    pub mod exhaustive_non_negative_finite_floats;
    pub mod exhaustive_non_positive_finite_floats;
    pub mod exhaustive_nonzero_finite_floats;
    pub mod exhaustive_positive_finite_floats;
    pub mod exhaustive_positive_floats_with_sci_exponent;
    pub mod exhaustive_positive_floats_with_sci_exponent_and_precision;
}
pub mod random {
    pub mod random_finite_floats;
    pub mod random_floats;
    pub mod random_negative_finite_floats;
    pub mod random_non_negative_finite_floats;
    pub mod random_non_positive_finite_floats;
    pub mod random_nonzero_finite_floats;
    pub mod random_positive_finite_floats;
    pub mod striped_random_negative_finite_floats;
    pub mod striped_random_non_negative_finite_floats;
    pub mod striped_random_non_positive_finite_floats;
    pub mod striped_random_nonzero_finite_floats;
    pub mod striped_random_positive_finite_floats;
}
