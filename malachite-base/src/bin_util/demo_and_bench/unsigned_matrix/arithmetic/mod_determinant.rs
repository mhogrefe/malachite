// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModDeterminant;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_determinant::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_determinant);
    register_demo!(
        runner,
        demo_unsigned_matrix_mod_determinant_large_dimensions
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_determinant_evaluation_strategy
    );
    register_bench!(runner, benchmark_unsigned_matrix_mod_determinant_algorithms);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_determinant_prime_algorithms
    );
}

fn demo_unsigned_matrix_mod_determinant(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_determinant({m}) = {}", (&a).mod_determinant(m));
    }
}

fn demo_unsigned_matrix_mod_determinant_large_dimensions(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    let mut config = config.clone();
    config.insert("mean_dimension_n", 60);
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_11()
        .get(gm, &config)
        .take(limit)
    {
        println!("{a}.mod_determinant({m}) = {}", (&a).mod_determinant(m));
    }
}

fn benchmark_unsigned_matrix_mod_determinant_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_determinant(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("UnsignedMatrix.mod_determinant(u64)", &mut |(a, m)| {
                no_out!(a.mod_determinant(m));
            }),
            ("(&UnsignedMatrix).mod_determinant(u64)", &mut |(a, m)| {
                no_out!((&a).mod_determinant(m));
            }),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_determinant_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix).mod_determinant(u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_determinant(m));
            }),
            ("Berkowitz", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_determinant_berkowitz(&a, m));
            }),
            ("Howell", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_determinant_howell::<u64, i64>(&a, m));
            }),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_determinant_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix).mod_determinant(u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_determinant(m));
            }),
            ("Berkowitz", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_determinant_berkowitz(&a, m));
            }),
            ("Howell", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_determinant_howell::<u64, i64>(&a, m));
            }),
            ("LU", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_determinant_lu(&a, m));
            }),
        ],
    );
}
