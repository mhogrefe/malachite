// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModMul;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_vector_natural_triple_gen_var_2;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_mul_vector::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_mul_natural_matrix);
    register_bench!(
        runner,
        benchmark_natural_vector_mod_mul_natural_matrix_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_vector_mod_mul_natural_matrix_algorithms
    );
}

fn demo_natural_vector_mod_mul_natural_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, v, m) in natural_matrix_natural_vector_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("{v}.mod_mul({a}, {m}) = {}", (&v).mod_mul(&a, &m));
    }
}

fn benchmark_natural_vector_mod_mul_natural_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_mul(NaturalMatrix, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_vector_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalVector.mod_mul(NaturalMatrix, Natural)",
                &mut |(a, v, m)| {
                    no_out!(v.mod_mul(a, m));
                },
            ),
            (
                "NaturalVector.mod_mul(NaturalMatrix, &Natural)",
                &mut |(a, v, m)| {
                    no_out!(v.mod_mul(a, &m));
                },
            ),
            (
                "NaturalVector.mod_mul(&NaturalMatrix, Natural)",
                &mut |(a, v, m)| {
                    no_out!(v.mod_mul(&a, m));
                },
            ),
            (
                "NaturalVector.mod_mul(&NaturalMatrix, &Natural)",
                &mut |(a, v, m)| {
                    no_out!(v.mod_mul(&a, &m));
                },
            ),
            (
                "(&NaturalVector).mod_mul(NaturalMatrix, Natural)",
                &mut |(a, v, m)| {
                    no_out!((&v).mod_mul(a, m));
                },
            ),
            (
                "(&NaturalVector).mod_mul(NaturalMatrix, &Natural)",
                &mut |(a, v, m)| {
                    no_out!((&v).mod_mul(a, &m));
                },
            ),
            (
                "(&NaturalVector).mod_mul(&NaturalMatrix, Natural)",
                &mut |(a, v, m)| {
                    no_out!((&v).mod_mul(&a, m));
                },
            ),
            (
                "(&NaturalVector).mod_mul(&NaturalMatrix, &Natural)",
                &mut |(a, v, m)| {
                    no_out!((&v).mod_mul(&a, &m));
                },
            ),
        ],
    );
}

fn benchmark_natural_vector_mod_mul_natural_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_mul(NaturalMatrix, Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_vector_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, v, m)| {
                no_out!((&v).mod_mul(&a, &m));
            }),
            ("naive", &mut |(a, v, m)| {
                no_out!(natural_vector_mod_mul_natural_matrix_naive(&v, &a, &m));
            }),
        ],
    );
}
