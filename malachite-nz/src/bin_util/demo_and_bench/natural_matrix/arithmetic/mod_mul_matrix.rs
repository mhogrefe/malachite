// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModMul, ModMulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_matrix_natural_triple_gen_var_2;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_mul_matrix::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_mul_matrix);
    register_demo!(runner, demo_natural_matrix_mod_mul_matrix_assign);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_mul_matrix_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_matrix_mod_mul_matrix_algorithms);
}

fn demo_natural_matrix_mod_mul_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in natural_matrix_natural_matrix_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_mul({b}, {m}) = {}", (&a).mod_mul(&b, &m));
    }
}

fn demo_natural_matrix_mod_mul_matrix_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, m) in natural_matrix_natural_matrix_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_mul_assign(&b, &m);
        println!("a := {a_old}; a.mod_mul_assign({b}, {m}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_mul_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_mul(NaturalMatrix, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_matrix_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_mul(NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_mul(b, m));
                },
            ),
            (
                "NaturalMatrix.mod_mul(NaturalMatrix, &Natural)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_mul(b, &m));
                },
            ),
            (
                "NaturalMatrix.mod_mul(&NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_mul(&b, m));
                },
            ),
            (
                "NaturalMatrix.mod_mul(&NaturalMatrix, &Natural)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_mul(&b, &m));
                },
            ),
            (
                "(&NaturalMatrix).mod_mul(NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_mul(b, m));
                },
            ),
            (
                "(&NaturalMatrix).mod_mul(NaturalMatrix, &Natural)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_mul(b, &m));
                },
            ),
            (
                "(&NaturalMatrix).mod_mul(&NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_mul(&b, m));
                },
            ),
            (
                "(&NaturalMatrix).mod_mul(&NaturalMatrix, &Natural)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_mul(&b, &m));
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_mul_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_mul(NaturalMatrix, Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_matrix_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, b, m)| {
                no_out!((&a).mod_mul(&b, &m));
            }),
            ("naive", &mut |(a, b, m)| {
                no_out!(natural_matrix_mod_mul_matrix_naive(&a, &b, &m));
            }),
            ("using * and %", &mut |(a, b, m)| {
                no_out!(&a * &b % &m);
            }),
        ],
    );
}
