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
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_from_rational);
    register_bench!(runner, benchmark_rational_polynomial_from_rational);
}

fn demo_rational_polynomial_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "RationalPolynomial::from({}) = {}",
            x.clone(),
            RationalPolynomial::from(x)
        );
    }
}

fn benchmark_rational_polynomial_from_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial::from(Rational)",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            let _ = RationalPolynomial::from(x);
        })],
    );
}
