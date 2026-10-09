// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_significant_bits);
    register_bench!(runner, benchmark_natural_polynomial_significant_bits);
}

fn demo_natural_polynomial_significant_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).significant_bits() = {}", p.significant_bits());
    }
}

fn benchmark_natural_polynomial_significant_bits(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.significant_bits()",
        BenchmarkType::Single,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| {
            no_out!(p.significant_bits());
        })],
    );
}
