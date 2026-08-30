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
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::float_gaussian_integer_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_partial_eq_gaussian_integer);
    register_demo!(runner, demo_gaussian_integer_partial_eq_float);
    register_bench!(runner, benchmark_float_partial_eq_gaussian_integer);
    register_bench!(runner, benchmark_gaussian_integer_partial_eq_float);
}

fn demo_float_partial_eq_gaussian_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_gaussian_integer_pair_gen()
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

fn demo_gaussian_integer_partial_eq_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_gaussian_integer_pair_gen()
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
fn benchmark_float_partial_eq_gaussian_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float == GaussianInteger",
        BenchmarkType::Single,
        float_gaussian_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_gaussian_integer_max_complexity_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x == y))],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_gaussian_integer_partial_eq_float(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger == Float",
        BenchmarkType::Single,
        float_gaussian_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_gaussian_integer_max_complexity_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y == x))],
    );
}
