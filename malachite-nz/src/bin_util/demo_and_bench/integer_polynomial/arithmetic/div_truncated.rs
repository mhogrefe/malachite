// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    CheckedDivTruncated, DivTruncated, DivTruncatedAssign, Polynomial,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::div_truncated::*;
use malachite_nz::integer_polynomial::arithmetic::inverse_truncated::inverse_truncated_dispatch;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_integer_polynomial_bit_bucketer, triple_1_2_integer_polynomial_max_bit_bucketer,
};
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::div_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_div_truncated);
    register_demo!(runner, demo_integer_polynomial_div_truncated_val_ref);
    register_demo!(runner, demo_integer_polynomial_div_truncated_ref_val);
    register_demo!(runner, demo_integer_polynomial_div_truncated_ref_ref);
    register_demo!(runner, demo_integer_polynomial_div_truncated_assign);
    register_demo!(runner, demo_integer_polynomial_div_truncated_assign_ref);
    register_demo!(runner, demo_integer_polynomial_checked_div_truncated);
    register_demo!(
        runner,
        demo_integer_polynomial_checked_div_truncated_ref_ref
    );
    register_demo!(runner, demo_integer_polynomial_checked_div_truncated_exact);
    register_demo!(runner, demo_integer_polynomial_div_truncated_long);

    register_bench!(
        runner,
        benchmark_integer_polynomial_div_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_div_truncated_algorithms
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_div_truncated_assign_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_div_truncated_constant_dividend_algorithms
    );
}

fn demo_integer_polynomial_div_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).div_truncated({q_old}, {len}) = {}",
            p.div_truncated(q, len)
        );
    }
}

fn demo_integer_polynomial_div_truncated_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).div_truncated(&({q_old}), {len}) = {}",
            p.div_truncated(&q, len)
        );
    }
}

fn demo_integer_polynomial_div_truncated_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "(&({p_old})).div_truncated({q_old}, {len}) = {}",
            (&p).div_truncated(q, len)
        );
    }
}

fn demo_integer_polynomial_div_truncated_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "(&({p_old})).div_truncated(&({q_old}), {len}) = {}",
            (&p).div_truncated(&q, len)
        );
    }
}

fn demo_integer_polynomial_div_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.div_truncated_assign(q, len);
        println!("p := {p_old}; p.div_truncated_assign({q_old}, {len}); p = {p}");
    }
}

fn demo_integer_polynomial_div_truncated_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.div_truncated_assign(&q, len);
        println!("p := {p_old}; p.div_truncated_assign(&({q_old}), {len}); p = {p}");
    }
}

fn demo_integer_polynomial_checked_div_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_div_truncated({q_old}, {len}) = {:?}",
            p.checked_div_truncated(q, len)
        );
    }
}

fn demo_integer_polynomial_checked_div_truncated_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "(&({p_old})).checked_div_truncated(&({q_old}), {len}) = {:?}",
            (&p).checked_div_truncated(&q, len)
        );
    }
}

fn demo_integer_polynomial_checked_div_truncated_exact(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_div_truncated({q_old}, {len}) = {:?}",
            p.checked_div_truncated(q, len)
        );
    }
}

// The generator configuration of the long demos: polynomials of about 150 coefficients of about 8
// bits, long enough to reach the Newton and divide-and-conquer algorithms.
fn long_config(config: &GenConfig) -> GenConfig {
    let mut config = config.clone();
    if config.get_or("mean_length_n", 0) == 0 {
        config.insert("mean_length_n", 150);
    }
    if config.get_or("mean_bits_n", 0) == 0 {
        config.insert("mean_bits_n", 8);
    }
    config
}

// Quotients of long series, truncated to 16 more coefficients than the divisor has.
fn demo_integer_polynomial_div_truncated_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, _) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4()
        .get(gm, &long_config(config))
        .take(limit)
    {
        let len = q.len() + 16;
        println!(
            "(&({p})).div_truncated(&({q}), {len}) = {}",
            (&p).div_truncated(&q, len)
        );
    }
}

fn benchmark_integer_polynomial_div_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_truncated(IntegerPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "IntegerPolynomial.div_truncated(IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!(p.div_truncated(q, len));
                },
            ),
            (
                "IntegerPolynomial.div_truncated(&IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!(p.div_truncated(&q, len));
                },
            ),
            (
                "(&IntegerPolynomial).div_truncated(IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!((&p).div_truncated(q, len));
                },
            ),
            (
                "(&IntegerPolynomial).div_truncated(&IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!((&p).div_truncated(&q, len));
                },
            ),
        ],
    );
}

fn benchmark_integer_polynomial_div_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_truncated(IntegerPolynomial, u64)",
        BenchmarkType::Algorithms,
        integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4()
            .get(gm, config)
            .filter(|(p, _, len)| *len != 0 && *p != 0u32),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, len)| {
                no_out!((&p).div_truncated(&q, len));
            }),
            ("basecase", &mut |(p, q, len)| {
                no_out!(div_truncated_basecase(
                    p.coefficients_asc(),
                    q.coefficients_asc(),
                    usize::exact_from(len)
                ));
            }),
            ("divide-and-conquer", &mut |(p, q, len)| {
                no_out!(div_truncated_divide_and_conquer(
                    p.coefficients_asc(),
                    q.coefficients_asc(),
                    usize::exact_from(len)
                ));
            }),
            ("naive", &mut |(p, q, len)| {
                no_out!(div_truncated_naive(&p, &q, len));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_div_truncated_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_truncated_assign(IntegerPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "IntegerPolynomial.div_truncated_assign(IntegerPolynomial, u64)",
                &mut |(mut p, q, len)| p.div_truncated_assign(q, len),
            ),
            (
                "IntegerPolynomial.div_truncated_assign(&IntegerPolynomial, u64)",
                &mut |(mut p, q, len)| p.div_truncated_assign(&q, len),
            ),
        ],
    );
}

fn benchmark_integer_polynomial_div_truncated_constant_dividend_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_truncated(IntegerPolynomial, u64)",
        BenchmarkType::Algorithms,
        integer_polynomial_unsigned_pair_gen_var_7()
            .get(gm, config)
            .filter(|(q, _)| *q != 0u32),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("q"),
        &mut [
            ("basecase", &mut |(q, _)| {
                no_out!(div_truncated_basecase(
                    &[Integer::from(3)],
                    q.coefficients_asc(),
                    usize::exact_from(q.len())
                ));
            }),
            ("inverse", &mut |(q, _)| {
                let mut inverse =
                    inverse_truncated_dispatch(q.coefficients_asc(), usize::exact_from(q.len()));
                for c in &mut inverse {
                    *c *= Integer::from(3);
                }
                no_out!(inverse);
            }),
        ],
    );
}
