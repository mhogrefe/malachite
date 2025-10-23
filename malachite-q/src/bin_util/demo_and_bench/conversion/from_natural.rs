// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_gen;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_from_natural);
    register_demo!(runner, demo_rational_from_natural_ref);
    register_bench!(runner, benchmark_rational_from_natural_evaluation_strategy);
}

fn demo_rational_from_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        let n_clone = n.clone();
        println!("Rational::from({}) = {}", n_clone, Rational::from(n));
    }
}

fn demo_rational_from_natural_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("Rational::from(&{}) = {}", n, Rational::from(&n));
    }
}

#[allow(unused_must_use)]
fn benchmark_rational_from_natural_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("Rational::from(Natural)", &mut |n| {
                no_out!(Rational::from(n));
            }),
            ("Rational::from(&Natural)", &mut |n| {
                no_out!(Rational::from(&n));
            }),
        ],
    );
}
