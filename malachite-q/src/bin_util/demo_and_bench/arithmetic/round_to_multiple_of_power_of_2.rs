// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    PowerOf2, RoundToMultiple, RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::triple_1_2_rational_bit_i64_max_bucketer;
use malachite_q::test_util::generators::rational_signed_rounding_mode_triple_gen_var_1;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_round_to_multiple_of_power_of_2_assign);
    register_demo!(runner, demo_rational_round_to_multiple_of_power_of_2);
    register_demo!(runner, demo_rational_round_to_multiple_of_power_of_2_ref);

    register_bench!(
        runner,
        benchmark_rational_round_to_multiple_of_power_of_2_assign
    );
    register_bench!(
        runner,
        benchmark_rational_round_to_multiple_of_power_of_2_algorithms
    );
    register_bench!(
        runner,
        benchmark_rational_round_to_multiple_of_power_of_2_evaluation_strategy
    );
}

fn demo_rational_round_to_multiple_of_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, pow, rm) in rational_signed_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.round_to_multiple_of_power_of_2_assign(pow, rm);
        println!(
            "x := {n_old}; x.round_to_multiple_of_power_of_2_assign({pow}, {rm}) = {o:?}; x = {n}"
        );
    }
}

fn demo_rational_round_to_multiple_of_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow, rm) in rational_signed_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "({}).round_to_multiple_of_power_of_2({}, {}) = {:?}",
            n_old,
            pow,
            rm,
            n.round_to_multiple_of_power_of_2(pow, rm)
        );
    }
}

fn demo_rational_round_to_multiple_of_power_of_2_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, pow, rm) in rational_signed_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).round_to_multiple_of_power_of_2({}, {}) = {:?}",
            n,
            pow,
            rm,
            (&n).round_to_multiple_of_power_of_2(pow, rm)
        );
    }
}

fn benchmark_rational_round_to_multiple_of_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.round_to_multiple_of_power_of_2_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        rational_signed_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_i64_max_bucketer("n", "pow"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.round_to_multiple_of_power_of_2_assign(y, rm))
        })],
    );
}

#[allow(clippy::unnecessary_operation, unused_must_use)]
fn benchmark_rational_round_to_multiple_of_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.round_to_multiple_of_power_of_2(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        rational_signed_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_i64_max_bucketer("n", "pow"),
        &mut [
            ("default", &mut |(x, y, rm)| {
                no_out!(x.round_to_multiple_of_power_of_2(y, rm))
            }),
            ("using round_to_multiple", &mut |(x, y, rm)| {
                no_out!(x.round_to_multiple(Rational::power_of_2(y), rm))
            }),
        ],
    );
}

fn benchmark_rational_round_to_multiple_of_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.round_to_multiple_of_power_of_2(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_signed_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_i64_max_bucketer("n", "pow"),
        &mut [
            (
                "Rational.round_to_multiple_of_power_of_2(u64, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.round_to_multiple_of_power_of_2(y, rm)),
            ),
            (
                "(&Rational).round_to_multiple_of_power_of_2(u64, RoundingMode)",
                &mut |(x, y, rm)| no_out!((&x).round_to_multiple_of_power_of_2(y, rm)),
            ),
        ],
    );
}
