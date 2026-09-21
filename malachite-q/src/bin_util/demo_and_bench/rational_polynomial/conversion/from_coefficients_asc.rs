// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::bench::bucketers::vec_rational_sum_bits_bucketer;
use malachite_q::test_util::generators::rational_vec_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_from_coefficients_asc);
    register_bench!(runner, benchmark_rational_polynomial_from_coefficients_asc);
}

fn demo_rational_polynomial_from_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in rational_vec_gen().get(gm, config).take(limit) {
        println!(
            "RationalPolynomial::from_coefficients_asc({}) = {}",
            xs.to_debug_string(),
            RationalPolynomial::from_coefficients_asc(xs.clone())
        );
    }
}

fn benchmark_rational_polynomial_from_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial::from_coefficients_asc(Vec<Rational>)",
        BenchmarkType::Single,
        rational_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_rational_sum_bits_bucketer(),
        &mut [("Malachite", &mut |xs| {
            no_out!(RationalPolynomial::from_coefficients_asc(xs));
        })],
    );
}
