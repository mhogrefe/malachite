// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedAbs;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_gen;
use malachite_q::test_util::rational_vector::arithmetic::checked_abs::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_checked_abs);
    register_demo!(runner, demo_rational_vector_checked_abs_ref);
    register_bench!(runner, benchmark_rational_vector_checked_abs_algorithms);
    register_bench!(
        runner,
        benchmark_rational_vector_checked_abs_evaluation_strategy
    );
}

fn demo_rational_vector_checked_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!("{v_old}.checked_abs() = {:?}", v.checked_abs());
    }
}

fn demo_rational_vector_checked_abs_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!("(&{v}).checked_abs() = {:?}", (&v).checked_abs());
    }
}

fn benchmark_rational_vector_checked_abs_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.checked_abs()",
        BenchmarkType::Algorithms,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |v| {
                no_out!((&v).checked_abs());
            }),
            ("naive", &mut |v| {
                no_out!(rational_vector_checked_abs_naive(&v));
            }),
        ],
    );
}

fn benchmark_rational_vector_checked_abs_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.checked_abs()",
        BenchmarkType::EvaluationStrategy,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [
            ("RationalVector.checked_abs()", &mut |v| {
                no_out!(v.checked_abs());
            }),
            ("(&RationalVector).checked_abs()", &mut |v| {
                no_out!((&v).checked_abs());
            }),
        ],
    );
}
