// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_polynomial::arithmetic::mul::mul_divide_after;
use malachite_q::test_util::bench::bucketers::pair_rational_polynomial_max_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_pair_gen;
use malachite_q::test_util::rational_polynomial::arithmetic::mul::{mul_naive, mul_then_reduce};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_mul);
    register_demo!(runner, demo_rational_polynomial_mul_val_ref);
    register_demo!(runner, demo_rational_polynomial_mul_ref_val);
    register_demo!(runner, demo_rational_polynomial_mul_ref_ref);
    register_demo!(runner, demo_rational_polynomial_mul_assign);
    register_demo!(runner, demo_rational_polynomial_mul_assign_ref);

    register_bench!(
        runner,
        benchmark_rational_polynomial_mul_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_polynomial_mul_algorithms);
    register_bench!(
        runner,
        benchmark_rational_polynomial_mul_assign_evaluation_strategy
    );
}

fn demo_rational_polynomial_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}) * ({q_old}) = {}", p * q);
    }
}

fn demo_rational_polynomial_mul_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("({p_old}) * &({q}) = {}", p * &q);
    }
}

fn demo_rational_polynomial_mul_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        let q_old = q.clone();
        println!("&({p}) * ({q_old}) = {}", &p * q);
    }
}

fn demo_rational_polynomial_mul_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        println!("&({p}) * &({q}) = {}", &p * &q);
    }
}

fn demo_rational_polynomial_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        p *= q;
        println!("p := {p_old}; p *= {q_old}; p = {p}");
    }
}

fn demo_rational_polynomial_mul_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p *= &q;
        println!("p := {p_old}; p *= &{q}; p = {p}");
    }
}

fn benchmark_rational_polynomial_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial * RationalPolynomial",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("RationalPolynomial * RationalPolynomial", &mut |(p, q)| {
                no_out!(p * q);
            }),
            ("RationalPolynomial * &RationalPolynomial", &mut |(p, q)| {
                no_out!(p * &q);
            }),
            ("&RationalPolynomial * RationalPolynomial", &mut |(p, q)| {
                no_out!(&p * q);
            }),
            (
                "&RationalPolynomial * &RationalPolynomial",
                &mut |(p, q)| {
                    no_out!(&p * &q);
                },
            ),
        ],
    );
}

fn benchmark_rational_polynomial_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial * RationalPolynomial",
        BenchmarkType::Algorithms,
        rational_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| no_out!(p * q)),
            ("naive", &mut |(p, q)| no_out!(mul_naive(&p, &q))),
            ("multiply then reduce", &mut |(p, q)| {
                no_out!(mul_then_reduce(&p, &q));
            }),
            ("divide after", &mut |(p, q)| {
                no_out!(mul_divide_after(&p, &q));
            }),
        ],
    );
}

fn benchmark_rational_polynomial_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial *= RationalPolynomial",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "RationalPolynomial *= RationalPolynomial",
                &mut |(mut p, q)| p *= q,
            ),
            (
                "RationalPolynomial *= &RationalPolynomial",
                &mut |(mut p, q)| p *= &q,
            ),
        ],
    );
}

