// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModMul;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_mul_vector::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_mod_mul_unsigned_matrix);
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_mul_unsigned_matrix_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_mul_unsigned_matrix_algorithms
    );
}

fn demo_unsigned_vector_mod_mul_unsigned_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, v, m) in unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("{v}.mod_mul({a}, {m}) = {}", (&v).mod_mul(&a, m));
    }
}

fn benchmark_unsigned_vector_mod_mul_unsigned_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.mod_mul(UnsignedMatrix<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedVector.mod_mul(UnsignedMatrix, u64)",
                &mut |(a, v, m)| {
                    no_out!(v.mod_mul(a, m));
                },
            ),
            (
                "UnsignedVector.mod_mul(&UnsignedMatrix, u64)",
                &mut |(a, v, m)| {
                    no_out!(v.mod_mul(&a, m));
                },
            ),
            (
                "(&UnsignedVector).mod_mul(UnsignedMatrix, u64)",
                &mut |(a, v, m)| {
                    no_out!((&v).mod_mul(a, m));
                },
            ),
            (
                "(&UnsignedVector).mod_mul(&UnsignedMatrix, u64)",
                &mut |(a, v, m)| {
                    no_out!((&v).mod_mul(&a, m));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_vector_mod_mul_unsigned_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.mod_mul(UnsignedMatrix<u64>, u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, v, m)| {
                no_out!((&v).mod_mul(&a, m));
            }),
            ("naive", &mut |(a, v, m)| {
                no_out!(unsigned_vector_mod_mul_unsigned_matrix_naive(&v, &a, m));
            }),
        ],
    );
}
