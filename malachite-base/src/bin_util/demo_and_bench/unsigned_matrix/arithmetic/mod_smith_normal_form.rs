// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModSmithNormalForm, ModSmithNormalFormAssign};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_9;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_smith_normal_form);
    register_demo!(runner, demo_unsigned_matrix_mod_smith_normal_form_ref);
    register_demo!(runner, demo_unsigned_matrix_mod_smith_normal_form_assign);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_smith_normal_form_evaluation_strategy
    );
}

fn demo_unsigned_matrix_mod_smith_normal_form(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_smith_normal_form({m}) = {}",
            a.mod_smith_normal_form(m)
        );
    }
}

fn demo_unsigned_matrix_mod_smith_normal_form_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_smith_normal_form({m}) = {}",
            (&a).mod_smith_normal_form(m)
        );
    }
}

fn demo_unsigned_matrix_mod_smith_normal_form_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, m) in unsigned_matrix_unsigned_pair_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_smith_normal_form_assign(m);
        println!("a := {a_old}; a.mod_smith_normal_form_assign({m}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_smith_normal_form_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_smith_normal_form(m)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("UnsignedMatrix.mod_smith_normal_form(m)", &mut |(a, m)| {
                no_out!(a.mod_smith_normal_form(m));
            }),
            (
                "(&UnsignedMatrix).mod_smith_normal_form(m)",
                &mut |(a, m)| {
                    no_out!((&a).mod_smith_normal_form(m));
                },
            ),
            (
                "UnsignedMatrix.mod_smith_normal_form_assign(m)",
                &mut |(mut a, m)| {
                    a.mod_smith_normal_form_assign(m);
                },
            ),
        ],
    );
}
