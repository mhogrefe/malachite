// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::u64_polynomial_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::u64_polynomial_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_from_u64_polynomial);
    register_bench!(runner, benchmark_natural_polynomial_from_u64_polynomial);
}

fn demo_natural_polynomial_from_u64_polynomial(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in u64_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "NaturalPolynomial::from({}) = {}",
            p.clone(),
            NaturalPolynomial::from(p)
        );
    }
}

fn benchmark_natural_polynomial_from_u64_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial::from(U64Polynomial)",
        BenchmarkType::Single,
        u64_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &u64_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| {
            let _ = NaturalPolynomial::from(p);
        })],
    );
}
