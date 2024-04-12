// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedRoot;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_signed_pair_gen_var_4, rational_unsigned_pair_gen_var_4,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_checked_root_u64);
    register_demo!(runner, demo_rational_checked_root_u64_ref);
    register_demo!(runner, demo_rational_checked_root_i64);
    register_demo!(runner, demo_rational_checked_root_i64_ref);

    register_bench!(
        runner,
        benchmark_rational_checked_root_u64_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_checked_root_i64_evaluation_strategy
    );
}

fn demo_rational_checked_root_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in rational_unsigned_pair_gen_var_4::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).checked_root({}) = {:?}",
            x,
            exp,
            x.clone().checked_root(exp)
        );
    }
}

fn demo_rational_checked_root_u64_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in rational_unsigned_pair_gen_var_4::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).checked_root({}) = {:?}",
            x,
            exp,
            (&x).checked_root(exp)
        );
    }
}

fn demo_rational_checked_root_i64(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in rational_signed_pair_gen_var_4::<i64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).checked_root({}) = {:?}",
            x,
            exp,
            x.clone().checked_root(exp)
        );
    }
}

fn demo_rational_checked_root_i64_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in rational_signed_pair_gen_var_4::<i64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).checked_root({}) = {:?}",
            x,
            exp,
            (&x).checked_root(exp)
        );
    }
}

fn benchmark_rational_checked_root_u64_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.checked_root(u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_4::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Rational.checked_root(u64)", &mut |(x, exp)| {
                no_out!(x.checked_root(exp))
            }),
            ("(&Rational).checked_root(u64)", &mut |(x, exp)| {
                no_out!((&x).checked_root(exp))
            }),
        ],
    );
}

fn benchmark_rational_checked_root_i64_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.checked_root(i64)",
        BenchmarkType::EvaluationStrategy,
        rational_signed_pair_gen_var_4::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Rational.checked_root(i64)", &mut |(x, exp)| {
                no_out!(x.checked_root(exp))
            }),
            ("(&Rational).checked_root(i64)", &mut |(x, exp)| {
                no_out!((&x).checked_root(exp))
            }),
        ],
    );
}
