// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{PseudoDivAssignRem, PseudoDivRem};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen_var_3;
use malachite_nz::test_util::integer_polynomial::arithmetic::pseudo_div_rem::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_pseudo_div_rem);
    register_demo!(runner, demo_integer_polynomial_pseudo_div_rem_ref_ref);
    register_demo!(runner, demo_integer_polynomial_pseudo_div_assign_rem);
    register_bench!(
        runner,
        benchmark_integer_polynomial_pseudo_div_rem_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_pseudo_div_rem_algorithms
    );
}

fn demo_integer_polynomial_pseudo_div_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let (s, r, d) = p.pseudo_div_rem(q);
        println!("({p_old}).pseudo_div_rem({q_old}) = ({s}, {r}, {d})");
    }
}

fn demo_integer_polynomial_pseudo_div_rem_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (s, r, d) = (&p).pseudo_div_rem(&q);
        println!("(&({p})).pseudo_div_rem(&({q})) = ({s}, {r}, {d})");
    }
}

fn demo_integer_polynomial_pseudo_div_assign_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let (r, d) = p.pseudo_div_assign_rem(q);
        println!("p := {p_old}; p.pseudo_div_assign_rem({q_old}) = ({r}, {d}); p = {p}");
    }
}

fn benchmark_integer_polynomial_pseudo_div_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.pseudo_div_rem(IntegerPolynomial)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.pseudo_div_rem(q)", &mut |(p, q)| {
                no_out!(p.pseudo_div_rem(q));
            }),
            ("p.pseudo_div_rem(&q)", &mut |(p, q)| {
                no_out!(p.pseudo_div_rem(&q));
            }),
            ("(&p).pseudo_div_rem(q)", &mut |(p, q)| {
                no_out!((&p).pseudo_div_rem(q));
            }),
            ("(&p).pseudo_div_rem(&q)", &mut |(p, q)| {
                no_out!((&p).pseudo_div_rem(&q));
            }),
            ("p.pseudo_div_assign_rem(q)", &mut |(mut p, q)| {
                no_out!(p.pseudo_div_assign_rem(q));
            }),
            ("p.pseudo_div_assign_rem(&q)", &mut |(mut p, q)| {
                no_out!(p.pseudo_div_assign_rem(&q));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_pseudo_div_rem_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.pseudo_div_rem(IntegerPolynomial)",
        BenchmarkType::Algorithms,
        integer_polynomial_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!((&p).pseudo_div_rem(&q));
            }),
            ("naive", &mut |(p, q)| {
                no_out!(pseudo_div_rem_naive(&p, &q));
            }),
            ("basecase", &mut |(p, q)| {
                no_out!(pseudo_div_rem_basecase_wrapper(&p, &q));
            }),
            ("divide-and-conquer", &mut |(p, q)| {
                no_out!(pseudo_div_rem_divide_and_conquer_wrapper(&p, &q));
            }),
            ("Cohen", &mut |(p, q)| {
                no_out!(pseudo_div_rem_cohen(&p, &q));
            }),
        ],
    );
}
