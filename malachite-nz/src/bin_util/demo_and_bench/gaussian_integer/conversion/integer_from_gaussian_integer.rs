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
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::gaussian_integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_try_from_gaussian_integer);
    register_demo!(runner, demo_integer_try_from_gaussian_integer_ref);
    register_demo!(runner, demo_integer_convertible_from_gaussian_integer);

    register_bench!(
        runner,
        benchmark_integer_try_from_gaussian_integer_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_convertible_from_gaussian_integer);
}

fn demo_integer_try_from_gaussian_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        let x_clone = x.clone();
        println!(
            "Integer::try_from({}) = {:?}",
            x_clone,
            Integer::try_from(x)
        );
    }
}

fn demo_integer_try_from_gaussian_integer_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("Integer::try_from(&{}) = {:?}", x, Integer::try_from(&x));
    }
}

fn demo_integer_convertible_from_gaussian_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to an Integer",
            x,
            if Integer::convertible_from(&x) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_integer_try_from_gaussian_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::try_from(GaussianInteger)",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("Integer::try_from(GaussianInteger)", &mut |x| {
                no_out!(Integer::try_from(x).ok());
            }),
            ("Integer::try_from(&GaussianInteger)", &mut |x| {
                no_out!(Integer::try_from(&x).ok());
            }),
        ],
    );
}

fn benchmark_integer_convertible_from_gaussian_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::convertible_from(&GaussianInteger)",
        BenchmarkType::Single,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(Integer::convertible_from(&x)))],
    );
}
