// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::Bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_from_columns);
    register_demo!(runner, demo_unsigned_matrix_from_owned_columns);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_from_columns_evaluation_strategy
    );
}

fn columns_of(m: &UnsignedMatrix<u64>) -> Vec<UnsignedVector<u64>> {
    (0..m.width())
        .map(|j| {
            UnsignedVector::from_owned_elements(
                (0..m.height())
                    .map(|i| m.elements[i * m.width() + j])
                    .collect(),
            )
        })
        .collect()
}

fn demo_unsigned_matrix_from_columns(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in unsigned_matrix_gen().get(gm, config).take(limit) {
        let vs = columns_of(&m);
        println!(
            "UnsignedMatrix::from_columns({:?}) = {}",
            vs,
            UnsignedMatrix::from_columns(&vs)
        );
    }
}

fn demo_unsigned_matrix_from_owned_columns(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in unsigned_matrix_gen().get(gm, config).take(limit) {
        let vs = columns_of(&m);
        println!(
            "UnsignedMatrix::from_owned_columns({:?}) = {}",
            vs,
            UnsignedMatrix::from_owned_columns(vs.clone())
        );
    }
}

fn benchmark_unsigned_matrix_from_columns_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix::from_columns(&[UnsignedVector])",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_gen()
            .get(gm, config)
            .map(|m| (columns_of(&m), m)),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|(_, m)| m.elements.len(),
            bucketing_label: "m.elements.len()".to_string(),
        },
        &mut [
            (
                "UnsignedMatrix::from_columns(&[UnsignedVector])",
                &mut |(vs, _)| {
                    no_out!(UnsignedMatrix::from_columns(&vs));
                },
            ),
            (
                "UnsignedMatrix::from_owned_columns(Vec<UnsignedVector<u64>>)",
                &mut |(vs, _)| {
                    no_out!(UnsignedMatrix::from_owned_columns(vs));
                },
            ),
        ],
    );
}
