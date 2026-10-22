// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::ModSolve;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_solve::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_solve);
    register_demo!(runner, demo_unsigned_matrix_mod_solve_solvable);
    register_demo!(runner, demo_unsigned_matrix_mod_solve_vector);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_solve_evaluation_strategy
    );
    register_bench!(runner, benchmark_unsigned_matrix_mod_solve_algorithms);
}

// The first column of `b`, as a vector, if `b` has a column.
fn first_column(b: &UnsignedMatrix<u64>) -> Option<UnsignedVector<u64>> {
    if b.width() == 0 {
        None
    } else {
        Some(UnsignedVector::from_owned_elements(
            b.column_iter(0).copied().collect(),
        ))
    }
}

fn demo_unsigned_matrix_mod_solve(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_solve({b}, {m}) = {:?}", (&a).mod_solve(&b, m));
    }
}

fn demo_unsigned_matrix_mod_solve_solvable(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_solve({b}, {m}) = {:?}", (&a).mod_solve(&b, m));
    }
}

fn demo_unsigned_matrix_mod_solve_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_9()
        .get(gm, config)
        .filter_map(|(a, b, m)| first_column(&b).map(|v| (a, v, m)))
        .take(limit)
    {
        println!("{a}.mod_solve({b}, {m}) = {:?}", (&a).mod_solve(&b, m));
    }
}

fn benchmark_unsigned_matrix_mod_solve_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_solve(UnsignedMatrix, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_solve(UnsignedMatrix, u64)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve(b, m));
                },
            ),
            (
                "UnsignedMatrix.mod_solve(&UnsignedMatrix, u64)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_solve(&b, m));
                },
            ),
            (
                "(&UnsignedMatrix).mod_solve(UnsignedMatrix, u64)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve(b, m));
                },
            ),
            (
                "(&UnsignedMatrix).mod_solve(&UnsignedMatrix, u64)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_solve(&b, m));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_solve_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix).mod_solve(&UnsignedMatrix, u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, b, m)| {
                no_out!((&a).mod_solve(&b, m));
            }),
            ("naive", &mut |(a, b, m)| {
                no_out!(unsigned_matrix_mod_solve_naive(&a, &b, m));
            }),
        ],
    );
}
