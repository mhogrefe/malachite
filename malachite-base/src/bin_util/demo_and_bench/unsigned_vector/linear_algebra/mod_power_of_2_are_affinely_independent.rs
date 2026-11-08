// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::linear_algebra::traits::ModPowerOf2AreAffinelyIndependent;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::unsigned_vector::linear_algebra::mod_power_of_2_are_affinely_independent::*;
use malachite_base::vector::Vector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_unsigned_vector_mod_power_of_2_are_affinely_independent
    );
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_are_affinely_independent_algorithms
    );
}

fn rows(matrix: &UnsignedMatrix<u64>) -> Vec<UnsignedVector<u64>> {
    matrix
        .rows()
        .map(|row| UnsignedVector::from_elements(row.as_slice()))
        .collect()
}

fn demo_unsigned_vector_mod_power_of_2_are_affinely_independent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (matrix, pow) in unsigned_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "mod_power_of_2_are_affinely_independent(rows of {}, {}) = {}",
            matrix,
            pow,
            UnsignedVector::mod_power_of_2_are_affinely_independent(&rows(&matrix), pow)
        );
    }
}

fn benchmark_unsigned_vector_mod_power_of_2_are_affinely_independent_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector::mod_power_of_2_are_affinely_independent",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("homogeneous coordinates", &mut |(matrix, pow)| {
                no_out!(
                    mod_power_of_2_are_affinely_independent_by_homogeneous_coordinates(
                        &rows(&matrix),
                        pow
                    )
                );
            }),
            ("subtraction", &mut |(matrix, pow)| {
                no_out!(mod_power_of_2_are_affinely_independent_by_subtraction(
                    &rows(&matrix),
                    pow
                ));
            }),
            ("reduction modulo 2", &mut |(matrix, pow)| {
                no_out!(mod_power_of_2_are_affinely_independent_by_reduction_mod_2(
                    &rows(&matrix),
                    pow
                ));
            }),
        ],
    );
}
