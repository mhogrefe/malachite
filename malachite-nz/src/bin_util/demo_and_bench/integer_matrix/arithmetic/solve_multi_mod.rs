// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bin_util::demo_and_bench::integer_matrix::arithmetic::solve::format_solution;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::arithmetic::solve_multi_mod::solve_multi_mod_den;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_7;
use malachite_nz::test_util::integer_matrix::arithmetic::solve::integer_matrix_low_rank;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_solve_multi_mod_den);
    register_demo!(runner, demo_integer_matrix_solve_multi_mod_den_low_rank);
}

fn demo_integer_matrix_solve_multi_mod_den(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        println!(
            "solve_multi_mod_den({a}, {b}) = {}",
            format_solution(solve_multi_mod_den(&a, &b))
        );
    }
}

// Systems whose matrix has rank at most the width of the right-hand side, when that is smaller than
// its dimension, so that many are singular.
fn demo_integer_matrix_solve_multi_mod_den_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        let a = integer_matrix_low_rank(&a, &b);
        println!(
            "solve_multi_mod_den({a}, {b}) = {}",
            format_solution(solve_multi_mod_den(&a, &b))
        );
    }
}
