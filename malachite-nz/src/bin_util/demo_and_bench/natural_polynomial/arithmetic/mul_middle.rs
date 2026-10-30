// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{MulMiddle, MulMiddleAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mul_middle::mul_middle_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mul_middle);
    register_demo!(runner, demo_natural_polynomial_mul_middle_val_ref);
    register_demo!(runner, demo_natural_polynomial_mul_middle_ref_val);
    register_demo!(runner, demo_natural_polynomial_mul_middle_ref_ref);
    register_demo!(runner, demo_natural_polynomial_mul_middle_assign);
    register_demo!(runner, demo_natural_polynomial_mul_middle_assign_ref);

    register_bench!(
        runner,
        benchmark_natural_polynomial_mul_middle_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_mul_middle_algorithms);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mul_middle_assign_evaluation_strategy
    );
}

fn demo_natural_polynomial_mul_middle(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, low, high) in
        natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mul_middle({q_old}, {low}, {high}) = {}",
            p.mul_middle(q, low, high)
        );
    }
}

fn demo_natural_polynomial_mul_middle_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, low, high) in
        natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mul_middle(&({q_old}), {low}, {high}) = {}",
            p.mul_middle(&q, low, high)
        );
    }
}

fn demo_natural_polynomial_mul_middle_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, low, high) in
        natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "(&({p_old})).mul_middle({q_old}, {low}, {high}) = {}",
            (&p).mul_middle(q, low, high)
        );
    }
}

fn demo_natural_polynomial_mul_middle_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, low, high) in
        natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "(&({p_old})).mul_middle(&({q_old}), {low}, {high}) = {}",
            (&p).mul_middle(&q, low, high)
        );
    }
}

fn demo_natural_polynomial_mul_middle_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, low, high) in
        natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mul_middle_assign(q, low, high);
        println!("p := {p_old}; p.mul_middle_assign({q_old}, {low}, {high}); p = {p}");
    }
}

fn demo_natural_polynomial_mul_middle_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, low, high) in
        natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mul_middle_assign(&q, low, high);
        println!("p := {p_old}; p.mul_middle_assign(&({q_old}), {low}, {high}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mul_middle_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mul_middle(NaturalPolynomial, u64, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "NaturalPolynomial.mul_middle(NaturalPolynomial, u64, u64)",
                &mut |(p, q, low, high)| {
                    no_out!(p.mul_middle(q, low, high));
                },
            ),
            (
                "NaturalPolynomial.mul_middle(&NaturalPolynomial, u64, u64)",
                &mut |(p, q, low, high)| {
                    no_out!(p.mul_middle(&q, low, high));
                },
            ),
            (
                "(&NaturalPolynomial).mul_middle(NaturalPolynomial, u64, u64)",
                &mut |(p, q, low, high)| {
                    no_out!((&p).mul_middle(q, low, high));
                },
            ),
            (
                "(&NaturalPolynomial).mul_middle(&NaturalPolynomial, u64, u64)",
                &mut |(p, q, low, high)| {
                    no_out!((&p).mul_middle(&q, low, high));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mul_middle_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mul_middle(NaturalPolynomial, u64, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, low, high)| {
                no_out!(p.mul_middle(q, low, high));
            }),
            ("naive", &mut |(p, q, low, high)| {
                no_out!(mul_middle_naive(&p, &q, low, high));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mul_middle_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mul_middle_assign(NaturalPolynomial, u64, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "NaturalPolynomial.mul_middle_assign(NaturalPolynomial, u64, u64)",
                &mut |(mut p, q, low, high)| p.mul_middle_assign(q, low, high),
            ),
            (
                "NaturalPolynomial.mul_middle_assign(&NaturalPolynomial, u64, u64)",
                &mut |(mut p, q, low, high)| p.mul_middle_assign(&q, low, high),
            ),
        ],
    );
}
