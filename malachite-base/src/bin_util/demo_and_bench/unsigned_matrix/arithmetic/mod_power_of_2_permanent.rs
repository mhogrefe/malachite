// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::ModPowerOf2Permanent;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_permanent::*;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_permanent);
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_permanent_ref);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_permanent_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_permanent_algorithms
    );
}

fn demo_unsigned_matrix_mod_power_of_2_permanent(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in unsigned_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .filter(|(a, _)| a.is_square() && a.height() <= 8)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_power_of_2_permanent({pow}) = {}",
            a.mod_power_of_2_permanent(pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_permanent_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, pow) in unsigned_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .filter(|(a, _)| a.is_square() && a.height() <= 8)
        .take(limit)
    {
        println!(
            "(&{a}).mod_power_of_2_permanent({pow}) = {}",
            (&a).mod_power_of_2_permanent(pow)
        );
    }
}

fn benchmark_unsigned_matrix_mod_power_of_2_permanent_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_power_of_2_permanent(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_4()
            .get(gm, config)
            .filter(|(a, _)| a.is_square() && a.height() <= 8),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_power_of_2_permanent(u64)",
                &mut |(a, pow)| {
                    no_out!(a.mod_power_of_2_permanent(pow));
                },
            ),
            (
                "(&UnsignedMatrix).mod_power_of_2_permanent(u64)",
                &mut |(a, pow)| {
                    no_out!((&a).mod_power_of_2_permanent(pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_power_of_2_permanent_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_power_of_2_permanent(u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_4()
            .get(gm, config)
            .filter(|(a, _)| a.is_square() && a.height() <= 8),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, pow)| {
                no_out!(a.mod_power_of_2_permanent(pow));
            }),
            ("without looking for structure", &mut |(a, pow)| {
                let n = usize::exact_from(a.height());
                no_out!(ModPowerOf2Ring::new(pow).permanent_dense(a.elements_ref(), n));
            }),
            ("without Glynn's formula", &mut |(a, pow)| {
                let n = usize::exact_from(a.height());
                no_out!(permanent_dense(
                    &ModPowerOf2Ring::new(pow),
                    a.elements_ref(),
                    n
                ));
            }),
            ("naive", &mut |(a, pow)| {
                no_out!(unsigned_matrix_mod_power_of_2_permanent_naive(&a, pow));
            }),
        ],
    );
}
