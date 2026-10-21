// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModMul, ModSquare, ModSquareAssign};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_8;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_square::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_square);
    register_demo!(runner, demo_unsigned_matrix_mod_square_ref);
    register_demo!(runner, demo_unsigned_matrix_mod_square_assign);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_square_evaluation_strategy
    );
    register_bench!(runner, benchmark_unsigned_matrix_mod_square_algorithms);
}

fn demo_unsigned_matrix_mod_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.mod_square({m}) = {}", a.mod_square(m));
    }
}

fn demo_unsigned_matrix_mod_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).mod_square({m}) = {}", (&a).mod_square(m));
    }
}

fn demo_unsigned_matrix_mod_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, m) in unsigned_matrix_unsigned_pair_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_square_assign(m);
        println!("a := {a_old}; a.mod_square_assign({m}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64>.mod_square(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("UnsignedMatrix.mod_square(u64)", &mut |(a, m)| {
                no_out!(a.mod_square(m));
            }),
            ("(&UnsignedMatrix).mod_square(u64)", &mut |(a, m)| {
                no_out!((&a).mod_square(m));
            }),
            (
                "UnsignedMatrix.mod_square_assign(u64)",
                &mut |(mut a, m)| {
                    a.mod_square_assign(m);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix<u64>).mod_square(u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_square(m));
            }),
            ("using mod_mul", &mut |(a, m)| {
                no_out!((&a).mod_mul(&a, m));
            }),
            ("naive", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_square_naive(&a, m));
            }),
        ],
    );
}
