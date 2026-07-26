// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_integer_max_bit_bucketer;
use malachite_q::test_util::generators::rational_integer_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_eq_abs_integer);
    register_demo!(runner, demo_integer_eq_abs_rational);

    register_bench!(runner, benchmark_rational_eq_abs_integer_algorithms);
    register_bench!(runner, benchmark_integer_eq_abs_rational_algorithms);
}

fn demo_rational_eq_abs_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_integer_pair_gen().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_integer_eq_abs_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_integer_pair_gen().get(gm, config).take(limit) {
        if y.eq_abs(&x) {
            println!("|{y}| = |{x}|");
        } else {
            println!("|{y}| ≠ |{x}|");
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_rational_eq_abs_integer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.eq_abs(&Integer)",
        BenchmarkType::Single,
        rational_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.eq_abs(&y))),
            ("using abs", &mut |(x, y)| no_out!(x.abs() == y.abs())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_eq_abs_rational_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.eq_abs(&Rational)",
        BenchmarkType::Single,
        rational_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(y.eq_abs(&x))),
            ("using abs", &mut |(x, y)| no_out!(y.abs() == x.abs())),
        ],
    );
}
