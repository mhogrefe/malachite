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
    average::register(runner);
    canonical_unit_i_pow::register(runner);
    canonicalize_gaussian_unit::register(runner);
    canonicalize_unit::register(runner);
    cbrt::register(runner);
    conjugate::register(runner);
    div::register(runner);
    dot::register(runner);
    hypot::register(runner);
    is_power_of_2::register(runner);
    is_unit::register(runner);
    mul::register(runner);
    mul_add_mul::register(runner);
    mul_sub_mul::register(runner);
    neg::register(runner);
    pow::register(runner);
    power_of_2::register(runner);
    product::register(runner);
    reciprocal::register(runner);
    reciprocal_sqrt::register(runner);
    rem::register(runner);
    root::register(runner);
    round_to_integer::register(runner);
    shl::register(runner);
    shl_round::register(runner);
    shr::register(runner);
    shr_round::register(runner);
    sign::register(runner);
    sqrt::register(runner);
    square::register(runner);
    sub::register(runner);
    sub_mul::register(runner);
    sum::register(runner);
}

mod abs;
mod abs_squared;
mod add;
mod add_mul;
mod average;
mod canonical_unit_i_pow;
mod canonicalize_gaussian_unit;
mod canonicalize_unit;
mod cbrt;
mod conjugate;
mod div;
mod dot;
mod hypot;
mod is_power_of_2;
mod is_unit;
mod mul;
mod mul_add_mul;
mod mul_sub_mul;
mod neg;
mod pow;
mod power_of_2;
mod product;
mod reciprocal;
mod reciprocal_sqrt;
mod rem;
mod root;
mod round_to_integer;
mod shl;
mod shl_round;
mod shr;
mod shr_round;
mod sign;
mod sqrt;
mod square;
mod sub;
mod sub_mul;
mod sum;
