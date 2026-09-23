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
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_natural_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_partial_eq_natural);
    register_demo!(runner, demo_natural_partial_eq_rational_polynomial);

    register_bench!(
        runner,
        benchmark_rational_polynomial_partial_eq_natural_algorithms
    );
}

fn demo_rational_polynomial_partial_eq_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, n) in rational_polynomial_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if p == n {
            println!("{p} = {n}");
        } else {
            println!("{p} ≠ {n}");
        }
    }
}

fn demo_natural_partial_eq_rational_polynomial(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, n) in rational_polynomial_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if n == p {
            println!("{n} = {p}");
        } else {
            println!("{n} ≠ {p}");
        }
    }
}

// The comparison's result is what is being timed, and the converting arm compares with an owned
// polynomial on purpose.
#[allow(clippy::cmp_owned, clippy::no_effect, unused_must_use)]
fn benchmark_rational_polynomial_partial_eq_natural_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial == Natural",
        BenchmarkType::Algorithms,
        rational_polynomial_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, n)| no_out!(p == n)),
            ("converting the Natural to a polynomial", &mut |(p, n)| {
                no_out!(p == RationalPolynomial::from(Rational::from(n)));
            }),
        ],
    );
}
