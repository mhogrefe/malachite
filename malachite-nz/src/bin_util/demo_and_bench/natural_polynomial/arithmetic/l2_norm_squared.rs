// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::L2NormSquared;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_gen;
use malachite_nz::test_util::natural_polynomial::arithmetic::l2_norm_squared::l2_norm_squared_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_l2_norm_squared);
    register_bench!(
        runner,
        benchmark_natural_polynomial_l2_norm_squared_algorithms
    );
}

fn demo_natural_polynomial_l2_norm_squared(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        println!("(&({p})).l2_norm_squared() = {}", (&p).l2_norm_squared());
    }
}

fn benchmark_natural_polynomial_l2_norm_squared_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).l2_norm_squared()",
        BenchmarkType::Algorithms,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |p| no_out!((&p).l2_norm_squared())),
            ("naive", &mut |p| no_out!(l2_norm_squared_naive(&p))),
        ],
    );
}
