// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::arithmetic::traits::FromRoots;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_vec_gen;
use malachite_q::test_util::rational_polynomial::arithmetic::from_roots::from_roots_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_from_roots);
    register_bench!(runner, benchmark_rational_polynomial_from_roots_algorithms);
}

fn demo_rational_polynomial_from_roots(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in rational_vec_gen().get(gm, config).take(limit) {
        println!(
            "RationalPolynomial::from_roots(&{xs:?}) = {}",
            RationalPolynomial::from_roots(&xs)
        );
    }
}

fn benchmark_rational_polynomial_from_roots_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial::from_roots(&[Rational])",
        BenchmarkType::Algorithms,
        rational_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |xs| {
                no_out!(RationalPolynomial::from_roots(&xs));
            }),
            ("naive", &mut |xs| no_out!(from_roots_naive(&xs))),
        ],
    );
}
