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
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::conversion::continued_fraction::to_continued_fraction::RationalContinuedFraction;
use malachite_q::conversion::traits::ContinuedFraction;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_continued_fraction);
    register_demo!(runner, demo_rational_continued_fraction_ref);
    register_bench!(
        runner,
        benchmark_rational_continued_fraction_evaluation_strategy
    );
}

fn helper(p: (Integer, RationalContinuedFraction)) -> (Integer, Vec<Natural>) {
    (p.0, p.1.collect_vec())
}

fn demo_rational_continued_fraction(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "continued_fraction({}) = {:?}",
            x.clone(),
            helper(x.continued_fraction())
        );
    }
}

fn demo_rational_continued_fraction_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "continued_fraction(&{}) = {:?}",
            x,
            helper((&x).continued_fraction())
        );
    }
}

fn benchmark_rational_continued_fraction_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.continued_fraction()",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("Rational.continued_fraction()", &mut |n| {
                no_out!(n.continued_fraction().1.collect_vec())
            }),
            ("(&Rational).continued_fraction()", &mut |n| {
                no_out!((&n).continued_fraction().1.collect_vec())
            }),
        ],
    );
}
