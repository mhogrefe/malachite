// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_from_coefficients_asc);
    register_bench!(runner, benchmark_unsigned_polynomial_from_coefficients_asc);
}

fn demo_unsigned_polynomial_from_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u64>().get(gm, config).take(limit) {
        println!(
            "UnsignedPolynomial::<u64>::from_coefficients_asc({}) = {}",
            xs.to_debug_string(),
            UnsignedPolynomial::<u64>::from_coefficients_asc(xs.clone())
        );
    }
}

fn benchmark_unsigned_polynomial_from_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial::<u64>::from_coefficients_asc(Vec<u64>)",
        BenchmarkType::Single,
        unsigned_vec_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| {
            no_out!(UnsignedPolynomial::<u64>::from_coefficients_asc(xs));
        })],
    );
}
