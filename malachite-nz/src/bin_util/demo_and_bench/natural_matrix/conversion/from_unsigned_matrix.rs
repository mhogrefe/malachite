// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_matrix::NaturalMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_from_unsigned_matrix);
    register_demo!(runner, demo_natural_matrix_from_unsigned_matrix_ref);
    register_bench!(
        runner,
        benchmark_natural_matrix_from_unsigned_matrix_evaluation_strategy
    );
}

fn demo_natural_matrix_from_unsigned_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in unsigned_matrix_gen().get(gm, config).take(limit) {
        println!(
            "NaturalMatrix::from({}) = {}",
            m.clone(),
            NaturalMatrix::from(m)
        );
    }
}

fn demo_natural_matrix_from_unsigned_matrix_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in unsigned_matrix_gen().get(gm, config).take(limit) {
        println!("NaturalMatrix::from(&{}) = {}", m, NaturalMatrix::from(&m));
    }
}

fn benchmark_natural_matrix_from_unsigned_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix::from(UnsignedMatrix<u64>)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_matrix_size_bucketer("m"),
        &mut [
            ("NaturalMatrix::from(UnsignedMatrix<u64>)", &mut |m| {
                let _ = NaturalMatrix::from(m);
            }),
            ("NaturalMatrix::from(&UnsignedMatrix<u64>)", &mut |m| {
                let _ = NaturalMatrix::from(&m);
            }),
        ],
    );
}
