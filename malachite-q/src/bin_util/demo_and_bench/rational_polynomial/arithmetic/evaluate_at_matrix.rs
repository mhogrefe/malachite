// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::arithmetic::traits::Evaluate;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::*;
use malachite_q::test_util::generators::*;
use malachite_q::test_util::rational_polynomial::arithmetic::evaluate_at_matrix::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_evaluate_at_matrix);
    register_demo!(runner, demo_rational_polynomial_evaluate_at_matrix_ref);
    register_bench!(
        runner,
        benchmark_rational_polynomial_evaluate_at_matrix_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_evaluate_at_matrix_algorithms
    );
}

fn demo_rational_polynomial_evaluate_at_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, a) in rational_polynomial_rational_matrix_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (p_old, a_old) = (p.clone(), a.clone());
        println!("(&({p_old})).evaluate({a_old}) = {}", (&p).evaluate(a));
    }
}

fn demo_rational_polynomial_evaluate_at_matrix_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, a) in rational_polynomial_rational_matrix_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).evaluate(&{a}) = {}", (&p).evaluate(&a));
    }
}

fn benchmark_rational_polynomial_evaluate_at_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalPolynomial).evaluate(RationalMatrix)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_rational_matrix_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_len_times_matrix_height_bucketer("p", "a"),
        &mut [
            ("(&p).evaluate(a)", &mut |(p, a)| {
                no_out!((&p).evaluate(a));
            }),
            ("(&p).evaluate(&a)", &mut |(p, a)| {
                no_out!((&p).evaluate(&a));
            }),
        ],
    );
}

fn benchmark_rational_polynomial_evaluate_at_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalPolynomial).evaluate(RationalMatrix)",
        BenchmarkType::Algorithms,
        rational_polynomial_rational_matrix_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_len_times_matrix_height_bucketer("p", "a"),
        &mut [
            ("default", &mut |(p, a)| {
                no_out!((&p).evaluate(&a));
            }),
            ("Horner", &mut |(p, a)| {
                no_out!(evaluate_at_matrix_horner_rational(&p, &a));
            }),
            ("Paterson-Stockmeyer", &mut |(p, a)| {
                no_out!(evaluate_at_matrix_paterson_stockmeyer_rational(&p, &a));
            }),
            ("naive", &mut |(p, a)| {
                no_out!(evaluate_at_matrix_naive_rational(&p, &a));
            }),
        ],
    );
}
