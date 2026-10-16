// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_gen;
use malachite_q::rational_matrix::RationalMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_from_natural_matrix);
    register_bench!(runner, benchmark_rational_matrix_from_natural_matrix);
}

fn demo_rational_matrix_from_natural_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in natural_matrix_gen().get(gm, config).take(limit) {
        println!(
            "RationalMatrix::from({}) = {}",
            m.clone(),
            RationalMatrix::from(m)
        );
    }
}

fn benchmark_rational_matrix_from_natural_matrix(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix::from(NaturalMatrix)",
        BenchmarkType::Single,
        natural_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |m| {
            let _ = RationalMatrix::from(m);
        })],
    );
}
