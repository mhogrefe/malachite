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
use malachite_q::test_util::bench::bucketers::pair_gaussian_rational_natural_max_bit_bucketer;
use malachite_q::test_util::generators::gaussian_rational_natural_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_partial_eq_natural);
    register_demo!(runner, demo_natural_partial_eq_gaussian_rational);
    register_bench!(runner, benchmark_gaussian_rational_partial_eq_natural);
    register_bench!(runner, benchmark_natural_partial_eq_gaussian_rational);
}

fn demo_gaussian_rational_partial_eq_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if x == y {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_natural_partial_eq_gaussian_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if y == x {
            println!("{y} = {x}");
        } else {
            println!("{y} ≠ {x}");
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_gaussian_rational_partial_eq_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational == Natural",
        BenchmarkType::Single,
        gaussian_rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x == y))],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_partial_eq_gaussian_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural == GaussianRational",
        BenchmarkType::Single,
        gaussian_rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y == x))],
    );
}
