// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs::register(runner);
    abs_squared::register(runner);
    add::register(runner);
    add_mul::register(runner);
    add_rational::register(runner);
    average::register(runner);
    canonical_unit_i_pow::register(runner);
    canonicalize_gaussian_unit::register(runner);
    canonicalize_unit::register(runner);
    cbrt::register(runner);
    conjugate::register(runner);
    div::register(runner);
    div_rational::register(runner);
    dot::register(runner);
    fractional_part::register(runner);
    hypot::register(runner);
    is_power_of_2::register(runner);
    is_unit::register(runner);
    mul::register(runner);
    mul_add_mul::register(runner);
    mul_rational::register(runner);
    mul_sub_mul::register(runner);
    neg::register(runner);
    positive_difference::register(runner);
    pow::register(runner);
    power_of_2::register(runner);
    product::register(runner);
    rational_div_big_exponent_float::register(runner);
    reciprocal::register(runner);
    reciprocal_sqrt::register(runner);
    rem::register(runner);
    root::register(runner);
    round_to_integer::register(runner);
    shl::register(runner);
    shr::register(runner);
    sign::register(runner);
    sqrt::register(runner);
    square::register(runner);
    sub::register(runner);
    sub_mul::register(runner);
    sub_rational::register(runner);
    sum::register(runner);
}

mod abs;
mod abs_squared;
mod add;
mod add_mul;
mod add_rational;
mod average;
mod canonical_unit_i_pow;
mod canonicalize_gaussian_unit;
mod canonicalize_unit;
mod cbrt;
mod conjugate;
mod div;
mod div_rational;
mod dot;
mod fractional_part;
mod hypot;
mod is_power_of_2;
mod is_unit;
mod mul;
mod mul_add_mul;
mod mul_rational;
mod mul_sub_mul;
mod neg;
mod positive_difference;
mod pow;
mod power_of_2;
mod product;
mod rational_div_big_exponent_float;
mod reciprocal;
mod reciprocal_sqrt;
mod rem;
mod root;
mod round_to_integer;
mod shl;
mod shr;
mod sign;
mod sqrt;
mod square;
mod sub;
mod sub_mul;
mod sub_rational;
mod sum;
