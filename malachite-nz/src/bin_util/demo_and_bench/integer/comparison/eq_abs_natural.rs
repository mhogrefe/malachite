// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_natural_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_eq_abs_natural);
    register_demo!(runner, demo_natural_eq_abs_integer);

    register_bench!(runner, benchmark_integer_eq_abs_natural);
    register_bench!(runner, benchmark_natural_eq_abs_integer);
}

fn demo_integer_eq_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_natural_eq_abs_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if y.eq_abs(&x) {
            println!("|{y}| = |{x}|");
        } else {
            println!("|{y}| ≠ |{x}|");
        }
    }
}

fn benchmark_integer_eq_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.eq_abs(&Natural)",
        BenchmarkType::Single,
        integer_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.eq_abs(&y)))],
    );
}

fn benchmark_natural_eq_abs_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.eq_abs(&Integer)",
        BenchmarkType::Single,
        integer_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.eq_abs(&x)))],
    );
}
