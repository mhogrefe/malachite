// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModInverse, ModSolve};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_inverse);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_inverse_evaluation_strategy
    );
    register_bench!(runner, benchmark_unsigned_matrix_mod_inverse_algorithms);
}

fn demo_unsigned_matrix_mod_inverse(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_12()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_inverse({m}) = {:?}", (&a).mod_inverse(m));
    }
}

fn benchmark_unsigned_matrix_mod_inverse_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_inverse(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("UnsignedMatrix.mod_inverse(u64)", &mut |(a, m)| {
                no_out!(a.mod_inverse(m));
            }),
            ("(&UnsignedMatrix).mod_inverse(u64)", &mut |(a, m)| {
                no_out!((&a).mod_inverse(m));
            }),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_inverse_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix).mod_inverse(u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_inverse(m));
            }),
            ("solving against the identity", &mut |(a, m)| {
                let n = a.height();
                no_out!((&a).mod_solve(UnsignedMatrix::identity(n), m));
            }),
        ],
    );
}
