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
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::bench::bucketers::pair_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_integer_matrix_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_partial_eq_integer_matrix);
    register_demo!(runner, demo_integer_matrix_partial_eq_rational_matrix);
    register_bench!(
        runner,
        benchmark_rational_matrix_partial_eq_integer_matrix_algorithms
    );
}

fn demo_rational_matrix_partial_eq_integer_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, n) in rational_matrix_integer_matrix_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if m == n {
            println!("{m} = {n}");
        } else {
            println!("{m} ≠ {n}");
        }
    }
}

fn demo_integer_matrix_partial_eq_rational_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, n) in rational_matrix_integer_matrix_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if n == m {
            println!("{n} = {m}");
        } else {
            println!("{n} ≠ {m}");
        }
    }
}

// The comparison's result is what is being timed, and the converting arm compares with an owned
// matrix on purpose.
#[allow(clippy::cmp_owned, clippy::no_effect, clippy::op_ref, unused_must_use)]
fn benchmark_rational_matrix_partial_eq_integer_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix == IntegerMatrix",
        BenchmarkType::Algorithms,
        rational_matrix_integer_matrix_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("m"),
        &mut [
            ("default", &mut |(m, n)| no_out!(m == n)),
            ("converting the IntegerMatrix", &mut |(m, n)| {
                no_out!(&m == &RationalMatrix::from(n));
            }),
        ],
    );
}
