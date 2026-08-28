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
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_try_from_gaussian_integer);
    register_demo!(runner, demo_rational_try_from_gaussian_integer_ref);
    register_demo!(runner, demo_rational_convertible_from_gaussian_integer);

    register_bench!(
        runner,
        benchmark_rational_try_from_gaussian_integer_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_convertible_from_gaussian_integer);
}

fn demo_rational_try_from_gaussian_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        let x_clone = x.clone();
        println!(
            "Rational::try_from({}) = {:?}",
            x_clone,
            Rational::try_from(x)
        );
    }
}

fn demo_rational_try_from_gaussian_integer_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("Rational::try_from(&{}) = {:?}", x, Rational::try_from(&x));
    }
}

fn demo_rational_convertible_from_gaussian_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a Rational",
            x,
            if Rational::convertible_from(&x) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_rational_try_from_gaussian_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::try_from(GaussianInteger)",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("Rational::try_from(GaussianInteger)", &mut |x| {
                no_out!(Rational::try_from(x).ok());
            }),
            ("Rational::try_from(&GaussianInteger)", &mut |x| {
                no_out!(Rational::try_from(&x).ok());
            }),
        ],
    );
}

fn benchmark_rational_convertible_from_gaussian_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::convertible_from(&GaussianInteger)",
        BenchmarkType::Single,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(Rational::convertible_from(&x));
        })],
    );
}
