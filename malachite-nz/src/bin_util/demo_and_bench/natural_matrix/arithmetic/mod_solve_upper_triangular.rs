// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModSolveUpperTriangular;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_matrix_natural_triple_gen_var_4;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_solve_upper_triangular::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_solve_upper_triangular);
    register_demo!(
        runner,
        demo_natural_matrix_mod_solve_upper_triangular_unit_diagonal
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_solve_upper_triangular_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_solve_upper_triangular_algorithms
    );
}

fn demo_natural_matrix_mod_solve_upper_triangular(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in natural_matrix_natural_matrix_natural_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{a}.mod_solve_upper_triangular({b}, {m}, false) = {}",
            (&a).mod_solve_upper_triangular(&b, &m, false)
        );
    }
}

fn demo_natural_matrix_mod_solve_upper_triangular_unit_diagonal(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, m) in natural_matrix_natural_matrix_natural_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{a}.mod_solve_upper_triangular({b}, {m}, true) = {}",
            (&a).mod_solve_upper_triangular(&b, &m, true)
        );
    }
}

fn benchmark_natural_matrix_mod_solve_upper_triangular_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_solve_upper_triangular(NaturalMatrix, Natural, bool)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_matrix_natural_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_solve_upper_triangular(NaturalMatrix, Natural, bool)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve_upper_triangular(b, m, false));
                },
            ),
            (
                "NaturalMatrix.mod_solve_upper_triangular(NaturalMatrix, &Natural, bool)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve_upper_triangular(b, &m, false));
                },
            ),
            (
                "NaturalMatrix.mod_solve_upper_triangular(&NaturalMatrix, Natural, bool)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve_upper_triangular(&b, m, false));
                },
            ),
            (
                "NaturalMatrix.mod_solve_upper_triangular(&NaturalMatrix, &Natural, bool)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve_upper_triangular(&b, &m, false));
                },
            ),
            (
                "&NaturalMatrix.mod_solve_upper_triangular(NaturalMatrix, Natural, bool)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve_upper_triangular(b, m, false));
                },
            ),
            (
                "&NaturalMatrix.mod_solve_upper_triangular(NaturalMatrix, &Natural, bool)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve_upper_triangular(b, &m, false));
                },
            ),
            (
                "&NaturalMatrix.mod_solve_upper_triangular(&NaturalMatrix, Natural, bool)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve_upper_triangular(&b, m, false));
                },
            ),
            (
                "&NaturalMatrix.mod_solve_upper_triangular(&NaturalMatrix, &Natural, bool)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve_upper_triangular(&b, &m, false));
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_solve_upper_triangular_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_solve_upper_triangular(&NaturalMatrix, &Natural, bool)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_matrix_natural_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, b, m)| {
                no_out!((&a).mod_solve_upper_triangular(&b, &m, false));
            }),
            ("naive", &mut |(a, b, m)| {
                no_out!(natural_matrix_mod_solve_upper_triangular_naive(
                    &a, &b, &m, false
                ));
            }),
        ],
    );
}
