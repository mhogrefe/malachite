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
    abs_diff::register(runner);
    add::register(runner);
    add_mul::register(runner);
    add_mul_shl::register(runner);
    average::register(runner);
    balanced_mod::register(runner);
    balanced_mod_power_of_2::register(runner);
    binomial_coefficient::register(runner);
    canonical_unit_i_pow::register(runner);
    canonicalize_gaussian_unit::register(runner);
    canonicalize_unit::register(runner);
    conjugate::register(runner);
    crt::register(runner);
    div::register(runner);
    div_euclidean::register(runner);
    div_exact::register(runner);
    div_mod::register(runner);
    div_mod_euclidean::register(runner);
    div_round::register(runner);
    divisible_by::register(runner);
    divisible_by_power_of_2::register(runner);
    eq_mod::register(runner);
    eq_mod_power_of_2::register(runner);
    extended_gcd::register(runner);
    falling_factorial::register(runner);
    gcd::register(runner);
    is_power_of_2::register(runner);
    is_unit::register(runner);
    kronecker_symbol::register(runner);
    mod_euclidean::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    mul::register(runner);
    mul_add_mul::register(runner);
    mul_shr_round::register(runner);
    mul_sub_mul::register(runner);
    multi_crt::register(runner);
    neg::register(runner);
    parity::register(runner);
    pow::register(runner);
    power_of_2::register(runner);
    rising_factorial::register(runner);
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
    sub_mul_shl::register(runner);
}

mod abs;
mod abs_diff;
mod abs_squared;
mod add;
mod add_mul;
mod add_mul_shl;
mod average;
mod balanced_mod;
mod balanced_mod_power_of_2;
mod binomial_coefficient;
mod canonical_unit_i_pow;
mod canonicalize_gaussian_unit;
mod canonicalize_unit;
mod conjugate;
mod crt;
mod div;
mod div_euclidean;
mod div_exact;
mod div_mod;
mod div_mod_euclidean;
mod div_round;
mod divisible_by;
mod divisible_by_power_of_2;
mod eq_mod;
mod eq_mod_power_of_2;
mod extended_gcd;
mod falling_factorial;
mod gcd;
mod is_power_of_2;
mod is_unit;
mod kronecker_symbol;
mod mod_euclidean;
mod mod_op;
mod mod_power_of_2;
mod mul;
mod mul_add_mul;
mod mul_shr_round;
mod mul_sub_mul;
mod multi_crt;
mod neg;
mod parity;
mod pow;
mod power_of_2;
mod rising_factorial;
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
mod sub_mul_shl;
