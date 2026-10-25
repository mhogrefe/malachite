// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::Bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_from_row_slices);
    register_bench!(runner, benchmark_rational_matrix_from_row_slices);
}

fn row_slices(m: &RationalMatrix) -> Vec<&[Rational]> {
    let width = usize::exact_from(m.width());
    if width == 0 {
        vec![&[]; usize::exact_from(m.height())]
    } else {
        m.elements_ref().chunks_exact(width).collect()
    }
}

fn demo_rational_matrix_from_row_slices(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in rational_matrix_gen().get(gm, config).take(limit) {
        let rows = row_slices(&m);
        println!(
            "RationalMatrix::from_row_slices({:?}) = {}",
            rows,
            RationalMatrix::from_row_slices(&rows)
        );
    }
}

fn benchmark_rational_matrix_from_row_slices(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix::from_row_slices(&[&[Rational]])",
        BenchmarkType::Single,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|m| m.elements_ref().len(),
            bucketing_label: "m.elements_ref().len()".to_string(),
        },
        &mut [("Malachite", &mut |m| {
            no_out!(RationalMatrix::from_row_slices(&row_slices(&m)));
        })],
    );
}
