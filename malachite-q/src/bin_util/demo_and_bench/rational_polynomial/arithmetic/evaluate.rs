// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{Evaluate, EvaluateMany};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_q::Rational;
use malachite_q::rational_polynomial::arithmetic::evaluate::{
    evaluate_integer_polynomial_divide_and_conquer, evaluate_integer_polynomial_horner,
};
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::{
    integer_polynomial_rational_pair_gen, integer_polynomial_rational_pair_gen_var_1,
    integer_polynomial_rational_vec_pair_gen, rational_polynomial_integer_pair_gen,
    rational_polynomial_integer_vec_pair_gen, rational_polynomial_rational_pair_gen,
    rational_polynomial_rational_vec_pair_gen,
};
use malachite_q::test_util::rational_polynomial::arithmetic::evaluate::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_evaluate_rational);
    register_demo!(runner, demo_integer_polynomial_evaluate_rational_ref);
    register_demo!(runner, demo_integer_polynomial_evaluate_rational_long);
    register_demo!(runner, demo_integer_polynomial_evaluate_rational_horner);
    register_demo!(
        runner,
        demo_integer_polynomial_evaluate_rational_divide_and_conquer
    );
    register_demo!(
        runner,
        demo_integer_polynomial_evaluate_rational_divide_and_conquer_long
    );
    register_demo!(runner, demo_rational_polynomial_evaluate);
    register_demo!(runner, demo_rational_polynomial_evaluate_ref);
    register_demo!(runner, demo_rational_polynomial_evaluate_integer);
    register_demo!(runner, demo_rational_polynomial_evaluate_integer_ref);

    register_bench!(
        runner,
        benchmark_integer_polynomial_evaluate_rational_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_evaluate_rational_algorithms
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_evaluate_rational_algorithms_long
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_evaluate_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_polynomial_evaluate_algorithms);
    register_bench!(
        runner,
        benchmark_rational_polynomial_evaluate_integer_evaluation_strategy
    );
    register_demo!(runner, demo_integer_polynomial_evaluate_many_rational);
    register_demo!(runner, demo_rational_polynomial_evaluate_many);
    register_demo!(runner, demo_rational_polynomial_evaluate_many_integer);
    register_bench!(
        runner,
        benchmark_integer_polynomial_evaluate_many_rational_algorithms
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_evaluate_many_algorithms
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_evaluate_many_integer_algorithms
    );
}

fn demo_integer_polynomial_evaluate_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in integer_polynomial_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("(&({p})).evaluate({x_old}) = {}", (&p).evaluate(x));
    }
}

fn demo_integer_polynomial_evaluate_rational_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in integer_polynomial_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).evaluate({x}) = {}", (&p).evaluate(&x));
    }
}

// Polynomials of degree at least 40 at small values.
fn demo_integer_polynomial_evaluate_rational_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in integer_polynomial_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).evaluate({x}) = {}", (&p).evaluate(&x));
    }
}

fn demo_integer_polynomial_evaluate_rational_horner(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in integer_polynomial_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_horner({x}) = {}",
            evaluate_integer_polynomial_horner(p.coefficients_asc(), &x)
        );
    }
}

fn demo_integer_polynomial_evaluate_rational_divide_and_conquer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x) in integer_polynomial_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_divide_and_conquer({x}) = {}",
            evaluate_integer_polynomial_divide_and_conquer(p.coefficients_asc(), &x)
        );
    }
}

fn demo_integer_polynomial_evaluate_rational_divide_and_conquer_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x) in integer_polynomial_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_divide_and_conquer({x}) = {}",
            evaluate_integer_polynomial_divide_and_conquer(p.coefficients_asc(), &x)
        );
    }
}

fn benchmark_integer_polynomial_evaluate_rational_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerPolynomial).evaluate(Rational)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("(&IntegerPolynomial).evaluate(Rational)", &mut |(p, x)| {
                no_out!((&p).evaluate(x));
            }),
            ("(&IntegerPolynomial).evaluate(&Rational)", &mut |(p, x)| {
                no_out!((&p).evaluate(&x));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_evaluate_rational_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerPolynomial).evaluate(&Rational)",
        BenchmarkType::Algorithms,
        integer_polynomial_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x)| no_out!((&p).evaluate(&x))),
            ("Horner", &mut |(p, x)| {
                no_out!(evaluate_integer_polynomial_horner(p.coefficients_asc(), &x));
            }),
            ("divide and conquer", &mut |(p, x)| {
                no_out!(evaluate_integer_polynomial_divide_and_conquer(
                    p.coefficients_asc(),
                    &x
                ));
            }),
            ("naive", &mut |(p, x)| {
                no_out!(evaluate_integer_polynomial_naive(&p, &x));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_evaluate_rational_algorithms_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerPolynomial).evaluate(&Rational)",
        BenchmarkType::Algorithms,
        integer_polynomial_rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x)| no_out!((&p).evaluate(&x))),
            ("Horner", &mut |(p, x)| {
                no_out!(evaluate_integer_polynomial_horner(p.coefficients_asc(), &x));
            }),
            ("divide and conquer", &mut |(p, x)| {
                no_out!(evaluate_integer_polynomial_divide_and_conquer(
                    p.coefficients_asc(),
                    &x
                ));
            }),
            ("naive", &mut |(p, x)| {
                no_out!(evaluate_integer_polynomial_naive(&p, &x));
            }),
        ],
    );
}

