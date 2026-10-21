// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPow, ModPowAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_natural_triple_gen_var_2;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_pow::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_pow);
    register_demo!(runner, demo_natural_matrix_mod_pow_ref);
    register_demo!(runner, demo_natural_matrix_mod_pow_assign);
    register_bench!(runner, benchmark_natural_matrix_mod_pow_evaluation_strategy);
    register_bench!(runner, benchmark_natural_matrix_mod_pow_algorithms);
}

fn demo_natural_matrix_mod_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp, m) in natural_matrix_unsigned_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.mod_pow({exp}, {m}) = {}", a.mod_pow(exp, &m));
    }
}

fn demo_natural_matrix_mod_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp, m) in natural_matrix_unsigned_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).mod_pow({exp}, {m}) = {}", (&a).mod_pow(exp, &m));
    }
}

fn demo_natural_matrix_mod_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, exp, m) in natural_matrix_unsigned_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_pow_assign(exp, &m);
        println!("a := {a_old}; a.mod_pow_assign({exp}, {m}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_pow(u64, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_unsigned_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.mod_pow(u64, Natural)", &mut |(a, exp, m)| {
                no_out!(a.mod_pow(exp, m));
            }),
            ("NaturalMatrix.mod_pow(u64, &Natural)", &mut |(
                a,
                exp,
                m,
            )| {
                no_out!(a.mod_pow(exp, &m));
            }),
            (
                "(&NaturalMatrix).mod_pow(u64, Natural)",
                &mut |(a, exp, m)| {
                    no_out!((&a).mod_pow(exp, m));
                },
            ),
            (
                "(&NaturalMatrix).mod_pow(u64, &Natural)",
                &mut |(a, exp, m)| {
                    no_out!((&a).mod_pow(exp, &m));
                },
            ),
            (
                "NaturalMatrix.mod_pow_assign(u64, Natural)",
                &mut |(mut a, exp, m)| {
                    a.mod_pow_assign(exp, m);
                },
            ),
            (
                "NaturalMatrix.mod_pow_assign(u64, &Natural)",
                &mut |(mut a, exp, m)| {
                    a.mod_pow_assign(exp, &m);
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_pow(u64, Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_unsigned_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, exp, m)| {
                no_out!((&a).mod_pow(exp, &m));
            }),
            ("naive", &mut |(a, exp, m)| {
                no_out!(natural_matrix_mod_pow_naive(&a, exp, &m));
            }),
        ],
    );
}
