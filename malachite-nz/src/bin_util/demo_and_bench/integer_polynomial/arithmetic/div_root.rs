// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{DivRoot, DivRootAssign, Evaluate};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_integer_pair_gen;
use malachite_nz::test_util::integer_polynomial::arithmetic::pseudo_div_rem::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_div_root);
    register_demo!(runner, demo_integer_polynomial_div_root_ref_ref);
    register_demo!(runner, demo_integer_polynomial_div_root_assign);
    register_bench!(
        runner,
        benchmark_integer_polynomial_div_root_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_polynomial_div_root_algorithms);
}

fn demo_integer_polynomial_div_root(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let c_old = c.clone();
        let (q, r) = p.div_root(c);
        println!("({p_old}).div_root({c_old}) = ({q}, {r})");
    }
}

fn demo_integer_polynomial_div_root_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let (q, r) = (&p).div_root(&c);
        println!("(&({p})).div_root(&({c})) = ({q}, {r})");
    }
}

fn demo_integer_polynomial_div_root_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, c) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let c_old = c.clone();
        let r = p.div_root_assign(c);
        println!("p := {p_old}; p.div_root_assign({c_old}) = {r}; p = {p}");
    }
}

fn benchmark_integer_polynomial_div_root_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_root(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("p.div_root(c)", &mut |(p, c)| {
                no_out!(p.div_root(c));
            }),
            ("p.div_root(&c)", &mut |(p, c)| {
                no_out!(p.div_root(&c));
            }),
            ("(&p).div_root(c)", &mut |(p, c)| {
                no_out!((&p).div_root(c));
            }),
            ("(&p).div_root(&c)", &mut |(p, c)| {
                no_out!((&p).div_root(&c));
            }),
            ("p.div_root_assign(c)", &mut |(mut p, c)| {
                no_out!(p.div_root_assign(c));
            }),
            ("p.div_root_assign(&c)", &mut |(mut p, c)| {
                no_out!(p.div_root_assign(&c));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_div_root_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_root(Integer)",
        BenchmarkType::Algorithms,
        integer_polynomial_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, c)| {
                no_out!((&p).div_root(&c));
            }),
            ("naive", &mut |(p, c)| {
                no_out!(div_root_naive(&p, &c));
            }),
            ("evaluation only", &mut |(p, c)| {
                no_out!((&p).evaluate(&c));
            }),
        ],
    );
}
