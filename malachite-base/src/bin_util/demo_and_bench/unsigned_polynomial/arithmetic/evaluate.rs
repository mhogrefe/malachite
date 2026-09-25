// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::EvaluateModPowerOf2;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::evaluate::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_evaluate_mod_power_of_2);
    register_demo!(runner, demo_unsigned_polynomial_evaluate_mod_power_of_2_ref);

    register_bench!(
        runner,
        benchmark_unsigned_polynomial_evaluate_mod_power_of_2_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_evaluate_mod_power_of_2_algorithms
    );
}

fn demo_unsigned_polynomial_evaluate_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).evaluate_mod_power_of_2({x}, {pow}) = {}",
            p.evaluate_mod_power_of_2(x, pow)
        );
    }
}

fn demo_unsigned_polynomial_evaluate_mod_power_of_2_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_mod_power_of_2({x}, {pow}) = {}",
            (&p).evaluate_mod_power_of_2(x, pow)
        );
    }
}

fn benchmark_unsigned_polynomial_evaluate_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.evaluate_mod_power_of_2(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_bit_bucketer("p"),
        &mut [
            (
                "UnsignedPolynomial<u64>.evaluate_mod_power_of_2(u64, u64)",
                &mut |(p, x, pow)| no_out!(p.evaluate_mod_power_of_2(x, pow)),
            ),
            (
                "(&UnsignedPolynomial<u64>).evaluate_mod_power_of_2(u64, u64)",
                &mut |(p, x, pow)| no_out!((&p).evaluate_mod_power_of_2(x, pow)),
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_evaluate_mod_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.evaluate_mod_power_of_2(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x, pow)| {
                no_out!(p.evaluate_mod_power_of_2(x, pow));
            }),
            ("naive", &mut |(p, x, pow)| {
                no_out!(evaluate_mod_power_of_2_naive(&p, x, pow));
            }),
        ],
    );
}
