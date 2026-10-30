// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::polynomial::CheckedDivExact;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_pair_gen_var_4,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_div_exact_by_polynomial);
    register_demo!(
        runner,
        demo_integer_polynomial_div_exact_by_polynomial_ref_ref
    );
    register_demo!(
        runner,
        demo_integer_polynomial_div_exact_by_polynomial_assign
    );
    register_demo!(runner, demo_integer_polynomial_checked_div_exact);
    register_demo!(runner, demo_integer_polynomial_checked_div_exact_divisible);
    register_bench!(
        runner,
        benchmark_integer_polynomial_div_exact_by_polynomial_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_div_exact_by_polynomial_algorithms
    );
}

fn demo_integer_polynomial_div_exact_by_polynomial(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).div_exact({q_old}) = {}", p.div_exact(q));
    }
}

fn demo_integer_polynomial_div_exact_by_polynomial_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).div_exact(&({q})) = {}", (&p).div_exact(&q));
    }
}

fn demo_integer_polynomial_div_exact_by_polynomial_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.div_exact_assign(q);
        println!("p := {p_old}; p.div_exact_assign({q_old}); p = {p}");
    }
}

fn demo_integer_polynomial_checked_div_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_div_exact({q_old}) = {}",
            p.checked_div_exact(q)
                .map_or_else(|| "None".to_string(), |s| s.to_string())
        );
    }
}

fn demo_integer_polynomial_checked_div_exact_divisible(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_div_exact(&({q})) = {}",
            (&p).checked_div_exact(&q)
                .map_or_else(|| "None".to_string(), |s| s.to_string())
        );
    }
}

fn benchmark_integer_polynomial_div_exact_by_polynomial_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_exact(IntegerPolynomial)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.div_exact(q)", &mut |(p, q)| {
                no_out!(p.div_exact(q));
            }),
            ("p.div_exact(&q)", &mut |(p, q)| {
                no_out!(p.div_exact(&q));
            }),
            ("(&p).div_exact(q)", &mut |(p, q)| {
                no_out!((&p).div_exact(q));
            }),
            ("(&p).div_exact(&q)", &mut |(p, q)| {
                no_out!((&p).div_exact(&q));
            }),
            ("p.div_exact_assign(q)", &mut |(mut p, q)| {
                no_out!(p.div_exact_assign(q));
            }),
            ("p.div_exact_assign(&q)", &mut |(mut p, q)| {
                no_out!(p.div_exact_assign(&q));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_div_exact_by_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_exact(IntegerPolynomial)",
        BenchmarkType::Algorithms,
        integer_polynomial_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!((&p).div_exact(&q));
            }),
            ("using div", &mut |(p, q)| {
                no_out!(&p / &q);
            }),
            ("checked", &mut |(p, q)| {
                no_out!((&p).checked_div_exact(&q));
            }),
        ],
    );
}
