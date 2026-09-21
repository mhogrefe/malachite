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
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::vec_natural_sum_bits_bucketer;
use malachite_nz::test_util::generators::natural_vec_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_from_coefficients_asc);
    register_bench!(runner, benchmark_natural_polynomial_from_coefficients_asc);
}

fn demo_natural_polynomial_from_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in natural_vec_gen().get(gm, config).take(limit) {
        println!(
            "NaturalPolynomial::from_coefficients_asc({}) = {}",
            xs.to_debug_string(),
            NaturalPolynomial::from_coefficients_asc(xs.clone())
        );
    }
}

fn benchmark_natural_polynomial_from_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial::from_coefficients_asc(Vec<Natural>)",
        BenchmarkType::Single,
        natural_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_natural_sum_bits_bucketer(),
        &mut [("Malachite", &mut |xs| {
            no_out!(NaturalPolynomial::from_coefficients_asc(xs));
        })],
    );
}
