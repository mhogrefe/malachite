// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Square, ModPowerOf2SquareAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_square::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_power_of_2_square);
    register_demo!(runner, demo_unsigned_polynomial_mod_power_of_2_square_ref);
    register_demo!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_square_assign
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_square_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_square_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_square({pow}) = {}",
            p.mod_power_of_2_square(pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_square_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, _, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_square({pow}) = {}",
            (&p).mod_power_of_2_square(pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_square_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, _, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_square_assign(pow);
        println!("p := {p_old}; p.mod_power_of_2_square_assign({pow}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_power_of_2_square(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            (
                "UnsignedPolynomial<u64>.mod_power_of_2_square(u64)",
                &mut |(p, _, pow)| {
                    no_out!(p.mod_power_of_2_square(pow));
                },
            ),
            (
                "(&UnsignedPolynomial<u64>).mod_power_of_2_square(u64)",
                &mut |(p, _, pow)| {
                    no_out!((&p).mod_power_of_2_square(pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_power_of_2_square(u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, _, pow)| {
                no_out!(p.mod_power_of_2_square(pow));
            }),
            ("naive", &mut |(p, _, pow)| {
                no_out!(mod_power_of_2_square_polynomial_naive(&p, pow));
            }),
        ],
    );
}
