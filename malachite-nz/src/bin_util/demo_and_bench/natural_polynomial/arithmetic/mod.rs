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
    bit_pack::register(runner);
    bit_unpack::register(runner);
    canonical_primitive_part::register(runner);
    canonicalize_unit::register(runner);
    compose::register(runner);
    compose_power_of_x::register(runner);
    content::register(runner);
    deflate_power_of_x::register(runner);
    div_power_of_x::register(runner);
    evaluate::register(runner);
    exponent_gcd::register(runner);
    floor_l2_norm::register(runner);
    height::register(runner);
    is_unit::register(runner);
    l2_norm_squared::register(runner);
    mul::register(runner);
    mul_middle::register(runner);
    mul_power_of_x::register(runner);
    pow::register(runner);
    shl::register(runner);
    square::register(runner);
    taylor_shift::register(runner);
}

mod add;
mod bit_pack;
mod bit_unpack;
mod canonical_primitive_part;
mod canonicalize_unit;
mod compose;
mod compose_power_of_x;
mod content;
mod deflate_power_of_x;
mod div_power_of_x;
mod evaluate;
mod exponent_gcd;
mod floor_l2_norm;
mod height;
mod is_unit;
mod l2_norm_squared;
mod mul;
mod mul_middle;
mod mul_power_of_x;
mod pow;
mod shl;
mod square;
mod taylor_shift;
