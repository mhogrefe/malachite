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
use malachite_nz::test_util::bench::bucketers::natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_gen;
use malachite_nz::test_util::natural_polynomial::arithmetic::square::square_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_square);
    register_demo!(runner, demo_natural_polynomial_square_ref);
    register_demo!(runner, demo_natural_polynomial_square_assign);

    register_bench!(
        runner,
        benchmark_natural_polynomial_square_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_square_algorithms);
}

fn demo_natural_polynomial_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("({p_old}).square() = {}", p.square());
    }
}

fn demo_natural_polynomial_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        println!("(&({p})).square() = {}", (&p).square());
    }
}

fn demo_natural_polynomial_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut p in natural_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p.square_assign();
        println!("p := {p_old}; p.square_assign(); p = {p}");
    }
}

fn benchmark_natural_polynomial_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.square()",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [
            ("NaturalPolynomial.square()", &mut |p| {
                no_out!(p.square());
            }),
            ("(&NaturalPolynomial).square()", &mut |p| {
                no_out!((&p).square());
            }),
        ],
    );
}

fn benchmark_natural_polynomial_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.square()",
        BenchmarkType::Algorithms,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |p| {
                no_out!(p.square());
            }),
            ("using *", &mut |p| {
                no_out!(&p * &p);
            }),
            ("naive", &mut |p| {
                no_out!(square_naive(&p));
            }),
        ],
    );
}
