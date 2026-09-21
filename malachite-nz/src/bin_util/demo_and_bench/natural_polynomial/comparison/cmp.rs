// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_pair_gen;
use malachite_nz::test_util::natural_polynomial::comparison::cmp::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_cmp);

    register_bench!(runner, benchmark_natural_polynomial_cmp_algorithms);
}

fn demo_natural_polynomial_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in natural_polynomial_pair_gen().get(gm, config).take(limit) {
        println!("({}).cmp(&{}) = {:?}", p, q, p.cmp(&q));
    }
}

// `cmp`'s result is what is being timed, so the benchmark discards it on purpose.
#[allow(unused_must_use)]
fn benchmark_natural_polynomial_cmp_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.cmp(&NaturalPolynomial)",
        BenchmarkType::Algorithms,
        natural_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| no_out!(p.cmp(&q))),
            ("evaluating both polynomials", &mut |(p, q)| {
                no_out!(natural_polynomial_cmp_evaluated(&p, &q));
            }),
        ],
    );
}
