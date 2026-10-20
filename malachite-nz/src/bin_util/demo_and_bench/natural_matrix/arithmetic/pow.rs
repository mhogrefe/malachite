// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_7;
use malachite_nz::test_util::natural_matrix::arithmetic::pow::natural_matrix_pow_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_pow);
    register_demo!(runner, demo_natural_matrix_pow_ref);
    register_demo!(runner, demo_natural_matrix_pow_assign);

    register_bench!(runner, benchmark_natural_matrix_pow_evaluation_strategy);
    register_bench!(runner, benchmark_natural_matrix_pow_algorithms);
}

fn demo_natural_matrix_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp) in natural_matrix_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.pow({exp}) = {}", a.pow(exp));
    }
}

fn demo_natural_matrix_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp) in natural_matrix_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).pow({exp}) = {}", (&a).pow(exp));
    }
}

fn demo_natural_matrix_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, exp) in natural_matrix_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.pow_assign(exp);
        println!("a := {a_old}; a.pow_assign({exp}); a = {a}");
    }
}

fn benchmark_natural_matrix_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.pow(u64)", &mut |(a, exp)| {
                no_out!(a.pow(exp));
            }),
            ("(&NaturalMatrix).pow(u64)", &mut |(a, exp)| {
                no_out!((&a).pow(exp));
            }),
            ("NaturalMatrix.pow_assign(u64)", &mut |(mut a, exp)| {
                a.pow_assign(exp);
            }),
        ],
    );
}

fn benchmark_natural_matrix_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).pow(u64)",
        BenchmarkType::Algorithms,
        natural_matrix_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, exp)| {
                no_out!((&a).pow(exp));
            }),
            ("naive", &mut |(a, exp)| {
                no_out!(natural_matrix_pow_naive(&a, exp));
            }),
        ],
    );
}
