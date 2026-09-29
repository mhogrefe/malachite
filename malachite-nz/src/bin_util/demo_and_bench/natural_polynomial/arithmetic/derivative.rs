// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{Derivative, DerivativeAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_gen;
use malachite_nz::test_util::natural_polynomial::arithmetic::derivative::derivative_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_derivative);
    register_demo!(runner, demo_natural_polynomial_derivative_ref);
    register_demo!(runner, demo_natural_polynomial_derivative_assign);
    register_bench!(
        runner,
        benchmark_natural_polynomial_derivative_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_derivative_algorithms);
}

fn demo_natural_polynomial_derivative(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("({p_old}).derivative() = {}", p.derivative());
    }
}

fn demo_natural_polynomial_derivative_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        println!("(&({p})).derivative() = {}", (&p).derivative());
    }
}

fn demo_natural_polynomial_derivative_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut p in natural_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p.derivative_assign();
        println!("p := {p_old}; p.derivative_assign(); p = {p}");
    }
}

fn benchmark_natural_polynomial_derivative_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.derivative()",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [
            ("p.derivative()", &mut |p| no_out!(p.derivative())),
            ("(&p).derivative()", &mut |p| no_out!((&p).derivative())),
            ("p.derivative_assign()", &mut |mut p| p.derivative_assign()),
        ],
    );
}

fn benchmark_natural_polynomial_derivative_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.derivative()",
        BenchmarkType::Algorithms,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |p| no_out!((&p).derivative())),
            ("naive", &mut |p| no_out!(derivative_naive(&p))),
        ],
    );
}
