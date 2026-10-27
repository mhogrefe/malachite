// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ComposeTruncated, ComposeTruncatedAssign};
use malachite_base::test_util::bench::bucketers::triple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::triple_1_2_rational_polynomial_max_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2,
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_3,
};
use malachite_q::test_util::rational_polynomial::arithmetic::compose_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_compose_truncated);
    register_demo!(runner, demo_rational_polynomial_compose_truncated_val_ref);
    register_demo!(runner, demo_rational_polynomial_compose_truncated_ref_val);
    register_demo!(runner, demo_rational_polynomial_compose_truncated_ref_ref);
    register_demo!(runner, demo_rational_polynomial_compose_truncated_assign);
    register_demo!(
        runner,
        demo_rational_polynomial_compose_truncated_assign_ref
    );
    register_demo!(runner, demo_rational_polynomial_compose_truncated_long);
    register_bench!(
        runner,
        benchmark_rational_polynomial_compose_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_compose_truncated_algorithms
    );
}

fn demo_rational_polynomial_compose_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).compose_truncated({q_old}, {len}) = {}",
            p.compose_truncated(q, len)
        );
    }
}

fn demo_rational_polynomial_compose_truncated_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).compose_truncated(&({q}), {len}) = {}",
            p.compose_truncated(&q, len)
        );
    }
}

fn demo_rational_polynomial_compose_truncated_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!(
            "(&({p})).compose_truncated({q_old}, {len}) = {}",
            (&p).compose_truncated(q, len)
        );
    }
}

fn demo_rational_polynomial_compose_truncated_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).compose_truncated(&({q}), {len}) = {}",
            (&p).compose_truncated(&q, len)
        );
    }
}

fn demo_rational_polynomial_compose_truncated_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.compose_truncated_assign(q, len);
        println!("p := {p_old}; p.compose_truncated_assign({q_old}, {len}); p = {p}");
    }
}

fn demo_rational_polynomial_compose_truncated_assign_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.compose_truncated_assign(&q, len);
        println!("p := {p_old}; p.compose_truncated_assign(&({q}), {len}); p = {p}");
    }
}

// Lengths up to 1000, so that all three algorithms are reached.
fn demo_rational_polynomial_compose_truncated_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).compose_truncated(&({q}), {len}) = {}",
            (&p).compose_truncated(&q, len)
        );
    }
}

fn benchmark_rational_polynomial_compose_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.compose_truncated(RationalPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "RationalPolynomial.compose_truncated(RationalPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!(p.compose_truncated(q, len));
                },
            ),
            (
                "RationalPolynomial.compose_truncated(&RationalPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!(p.compose_truncated(&q, len));
                },
            ),
            (
                "(&RationalPolynomial).compose_truncated(RationalPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!((&p).compose_truncated(q, len));
                },
            ),
            (
                "(&RationalPolynomial).compose_truncated(&RationalPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!((&p).compose_truncated(&q, len));
                },
            ),
        ],
    );
}

fn benchmark_rational_polynomial_compose_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalPolynomial).compose_truncated(&RationalPolynomial, u64)",
        BenchmarkType::Algorithms,
        rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("len"),
        &mut [
            ("default", &mut |(p, q, len)| {
                no_out!((&p).compose_truncated(&q, len));
            }),
            ("naive", &mut |(p, q, len)| {
                no_out!(rational_polynomial_compose_truncated_naive(&p, &q, len));
            }),
        ],
    );
}
