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
use malachite_nz::natural::Natural;
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, rational_bit_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_rounding_mode_pair_gen_var_1};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_try_from_rational);
    register_demo!(runner, demo_natural_try_from_rational_ref);
    register_demo!(runner, demo_natural_convertible_from_rational);
    register_demo!(runner, demo_natural_rounding_from_rational);
    register_demo!(runner, demo_natural_rounding_from_rational_ref);

    register_bench!(
        runner,
        benchmark_natural_try_from_rational_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_convertible_from_rational);
    register_bench!(
        runner,
        benchmark_natural_rounding_from_rational_evaluation_strategy
    );
}

fn demo_natural_try_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        let x_clone = x.clone();
        println!(
            "Natural::try_from({}) = {:?}",
            x_clone,
            Natural::try_from(x)
        );
    }
}

fn demo_natural_try_from_rational_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!("Natural::try_from(&{}) = {:?}", x, Natural::try_from(&x));
    }
}

fn demo_natural_convertible_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a Natural",
            x,
            if Natural::convertible_from(&x) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_natural_rounding_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in rational_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_clone = x.clone();
        println!(
            "Natural::rounding_from({}, {}) = {:?}",
            x_clone,
            rm,
            Natural::rounding_from(x, rm)
        );
    }
}

fn demo_natural_rounding_from_rational_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in rational_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::rounding_from(&{}, {}) = {:?}",
            x,
            rm,
            Natural::rounding_from(&x, rm)
        );
    }
}

fn benchmark_natural_try_from_rational_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::try_from(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("Natural::try_from(Rational)", &mut |x| {
                no_out!(Natural::try_from(x).ok())
            }),
            ("Natural::try_from(&Rational)", &mut |x| {
                no_out!(Natural::try_from(&x).ok())
            }),
        ],
    );
}

fn benchmark_natural_convertible_from_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::convertible_from(Rational)",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(Natural::convertible_from(&x)))],
    );
}

fn benchmark_natural_rounding_from_rational_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::rounding_from(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_rounding_mode_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Natural::rounding_from(Rational)", &mut |(x, rm)| {
                no_out!(Natural::rounding_from(x, rm))
            }),
            ("Natural::rounding_from(&Rational)", &mut |(x, rm)| {
                no_out!(Natural::rounding_from(&x, rm))
            }),
        ],
    );
}
