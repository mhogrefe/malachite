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
use malachite_q::test_util::bench::bucketers::pair_rational_polynomial_max_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_pair_gen;
use malachite_q::test_util::rational_polynomial::comparison::cmp::{
    rational_polynomial_cmp_evaluated, rational_polynomial_cmp_naive,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_cmp);

    register_bench!(runner, benchmark_rational_polynomial_cmp_algorithms);
}

fn demo_rational_polynomial_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        println!("({}).cmp(&{}) = {:?}", p, q, p.cmp(&q));
    }
}

// The screens in the real implementation exist to avoid what the naive algorithms do: building a
// `Rational` per coefficient, which reduces it, or forming the difference and evaluating it.
#[allow(unused_must_use)]
fn benchmark_rational_polynomial_cmp_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.cmp(&RationalPolynomial)",
        BenchmarkType::Algorithms,
        rational_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| no_out!(p.cmp(&q))),
            ("materializing each coefficient", &mut |(p, q)| {
                no_out!(rational_polynomial_cmp_naive(&p, &q));
            }),
            ("evaluating the difference", &mut |(p, q)| {
                no_out!(rational_polynomial_cmp_evaluated(&p, &q));
            }),
        ],
    );
}
