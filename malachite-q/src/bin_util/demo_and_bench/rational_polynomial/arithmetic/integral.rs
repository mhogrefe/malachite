// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{Integral, IntegralAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_gen;
use malachite_q::test_util::rational_polynomial::arithmetic::integral::integral_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_integral);
    register_demo!(runner, demo_rational_polynomial_integral_ref);
    register_demo!(runner, demo_rational_polynomial_integral_assign);
    register_bench!(
        runner,
        benchmark_rational_polynomial_integral_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_polynomial_integral_algorithms);
}

fn demo_rational_polynomial_integral(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("({p_old}).integral() = {}", p.integral());
    }
}

fn demo_rational_polynomial_integral_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!("(&({p})).integral() = {}", (&p).integral());
    }
}

fn demo_rational_polynomial_integral_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut p in rational_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p.integral_assign();
        println!("p := {p_old}; p.integral_assign(); p = {p}");
    }
}

fn benchmark_rational_polynomial_integral_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.integral()",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [
            ("p.integral()", &mut |p| no_out!(p.integral())),
            ("(&p).integral()", &mut |p| no_out!((&p).integral())),
            ("p.integral_assign()", &mut |mut p| p.integral_assign()),
        ],
    );
}

fn benchmark_rational_polynomial_integral_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.integral()",
        BenchmarkType::Algorithms,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |p| no_out!((&p).integral())),
            ("naive", &mut |p| no_out!(integral_naive(&p))),
        ],
    );
}
