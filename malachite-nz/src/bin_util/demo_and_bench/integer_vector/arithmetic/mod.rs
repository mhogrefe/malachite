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
    balanced_mod::register(runner);
    balanced_mod_power_of_2::register(runner);
    canonical_primitive_part::register(runner);
    canonicalize_sign::register(runner);
    content::register(runner);
    content_chained::register(runner);
    div_exact::register(runner);
    dot_general::register(runner);
    entrywise_abs::register(runner);
    entrywise_div_round::register(runner);
    entrywise_shl_round::register(runner);
    entrywise_shr_round::register(runner);
    height::register(runner);
    l1_norm::register(runner);
    max_bits::register(runner);
    max_limbs::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    multi_crt::register(runner);
    neg::register(runner);
    scalar_div::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    shr::register(runner);
    sub::register(runner);
}

mod add;
mod balanced_mod;
mod balanced_mod_power_of_2;
mod canonical_primitive_part;
mod canonicalize_sign;
mod content;
mod content_chained;
mod div_exact;
mod dot_general;
mod entrywise_abs;
mod entrywise_div_round;
mod entrywise_shl_round;
mod entrywise_shr_round;
mod height;
mod l1_norm;
mod max_bits;
mod max_limbs;
mod mod_op;
mod mod_power_of_2;
mod multi_crt;
mod neg;
mod scalar_div;
mod scalar_mul;
mod shl;
mod shr;
mod sub;
