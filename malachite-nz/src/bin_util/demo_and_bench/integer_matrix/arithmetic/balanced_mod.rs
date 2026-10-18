// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{BalancedMod, BalancedModAssign};
use malachite_base::num::basic::traits::{NegativeOne, One, Two};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_integer_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_balanced_mod);
    register_demo!(runner, demo_integer_matrix_balanced_mod_ref);
    register_demo!(runner, demo_integer_matrix_balanced_mod_assign);
    register_demo!(runner, demo_integer_matrix_balanced_mod_small_moduli);

    register_bench!(
        runner,
        benchmark_integer_matrix_balanced_mod_evaluation_strategy
    );
}

fn demo_integer_matrix_balanced_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in integer_matrix_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.balanced_mod({m}) = {}", a.balanced_mod(&m));
    }
}

fn demo_integer_matrix_balanced_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in integer_matrix_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).balanced_mod({m}) = {}", (&a).balanced_mod(&m));
    }
}

fn demo_integer_matrix_balanced_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, m) in integer_matrix_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.balanced_mod_assign(&m);
        println!("a := {a_old}; a.balanced_mod_assign({m}); a = {a}");
    }
}

fn demo_integer_matrix_balanced_mod_small_moduli(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        for m in [Integer::ONE, Integer::NEGATIVE_ONE, Integer::TWO] {
            println!("(&{a}).balanced_mod({m}) = {}", (&a).balanced_mod(&m));
        }
    }
}

fn benchmark_integer_matrix_balanced_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.balanced_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.balanced_mod(Integer)", &mut |(a, m)| {
                no_out!(a.balanced_mod(m));
            }),
            ("IntegerMatrix.balanced_mod(&Integer)", &mut |(a, m)| {
                no_out!(a.balanced_mod(&m));
            }),
            ("(&IntegerMatrix).balanced_mod(Integer)", &mut |(a, m)| {
                no_out!((&a).balanced_mod(m));
            }),
            ("(&IntegerMatrix).balanced_mod(&Integer)", &mut |(a, m)| {
                no_out!((&a).balanced_mod(&m));
            }),
            (
                "IntegerMatrix.balanced_mod_assign(Integer)",
                &mut |(mut a, m)| {
                    a.balanced_mod_assign(m);
                },
            ),
            (
                "IntegerMatrix.balanced_mod_assign(&Integer)",
                &mut |(mut a, m)| {
                    a.balanced_mod_assign(&m);
                },
            ),
        ],
    );
}
