// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_try_from_rational_matrix);
    register_demo!(runner, demo_integer_matrix_convertible_from_rational_matrix);
    register_bench!(
        runner,
        benchmark_integer_matrix_try_from_rational_matrix_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_convertible_from_rational_matrix_algorithms
    );
}

fn demo_integer_matrix_try_from_rational_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in rational_matrix_gen().get(gm, config).take(limit) {
        println!(
            "IntegerMatrix::try_from({}) = {:?}",
            m,
            IntegerMatrix::try_from(&m).map(|n| n.to_string())
        );
    }
}

fn demo_integer_matrix_convertible_from_rational_matrix(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for m in rational_matrix_gen().get(gm, config).take(limit) {
        println!(
            "IntegerMatrix::convertible_from(&{}) = {}",
            m,
            IntegerMatrix::convertible_from(&m)
        );
    }
}

fn benchmark_integer_matrix_try_from_rational_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix::try_from(RationalMatrix)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("m"),
        &mut [
            ("IntegerMatrix::try_from(RationalMatrix)", &mut |m| {
                let _ = IntegerMatrix::try_from(m);
            }),
            ("IntegerMatrix::try_from(&RationalMatrix)", &mut |m| {
                let _ = IntegerMatrix::try_from(&m);
            }),
        ],
    );
}

fn benchmark_integer_matrix_convertible_from_rational_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix::convertible_from(&RationalMatrix)",
        BenchmarkType::Algorithms,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("m"),
        &mut [
            ("standard", &mut |m| {
                no_out!(IntegerMatrix::convertible_from(&m));
            }),
            ("using try_from", &mut |m| {
                no_out!(IntegerMatrix::try_from(&m).is_ok());
            }),
        ],
    );
}
