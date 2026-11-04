// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen_var_7;
use malachite_nz::test_util::integer_polynomial::arithmetic::gcd::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_gcd);
    register_demo!(runner, demo_integer_polynomial_gcd_ref_ref);
    register_demo!(runner, demo_integer_polynomial_gcd_assign);
    register_bench!(runner, benchmark_integer_polynomial_gcd_evaluation_strategy);
    register_bench!(runner, benchmark_integer_polynomial_gcd_algorithms);
}

fn demo_integer_polynomial_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).gcd({q_old}) = {}", p.gcd(q));
    }
}

fn demo_integer_polynomial_gcd_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).gcd(&({q})) = {}", (&p).gcd(&q));
    }
}

fn demo_integer_polynomial_gcd_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.gcd_assign(q);
        println!("p := {p_old}; p.gcd_assign({q_old}); p = {p}");
    }
}

fn benchmark_integer_polynomial_gcd_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.gcd(IntegerPolynomial)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.gcd(q)", &mut |(p, q)| {
                no_out!(p.gcd(q));
            }),
            ("p.gcd(&q)", &mut |(p, q)| {
                no_out!(p.gcd(&q));
            }),
            ("(&p).gcd(q)", &mut |(p, q)| {
                no_out!((&p).gcd(q));
            }),
            ("(&p).gcd(&q)", &mut |(p, q)| {
                no_out!((&p).gcd(&q));
            }),
            ("p.gcd_assign(q)", &mut |(mut p, q)| {
                no_out!(p.gcd_assign(q));
            }),
            ("p.gcd_assign(&q)", &mut |(mut p, q)| {
                no_out!(p.gcd_assign(&q));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_gcd_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.gcd(IntegerPolynomial)",
        BenchmarkType::Algorithms,
        integer_polynomial_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!((&p).gcd(&q));
            }),
            ("subresultant", &mut |(p, q)| {
                no_out!(gcd_subresultant_wrapper(&p, &q));
            }),
            ("heuristic", &mut |(p, q)| {
                no_out!(gcd_heuristic_wrapper(&p, &q));
            }),
            ("modular", &mut |(p, q)| {
                no_out!(gcd_modular_wrapper(&p, &q));
            }),
        ],
    );
}
