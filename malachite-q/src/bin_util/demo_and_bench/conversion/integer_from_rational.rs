// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, rational_bit_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_rounding_mode_pair_gen_var_2};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_try_from_rational);
    register_demo!(runner, demo_integer_try_from_rational_ref);
    register_demo!(runner, demo_integer_convertible_from_rational);
    register_demo!(runner, demo_integer_rounding_from_rational);
    register_demo!(runner, demo_integer_rounding_from_rational_ref);

    register_bench!(
        runner,
        benchmark_integer_try_from_rational_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_convertible_from_rational);
    register_bench!(
        runner,
        benchmark_integer_rounding_from_rational_evaluation_strategy
    );
}

fn demo_integer_try_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        let x_clone = x.clone();
        println!(
            "Integer::try_from({}) = {:?}",
            x_clone,
            Integer::try_from(x)
        );
    }
}

fn demo_integer_try_from_rational_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!("Integer::try_from(&{}) = {:?}", x, Integer::try_from(&x));
    }
}

fn demo_integer_convertible_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a Integer",
            x,
            if Integer::convertible_from(&x) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_integer_rounding_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in rational_rounding_mode_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_clone = x.clone();
        println!(
            "Integer::rounding_from({}, {}) = {:?}",
            x_clone,
            rm,
            Integer::rounding_from(x, rm)
        );
    }
}

fn demo_integer_rounding_from_rational_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in rational_rounding_mode_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Integer::rounding_from(&{}, {}) = {:?}",
            x,
            rm,
            Integer::rounding_from(&x, rm)
        );
    }
}

fn benchmark_integer_try_from_rational_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::try_from(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("Integer::try_from(Rational)", &mut |x| {
                no_out!(Integer::try_from(x).ok())
            }),
            ("Integer::try_from(&Rational)", &mut |x| {
                no_out!(Integer::try_from(&x).ok())
            }),
        ],
    );
}

fn benchmark_integer_convertible_from_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::convertible_from(Rational)",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(Integer::convertible_from(&x)))],
    );
}

fn benchmark_integer_rounding_from_rational_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::rounding_from(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_rounding_mode_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Integer::rounding_from(Rational)", &mut |(x, rm)| {
                no_out!(Integer::rounding_from(x, rm))
            }),
            ("Integer::rounding_from(&Rational)", &mut |(x, rm)| {
                no_out!(Integer::rounding_from(&x, rm))
            }),
        ],
    );
}
