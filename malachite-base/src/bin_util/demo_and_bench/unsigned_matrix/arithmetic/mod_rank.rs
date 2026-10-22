// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModRank;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_rank::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_rank);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_rank_evaluation_strategy
    );
    register_bench!(runner, benchmark_unsigned_matrix_mod_rank_algorithms);
}

fn demo_unsigned_matrix_mod_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_rank({m}) = {}", (&a).mod_rank(m));
    }
}

fn benchmark_unsigned_matrix_mod_rank_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_rank(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("UnsignedMatrix.mod_rank(u64)", &mut |(a, m)| {
                no_out!(a.mod_rank(m));
            }),
            ("(&UnsignedMatrix).mod_rank(u64)", &mut |(a, m)| {
                no_out!((&a).mod_rank(m));
            }),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_rank_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix).mod_rank(u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_rank(m));
            }),
            ("naive", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_rank_naive(&a, m));
            }),
        ],
    );
}
