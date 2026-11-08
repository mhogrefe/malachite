// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::GeneralizedCross;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_gen_var_5;
use malachite_nz::test_util::integer_vector::arithmetic::generalized_cross::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_generalized_cross);
    register_bench!(
        runner,
        benchmark_integer_vector_generalized_cross_algorithms
    );
}

fn rows(a: &IntegerMatrix) -> Vec<IntegerVector> {
    a.rows()
        .map(|row| IntegerVector::from_elements(row.as_slice()))
        .collect()
}

fn demo_integer_vector_generalized_cross(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_5().get(gm, config).take(limit) {
        println!(
            "generalized_cross(rows of {a}) = {}",
            IntegerVector::generalized_cross(&rows(&a))
        );
    }
}

fn benchmark_integer_vector_generalized_cross_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector::generalized_cross",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(IntegerVector::generalized_cross(&rows(&a)));
            }),
            ("determinants", &mut |a| {
                no_out!(generalized_cross_by_determinants(&rows(&a)));
            }),
        ],
    );
}
