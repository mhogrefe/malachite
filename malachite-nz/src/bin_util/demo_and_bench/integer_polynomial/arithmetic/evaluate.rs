// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::Evaluate;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::arithmetic::evaluate::{
    evaluate_divide_and_conquer, evaluate_horner,
};
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_integer_pair_gen, integer_polynomial_integer_pair_gen_var_2,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::evaluate::evaluate_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_evaluate);
    register_demo!(runner, demo_integer_polynomial_evaluate_ref);
    register_demo!(runner, demo_integer_polynomial_evaluate_long);
    register_demo!(runner, demo_integer_polynomial_evaluate_horner);
    register_demo!(runner, demo_integer_polynomial_evaluate_divide_and_conquer);
    register_demo!(
        runner,
        demo_integer_polynomial_evaluate_divide_and_conquer_long
    );

    register_bench!(
        runner,
        benchmark_integer_polynomial_evaluate_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_polynomial_evaluate_algorithms);
    register_bench!(
        runner,
        benchmark_integer_polynomial_evaluate_algorithms_long
    );
}

fn demo_integer_polynomial_evaluate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("(&({p})).evaluate({x_old}) = {}", (&p).evaluate(x));
    }
}

fn demo_integer_polynomial_evaluate_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).evaluate({x}) = {}", (&p).evaluate(&x));
    }
}

// Polynomials of degree at least 50, which take the divide-and-conquer path when the value has more
// than one limb.
fn demo_integer_polynomial_evaluate_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in integer_polynomial_integer_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).evaluate({x}) = {}", (&p).evaluate(&x));
    }
}

fn demo_integer_polynomial_evaluate_horner(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_horner({x}) = {}",
            evaluate_horner(p.coefficients_asc(), &x)
        );
    }
}

fn demo_integer_polynomial_evaluate_divide_and_conquer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x) in integer_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_divide_and_conquer({x}) = {}",
            evaluate_divide_and_conquer(p.coefficients_asc(), &x)
        );
    }
}

fn demo_integer_polynomial_evaluate_divide_and_conquer_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x) in integer_polynomial_integer_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_divide_and_conquer({x}) = {}",
            evaluate_divide_and_conquer(p.coefficients_asc(), &x)
        );
    }
}

fn benchmark_integer_polynomial_evaluate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerPolynomial).evaluate(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("(&IntegerPolynomial).evaluate(Integer)", &mut |(p, x)| {
                no_out!((&p).evaluate(x));
            }),
            ("(&IntegerPolynomial).evaluate(&Integer)", &mut |(p, x)| {
                no_out!((&p).evaluate(&x));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_evaluate_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerPolynomial).evaluate(&Integer)",
        BenchmarkType::Algorithms,
        integer_polynomial_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x)| no_out!((&p).evaluate(&x))),
            ("Horner", &mut |(p, x)| {
                no_out!(evaluate_horner(p.coefficients_asc(), &x));
            }),
            ("divide and conquer", &mut |(p, x)| {
                no_out!(evaluate_divide_and_conquer(p.coefficients_asc(), &x));
            }),
            ("naive", &mut |(p, x)| no_out!(evaluate_naive(&p, &x))),
        ],
    );
}

fn benchmark_integer_polynomial_evaluate_algorithms_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerPolynomial).evaluate(&Integer)",
        BenchmarkType::Algorithms,
        integer_polynomial_integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x)| no_out!((&p).evaluate(&x))),
            ("Horner", &mut |(p, x)| {
                no_out!(evaluate_horner(p.coefficients_asc(), &x));
            }),
            ("divide and conquer", &mut |(p, x)| {
                no_out!(evaluate_divide_and_conquer(p.coefficients_asc(), &x));
            }),
            ("naive", &mut |(p, x)| no_out!(evaluate_naive(&p, &x))),
        ],
    );
}
