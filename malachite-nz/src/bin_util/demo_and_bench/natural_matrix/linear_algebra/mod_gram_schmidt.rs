// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_matrix::linear_algebra::gram_schmidt::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_gram_schmidt);
    register_demo!(runner, demo_natural_matrix_mod_gram_schmidt_ref_ref);
    register_demo!(runner, demo_natural_matrix_mod_gram_schmidt_smooth_moduli);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_gram_schmidt_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_matrix_mod_gram_schmidt_algorithms);
}

fn demo_natural_matrix_mod_gram_schmidt(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let m_old = m.clone();
        println!(
            "{a_old}.mod_gram_schmidt({m_old}) = {}",
            a.mod_gram_schmidt(m)
                .map_or_else(|| "None".to_string(), |b| b.to_string())
        );
    }
}

fn demo_natural_matrix_mod_gram_schmidt_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_gram_schmidt(&{m}) = {}",
            (&a).mod_gram_schmidt(&m)
                .map_or_else(|| "None".to_string(), |b| b.to_string())
        );
    }
}

fn demo_natural_matrix_mod_gram_schmidt_smooth_moduli(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, m) in natural_matrix_natural_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_gram_schmidt(&{m}) = {}",
            (&a).mod_gram_schmidt(&m)
                .map_or_else(|| "None".to_string(), |b| b.to_string())
        );
    }
}

fn benchmark_natural_matrix_mod_gram_schmidt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_gram_schmidt(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("a.mod_gram_schmidt(m)", &mut |(a, m)| {
                no_out!(a.mod_gram_schmidt(m));
            }),
            ("a.mod_gram_schmidt(&m)", &mut |(a, m)| {
                no_out!(a.mod_gram_schmidt(&m));
            }),
            ("(&a).mod_gram_schmidt(m)", &mut |(a, m)| {
                no_out!((&a).mod_gram_schmidt(m));
            }),
            ("(&a).mod_gram_schmidt(&m)", &mut |(a, m)| {
                no_out!((&a).mod_gram_schmidt(&m));
            }),
        ],
    );
}

fn benchmark_natural_matrix_mod_gram_schmidt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_gram_schmidt(&Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("classical", &mut |(a, m)| {
                no_out!((&a).mod_gram_schmidt(&m));
            }),
            ("modified", &mut |(a, m)| {
                no_out!(mod_gram_schmidt_modified(&a, &m));
            }),
        ],
    );
}
