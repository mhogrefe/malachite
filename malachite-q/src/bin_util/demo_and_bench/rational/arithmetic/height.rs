// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_to_height);
    register_demo!(runner, demo_rational_into_height);
    register_demo!(runner, demo_rational_height_significant_bits);

    register_bench!(runner, benchmark_rational_height_evaluation_strategy);
    register_bench!(runner, benchmark_rational_height_significant_bits);
}

fn demo_rational_to_height(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!("{x}.to_height() = {}", x.to_height());
    }
}

fn demo_rational_into_height(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{x_old}.into_height() = {}", x.into_height());
    }
}

fn demo_rational_height_significant_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "{x}.height_significant_bits() = {}",
            x.height_significant_bits()
        );
    }
}

fn benchmark_rational_height_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.to_height()",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("Rational.to_height()", &mut |x| {
                no_out!(x.to_height());
            }),
            ("Rational.into_height()", &mut |x| {
                no_out!(x.into_height());
            }),
        ],
    );
}

fn benchmark_rational_height_significant_bits(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.height_significant_bits()",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(x.height_significant_bits());
        })],
    );
}
