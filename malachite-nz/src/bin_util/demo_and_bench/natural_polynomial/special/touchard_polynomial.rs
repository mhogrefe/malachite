// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::special::traits::{
    TouchardPolynomial, TouchardPolynomialTruncated,
};
use malachite_base::test_util::bench::bucketers::{
    unsigned_direct_bucketer, usize_convertible_pair_max_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_gen_var_5, unsigned_pair_gen_var_28};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::natural_polynomial::special::touchard_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_touchard_polynomial);
    register_demo!(
        runner,
        demo_natural_polynomial_touchard_polynomial_truncated
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_touchard_polynomial_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_touchard_polynomial_truncated_algorithms
    );
}

fn demo_natural_polynomial_touchard_polynomial(gm: GenMode, config: &GenConfig, limit: usize) {
    // The nth Touchard polynomial has n + 1 coefficients of up to about n log n bits, so the
    // argument is kept to u8 scale.
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "touchard_polynomial({}) = {}",
            n,
            NaturalPolynomial::touchard_polynomial(u64::from(n))
        );
    }
}

fn demo_natural_polynomial_touchard_polynomial_truncated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, len) in unsigned_pair_gen_var_28::<u8, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "touchard_polynomial_truncated({}, {}) = {}",
            n,
            len,
            NaturalPolynomial::touchard_polynomial_truncated(u64::from(n), u64::from(len))
        );
    }
}

fn benchmark_natural_polynomial_touchard_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial::touchard_polynomial(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(NaturalPolynomial::touchard_polynomial(u64::from(n)));
            }),
            ("triangular", &mut |n| {
                let n = u64::from(n);
                no_out!(touchard_polynomial_truncated_triangular(n, n + 1));
            }),
            ("convolution", &mut |n| {
                let n = u64::from(n);
                no_out!(touchard_polynomial_truncated_convolution(n, n + 1));
            }),
            ("multi-mod", &mut |n| {
                let n = u64::from(n);
                no_out!(touchard_polynomial_truncated_multi_mod(n, n + 1));
            }),
            ("naive", &mut |n| {
                let n = u64::from(n);
                no_out!(touchard_polynomial_truncated_naive(n, n + 1));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_touchard_polynomial_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial::touchard_polynomial_truncated(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_28::<u16, u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "len"),
        &mut [
            ("default", &mut |(n, len)| {
                no_out!(NaturalPolynomial::touchard_polynomial_truncated(
                    u64::from(n),
                    u64::from(len)
                ));
            }),
            ("triangular", &mut |(n, len)| {
                no_out!(touchard_polynomial_truncated_triangular(
                    u64::from(n),
                    u64::from(len)
                ));
            }),
            ("convolution", &mut |(n, len)| {
                no_out!(touchard_polynomial_truncated_convolution(
                    u64::from(n),
                    u64::from(len)
                ));
            }),
            ("multi-mod", &mut |(n, len)| {
                no_out!(touchard_polynomial_truncated_multi_mod(
                    u64::from(n),
                    u64::from(len)
                ));
            }),
            ("naive", &mut |(n, len)| {
                no_out!(touchard_polynomial_truncated_naive(
                    u64::from(n),
                    u64::from(len)
                ));
            }),
        ],
    );
}
