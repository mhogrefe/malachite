// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::big_exponent_float_complexity_bucketer;
use malachite_float::test_util::generators::big_exponent_float_gen_var_2;
use malachite_nz::gaussian_integer::GaussianInteger;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_gaussian_integer_try_from);
    register_demo!(
        runner,
        demo_big_exponent_float_gaussian_integer_convertible_from
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_gaussian_integer_try_from_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_gaussian_integer_convertible_from
    );
}

fn demo_big_exponent_float_gaussian_integer_try_from(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in big_exponent_float_gen_var_2().get(gm, config).take(limit) {
        println!(
            "GaussianInteger::try_from({x:#x}) = {:?}",
            GaussianInteger::try_from(&x)
        );
    }
}

fn demo_big_exponent_float_gaussian_integer_convertible_from(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in big_exponent_float_gen_var_2().get(gm, config).take(limit) {
        println!(
            "GaussianInteger::convertible_from({x:#x}) = {}",
            GaussianInteger::convertible_from(&x)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_big_exponent_float_gaussian_integer_try_from_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger::try_from(BigExponentFloat)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("by value", &mut |x| {
                no_out!(GaussianInteger::try_from(x));
            }),
            ("by reference", &mut |x| {
                no_out!(GaussianInteger::try_from(&x));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_gaussian_integer_convertible_from(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger::convertible_from(&BigExponentFloat)",
        BenchmarkType::Single,
        big_exponent_float_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(GaussianInteger::convertible_from(&x));
        })],
    );
}
