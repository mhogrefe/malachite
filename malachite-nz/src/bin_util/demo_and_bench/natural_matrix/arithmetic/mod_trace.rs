// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModTrace;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_to_mod_trace);
    register_demo!(runner, demo_natural_matrix_into_mod_trace);
    register_demo!(runner, demo_natural_matrix_to_mod_trace_ref);
    register_demo!(runner, demo_natural_matrix_into_mod_trace_ref);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_trace_evaluation_strategy
    );
}

fn demo_natural_matrix_to_mod_trace(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.to_mod_trace({}) = {}", a, m, a.to_mod_trace(m.clone()));
    }
}

fn demo_natural_matrix_into_mod_trace(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{}.into_mod_trace({}) = {}",
            a_old,
            m,
            a.into_mod_trace(m.clone())
        );
    }
}

fn demo_natural_matrix_to_mod_trace_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.to_mod_trace({}) = {}", a, m, a.to_mod_trace(&m));
    }
}

fn demo_natural_matrix_into_mod_trace_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{}.into_mod_trace({}) = {}", a_old, m, a.into_mod_trace(&m));
    }
}

fn benchmark_natural_matrix_mod_trace_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.to_mod_trace(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.to_mod_trace(Natural)", &mut |(a, m)| {
                no_out!(a.to_mod_trace(m));
            }),
            ("NaturalMatrix.into_mod_trace(Natural)", &mut |(a, m)| {
                no_out!(a.into_mod_trace(m));
            }),
            ("NaturalMatrix.to_mod_trace(&Natural)", &mut |(a, m)| {
                no_out!(a.to_mod_trace(&m));
            }),
            ("NaturalMatrix.into_mod_trace(&Natural)", &mut |(a, m)| {
                no_out!(a.into_mod_trace(&m));
            }),
        ],
    );
}