fn demo_rational_polynomial_evaluate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in rational_polynomial_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("(&({p})).evaluate({x_old}) = {}", (&p).evaluate(x));
    }
}

fn demo_rational_polynomial_evaluate_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in rational_polynomial_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).evaluate({x}) = {}", (&p).evaluate(&x));
    }
}

fn benchmark_rational_polynomial_evaluate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalPolynomial).evaluate(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("(&RationalPolynomial).evaluate(Rational)", &mut |(p, x)| {
                no_out!((&p).evaluate(x));
            }),
            (
                "(&RationalPolynomial).evaluate(&Rational)",
                &mut |(p, x)| {
                    no_out!((&p).evaluate(&x));
                },
            ),
        ],
    );
}

fn benchmark_rational_polynomial_evaluate_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalPolynomial).evaluate(&Rational)",
        BenchmarkType::Algorithms,
        rational_polynomial_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x)| no_out!((&p).evaluate(&x))),
            ("naive", &mut |(p, x)| {
                no_out!(evaluate_rational_polynomial_naive(&p, &x));
            }),
        ],
    );
}

fn demo_rational_polynomial_evaluate_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in rational_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("(&({p})).evaluate({x_old}) = {}", (&p).evaluate(x));
    }
}

fn demo_rational_polynomial_evaluate_integer_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in rational_polynomial_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).evaluate({x}) = {}", (&p).evaluate(&x));
    }
}

fn benchmark_rational_polynomial_evaluate_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalPolynomial).evaluate(Integer)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("(&RationalPolynomial).evaluate(Integer)", &mut |(p, x)| {
                no_out!((&p).evaluate(x));
            }),
            ("(&RationalPolynomial).evaluate(&Integer)", &mut |(p, x)| {
                no_out!((&p).evaluate(&x));
            }),
            (
                "(&RationalPolynomial).evaluate(Rational::from(&Integer))",
                &mut |(p, x)| {
                    no_out!((&p).evaluate(Rational::from(&x)));
                },
            ),
        ],
    );
}

fn demo_integer_polynomial_evaluate_many_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, xs) in integer_polynomial_rational_vec_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let ys: Vec<String> = (&p)
            .evaluate_many(&xs)
            .iter()
            .map(ToString::to_string)
            .collect();
        let xs: Vec<String> = xs.iter().map(ToString::to_string).collect();
        println!("(&({p})).evaluate_many(&{xs:?}) = {ys:?}");
    }
}

fn demo_rational_polynomial_evaluate_many(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, xs) in rational_polynomial_rational_vec_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let ys: Vec<String> = (&p)
            .evaluate_many(&xs)
            .iter()
            .map(ToString::to_string)
            .collect();
        let xs: Vec<String> = xs.iter().map(ToString::to_string).collect();
        println!("(&({p})).evaluate_many(&{xs:?}) = {ys:?}");
    }
}

fn demo_rational_polynomial_evaluate_many_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, xs) in rational_polynomial_integer_vec_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let ys: Vec<String> = (&p)
            .evaluate_many(&xs)
            .iter()
            .map(ToString::to_string)
            .collect();
        let xs: Vec<String> = xs.iter().map(ToString::to_string).collect();
        println!("(&({p})).evaluate_many(&{xs:?}) = {ys:?}");
    }
}

fn benchmark_integer_polynomial_evaluate_many_rational_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerPolynomial).evaluate_many(&[Rational])",
        BenchmarkType::Algorithms,
        integer_polynomial_rational_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, xs)| no_out!((&p).evaluate_many(&xs))),
            ("naive", &mut |(p, xs)| {
                no_out!(evaluate_many_integer_polynomial_naive(&p, &xs));
            }),
        ],
    );
}

fn benchmark_rational_polynomial_evaluate_many_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalPolynomial).evaluate_many(&[Rational])",
        BenchmarkType::Algorithms,
        rational_polynomial_rational_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, xs)| no_out!((&p).evaluate_many(&xs))),
            ("naive", &mut |(p, xs)| {
                no_out!(evaluate_many_rational_polynomial_naive(&p, &xs));
            }),
        ],
    );
}

fn benchmark_rational_polynomial_evaluate_many_integer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalPolynomial).evaluate_many(&[Integer])",
        BenchmarkType::Algorithms,
        rational_polynomial_integer_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, xs)| no_out!((&p).evaluate_many(&xs))),
            ("naive", &mut |(p, xs)| {
                no_out!(evaluate_many_rational_polynomial_at_integers_naive(&p, &xs));
            }),
        ],
    );
}
