// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivAssignRem, DivRem};
use malachite_base::polynomial::{CheckedDivRem, PseudoDivRem};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_pair_gen_var_4,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::pseudo_div_rem::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_div_rem);
    register_demo!(runner, demo_integer_polynomial_div_rem_ref_ref);
    register_demo!(runner, demo_integer_polynomial_div_assign_rem);
    register_demo!(runner, demo_integer_polynomial_checked_div_rem);
    register_demo!(runner, demo_integer_polynomial_checked_div_rem_ref_ref);
    register_bench!(
        runner,
        benchmark_integer_polynomial_div_rem_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_polynomial_div_rem_algorithms);
}

fn demo_integer_polynomial_div_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let (s, r) = p.div_rem(q);
        println!("({p_old}).div_rem({q_old}) = ({s}, {r})");
    }
}

fn demo_integer_polynomial_div_rem_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (s, r) = (&p).div_rem(&q);
        println!("(&({p})).div_rem(&({q})) = ({s}, {r})");
    }
}

fn demo_integer_polynomial_div_assign_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let r = p.div_assign_rem(q);
        println!("p := {p_old}; p.div_assign_rem({q_old}) = {r}; p = {p}");
    }
}

fn demo_integer_polynomial_checked_div_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_div_rem({q_old}) = {}",
            p.checked_div_rem(q)
                .map_or_else(|| "None".to_string(), |(s, r)| format!("({s}, {r})"))
        );
    }
}

fn demo_integer_polynomial_checked_div_rem_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        println!(
            "(&({p})).checked_div_rem(&({q})) = {}",
            (&p).checked_div_rem(&q)
                .map_or_else(|| "None".to_string(), |(s, r)| format!("({s}, {r})"))
        );
    }
}

fn benchmark_integer_polynomial_div_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_rem(IntegerPolynomial)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.div_rem(q)", &mut |(p, q)| {
                no_out!(p.div_rem(q));
            }),
            ("p.div_rem(&q)", &mut |(p, q)| {
                no_out!(p.div_rem(&q));
            }),
            ("(&p).div_rem(q)", &mut |(p, q)| {
                no_out!((&p).div_rem(q));
            }),
            ("(&p).div_rem(&q)", &mut |(p, q)| {
                no_out!((&p).div_rem(&q));
            }),
            ("p.div_assign_rem(q)", &mut |(mut p, q)| {
                no_out!(p.div_assign_rem(q));
            }),
            ("p.div_assign_rem(&q)", &mut |(mut p, q)| {
                no_out!(p.div_assign_rem(&q));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_div_rem_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_rem(IntegerPolynomial)",
        BenchmarkType::Algorithms,
        integer_polynomial_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!((&p).div_rem(&q));
            }),
            ("naive", &mut |(p, q)| {
                no_out!(div_rem_naive(&p, &q));
            }),
            ("basecase", &mut |(p, q)| {
                no_out!(div_rem_basecase_wrapper(&p, &q));
            }),
            ("pseudo-division", &mut |(p, q)| {
                no_out!((&p).pseudo_div_rem(&q));
            }),
        ],
    );
}
