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
    content::register(runner);
    content_chained::register(runner);
    mod_add::register(runner);
    mod_is_reduced::register(runner);
    mod_mul::register(runner);
    mod_mul_vector::register(runner);
    mod_neg::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    mod_power_of_2_add::register(runner);
    mod_power_of_2_is_reduced::register(runner);
    mod_power_of_2_mul::register(runner);
    mod_power_of_2_mul_vector::register(runner);
    mod_power_of_2_neg::register(runner);
    mod_power_of_2_shl::register(runner);
    mod_power_of_2_sub::register(runner);
    mod_shl::register(runner);
    mod_sub::register(runner);
    mul_vector::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    vector_mod_mul::register(runner);
    vector_mod_power_of_2_mul::register(runner);
    vector_mul::register(runner);
}

mod add;
mod balanced_mod;
mod balanced_mod_power_of_2;
mod canonical_primitive_part;
mod content;
mod content_chained;
mod mod_add;
mod mod_is_reduced;
mod mod_mul;
mod mod_mul_vector;
mod mod_neg;
mod mod_op;
mod mod_power_of_2;
mod mod_power_of_2_add;
mod mod_power_of_2_is_reduced;
mod mod_power_of_2_mul;
mod mod_power_of_2_mul_vector;
mod mod_power_of_2_neg;
mod mod_power_of_2_shl;
mod mod_power_of_2_sub;
mod mod_shl;
mod mod_sub;
mod mul_vector;
mod scalar_mul;
mod shl;
mod vector_mod_mul;
mod vector_mod_power_of_2_mul;
mod vector_mul;
