// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{TaylorShift, TaylorShiftAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_polynomial_rational_pair_gen, rational_polynomial_rational_pair_gen_var_1,
    rational_polynomial_rational_pair_gen_var_2,
};
use malachite_q::test_util::rational_polynomial::arithmetic::taylor_shift::taylor_shift_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_taylor_shift);
    register_demo!(runner, demo_rational_polynomial_taylor_shift_ref);
    register_demo!(runner, demo_rational_polynomial_taylor_shift_assign);
    register_demo!(runner, demo_rational_polynomial_taylor_shift_long);
    register_demo!(runner, demo_rational_polynomial_taylor_shift_integer_shift);
    register_bench!(
        runner,
        benchmark_rational_polynomial_taylor_shift_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_taylor_shift_algorithms
    );
}

fn demo_rational_polynomial_taylor_shift(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in rational_polynomial_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).taylor_shift({c}) = {}", p.taylor_shift(&c));
    }
}

fn demo_rational_polynomial_taylor_shift_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in rational_polynomial_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).taylor_shift({c}) = {}", (&p).taylor_shift(&c));
    }
}

fn demo_rational_polynomial_taylor_shift_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, c) in rational_polynomial_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.taylor_shift_assign(&c);
        println!("p := {p_old}; p.taylor_shift_assign({c}); p = {p}");
    }
}

// Polynomials of degree at least 50, some long enough for the divide-and-conquer path.
fn demo_rational_polynomial_taylor_shift_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in rational_polynomial_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).taylor_shift({c}) = {}", (&p).taylor_shift(&c));
    }
}

// Integer shifts, which only shift the numerator.
fn demo_rational_polynomial_taylor_shift_integer_shift(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c) in rational_polynomial_rational_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).taylor_shift({c}) = {}", (&p).taylor_shift(&c));
    }
}

fn benchmark_rational_polynomial_taylor_shift_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.taylor_shift(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            (
                "RationalPolynomial.taylor_shift(Rational)",
                &mut |(p, c)| {
                    no_out!(p.taylor_shift(c));
                },
            ),
            (
                "RationalPolynomial.taylor_shift(&Rational)",
                &mut |(p, c)| {
                    no_out!(p.taylor_shift(&c));
                },
            ),
            (
                "(&RationalPolynomial).taylor_shift(Rational)",
                &mut |(p, c)| {
                    no_out!((&p).taylor_shift(c));
                },
            ),
            (
                "(&RationalPolynomial).taylor_shift(&Rational)",
                &mut |(p, c)| {
                    no_out!((&p).taylor_shift(&c));
                },
            ),
            (
                "RationalPolynomial.taylor_shift_assign(Rational)",
                &mut |(mut p, c)| {
                    p.taylor_shift_assign(c);
                },
            ),
            (
                "RationalPolynomial.taylor_shift_assign(&Rational)",
                &mut |(mut p, c)| {
                    p.taylor_shift_assign(&c);
                },
            ),
        ],
    );
}

fn benchmark_rational_polynomial_taylor_shift_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.taylor_shift(&Rational)",
        BenchmarkType::Algorithms,
        rational_polynomial_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, c)| {
                no_out!((&p).taylor_shift(&c));
            }),
            ("naive", &mut |(p, c)| {
                no_out!(taylor_shift_naive(&p, &c));
            }),
        ],
    );
}
