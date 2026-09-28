// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::ExponentGcd;
use malachite_base::test_util::bench::bucketers::unsigned_polynomial_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::exponent_gcd::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_exponent_gcd);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_exponent_gcd_algorithms
    );
}

fn demo_unsigned_polynomial_exponent_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).exponent_gcd() = {}", p.exponent_gcd());
    }
}

fn benchmark_unsigned_polynomial_exponent_gcd_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.exponent_gcd()",
        BenchmarkType::Algorithms,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |p| no_out!(p.exponent_gcd())),
            ("naive", &mut |p| no_out!(exponent_gcd_naive(&p))),
        ],
    );
}
