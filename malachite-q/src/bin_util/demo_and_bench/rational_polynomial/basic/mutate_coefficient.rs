// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_unsigned_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_mutate_coefficient);
    register_bench!(runner, benchmark_rational_polynomial_mutate_coefficient);
}

fn demo_rational_polynomial_mutate_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, i) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old = p.to_string();
        p.mutate_coefficient(i, |c| *c += Rational::ONE);
        println!("({old}).mutate_coefficient({i}, |c| *c += 1) = {p}");
    }
}

fn benchmark_rational_polynomial_mutate_coefficient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.mutate_coefficient(u64, FnOnce)",
        BenchmarkType::Single,
        rational_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(mut p, i)| {
            no_out!(p.mutate_coefficient(i, |c| *c += Rational::ONE));
        })],
    );
}
