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
    add_truncated::register(runner);
    balanced_mod::register(runner);
    bit_pack::register(runner);
    bit_unpack::register(runner);
    canonicalize_unit::register(runner);
    compose_power_of_x::register(runner);
    content::register(runner);
    derivative::register(runner);
    div_power_of_x::register(runner);
    evaluate::register(runner);
    deflate_power_of_x::register(runner);
    exponent_gcd::register(runner);
    floor_l2_norm::register(runner);
    height::register(runner);
    is_unit::register(runner);
    l2_norm_squared::register(runner);
    mod_add::register(runner);
    mod_add_truncated::register(runner);
    mod_derivative::register(runner);
    mod_integral::register(runner);
    mod_is_reduced::register(runner);
    mod_make_monic::register(runner);
    mod_mul::register(runner);
    mod_mul_truncated::register(runner);
    mod_neg::register(runner);
    mod_nth_derivative::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    mod_power_of_2_add::register(runner);
    mod_power_of_2_add_truncated::register(runner);
    mod_power_of_2_derivative::register(runner);
    mod_power_of_2_integral::register(runner);
    mod_power_of_2_mul::register(runner);
    mod_power_of_2_mul_truncated::register(runner);
    mod_power_of_2_neg::register(runner);
    mod_power_of_2_nth_derivative::register(runner);
    mod_power_of_2_shl::register(runner);
    mod_power_of_2_square::register(runner);
    mod_power_of_2_square_truncated::register(runner);
    mod_shl::register(runner);
    mod_square::register(runner);
    mod_square_truncated::register(runner);
    mod_power_of_2_sub::register(runner);
    mod_power_of_2_sub_truncated::register(runner);
    mod_sub::register(runner);
    mod_sub_truncated::register(runner);
    mul::register(runner);
    mul_power_of_x::register(runner);
    mul_truncated::register(runner);
    nth_derivative::register(runner);
    pow::register(runner);
    pow_truncated::register(runner);
    shl::register(runner);
    square::register(runner);
    square_truncated::register(runner);
}

mod add;
mod add_truncated;
mod balanced_mod;
mod bit_pack;
mod bit_unpack;
mod canonicalize_unit;
mod compose_power_of_x;
mod content;
mod deflate_power_of_x;
mod derivative;
mod div_power_of_x;
mod evaluate;
mod exponent_gcd;
mod floor_l2_norm;
mod height;
mod is_unit;
mod l2_norm_squared;
mod mod_add;
mod mod_add_truncated;
mod mod_derivative;
mod mod_integral;
mod mod_is_reduced;
mod mod_make_monic;
mod mod_mul;
mod mod_mul_truncated;
mod mod_neg;
mod mod_nth_derivative;
mod mod_op;
mod mod_power_of_2;
mod mod_power_of_2_add;
mod mod_power_of_2_add_truncated;
mod mod_power_of_2_derivative;
mod mod_power_of_2_integral;
mod mod_power_of_2_mul;
mod mod_power_of_2_mul_truncated;
mod mod_power_of_2_neg;
mod mod_power_of_2_nth_derivative;
mod mod_power_of_2_shl;
mod mod_power_of_2_square;
mod mod_power_of_2_square_truncated;
mod mod_power_of_2_sub;
mod mod_power_of_2_sub_truncated;
mod mod_shl;
mod mod_square;
mod mod_square_truncated;
mod mod_sub;
mod mod_sub_truncated;
mod mul;
mod mul_power_of_x;
mod mul_truncated;
mod nth_derivative;
mod pow;
mod pow_truncated;
mod shl;
mod square;
mod square_truncated;
