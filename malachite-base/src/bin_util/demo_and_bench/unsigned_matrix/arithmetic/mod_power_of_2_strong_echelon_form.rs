// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2StrongEchelonForm, ModPowerOf2StrongEchelonFormAssign, ModStrongEchelonForm,
};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_unsigned_matrix_mod_power_of_2_strong_echelon_form
    );
    register_demo!(
        runner,
        demo_unsigned_matrix_mod_power_of_2_strong_echelon_form_ref
    );
    register_demo!(
        runner,
        demo_unsigned_matrix_mod_power_of_2_strong_echelon_form_assign
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_strong_echelon_form_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_strong_echelon_form_algorithms
    );
}

fn demo_unsigned_matrix_mod_power_of_2_strong_echelon_form(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, pow) in unsigned_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_power_of_2_strong_echelon_form({pow}) = {}",
            a.mod_power_of_2_strong_echelon_form(pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_strong_echelon_form_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, pow) in unsigned_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_power_of_2_strong_echelon_form({pow}) = {}",
            (&a).mod_power_of_2_strong_echelon_form(pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_strong_echelon_form_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, pow) in unsigned_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_power_of_2_strong_echelon_form_assign(pow);
        println!("a := {a_old}; a.mod_power_of_2_strong_echelon_form_assign({pow}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_power_of_2_strong_echelon_form_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_power_of_2_strong_echelon_form(pow)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_power_of_2_strong_echelon_form(pow)",
                &mut |(a, pow)| {
                    no_out!(a.mod_power_of_2_strong_echelon_form(pow));
                },
            ),
            (
                "(&UnsignedMatrix).mod_power_of_2_strong_echelon_form(pow)",
                &mut |(a, pow)| {
                    no_out!((&a).mod_power_of_2_strong_echelon_form(pow));
                },
            ),
            (
                "UnsignedMatrix.mod_power_of_2_strong_echelon_form_assign(pow)",
                &mut |(mut a, pow)| {
                    a.mod_power_of_2_strong_echelon_form_assign(pow);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_power_of_2_strong_echelon_form_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_power_of_2_strong_echelon_form(pow)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_4()
            .get(gm, config)
            .filter(|(_, pow)| *pow < 64),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, pow)| {
                no_out!(a.mod_power_of_2_strong_echelon_form(pow));
            }),
            ("modulo 2^pow, general", &mut |(a, pow)| {
                no_out!(a.mod_strong_echelon_form(1u64 << pow));
            }),
        ],
    );
}
