// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_gen;
use malachite_q::test_util::rational_polynomial::arithmetic::mul::mul_then_reduce;
use malachite_q::test_util::rational_polynomial::arithmetic::square::square_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_square);
    register_demo!(runner, demo_rational_polynomial_square_ref);
    register_demo!(runner, demo_rational_polynomial_square_assign);
    register_bench!(
        runner,
        benchmark_rational_polynomial_square_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_polynomial_square_algorithms);
}

fn demo_rational_polynomial_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("({p_old}).square() = {}", p.square());
    }
}

fn demo_rational_polynomial_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!("(&({p})).square() = {}", (&p).square());
    }
}

fn demo_rational_polynomial_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut p in rational_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p.square_assign();
        println!("p := {p_old}; p.square_assign(); p = {p}");
    }
}

fn benchmark_rational_polynomial_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.square()",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [
            ("RationalPolynomial.square()", &mut |p| no_out!(p.square())),
            ("(&RationalPolynomial).square()", &mut |p| {
                no_out!((&p).square());
            }),
            ("RationalPolynomial.square_assign()", &mut |mut p| {
                p.square_assign();
            }),
        ],
    );
}

fn benchmark_rational_polynomial_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.square()",
        BenchmarkType::Algorithms,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |p| no_out!(p.square())),
            ("naive", &mut |p| no_out!(square_naive(&p))),
            ("multiply then reduce", &mut |p| {
                no_out!(mul_then_reduce(&p, &p));
            }),
        ],
    );
}
