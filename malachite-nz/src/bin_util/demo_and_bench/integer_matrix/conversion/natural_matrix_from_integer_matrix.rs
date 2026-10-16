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
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_try_from_integer_matrix);
    register_demo!(runner, demo_natural_matrix_convertible_from_integer_matrix);
    register_bench!(
        runner,
        benchmark_natural_matrix_try_from_integer_matrix_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_convertible_from_integer_matrix_algorithms
    );
}

fn demo_natural_matrix_try_from_integer_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in integer_matrix_gen().get(gm, config).take(limit) {
        println!(
            "NaturalMatrix::try_from({}) = {:?}",
            m,
            NaturalMatrix::try_from(&m).map(|n| n.to_string())
        );
    }
}

fn demo_natural_matrix_convertible_from_integer_matrix(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for m in integer_matrix_gen().get(gm, config).take(limit) {
        println!(
            "NaturalMatrix::convertible_from(&{}) = {}",
            m,
            NaturalMatrix::convertible_from(&m)
        );
    }
}

fn benchmark_natural_matrix_try_from_integer_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix::try_from(IntegerMatrix)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("m"),
        &mut [
            ("NaturalMatrix::try_from(IntegerMatrix)", &mut |m| {
                let _ = NaturalMatrix::try_from(m);
            }),
            ("NaturalMatrix::try_from(&IntegerMatrix)", &mut |m| {
                let _ = NaturalMatrix::try_from(&m);
            }),
        ],
    );
}

fn benchmark_natural_matrix_convertible_from_integer_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix::convertible_from(&IntegerMatrix)",
        BenchmarkType::Algorithms,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("m"),
        &mut [
            ("standard", &mut |m| {
                no_out!(NaturalMatrix::convertible_from(&m));
            }),
            ("using try_from", &mut |m| {
                no_out!(NaturalMatrix::try_from(&m).is_ok());
            }),
        ],
    );
}
