// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::bench::bucketers::unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_from_elements);
    register_demo!(runner, demo_unsigned_matrix_from_owned_elements);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_from_elements_evaluation_strategy
    );
}

fn demo_unsigned_matrix_from_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in unsigned_matrix_gen().get(gm, config).take(limit) {
        let xs = m.elements.clone();
        println!(
            "UnsignedMatrix::from_elements({}, {}, {:?}) = {}",
            m.height(),
            m.width(),
            xs,
            UnsignedMatrix::from_elements(m.height(), m.width(), &xs)
        );
    }
}

fn demo_unsigned_matrix_from_owned_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in unsigned_matrix_gen().get(gm, config).take(limit) {
        let xs = m.elements.clone();
        println!(
            "UnsignedMatrix::from_owned_elements({}, {}, {:?}) = {}",
            m.height(),
            m.width(),
            xs,
            UnsignedMatrix::from_owned_elements(m.height(), m.width(), xs.clone())
        );
    }
}

fn benchmark_unsigned_matrix_from_elements_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix::from_elements(usize, usize, &[u64])",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_matrix_size_bucketer("m"),
        &mut [
            (
                "UnsignedMatrix::from_elements(usize, usize, &[u64])",
                &mut |m| {
                    no_out!(UnsignedMatrix::from_elements(
                        m.height(),
                        m.width(),
                        &m.elements
                    ));
                },
            ),
            (
                "UnsignedMatrix::from_owned_elements(usize, usize, Vec<u64>)",
                &mut |m| {
                    let (height, width) = (m.height(), m.width());
                    no_out!(UnsignedMatrix::from_owned_elements(
                        height, width, m.elements
                    ));
                },
            ),
        ],
    );
}
