// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ExtendedGcd;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::bench::bucketers::pair_rational_polynomial_max_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_pair_gen_var_6;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_extended_gcd);
    register_demo!(runner, demo_rational_polynomial_extended_gcd_ref_ref);
    register_bench!(
        runner,
        benchmark_rational_polynomial_extended_gcd_evaluation_strategy
    );
}

fn show((g, s, t): (RationalPolynomial, RationalPolynomial, RationalPolynomial)) -> String {
    format!("({g}, {s}, {t})")
}

fn demo_rational_polynomial_extended_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).extended_gcd({q_old}) = {}",
            show(p.extended_gcd(q))
        );
    }
}

fn demo_rational_polynomial_extended_gcd_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).extended_gcd(&({q})) = {}",
            show((&p).extended_gcd(&q))
        );
    }
}

fn benchmark_rational_polynomial_extended_gcd_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.extended_gcd(RationalPolynomial)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.extended_gcd(q)", &mut |(p, q)| {
                no_out!(p.extended_gcd(q));
            }),
            ("p.extended_gcd(&q)", &mut |(p, q)| {
                no_out!(p.extended_gcd(&q));
            }),
            ("(&p).extended_gcd(q)", &mut |(p, q)| {
                no_out!((&p).extended_gcd(q));
            }),
            ("(&p).extended_gcd(&q)", &mut |(p, q)| {
                no_out!((&p).extended_gcd(&q));
            }),
        ],
    );
}
