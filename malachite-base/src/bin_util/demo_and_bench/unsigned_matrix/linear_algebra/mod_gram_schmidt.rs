// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::linear_algebra::gram_schmidt::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_gram_schmidt);
    register_demo!(runner, demo_unsigned_matrix_mod_gram_schmidt_ref);
    register_demo!(runner, demo_unsigned_matrix_mod_gram_schmidt_small_moduli);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_gram_schmidt_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_gram_schmidt_algorithms
    );
}

fn demo_unsigned_matrix_mod_gram_schmidt(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_gram_schmidt({m}) = {}",
            a.mod_gram_schmidt(m)
                .map_or_else(|| "None".to_string(), |b| b.to_string())
        );
    }
}

fn demo_unsigned_matrix_mod_gram_schmidt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_gram_schmidt({m}) = {}",
            (&a).mod_gram_schmidt(m)
                .map_or_else(|| "None".to_string(), |b| b.to_string())
        );
    }
}

fn demo_unsigned_matrix_mod_gram_schmidt_small_moduli(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_gram_schmidt({m}) = {}",
            (&a).mod_gram_schmidt(m)
                .map_or_else(|| "None".to_string(), |b| b.to_string())
        );
    }
}

fn benchmark_unsigned_matrix_mod_gram_schmidt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_gram_schmidt(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("UnsignedMatrix.mod_gram_schmidt(u64)", &mut |(a, m)| {
                no_out!(a.mod_gram_schmidt(m));
            }),
            ("(&UnsignedMatrix).mod_gram_schmidt(u64)", &mut |(a, m)| {
                no_out!((&a).mod_gram_schmidt(m));
            }),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_gram_schmidt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix).mod_gram_schmidt(u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("classical", &mut |(a, m)| {
                no_out!((&a).mod_gram_schmidt(m));
            }),
            ("modified", &mut |(a, m)| {
                no_out!(mod_gram_schmidt_modified(&a, m));
            }),
        ],
    );
}
