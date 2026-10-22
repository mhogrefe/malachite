// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModNullspace;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_nullspace::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_nullspace);
    register_demo!(runner, demo_natural_matrix_mod_nullspace_large_dimensions);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_nullspace_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_matrix_mod_nullspace_algorithms);
}

fn demo_natural_matrix_mod_nullspace(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_nullspace({m}) = {}", (&a).mod_nullspace(&m));
    }
}

fn demo_natural_matrix_mod_nullspace_large_dimensions(
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
        println!("{a}.mod_nullspace({m}) = {}", (&a).mod_nullspace(&m));
    }
}

fn benchmark_natural_matrix_mod_nullspace_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_nullspace(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.mod_nullspace(Natural)", &mut |(a, m)| {
                no_out!(a.mod_nullspace(m));
            }),
            ("NaturalMatrix.mod_nullspace(&Natural)", &mut |(a, m)| {
                no_out!(a.mod_nullspace(&m));
            }),
            ("(&NaturalMatrix).mod_nullspace(Natural)", &mut |(a, m)| {
                no_out!((&a).mod_nullspace(m));
            }),
            ("(&NaturalMatrix).mod_nullspace(&Natural)", &mut |(a, m)| {
                no_out!((&a).mod_nullspace(&m));
            }),
        ],
    );
}

fn benchmark_natural_matrix_mod_nullspace_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_nullspace(&Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_nullspace(&m));
            }),
            ("naive", &mut |(a, m)| {
                no_out!(natural_matrix_mod_nullspace_naive(&a, &m));
            }),
        ],
    );
}
