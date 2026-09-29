// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{NthDerivative, NthDerivativeAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_unsigned_pair_gen_var_1;
use malachite_q::test_util::rational_polynomial::arithmetic::nth_derivative::nth_derivative_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_nth_derivative);
    register_demo!(runner, demo_rational_polynomial_nth_derivative_ref);
    register_demo!(runner, demo_rational_polynomial_nth_derivative_assign);
    register_bench!(
        runner,
        benchmark_rational_polynomial_nth_derivative_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_nth_derivative_algorithms
    );
}

fn demo_rational_polynomial_nth_derivative(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, n) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).nth_derivative({n}) = {}", p.nth_derivative(n));
    }
}

fn demo_rational_polynomial_nth_derivative_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, n) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).nth_derivative({n}) = {}", (&p).nth_derivative(n));
    }
}

fn demo_rational_polynomial_nth_derivative_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, n) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.nth_derivative_assign(n);
        println!("p := {p_old}; p.nth_derivative_assign({n}); p = {p}");
    }
}

fn benchmark_rational_polynomial_nth_derivative_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.nth_derivative(u64)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("p.nth_derivative(n)", &mut |(p, n)| {
                no_out!(p.nth_derivative(n));
            }),
            ("(&p).nth_derivative(n)", &mut |(p, n)| {
                no_out!((&p).nth_derivative(n));
            }),
            ("p.nth_derivative_assign(n)", &mut |(mut p, n)| {
                p.nth_derivative_assign(n);
            }),
        ],
    );
}

fn benchmark_rational_polynomial_nth_derivative_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.nth_derivative(u64)",
        BenchmarkType::Algorithms,
        rational_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, n)| {
                no_out!((&p).nth_derivative(n));
            }),
            ("naive", &mut |(p, n)| {
                no_out!(nth_derivative_naive(&p, n));
            }),
        ],
    );
}
