// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs::register(runner);
    abs_diff::register(runner);
    add::register(runner);
    add_mul::register(runner);
    binomial_coefficient::register(runner);
    div::register(runner);
    div_exact::register(runner);
    div_mod::register(runner);
    div_round::register(runner);
    divisible_by::register(runner);
    divisible_by_power_of_2::register(runner);
    eq_mod::register(runner);
    eq_mod_power_of_2::register(runner);
    extended_gcd::register(runner);
    kronecker_symbol::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    mul::register(runner);
    neg::register(runner);
    parity::register(runner);
    pow::register(runner);
    power_of_2::register(runner);
    root::register(runner);
    round_to_multiple::register(runner);
    round_to_multiple_of_power_of_2::register(runner);
    shl::register(runner);
    shl_round::register(runner);
    shr::register(runner);
    shr_round::register(runner);
    sign::register(runner);
    sqrt::register(runner);
    square::register(runner);
    sub::register(runner);
    sub_mul::register(runner);
}

mod abs;
mod abs_diff;
mod add;
mod add_mul;
mod binomial_coefficient;
mod div;
mod div_exact;
mod div_mod;
mod div_round;
mod divisible_by;
mod divisible_by_power_of_2;
mod eq_mod;
mod eq_mod_power_of_2;
mod extended_gcd;
mod kronecker_symbol;
mod mod_op;
mod mod_power_of_2;
mod mul;
mod neg;
mod parity;
mod pow;
mod power_of_2;
mod root;
mod round_to_multiple;
mod round_to_multiple_of_power_of_2;
mod shl;
mod shl_round;
mod shr;
mod shr_round;
mod sign;
mod sqrt;
mod square;
mod sub;
mod sub_mul;
