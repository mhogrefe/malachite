// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Trace;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_to_trace);
    register_demo!(runner, demo_rational_matrix_into_trace);
    register_bench!(runner, benchmark_rational_matrix_trace_evaluation_strategy);
}

fn demo_rational_matrix_to_trace(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_1().get(gm, config).take(limit) {
        println!("{a}.to_trace() = {}", a.to_trace());
    }
}

fn demo_rational_matrix_into_trace(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old}.into_trace() = {}", a.into_trace());
    }
}

fn benchmark_rational_matrix_trace_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.to_trace()",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix.to_trace()", &mut |a| {
                no_out!(a.to_trace());
            }),
            ("RationalMatrix.into_trace()", &mut |a| {
                no_out!(a.into_trace());
            }),
        ],
    );
}
