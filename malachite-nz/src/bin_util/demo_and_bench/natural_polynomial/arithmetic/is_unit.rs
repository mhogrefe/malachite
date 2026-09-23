// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsUnit;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_is_unit);
    register_bench!(runner, benchmark_natural_polynomial_is_unit);
}

fn demo_natural_polynomial_is_unit(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        if p.is_unit() {
            println!("{p} is a unit");
        } else {
            println!("{p} is not a unit");
        }
    }
}

fn benchmark_natural_polynomial_is_unit(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.is_unit()",
        BenchmarkType::Single,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.is_unit()))],
    );
}
