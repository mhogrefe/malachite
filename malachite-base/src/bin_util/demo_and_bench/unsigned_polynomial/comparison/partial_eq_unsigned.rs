// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::pair_1_unsigned_polynomial_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_pair_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_partial_eq_unsigned);
    register_demo!(runner, demo_unsigned_partial_eq_unsigned_polynomial);

    register_bench!(
        runner,
        benchmark_unsigned_polynomial_partial_eq_unsigned_algorithms
    );
}

fn demo_unsigned_polynomial_partial_eq_unsigned(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in unsigned_polynomial_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if p == c {
            println!("{p} = {c}");
        } else {
            println!("{p} ≠ {c}");
        }
    }
}

fn demo_unsigned_partial_eq_unsigned_polynomial(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in unsigned_polynomial_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if c == p {
            println!("{c} = {p}");
        } else {
            println!("{c} ≠ {p}");
        }
    }
}

// The comparison's result is what is being timed, and the converting arm compares with an owned
// polynomial on purpose.
#[allow(clippy::cmp_owned, clippy::no_effect, unused_must_use)]
fn benchmark_unsigned_polynomial_partial_eq_unsigned_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64> == u64",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, c)| no_out!(p == c)),
            ("converting the value to a polynomial", &mut |(p, c)| {
                no_out!(p == UnsignedPolynomial::from(c));
            }),
        ],
    );
}
