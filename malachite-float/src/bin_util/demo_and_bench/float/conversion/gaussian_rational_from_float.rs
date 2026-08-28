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
use malachite_float::test_util::bench::bucketers::float_complexity_bucketer;
use malachite_float::test_util::generators::float_gen;
use malachite_q::gaussian_rational::GaussianRational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_try_from_float);
    register_demo!(runner, demo_gaussian_rational_try_from_float_ref);
    register_demo!(runner, demo_gaussian_rational_convertible_from_float);

    register_bench!(
        runner,
        benchmark_gaussian_rational_try_from_float_evaluation_strategy
    );
    register_bench!(runner, benchmark_gaussian_rational_convertible_from_float);
}

fn demo_gaussian_rational_try_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_clone = x.clone();
        println!(
            "GaussianRational::try_from({}) = {:?}",
            x_clone,
            GaussianRational::try_from(x)
        );
    }
}

fn demo_gaussian_rational_try_from_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "GaussianRational::try_from(&{}) = {:?}",
            x,
            GaussianRational::try_from(&x)
        );
    }
}

fn demo_gaussian_rational_convertible_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a GaussianRational",
            x,
            if GaussianRational::convertible_from(&x) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_gaussian_rational_try_from_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational::try_from(Float)",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("GaussianRational::try_from(Float)", &mut |x| {
                no_out!(GaussianRational::try_from(x).ok());
            }),
            ("GaussianRational::try_from(&Float)", &mut |x| {
                no_out!(GaussianRational::try_from(&x).ok());
            }),
        ],
    );
}

fn benchmark_gaussian_rational_convertible_from_float(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational::convertible_from(&Float)",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(GaussianRational::convertible_from(&x));
        })],
    );
}
