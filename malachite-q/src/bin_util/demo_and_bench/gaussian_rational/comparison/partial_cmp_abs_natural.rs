// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_gaussian_rational_natural_max_bit_bucketer;
use malachite_q::test_util::generators::gaussian_rational_natural_pair_gen;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_partial_cmp_abs_natural);
    register_demo!(runner, demo_natural_partial_cmp_abs_gaussian_rational);
    register_bench!(
        runner,
        benchmark_gaussian_rational_partial_cmp_abs_natural_algorithms
    );
}

fn demo_gaussian_rational_partial_cmp_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        match x.partial_cmp_abs(&y) {
            Some(Less) => println!("|{x}| < |{y}|"),
            Some(Equal) => println!("|{x}| = |{y}|"),
            Some(Greater) => println!("|{x}| > |{y}|"),
            None => println!("|{x}| and |{y}| are incomparable"),
        }
    }
}

fn demo_natural_partial_cmp_abs_gaussian_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        match y.partial_cmp_abs(&x) {
            Some(Less) => println!("|{y}| < |{x}|"),
            Some(Equal) => println!("|{y}| = |{x}|"),
            Some(Greater) => println!("|{y}| > |{x}|"),
            None => println!("|{y}| and |{x}| are incomparable"),
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_partial_cmp_abs_natural_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.partial_cmp_abs(&Natural)",
        BenchmarkType::Algorithms,
        gaussian_rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_rational_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y))),
            ("using abs_squared", &mut |(x, y)| {
                no_out!((&x).abs_squared().partial_cmp(&(&y).abs_squared()));
            }),
        ],
    );
}
