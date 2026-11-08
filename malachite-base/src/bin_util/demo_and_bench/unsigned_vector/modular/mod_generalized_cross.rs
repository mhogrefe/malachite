// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::modular::traits::ModGeneralizedCross;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_17;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_vector::modular::mod_generalized_cross::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_mod_generalized_cross);
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_generalized_cross_algorithms
    );
}

fn rows(a: &UnsignedMatrix<u64>) -> Vec<UnsignedVector<u64>> {
    a.rows()
        .map(|row| UnsignedVector::from_elements(row.as_slice()))
        .collect()
}

fn demo_unsigned_vector_mod_generalized_cross(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "mod_generalized_cross(rows of {a}, {m}) = {}",
            UnsignedVector::mod_generalized_cross(&rows(&a), m)
        );
    }
}

fn benchmark_unsigned_vector_mod_generalized_cross_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector::mod_generalized_cross",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_17().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!(UnsignedVector::mod_generalized_cross(&rows(&a), m));
            }),
            ("determinants", &mut |(a, m)| {
                no_out!(mod_generalized_cross_by_determinants(&rows(&a), m));
            }),
        ],
    );
}
