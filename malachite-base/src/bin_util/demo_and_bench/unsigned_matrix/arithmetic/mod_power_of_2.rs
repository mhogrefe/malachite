// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Assign};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2);
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_ref);
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_assign);

    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_evaluation_strategy
    );
}

fn demo_unsigned_matrix_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, pow) in unsigned_matrix_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let m_old = m.clone();
        println!(
            "({m_old}).mod_power_of_2({pow}) = {}",
            m.mod_power_of_2(pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, pow) in unsigned_matrix_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({m})).mod_power_of_2({pow}) = {}",
            (&m).mod_power_of_2(pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, pow) in unsigned_matrix_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let m_old = m.clone();
        m.mod_power_of_2_assign(pow);
        println!("m := {m_old}; m.mod_power_of_2_assign({pow}); m = {m}");
    }
}

fn benchmark_unsigned_matrix_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64>.mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("m"),
        &mut [
            (
                "UnsignedMatrix<u64>.mod_power_of_2(u64)",
                &mut |(m, pow)| {
                    no_out!(m.mod_power_of_2(pow));
                },
            ),
            (
                "(&UnsignedMatrix<u64>).mod_power_of_2(u64)",
                &mut |(m, pow)| {
                    no_out!((&m).mod_power_of_2(pow));
                },
            ),
            (
                "UnsignedMatrix<u64>.mod_power_of_2_assign(u64)",
                &mut |(mut m, pow)| m.mod_power_of_2_assign(pow),
            ),
        ],
    );
}
