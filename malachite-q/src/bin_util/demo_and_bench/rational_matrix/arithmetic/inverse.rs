// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bin_util::demo_and_bench::rational_matrix::arithmetic::solve::format_result;
use malachite_base::num::arithmetic::traits::Inverse;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_matrix_gen_var_2, rational_matrix_pair_gen_var_5,
};
use malachite_q::test_util::rational_matrix::arithmetic::solve::rational_matrix_low_rank;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_inverse);
    register_demo!(runner, demo_rational_matrix_inverse_ref);
    register_demo!(runner, demo_rational_matrix_inverse_low_rank);
    register_bench!(
        runner,
        benchmark_rational_matrix_inverse_evaluation_strategy
    );
}

fn demo_rational_matrix_inverse(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_2().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("inverse({a_old}) = {}", format_result(a.inverse()));
    }
}

fn demo_rational_matrix_inverse_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_2().get(gm, config).take(limit) {
        println!("(&{a}).inverse() = {:?}", (&a).inverse());
    }
}

// Square matrices of rank at most the width of a second matrix, when that is smaller than their
// dimension, so that many are singular.
fn demo_rational_matrix_inverse_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_5().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!("inverse({a}) = {}", format_result((&a).inverse()));
    }
}

fn benchmark_rational_matrix_inverse_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.inverse()",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix.inverse()", &mut |a| {
                no_out!(a.inverse());
            }),
            ("(&RationalMatrix).inverse()", &mut |a| {
                no_out!((&a).inverse());
            }),
        ],
    );
}
