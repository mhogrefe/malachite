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
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen_var_5;
use malachite_q::test_util::rational_vector::arithmetic::generalized_cross::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_generalized_cross);
    register_bench!(
        runner,
        benchmark_rational_vector_generalized_cross_algorithms
    );
}

fn rows(a: &RationalMatrix) -> Vec<RationalVector> {
    a.rows()
        .map(|row| RationalVector::from_elements(row.as_slice()))
        .collect()
}

fn demo_rational_vector_generalized_cross(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_5().get(gm, config).take(limit) {
        println!(
            "generalized_cross(rows of {a}) = {}",
            RationalVector::generalized_cross(&rows(&a))
        );
    }
}

fn benchmark_rational_vector_generalized_cross_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector::generalized_cross",
        BenchmarkType::Algorithms,
        rational_matrix_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(RationalVector::generalized_cross(&rows(&a)));
            }),
            ("determinants", &mut |a| {
                no_out!(generalized_cross_by_determinants(&rows(&a)));
            }),
        ],
    );
}
