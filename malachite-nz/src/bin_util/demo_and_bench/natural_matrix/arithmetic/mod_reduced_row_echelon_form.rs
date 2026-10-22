// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModReducedRowEchelonForm, ModReducedRowEchelonFormAssign,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_reduced_row_echelon_form::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_reduced_row_echelon_form);
    register_demo!(
        runner,
        demo_natural_matrix_mod_reduced_row_echelon_form_large_dimensions
    );
    register_demo!(runner, demo_natural_matrix_mod_reduced_row_echelon_form_ref);
    register_demo!(
        runner,
        demo_natural_matrix_mod_reduced_row_echelon_form_assign
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_reduced_row_echelon_form_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_reduced_row_echelon_form_algorithms
    );
}

fn demo_natural_matrix_mod_reduced_row_echelon_form(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let m_old = m.clone();
        println!(
            "{a_old}.mod_reduced_row_echelon_form({m_old}) = {}",
            a.mod_reduced_row_echelon_form(m)
        );
    }
}

fn demo_natural_matrix_mod_reduced_row_echelon_form_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, m) in natural_matrix_natural_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_reduced_row_echelon_form(&{m}) = {}",
            (&a).mod_reduced_row_echelon_form(&m)
        );
    }
}

fn demo_natural_matrix_mod_reduced_row_echelon_form_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, m) in natural_matrix_natural_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_reduced_row_echelon_form_assign(&m);
        println!("a := {a_old}; a.mod_reduced_row_echelon_form_assign({m}); a = {a}");
    }
}

fn demo_natural_matrix_mod_reduced_row_echelon_form_large_dimensions(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    let mut config = config.clone();
    config.insert("mean_dimension_n", 30);
    for (a, m) in natural_matrix_natural_pair_gen_var_5()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{a}.mod_reduced_row_echelon_form({m}) = {}",
            (&a).mod_reduced_row_echelon_form(&m)
        );
    }
}

fn benchmark_natural_matrix_mod_reduced_row_echelon_form_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_reduced_row_echelon_form(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_reduced_row_echelon_form(Natural)",
                &mut |(a, m)| {
                    no_out!(a.mod_reduced_row_echelon_form(m));
                },
            ),
            (
                "NaturalMatrix.mod_reduced_row_echelon_form(&Natural)",
                &mut |(a, m)| {
                    no_out!(a.mod_reduced_row_echelon_form(&m));
                },
            ),
            (
                "(&NaturalMatrix).mod_reduced_row_echelon_form(Natural)",
                &mut |(a, m)| {
                    no_out!((&a).mod_reduced_row_echelon_form(m));
                },
            ),
            (
                "(&NaturalMatrix).mod_reduced_row_echelon_form(&Natural)",
                &mut |(a, m)| {
                    no_out!((&a).mod_reduced_row_echelon_form(&m));
                },
            ),
            (
                "NaturalMatrix.mod_reduced_row_echelon_form_assign(Natural)",
                &mut |(mut a, m)| {
                    a.mod_reduced_row_echelon_form_assign(m);
                },
            ),
            (
                "NaturalMatrix.mod_reduced_row_echelon_form_assign(&Natural)",
                &mut |(mut a, m)| {
                    a.mod_reduced_row_echelon_form_assign(&m);
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_reduced_row_echelon_form_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_reduced_row_echelon_form(&Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_reduced_row_echelon_form(&m));
            }),
            ("naive", &mut |(a, m)| {
                no_out!(natural_matrix_mod_reduced_row_echelon_form_naive(&a, &m));
            }),
        ],
    );
}
