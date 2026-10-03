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
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_natural_pair_gen_var_1;
use malachite_q::rational_polynomial::RationalPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_rational_polynomial_from_numerator_and_denominator
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_from_numerator_and_denominator
    );
}

fn demo_rational_polynomial_from_numerator_and_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, d) in integer_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let d_old = d.clone();
        println!(
            "RationalPolynomial::from_numerator_and_denominator({n_old}, {d_old}) = {}",
            RationalPolynomial::from_numerator_and_denominator(n, d)
        );
    }
}

fn benchmark_rational_polynomial_from_numerator_and_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial::from_numerator_and_denominator(IntegerPolynomial, Natural)",
        BenchmarkType::Single,
        integer_polynomial_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, d)| {
            no_out!(RationalPolynomial::from_numerator_and_denominator(n, d));
        })],
    );
}
