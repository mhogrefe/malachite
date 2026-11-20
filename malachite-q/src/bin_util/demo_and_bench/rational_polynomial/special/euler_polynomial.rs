// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::rational_polynomial::special::euler_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_euler_polynomial);
    register_bench!(
        runner,
        benchmark_rational_polynomial_euler_polynomial_algorithms
    );
}

fn demo_rational_polynomial_euler_polynomial(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "euler_polynomial({}) = {}",
            n,
            RationalPolynomial::euler_polynomial(u64::from(n))
        );
    }
}

fn benchmark_rational_polynomial_euler_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial::euler_polynomial(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(RationalPolynomial::euler_polynomial(u64::from(n)));
            }),
            ("naive", &mut |n| {
                no_out!(euler_polynomial_naive(u64::from(n)));
            }),
        ],
    );
}
