// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::arithmetic::traits::EvaluateMany;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::{
    integer_polynomial_rational_vec_pair_gen, rational_polynomial_integer_vec_pair_gen,
    rational_polynomial_rational_vec_pair_gen,
};
use malachite_q::test_util::rational_polynomial::arithmetic::evaluate_many::*;

pub(crate) fn register(runner: &mut Runner) {
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
