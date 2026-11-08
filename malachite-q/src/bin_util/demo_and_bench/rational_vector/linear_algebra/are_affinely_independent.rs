// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::linear_algebra::traits::AreAffinelyIndependent;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_vector::RationalVector;
use malachite_q::rational_vector::linear_algebra::are_affinely_independent::*;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_are_affinely_independent);
    register_bench!(
        runner,
        benchmark_rational_vector_are_affinely_independent_algorithms
    );
}

fn rows(matrix: &RationalMatrix) -> Vec<RationalVector> {
    matrix
        .rows()
        .map(|row| RationalVector::from_elements(row.as_slice()))
        .collect()
}

fn demo_rational_vector_are_affinely_independent(gm: GenMode, config: &GenConfig, limit: usize) {
    for matrix in rational_matrix_gen().get(gm, config).take(limit) {
        println!(
            "are_affinely_independent(rows of {}) = {}",
            matrix,
            RationalVector::are_affinely_independent(&rows(&matrix))
        );
    }
}

fn benchmark_rational_vector_are_affinely_independent_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector::are_affinely_independent",
        BenchmarkType::Algorithms,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("homogeneous coordinates", &mut |matrix| {
                no_out!(are_affinely_independent_by_homogeneous_coordinates(&rows(
                    &matrix
                )));
            }),
            ("subtraction", &mut |matrix| {
                no_out!(are_affinely_independent_by_subtraction(&rows(&matrix)));
            }),
        ],
    );
}
