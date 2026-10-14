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
use malachite_nz::integer_polynomial::arithmetic::taylor_shift::{
    taylor_shift_divide_and_conquer, taylor_shift_horner,
};
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_integer_pair_gen, integer_polynomial_integer_pair_gen_var_2,
    integer_polynomial_integer_pair_gen_var_4,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::taylor_shift::taylor_shift_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_taylor_shift);
    register_demo!(runner, demo_integer_polynomial_taylor_shift_ref);
    register_demo!(runner, demo_integer_polynomial_taylor_shift_assign);
    register_demo!(runner, demo_integer_polynomial_taylor_shift_long);
    register_demo!(runner, demo_integer_polynomial_taylor_shift_small_shift);
    register_demo!(runner, demo_integer_polynomial_taylor_shift_horner);
    register_demo!(
        runner,
        demo_integer_polynomial_taylor_shift_divide_and_conquer
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_taylor_shift_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_polynomial_taylor_shift_algorithms);
    register_bench!(
        runner,
        benchmark_integer_polynomial_taylor_shift_algorithms_long
    );
}

fn demo_integer_polynomial_taylor_shift(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).taylor_shift({c}) = {}", p.taylor_shift(&c));
    }
}

fn demo_integer_polynomial_taylor_shift_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).taylor_shift({c}) = {}", (&p).taylor_shift(&c));
    }
}

fn demo_integer_polynomial_taylor_shift_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, c) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.taylor_shift_assign(&c);
        println!("p := {p_old}; p.taylor_shift_assign({c}); p = {p}");
    }
}

// Polynomials of degree at least 50, some long enough for the divide-and-conquer path.
fn demo_integer_polynomial_taylor_shift_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in integer_polynomial_integer_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).taylor_shift({c}) = {}", (&p).taylor_shift(&c));
    }
}

// Shifts between -3 and 3, so that 0, 1, and -1, which take their own paths, are common.
fn demo_integer_polynomial_taylor_shift_small_shift(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in integer_polynomial_integer_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).taylor_shift({c}) = {}", (&p).taylor_shift(&c));
    }
}

fn demo_integer_polynomial_taylor_shift_horner(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let mut xs = p.coefficients_asc().to_vec();
        taylor_shift_horner(&mut xs, &c);
        println!("taylor_shift_horner({p}, {c}) = {xs:?}");
    }
}

fn demo_integer_polynomial_taylor_shift_divide_and_conquer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c) in integer_polynomial_integer_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut xs = p.coefficients_asc().to_vec();
        taylor_shift_divide_and_conquer(&mut xs, &c);
        println!("taylor_shift_divide_and_conquer({p}, {c}) = {xs:?}");
    }
}

fn benchmark_integer_polynomial_taylor_shift_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.taylor_shift(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial.taylor_shift(Integer)", &mut |(p, c)| {
                no_out!(p.taylor_shift(c));
            }),
            ("IntegerPolynomial.taylor_shift(&Integer)", &mut |(p, c)| {
                no_out!(p.taylor_shift(&c));
            }),
            (
                "(&IntegerPolynomial).taylor_shift(Integer)",
                &mut |(p, c)| {
                    no_out!((&p).taylor_shift(c));
                },
            ),
            (
                "(&IntegerPolynomial).taylor_shift(&Integer)",
                &mut |(p, c)| {
                    no_out!((&p).taylor_shift(&c));
                },
            ),
            (
                "IntegerPolynomial.taylor_shift_assign(Integer)",
                &mut |(mut p, c)| {
                    p.taylor_shift_assign(c);
                },
            ),
            (
                "IntegerPolynomial.taylor_shift_assign(&Integer)",
                &mut |(mut p, c)| {
                    p.taylor_shift_assign(&c);
                },
            ),
        ],
    );
}

fn benchmark_integer_polynomial_taylor_shift_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.taylor_shift(&Integer)",
        BenchmarkType::Algorithms,
        integer_polynomial_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, c)| {
                no_out!((&p).taylor_shift(&c));
            }),
            ("Horner", &mut |(p, c)| {
                let mut xs = p.coefficients_asc().to_vec();
                taylor_shift_horner(&mut xs, &c);
            }),
            ("divide and conquer", &mut |(p, c)| {
                let mut xs = p.coefficients_asc().to_vec();
                taylor_shift_divide_and_conquer(&mut xs, &c);
            }),
            ("naive", &mut |(p, c)| {
                no_out!(taylor_shift_naive(&p, &c));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_taylor_shift_algorithms_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.taylor_shift(&Integer)",
        BenchmarkType::Algorithms,
        integer_polynomial_integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, c)| {
                no_out!((&p).taylor_shift(&c));
            }),
            ("Horner", &mut |(p, c)| {
                let mut xs = p.coefficients_asc().to_vec();
                taylor_shift_horner(&mut xs, &c);
            }),
            ("divide and conquer", &mut |(p, c)| {
                let mut xs = p.coefficients_asc().to_vec();
                taylor_shift_divide_and_conquer(&mut xs, &c);
            }),
        ],
    );
}
