// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::special::traits::{
    FallingFactorialPolynomial, FallingFactorialPolynomialTruncated,
};
use malachite_base::test_util::bench::bucketers::{
    unsigned_direct_bucketer, usize_convertible_pair_max_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_gen_var_5, unsigned_pair_gen_var_28};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::integer_polynomial::special::falling_factorial_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_falling_factorial_polynomial);
    register_demo!(
        runner,
        demo_integer_polynomial_falling_factorial_polynomial_truncated
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_falling_factorial_polynomial_algorithms
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_falling_factorial_polynomial_truncated_algorithms
    );
}

fn demo_integer_polynomial_falling_factorial_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    // The nth falling factorial polynomial has n + 1 coefficients of up to about n log n bits, so
    // the argument is kept to u8 scale.
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "falling_factorial_polynomial({}) = {}",
            n,
            IntegerPolynomial::falling_factorial_polynomial(u64::from(n))
        );
    }
}

fn demo_integer_polynomial_falling_factorial_polynomial_truncated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, len) in unsigned_pair_gen_var_28::<u8, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "falling_factorial_polynomial_truncated({}, {}) = {}",
            n,
            len,
            IntegerPolynomial::falling_factorial_polynomial_truncated(u64::from(n), u64::from(len))
        );
    }
}

fn benchmark_integer_polynomial_falling_factorial_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial::falling_factorial_polynomial(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(IntegerPolynomial::falling_factorial_polynomial(u64::from(
                    n
                )));
            }),
            ("sequential", &mut |n| {
                let n = u64::from(n);
                no_out!(falling_factorial_polynomial_truncated_sequential(n, n + 1));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_falling_factorial_polynomial_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial::falling_factorial_polynomial_truncated(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_28::<u16, u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "len"),
        &mut [
            ("default", &mut |(n, len)| {
                no_out!(IntegerPolynomial::falling_factorial_polynomial_truncated(
                    u64::from(n),
                    u64::from(len)
                ));
            }),
            ("sequential", &mut |(n, len)| {
                no_out!(falling_factorial_polynomial_truncated_sequential(
                    u64::from(n),
                    u64::from(len)
                ));
            }),
        ],
    );
}
