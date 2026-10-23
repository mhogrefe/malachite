// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bin_util::demo_and_bench::integer_matrix::arithmetic::solve::format_solution;
use malachite_base::num::arithmetic::traits::Inverse;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_matrix_gen_var_1, integer_matrix_pair_gen_var_7,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_inverse);
    register_demo!(runner, demo_integer_matrix_inverse_ref);
    register_demo!(runner, demo_integer_matrix_inverse_low_rank);
    register_bench!(runner, benchmark_integer_matrix_inverse_evaluation_strategy);
}

fn demo_integer_matrix_inverse(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("inverse({a_old}) = {}", format_solution(a.inverse()));
    }
}

fn demo_integer_matrix_inverse_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!("(&{a}).inverse() = {:?}", (&a).inverse());
    }
}

// Square matrices of rank at most the width of a second matrix, when that is smaller than their
// dimension, so that many are singular.
fn demo_integer_matrix_inverse_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        let a = super::solve::low_rank(&a, &b);
        println!("inverse({a}) = {}", format_solution((&a).inverse()));
    }
}

fn benchmark_integer_matrix_inverse_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.inverse()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.inverse()", &mut |a| {
                no_out!(a.inverse());
            }),
            ("(&IntegerMatrix).inverse()", &mut |a| {
                no_out!((&a).inverse());
            }),
        ],
    );
}
