// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModSolveLowerTriangular;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_solve_lower_triangular::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_solve_lower_triangular);
    register_demo!(
        runner,
        demo_unsigned_matrix_mod_solve_lower_triangular_unit_diagonal
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_solve_lower_triangular_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_solve_lower_triangular_algorithms
    );
}

fn demo_unsigned_matrix_mod_solve_lower_triangular(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{a}.mod_solve_lower_triangular({b}, {m}, false) = {}",
            (&a).mod_solve_lower_triangular(&b, m, false)
        );
    }
}

fn demo_unsigned_matrix_mod_solve_lower_triangular_unit_diagonal(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{a}.mod_solve_lower_triangular({b}, {m}, true) = {}",
            (&a).mod_solve_lower_triangular(&b, m, true)
        );
    }
}

fn benchmark_unsigned_matrix_mod_solve_lower_triangular_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_solve_lower_triangular(UnsignedMatrix, u64, bool)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_solve_lower_triangular(UnsignedMatrix, u64, bool)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve_lower_triangular(b, m, false));
                },
            ),
            (
                "UnsignedMatrix.mod_solve_lower_triangular(&UnsignedMatrix, u64, bool)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve_lower_triangular(&b, m, false));
                },
            ),
            (
                "(&UnsignedMatrix).mod_solve_lower_triangular(UnsignedMatrix, u64, bool)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve_lower_triangular(b, m, false));
                },
            ),
            (
                "(&UnsignedMatrix).mod_solve_lower_triangular(&UnsignedMatrix, u64, bool)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve_lower_triangular(&b, m, false));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_solve_lower_triangular_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix).mod_solve_lower_triangular(&UnsignedMatrix, u64, bool)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, b, m)| {
                no_out!((&a).mod_solve_lower_triangular(&b, m, false));
            }),
            ("naive", &mut |(a, b, m)| {
                no_out!(unsigned_matrix_mod_solve_lower_triangular_naive(
                    &a, &b, m, false
                ));
            }),
        ],
    );
}
