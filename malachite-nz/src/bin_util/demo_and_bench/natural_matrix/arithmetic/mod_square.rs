// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModMul, ModSquare, ModSquareAssign, Square};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_3;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_square::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_square);
    register_demo!(runner, demo_natural_matrix_mod_square_ref);
    register_demo!(runner, demo_natural_matrix_mod_square_assign);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_square_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_matrix_mod_square_algorithms);
}

fn demo_natural_matrix_mod_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.mod_square({m}) = {}", a.mod_square(&m));
    }
}

fn demo_natural_matrix_mod_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).mod_square({m}) = {}", (&a).mod_square(&m));
    }
}

fn demo_natural_matrix_mod_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, m) in natural_matrix_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_square_assign(&m);
        println!("a := {a_old}; a.mod_square_assign({m}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_square(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.mod_square(Natural)", &mut |(a, m)| {
                no_out!(a.mod_square(m));
            }),
            ("NaturalMatrix.mod_square(&Natural)", &mut |(a, m)| {
                no_out!(a.mod_square(&m));
            }),
            ("(&NaturalMatrix).mod_square(Natural)", &mut |(a, m)| {
                no_out!((&a).mod_square(m));
            }),
            ("(&NaturalMatrix).mod_square(&Natural)", &mut |(a, m)| {
                no_out!((&a).mod_square(&m));
            }),
            (
                "NaturalMatrix.mod_square_assign(Natural)",
                &mut |(mut a, m)| {
                    a.mod_square_assign(m);
                },
            ),
            (
                "NaturalMatrix.mod_square_assign(&Natural)",
                &mut |(mut a, m)| {
                    a.mod_square_assign(&m);
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_square(Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_square(&m));
            }),
            ("using mod_mul", &mut |(a, m)| {
                no_out!((&a).mod_mul(&a, &m));
            }),
            ("using square and %", &mut |(a, m)| {
                no_out!((&a).square() % &m);
            }),
            ("naive", &mut |(a, m)| {
                no_out!(natural_matrix_mod_square_naive(&a, &m));
            }),
        ],
    );
}
