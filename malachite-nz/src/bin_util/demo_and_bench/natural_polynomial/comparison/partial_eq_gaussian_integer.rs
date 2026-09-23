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
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_gaussian_integer_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_partial_eq_gaussian_integer);
    register_demo!(runner, demo_gaussian_integer_partial_eq_natural_polynomial);

    register_bench!(
        runner,
        benchmark_natural_polynomial_partial_eq_gaussian_integer_algorithms
    );
}

fn demo_natural_polynomial_partial_eq_gaussian_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, g) in natural_polynomial_gaussian_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if p == g {
            println!("{p} = {g}");
        } else {
            println!("{p} ≠ {g}");
        }
    }
}

fn demo_gaussian_integer_partial_eq_natural_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, g) in natural_polynomial_gaussian_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if g == p {
            println!("{g} = {p}");
        } else {
            println!("{g} ≠ {p}");
        }
    }
}

// The comparison's result is what is being timed, and the converting arm compares with an owned
// polynomial on purpose.
#[allow(clippy::cmp_owned, clippy::no_effect, unused_must_use)]
fn benchmark_natural_polynomial_partial_eq_gaussian_integer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial == GaussianInteger",
        BenchmarkType::Algorithms,
        natural_polynomial_gaussian_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, g)| no_out!(p == g)),
            (
                "converting the GaussianInteger to a polynomial",
                &mut |(p, g)| {
                    no_out!(
                        g.imaginary == 0u32
                            && Natural::try_from(g.real)
                                .is_ok_and(|n| p == NaturalPolynomial::from(n))
                    );
                },
            ),
        ],
    );
}
