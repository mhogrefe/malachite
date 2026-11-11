// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::Float;
use malachite_float::test_util::bench::bucketers::{
    big_exponent_float_complexity_bucketer, pair_1_big_exponent_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_rounding_mode_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_f64_rounding_from);
    register_demo!(runner, demo_big_exponent_float_f32_rounding_from);
    register_demo!(runner, demo_big_exponent_float_f64_try_from);
    register_demo!(runner, demo_big_exponent_float_f64_convertible_from);

    register_bench!(
        runner,
        benchmark_big_exponent_float_f64_rounding_from_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_f64_rounding_from_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_f64_try_from_evaluation_strategy
    );
    register_bench!(runner, benchmark_big_exponent_float_f64_convertible_from);
}

fn demo_big_exponent_float_f64_rounding_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in big_exponent_float_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .filter(|(x, rm)| *rm != Exact || f64::convertible_from(x))
        .take(limit)
    {
        let (y, o) = f64::rounding_from(&x, rm);
        println!(
            "f64::rounding_from({x:#x}, {rm}) = ({}, {o:?})",
            NiceFloat(y)
        );
    }
}

fn demo_big_exponent_float_f32_rounding_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in big_exponent_float_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .filter(|(x, rm)| *rm != Exact || f32::convertible_from(x))
        .take(limit)
    {
        let (y, o) = f32::rounding_from(&x, rm);
        println!(
            "f32::rounding_from({x:#x}, {rm}) = ({}, {o:?})",
            NiceFloat(y)
        );
    }
}

fn demo_big_exponent_float_f64_try_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "f64::try_from({x:#x}) = {:?}",
            f64::try_from(&x).map(NiceFloat)
        );
    }
}

fn demo_big_exponent_float_f64_convertible_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "f64::convertible_from({x:#x}) = {}",
            f64::convertible_from(&x)
        );
    }
}

fn benchmark_big_exponent_float_f64_rounding_from_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "f64::rounding_from(BigExponentFloat, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rounding_mode_pair_gen_var_1()
            .get(gm, config)
            .filter(|(x, rm)| *rm != Exact || f64::convertible_from(x)),
        gm.name(),
        limit,
        file_name,
        &pair_1_big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("by value", &mut |(x, rm)| {
                no_out!(f64::rounding_from(x, rm));
            }),
            ("by reference", &mut |(x, rm)| {
                no_out!(f64::rounding_from(&x, rm));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_big_exponent_float_f64_rounding_from_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "f64::rounding_from(&BigExponentFloat, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rounding_mode_pair_gen_var_1()
            .get(gm, config)
            .filter(|(x, rm)| {
                Float::convertible_from(x) && (*rm != Exact || f64::convertible_from(x))
            }),
        gm.name(),
        limit,
        file_name,
        &pair_1_big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("BigExponentFloat", &mut |(x, rm)| {
                no_out!(f64::rounding_from(&x, rm));
            }),
            ("Float", &mut |(x, rm)| {
                no_out!(f64::rounding_from(&Float::try_from(&x).unwrap(), rm));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_big_exponent_float_f64_try_from_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "f64::try_from(BigExponentFloat)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("by value", &mut |x| {
                no_out!(f64::try_from(x));
            }),
            ("by reference", &mut |x| {
                no_out!(f64::try_from(&x));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_f64_convertible_from(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "f64::convertible_from(&BigExponentFloat)",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(f64::convertible_from(&x));
        })],
    );
}
