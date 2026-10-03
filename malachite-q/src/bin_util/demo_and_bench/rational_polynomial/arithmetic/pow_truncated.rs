// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::polynomial::{Polynomial, PowTruncated, PowTruncatedAssign};
use malachite_base::test_util::bench::bucketers::triple_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::generators::rational_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_q::test_util::rational_polynomial::arithmetic::pow_truncated::pow_truncated_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_pow_truncated);
    register_demo!(runner, demo_rational_polynomial_pow_truncated_ref);
    register_demo!(runner, demo_rational_polynomial_pow_truncated_assign);

    register_bench!(
        runner,
        benchmark_rational_polynomial_pow_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_pow_truncated_algorithms
    );
}

fn demo_rational_polynomial_pow_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e, len) in rational_polynomial_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).pow_truncated({e}, {len}) = {}",
            p.pow_truncated(e, len)
        );
    }
}

fn demo_rational_polynomial_pow_truncated_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e, len) in rational_polynomial_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).pow_truncated({e}, {len}) = {}",
            (&p).pow_truncated(e, len)
        );
    }
}

fn demo_rational_polynomial_pow_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, e, len) in rational_polynomial_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.pow_truncated_assign(e, len);
        println!("p := {p_old}; p.pow_truncated_assign({e}, {len}); p = {p}");
    }
}

fn benchmark_rational_polynomial_pow_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.pow_truncated(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_unsigned_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("exp"),
        &mut [
            (
                "RationalPolynomial.pow_truncated(u64, u64)",
                &mut |(p, e, len)| {
                    no_out!(p.pow_truncated(e, len));
                },
            ),
            (
                "(&RationalPolynomial).pow_truncated(u64, u64)",
                &mut |(p, e, len)| {
                    no_out!((&p).pow_truncated(e, len));
                },
            ),
            (
                "RationalPolynomial.pow_truncated_assign(u64, u64)",
                &mut |(mut p, e, len)| p.pow_truncated_assign(e, len),
            ),
        ],
    );
}

fn benchmark_rational_polynomial_pow_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalPolynomial).pow_truncated(u64, u64)",
        BenchmarkType::Algorithms,
        rational_polynomial_unsigned_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("exp"),
        &mut [
            ("default", &mut |(p, e, len)| {
                no_out!((&p).pow_truncated(e, len));
            }),
            ("full power, then truncation", &mut |(p, e, len)| {
                no_out!((&p).pow(e).truncate(len));
            }),
            ("naive", &mut |(p, e, len)| {
                no_out!(pow_truncated_naive(&p, e, len));
            }),
        ],
    );
}
