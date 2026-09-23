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
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_q::test_util::bench::bucketers::rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_try_from_rational_polynomial);
    register_demo!(
        runner,
        demo_integer_polynomial_try_from_rational_polynomial_ref
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_try_from_rational_polynomial_evaluation_strategy
    );
}

fn demo_integer_polynomial_try_from_rational_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!(
            "IntegerPolynomial::try_from({}) = {:?}",
            p_old,
            IntegerPolynomial::try_from(p).map(|q| q.to_string())
        );
    }
}

fn demo_integer_polynomial_try_from_rational_polynomial_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "IntegerPolynomial::try_from(&{}) = {:?}",
            p,
            IntegerPolynomial::try_from(&p).map(|q| q.to_string())
        );
    }
}

fn benchmark_integer_polynomial_try_from_rational_polynomial_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial::try_from(RationalPolynomial)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [
            (
                "IntegerPolynomial::try_from(RationalPolynomial)",
                &mut |p| {
                    let _ = IntegerPolynomial::try_from(p);
                },
            ),
            (
                "IntegerPolynomial::try_from(&RationalPolynomial)",
                &mut |p| {
                    let _ = IntegerPolynomial::try_from(&p);
                },
            ),
        ],
    );
}
