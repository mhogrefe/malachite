// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::linear_algebra::traits::ModAreAffinelyIndependent;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::natural_vector::linear_algebra::mod_are_affinely_independent::*;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_are_affinely_independent);
    register_bench!(
        runner,
        benchmark_natural_vector_mod_are_affinely_independent_algorithms
    );
}

fn rows(matrix: &NaturalMatrix) -> Vec<NaturalVector> {
    matrix
        .rows()
        .map(|row| NaturalVector::from_elements(row.as_slice()))
        .collect()
}

fn demo_natural_vector_mod_are_affinely_independent(gm: GenMode, config: &GenConfig, limit: usize) {
    for (matrix, m) in natural_matrix_natural_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "mod_are_affinely_independent(rows of {}, {}) = {}",
            matrix,
            m,
            NaturalVector::mod_are_affinely_independent(&rows(&matrix), &m)
        );
    }
}

fn benchmark_natural_vector_mod_are_affinely_independent_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector::mod_are_affinely_independent",
        BenchmarkType::Algorithms,
        natural_matrix_natural_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("homogeneous coordinates", &mut |(matrix, m)| {
                no_out!(mod_are_affinely_independent_by_homogeneous_coordinates(
                    &rows(&matrix),
                    &m
                ));
            }),
            ("subtraction", &mut |(matrix, m)| {
                no_out!(mod_are_affinely_independent_by_subtraction(
                    &rows(&matrix),
                    &m
                ));
            }),
        ],
    );
}
