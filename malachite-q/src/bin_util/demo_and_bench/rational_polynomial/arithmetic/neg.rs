// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_neg);
    register_demo!(runner, demo_rational_polynomial_neg_ref);
    register_demo!(runner, demo_rational_polynomial_neg_assign);
    register_bench!(
        runner,
        benchmark_rational_polynomial_neg_evaluation_strategy
    );
}

fn demo_rational_polynomial_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("-({p_old}) = {}", -p);
    }
}

fn demo_rational_polynomial_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!("-&({p}) = {}", -&p);
    }
}

fn demo_rational_polynomial_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut p in rational_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p.neg_assign();
        println!("p := {p_old}; p.neg_assign(); p = {p}");
    }
}

fn benchmark_rational_polynomial_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-RationalPolynomial",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [
            ("-RationalPolynomial", &mut |p| no_out!(-p)),
            ("-&RationalPolynomial", &mut |p| no_out!(-&p)),
            ("RationalPolynomial.neg_assign()", &mut |mut p| {
                p.neg_assign();
            }),
        ],
    );
}
