// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs_diff::register(runner);
    abs_squared::register(runner);
    add::register(runner);
    add_mul::register(runner);
    add_mul_shl::register(runner);
    average::register(runner);
    canonical_unit_i_pow::register(runner);
    canonicalize_gaussian_unit::register(runner);
    canonicalize_unit::register(runner);
    checked_mul_sub_mul::register(runner);
    checked_sub::register(runner);
    checked_sub_mul::register(runner);
    conjugate::register(runner);
    div::register(runner);
    div_euclidean::register(runner);
    div_exact::register(runner);
    div_mod::register(runner);
    div_mod_euclidean::register(runner);
    div_round::register(runner);
    extended_gcd::register(runner);
    extended_gcd_partial::register(runner);
    #[cfg(feature = "float_helpers")]
    float::register(runner);
    is_power_of_2::register(runner);
    is_unit::register(runner);
    mul::register(runner);
    mul_add_mul::register(runner);
    mul_shr_round::register(runner);
    mul_sub_mul::register(runner);
    neg::register(runner);
    next_power_of_2::register(runner);
    parity::register(runner);
    pow::register(runner);
    power_of_2::register(runner);
    product_of_unsigneds::register(runner);
    root::register(runner);
    round_to_multiple::register(runner);
    round_to_multiple_of_power_of_2::register(runner);
    saturating_mul_sub_mul::register(runner);
    saturating_sub::register(runner);
    saturating_sub_mul::register(runner);
    schonhage_strassen::register(runner);
    shl::register(runner);
    shl_round::register(runner);
    shr::register(runner);
    shr_round::register(runner);
    sign::register(runner);
    sqrt::register(runner);
    square::register(runner);
    sub::register(runner);
    sub_mul::register(runner);
    widening_element_product::register(runner);
}

mod abs_diff;
mod abs_squared;
mod add;
mod add_mul;
mod add_mul_shl;
mod average;
mod canonical_unit_i_pow;
mod canonicalize_gaussian_unit;
mod canonicalize_unit;
mod checked_mul_sub_mul;
mod checked_sub;
mod checked_sub_mul;
mod conjugate;
mod div;
mod div_euclidean;
mod div_exact;
mod div_mod;
mod div_mod_euclidean;
mod div_round;
mod extended_gcd;
mod extended_gcd_partial;
#[cfg(feature = "float_helpers")]
mod float;
mod is_power_of_2;
mod is_unit;
mod mul;
mod mul_add_mul;
mod mul_shr_round;
mod mul_sub_mul;
mod neg;
mod next_power_of_2;
mod parity;
mod pow;
mod power_of_2;
mod product_of_unsigneds;
mod root;
mod round_to_multiple;
mod round_to_multiple_of_power_of_2;
mod saturating_mul_sub_mul;
mod saturating_sub;
mod saturating_sub_mul;
mod schonhage_strassen;
mod shl;
mod shl_round;
mod shr;
mod shr_round;
mod sign;
mod sqrt;
mod square;
mod sub;
mod sub_mul;
mod widening_element_product;
