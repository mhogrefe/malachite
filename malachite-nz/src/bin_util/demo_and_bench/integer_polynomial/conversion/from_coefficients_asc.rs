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
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::bench::bucketers::vec_integer_sum_bits_bucketer;
use malachite_nz::test_util::generators::integer_vec_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_from_coefficients_asc);
    register_bench!(runner, benchmark_integer_polynomial_from_coefficients_asc);
}

fn demo_integer_polynomial_from_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen().get(gm, config).take(limit) {
        println!(
            "IntegerPolynomial::from_coefficients_asc({}) = {}",
            xs.to_debug_string(),
            IntegerPolynomial::from_coefficients_asc(xs.clone())
        );
    }
}

fn benchmark_integer_polynomial_from_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial::from_coefficients_asc(Vec<Integer>)",
        BenchmarkType::Single,
        integer_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_integer_sum_bits_bucketer(),
        &mut [("Malachite", &mut |xs| {
            no_out!(IntegerPolynomial::from_coefficients_asc(xs));
        })],
    );
}
