// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{PseudoDiv, PseudoDivAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_pseudo_div);
    register_demo!(runner, demo_integer_polynomial_pseudo_div_ref_ref);
    register_demo!(runner, demo_integer_polynomial_pseudo_div_assign);
    register_bench!(
        runner,
        benchmark_integer_polynomial_pseudo_div_evaluation_strategy
    );
}

fn demo_integer_polynomial_pseudo_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let (s, d) = p.pseudo_div(q);
        println!("({p_old}).pseudo_div({q_old}) = ({s}, {d})");
    }
}

fn demo_integer_polynomial_pseudo_div_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (s, d) = (&p).pseudo_div(&q);
        println!("(&({p})).pseudo_div(&({q})) = ({s}, {d})");
    }
}

fn demo_integer_polynomial_pseudo_div_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let d = p.pseudo_div_assign(q);
        println!("p := {p_old}; p.pseudo_div_assign({q_old}) = {d}; p = {p}");
    }
}

fn benchmark_integer_polynomial_pseudo_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.pseudo_div(IntegerPolynomial)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.pseudo_div(q)", &mut |(p, q)| {
                no_out!(p.pseudo_div(q));
            }),
            ("p.pseudo_div(&q)", &mut |(p, q)| {
                no_out!(p.pseudo_div(&q));
            }),
            ("(&p).pseudo_div(q)", &mut |(p, q)| {
                no_out!((&p).pseudo_div(q));
            }),
            ("(&p).pseudo_div(&q)", &mut |(p, q)| {
                no_out!((&p).pseudo_div(&q));
            }),
            ("p.pseudo_div_assign(q)", &mut |(mut p, q)| {
                no_out!(p.pseudo_div_assign(q));
            }),
            ("p.pseudo_div_assign(&q)", &mut |(mut p, q)| {
                no_out!(p.pseudo_div_assign(&q));
            }),
        ],
    );
}
