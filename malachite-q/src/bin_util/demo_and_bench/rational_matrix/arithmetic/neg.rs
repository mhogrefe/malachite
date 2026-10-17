// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_neg);
    register_demo!(runner, demo_rational_matrix_neg_ref);
    register_demo!(runner, demo_rational_matrix_neg_assign);
    register_bench!(runner, benchmark_rational_matrix_neg_evaluation_strategy);
}

fn demo_rational_matrix_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("-({a_old}) = {}", -a);
    }
}

fn demo_rational_matrix_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen().get(gm, config).take(limit) {
        println!("-&({a}) = {}", -&a);
    }
}

fn demo_rational_matrix_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut a in rational_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.neg_assign();
        println!("a := {a_old}; a.neg_assign(); a = {a}");
    }
}

fn benchmark_rational_matrix_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-RationalMatrix",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("-RationalMatrix", &mut |a| no_out!(-a)),
            ("-&RationalMatrix", &mut |a| no_out!(-&a)),
            ("RationalMatrix.neg_assign()", &mut |mut a| {
                a.neg_assign();
            }),
        ],
    );
}
