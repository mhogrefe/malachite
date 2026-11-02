// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    add::register(runner);
    canonical_primitive_part::register(runner);
    canonicalize_unit::register(runner);
    compose::register(runner);
    compose_power_of_x::register(runner);
    content::register(runner);
    deflate_power_of_x::register(runner);
    div::register(runner);
    div_exact_by_polynomial::register(runner);
    div_power_of_x::register(runner);
    div_rem::register(runner);
    divisible_by::register(runner);
    evaluate::register(runner);
    exponent_gcd::register(runner);
    height::register(runner);
    is_unit::register(runner);
    l2_norm_squared::register(runner);
    make_monic::register(runner);
    mul::register(runner);
    mul_power_of_x::register(runner);
    neg::register(runner);
    pow::register(runner);
    rem::register(runner);
    rem_precomputed::register(runner);
    remove_power::register(runner);
    shl::register(runner);
    shr::register(runner);
    square::register(runner);
    sub::register(runner);
    taylor_shift::register(runner);
}

mod add;
mod canonical_primitive_part;
mod canonicalize_unit;
mod compose;
mod compose_power_of_x;
mod content;
mod deflate_power_of_x;
mod div;
mod div_exact_by_polynomial;
mod div_power_of_x;
mod div_rem;
mod divisible_by;
mod evaluate;
mod exponent_gcd;
mod height;
mod is_unit;
mod l2_norm_squared;
mod make_monic;
mod mul;
mod mul_power_of_x;
mod neg;
mod pow;
mod rem;
mod rem_precomputed;
mod remove_power;
mod shl;
mod shr;
mod square;
mod sub;
mod taylor_shift;
