// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{SmithNormalForm, SmithNormalFormAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_smith_normal_form);
    register_demo!(runner, demo_natural_matrix_smith_normal_form_val);
    register_demo!(runner, demo_natural_matrix_smith_normal_form_assign);
    register_bench!(
        runner,
        benchmark_natural_matrix_smith_normal_form_evaluation_strategy
    );
}

fn demo_natural_matrix_smith_normal_form(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in natural_matrix_gen().get(gm, config).take(limit) {
        println!("smith_normal_form({a}) = {}", (&a).smith_normal_form());
    }
}

fn demo_natural_matrix_smith_normal_form_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in natural_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old}.smith_normal_form() = {}", a.smith_normal_form());
    }
}

fn demo_natural_matrix_smith_normal_form_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut a in natural_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.smith_normal_form_assign();
        println!("a := {a_old}; a.smith_normal_form_assign(); a = {a}");
    }
}

fn benchmark_natural_matrix_smith_normal_form_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.smith_normal_form()",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.smith_normal_form()", &mut |a| {
                no_out!(a.smith_normal_form());
            }),
            ("(&NaturalMatrix).smith_normal_form()", &mut |a| {
                no_out!((&a).smith_normal_form());
            }),
            ("NaturalMatrix.smith_normal_form_assign()", &mut |mut a| {
                a.smith_normal_form_assign();
            }),
        ],
    );
}
