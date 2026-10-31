// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivAssignRem, DivRem};
use malachite_base::polynomial::CheckedDivRem;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_rational_polynomial_max_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_polynomial_pair_gen, rational_polynomial_pair_gen_var_3,
};
use malachite_q::test_util::rational_polynomial::arithmetic::div_rem::div_rem_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_div_rem);
    register_demo!(runner, demo_rational_polynomial_div_rem_ref_ref);
    register_demo!(runner, demo_rational_polynomial_div_assign_rem);
    register_demo!(runner, demo_rational_polynomial_checked_div_rem);
    register_bench!(
        runner,
        benchmark_rational_polynomial_div_rem_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_polynomial_div_rem_algorithms);
}

fn demo_rational_polynomial_div_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).div_rem({q_old}) = {:?}", p.div_rem(q));
    }
}

fn demo_rational_polynomial_div_rem_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).div_rem(&({q})) = {:?}", (&p).div_rem(&q));
    }
}

fn demo_rational_polynomial_div_assign_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in rational_polynomial_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let r = p.div_assign_rem(q);
        println!("p := {p_old}; p.div_assign_rem({q_old}) = {r}; p = {p}");
    }
}

fn demo_rational_polynomial_checked_div_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        println!(
            "(&({p})).checked_div_rem(&({q})) = {:?}",
            (&p).checked_div_rem(&q)
        );
    }
}

fn benchmark_rational_polynomial_div_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.div_rem(RationalPolynomial)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "RationalPolynomial.div_rem(RationalPolynomial)",
                &mut |(p, q)| {
                    no_out!(p.div_rem(q));
                },
            ),
            (
                "RationalPolynomial.div_rem(&RationalPolynomial)",
                &mut |(p, q)| {
                    no_out!(p.div_rem(&q));
                },
            ),
            (
                "(&RationalPolynomial).div_rem(RationalPolynomial)",
                &mut |(p, q)| {
                    no_out!((&p).div_rem(q));
                },
            ),
            (
                "(&RationalPolynomial).div_rem(&RationalPolynomial)",
                &mut |(p, q)| {
                    no_out!((&p).div_rem(&q));
                },
            ),
        ],
    );
}

fn benchmark_rational_polynomial_div_rem_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.div_rem(RationalPolynomial)",
        BenchmarkType::Algorithms,
        rational_polynomial_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!((&p).div_rem(&q));
            }),
            ("naive", &mut |(p, q)| {
                no_out!(div_rem_naive(&p, &q));
            }),
        ],
    );
}
