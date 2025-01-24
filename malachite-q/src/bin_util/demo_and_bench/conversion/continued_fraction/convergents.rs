// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::conversion::traits::Convergents;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::conversion::continued_fraction::convergents::convergents_alt;
use malachite_q::test_util::generators::rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_convergents);
    register_demo!(runner, demo_rational_convergents_ref);
    register_bench!(runner, benchmark_rational_convergents_algorithms);
    register_bench!(runner, benchmark_rational_convergents_evaluation_strategy);
}

fn demo_rational_convergents(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "convergents({}) = {:?}",
            x.clone(),
            x.convergents().collect_vec()
        );
    }
}

fn demo_rational_convergents_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "convergents(&{}) = {:?}",
            x,
            (&x).convergents().collect_vec()
        );
    }
}

fn benchmark_rational_convergents_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.convergents()",
        BenchmarkType::Algorithms,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.convergents().collect_vec())),
            ("naive", &mut |x| no_out!(convergents_alt(x).collect_vec())),
        ],
    );
}

fn benchmark_rational_convergents_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.convergents()",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("Rational.convergents()", &mut |x| {
                no_out!(x.convergents().collect_vec())
            }),
            ("(&Rational).convergents()", &mut |x| {
                no_out!((&x).convergents().collect_vec())
            }),
        ],
    );
}
