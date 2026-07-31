// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Average, AverageAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_rational_max_bit_bucketer;
use malachite_q::test_util::generators::rational_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_average);
    register_demo!(runner, demo_rational_average_assign);

    register_bench!(runner, benchmark_rational_average_evaluation_strategy);
}

fn demo_rational_average(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).average({}) = {}", x_old, y_old, x.average(y));
    }
}

fn demo_rational_average_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.average_assign(&y);
        println!("x := {x_old}; x.average_assign(&{y}); x = {x}");
    }
}

fn benchmark_rational_average_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.average(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("Rational.average(Rational)", &mut |(x, y)| {
                no_out!(x.average(y));
            }),
            ("Rational.average(&Rational)", &mut |(x, y)| {
                no_out!(x.average(&y));
            }),
            ("(&Rational).average(Rational)", &mut |(x, y)| {
                no_out!((&x).average(y));
            }),
            ("(&Rational).average(&Rational)", &mut |(x, y)| {
                no_out!((&x).average(&y));
            }),
        ],
    );
}
