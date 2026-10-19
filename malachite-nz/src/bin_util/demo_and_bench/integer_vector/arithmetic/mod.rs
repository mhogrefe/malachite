// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs_squared::register(runner);
    add::register(runner);
    add_dot::register(runner);
    add_dot_reversed::register(runner);
    add_mul::register(runner);
    add_mul_shl::register(runner);
    balanced_mod::register(runner);
    balanced_mod_power_of_2::register(runner);
    canonical_primitive_part::register(runner);
    canonicalize_sign::register(runner);
    checked_abs::register(runner);
    content::register(runner);
    content_chained::register(runner);
    cross::register(runner);
    div_exact::register(runner);
    dot::register(runner);
    dot_reversed::register(runner);
    dot_sign::register(runner);
    dot_general::register(runner);
    element_lcm::register(runner);
    element_product::register(runner);
    element_sum::register(runner);
    entrywise_abs::register(runner);
    entrywise_div_round::register(runner);
    entrywise_mul::register(runner);
    entrywise_shl_round::register(runner);
    entrywise_shr_round::register(runner);
    height::register(runner);
    l1_norm::register(runner);
    max_bits::register(runner);
    max_limbs::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    multi_crt::register(runner);
    multi_mod::register(runner);
    neg::register(runner);
    scalar_div::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    shr::register(runner);
    sub::register(runner);
    sub_dot::register(runner);
    sub_dot_reversed::register(runner);
    sub_mul::register(runner);
    sub_mul_shl::register(runner);
    sum::register(runner);
}

mod abs_squared;
mod add;
mod add_dot;
mod add_dot_reversed;
mod add_mul;
mod add_mul_shl;
mod balanced_mod;
mod balanced_mod_power_of_2;
mod canonical_primitive_part;
mod canonicalize_sign;
mod checked_abs;
mod content;
mod content_chained;
mod cross;
mod div_exact;
mod dot;
mod dot_general;
mod dot_reversed;
mod dot_sign;
mod element_lcm;
mod element_product;
mod element_sum;
mod entrywise_abs;
mod entrywise_div_round;
mod entrywise_mul;
mod entrywise_shl_round;
mod entrywise_shr_round;
mod height;
mod l1_norm;
mod max_bits;
mod max_limbs;
mod mod_op;
mod mod_power_of_2;
mod multi_crt;
mod multi_mod;
mod neg;
mod scalar_div;
mod scalar_mul;
mod shl;
mod shr;
mod sub;
mod sub_dot;
mod sub_dot_reversed;
mod sub_mul;
mod sub_mul_shl;
mod sum;
