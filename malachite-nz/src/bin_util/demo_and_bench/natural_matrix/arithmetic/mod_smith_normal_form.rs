// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModSmithNormalForm, ModSmithNormalFormAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_smith_normal_form);
    register_demo!(runner, demo_natural_matrix_mod_smith_normal_form_ref);
    register_demo!(runner, demo_natural_matrix_mod_smith_normal_form_assign);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_smith_normal_form_evaluation_strategy
    );
}

fn demo_natural_matrix_mod_smith_normal_form(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_smith_normal_form({m}) = {}",
            a.mod_smith_normal_form(&m)
        );
    }
}

fn demo_natural_matrix_mod_smith_normal_form_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_smith_normal_form({m}) = {}",
            (&a).mod_smith_normal_form(&m)
        );
    }
}

fn demo_natural_matrix_mod_smith_normal_form_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, m) in natural_matrix_natural_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_smith_normal_form_assign(&m);
        println!("a := {a_old}; a.mod_smith_normal_form_assign({m}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_smith_normal_form_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_smith_normal_form(m)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.mod_smith_normal_form(m)", &mut |(a, m)| {
                no_out!(a.mod_smith_normal_form(&m));
            }),
            (
                "(&NaturalMatrix).mod_smith_normal_form(m)",
                &mut |(a, m)| {
                    no_out!((&a).mod_smith_normal_form(&m));
                },
            ),
            (
                "NaturalMatrix.mod_smith_normal_form_assign(m)",
                &mut |(mut a, m)| {
                    a.mod_smith_normal_form_assign(&m);
                },
            ),
        ],
    );
}
