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
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_from_row_slices);
    register_bench!(runner, benchmark_unsigned_matrix_from_row_slices);
}

fn row_slices(m: &UnsignedMatrix<u64>) -> Vec<&[u64]> {
    let width = usize::exact_from(m.width());
    if width == 0 {
        vec![&[]; usize::exact_from(m.height())]
    } else {
        m.elements_ref().chunks_exact(width).collect()
    }
}

fn demo_unsigned_matrix_from_row_slices(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in unsigned_matrix_gen().get(gm, config).take(limit) {
        let rows = row_slices(&m);
        println!(
            "UnsignedMatrix::from_row_slices({:?}) = {}",
            rows,
            UnsignedMatrix::<u64>::from_row_slices(&rows)
        );
    }
}

fn benchmark_unsigned_matrix_from_row_slices(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix::from_row_slices(&[&[u64]])",
        BenchmarkType::Single,
        unsigned_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|m| m.elements_ref().len(),
            bucketing_label: "m.elements_ref().len()".to_string(),
        },
        &mut [("Malachite", &mut |m| {
            no_out!(UnsignedMatrix::<u64>::from_row_slices(&row_slices(&m)));
        })],
    );
}
