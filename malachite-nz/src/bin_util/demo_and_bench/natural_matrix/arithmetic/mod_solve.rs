// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::ModSolve;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_solve::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_solve);
    register_demo!(runner, demo_natural_matrix_mod_solve_solvable);
    register_demo!(runner, demo_natural_matrix_mod_solve_vector);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_solve_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_matrix_mod_solve_algorithms);
}

// The first column of `b`, as a vector, if `b` has a column.
fn first_column(b: &NaturalMatrix) -> Option<NaturalVector> {
    if b.width() == 0 {
        None
    } else {
        Some(b.column(0))
    }
}

fn demo_natural_matrix_mod_solve(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in natural_matrix_natural_matrix_natural_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_solve({b}, {m}) = {:?}", (&a).mod_solve(&b, &m));
    }
}

fn demo_natural_matrix_mod_solve_solvable(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in natural_matrix_natural_matrix_natural_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_solve({b}, {m}) = {:?}", (&a).mod_solve(&b, &m));
    }
}

fn demo_natural_matrix_mod_solve_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in natural_matrix_natural_matrix_natural_triple_gen_var_6()
        .get(gm, config)
        .filter_map(|(a, b, m)| first_column(&b).map(|v| (a, v, m)))
        .take(limit)
    {
        println!("{a}.mod_solve({b}, {m}) = {:?}", (&a).mod_solve(&b, &m));
    }
}

fn benchmark_natural_matrix_mod_solve_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_solve(NaturalMatrix, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_matrix_natural_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_solve(NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve(b, m));
                },
            ),
            (
                "NaturalMatrix.mod_solve(NaturalMatrix, &Natural)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve(b, &m));
                },
            ),
            (
                "NaturalMatrix.mod_solve(&NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve(&b, m));
                },
            ),
            (
                "NaturalMatrix.mod_solve(&NaturalMatrix, &Natural)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve(&b, &m));
                },
            ),
            (
                "&NaturalMatrix.mod_solve(NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve(b, m));
                },
            ),
            (
                "&NaturalMatrix.mod_solve(NaturalMatrix, &Natural)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve(b, &m));
                },
            ),
            (
                "&NaturalMatrix.mod_solve(&NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve(&b, m));
                },
            ),
            (
                "&NaturalMatrix.mod_solve(&NaturalMatrix, &Natural)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve(&b, &m));
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_solve_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_solve(&NaturalMatrix, &Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_matrix_natural_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, b, m)| {
                no_out!((&a).mod_solve(&b, &m));
            }),
            ("naive", &mut |(a, b, m)| {
                no_out!(natural_matrix_mod_solve_naive(&a, &b, &m));
            }),
        ],
    );
}
