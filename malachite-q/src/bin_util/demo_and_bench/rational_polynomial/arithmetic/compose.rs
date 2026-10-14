// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{Compose, ComposeAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_rational_polynomial_max_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_polynomial_pair_gen, rational_polynomial_pair_gen_var_1,
    rational_polynomial_pair_gen_var_2,
};
use malachite_q::test_util::rational_polynomial::arithmetic::compose::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_compose);
    register_demo!(runner, demo_rational_polynomial_compose_val_ref);
    register_demo!(runner, demo_rational_polynomial_compose_ref_val);
    register_demo!(runner, demo_rational_polynomial_compose_ref_ref);
    register_demo!(runner, demo_rational_polynomial_compose_assign);
    register_demo!(runner, demo_rational_polynomial_compose_assign_ref);
    register_demo!(runner, demo_rational_polynomial_compose_long);
    register_demo!(runner, demo_rational_polynomial_compose_linear);
    register_bench!(
        runner,
        benchmark_rational_polynomial_compose_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_polynomial_compose_algorithms);
    register_bench!(
        runner,
        benchmark_rational_polynomial_compose_algorithms_long
    );
}

fn demo_rational_polynomial_compose(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).compose({q_old}) = {}", p.compose(q));
    }
}

fn demo_rational_polynomial_compose_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("({p_old}).compose(&({q})) = {}", p.compose(&q));
    }
}

fn demo_rational_polynomial_compose_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        let q_old = q.clone();
        println!("(&({p})).compose({q_old}) = {}", (&p).compose(q));
    }
}

fn demo_rational_polynomial_compose_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        println!("(&({p})).compose(&({q})) = {}", (&p).compose(&q));
    }
}

fn demo_rational_polynomial_compose_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        p.compose_assign(q);
        println!("p := {p_old}; p.compose_assign({q_old}); p = {p}");
    }
}

fn demo_rational_polynomial_compose_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p.compose_assign(&q);
        println!("p := {p_old}; p.compose_assign(&({q})); p = {p}");
    }
}

// The first polynomial has degree at least 4 and the second at least 2, so that the
// divide-and-conquer path is taken.
fn demo_rational_polynomial_compose_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).compose(&({q})) = {}", (&p).compose(&q));
    }
}

// The first polynomial has degree at least 4 and the second is linear, so that the composition is a
// Taylor shift and a scaling.
fn demo_rational_polynomial_compose_linear(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).compose(&({q})) = {}", (&p).compose(&q));
    }
}

fn benchmark_rational_polynomial_compose_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.compose(RationalPolynomial)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "RationalPolynomial.compose(RationalPolynomial)",
                &mut |(p, q)| {
                    no_out!(p.compose(q));
                },
            ),
            (
                "RationalPolynomial.compose(&RationalPolynomial)",
                &mut |(p, q)| {
                    no_out!(p.compose(&q));
                },
            ),
            (
                "(&RationalPolynomial).compose(RationalPolynomial)",
                &mut |(p, q)| {
                    no_out!((&p).compose(q));
                },
            ),
            (
                "(&RationalPolynomial).compose(&RationalPolynomial)",
                &mut |(p, q)| {
                    no_out!((&p).compose(&q));
                },
            ),
            (
                "RationalPolynomial.compose_assign(RationalPolynomial)",
                &mut |(mut p, q)| {
                    p.compose_assign(q);
                },
            ),
            (
                "RationalPolynomial.compose_assign(&RationalPolynomial)",
                &mut |(mut p, q)| {
                    p.compose_assign(&q);
                },
            ),
        ],
    );
}

fn benchmark_rational_polynomial_compose_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.compose(RationalPolynomial)",
        BenchmarkType::Algorithms,
        rational_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!((&p).compose(&q));
            }),
            ("naive", &mut |(p, q)| {
                no_out!(compose_naive(&p, &q));
            }),
        ],
    );
}

fn benchmark_rational_polynomial_compose_algorithms_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.compose(RationalPolynomial)",
        BenchmarkType::Algorithms,
        rational_polynomial_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!((&p).compose(&q));
            }),
            ("naive", &mut |(p, q)| {
                no_out!(compose_naive(&p, &q));
            }),
        ],
    );
}
