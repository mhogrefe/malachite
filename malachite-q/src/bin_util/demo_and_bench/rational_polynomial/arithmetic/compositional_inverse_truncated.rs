// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::CompositionalInverseTruncated;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_unsigned_pair_gen_var_5;
use malachite_q::test_util::rational_polynomial::arithmetic::compositional_inverse_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_rational_polynomial_compositional_inverse_truncated
    );
    register_demo!(
        runner,
        demo_rational_polynomial_compositional_inverse_truncated_ref
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_compositional_inverse_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_compositional_inverse_truncated_algorithms
    );
}

fn show(p: Option<RationalPolynomial>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_rational_polynomial_compositional_inverse_truncated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len) in rational_polynomial_unsigned_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).compositional_inverse_truncated({len}) = {}",
            show(p.compositional_inverse_truncated(len))
        );
    }
}

fn demo_rational_polynomial_compositional_inverse_truncated_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len) in rational_polynomial_unsigned_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).compositional_inverse_truncated({len}) = {}",
            show((&p).compositional_inverse_truncated(len))
        );
    }
}

fn benchmark_rational_polynomial_compositional_inverse_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.compositional_inverse_truncated(u64)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_unsigned_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("p.compositional_inverse_truncated(len)", &mut |(p, len)| {
                no_out!(p.compositional_inverse_truncated(len));
            }),
            (
                "(&p).compositional_inverse_truncated(len)",
                &mut |(p, len)| {
                    no_out!((&p).compositional_inverse_truncated(len));
                },
            ),
        ],
    );
}

fn benchmark_rational_polynomial_compositional_inverse_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.compositional_inverse_truncated(u64)",
        BenchmarkType::Algorithms,
        rational_polynomial_unsigned_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, len)| {
                no_out!((&p).compositional_inverse_truncated(len));
            }),
            ("naive", &mut |(p, len)| {
                no_out!(compositional_inverse_truncated_naive(&p, len));
            }),
            ("Lagrange", &mut |(p, len)| {
                no_out!(compositional_inverse_truncated_lagrange_wrapper(&p, len));
            }),
            ("Lagrange fast", &mut |(p, len)| {
                no_out!(compositional_inverse_truncated_lagrange_fast_wrapper(
                    &p, len
                ));
            }),
            ("Newton", &mut |(p, len)| {
                no_out!(compositional_inverse_truncated_newton_wrapper(&p, len));
            }),
        ],
    );
}
