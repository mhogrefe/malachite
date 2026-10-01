// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{MulTruncated, MulTruncatedAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mul_truncated::mul_truncated_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mul_truncated);
    register_demo!(runner, demo_natural_polynomial_mul_truncated_val_ref);
    register_demo!(runner, demo_natural_polynomial_mul_truncated_ref_val);
    register_demo!(runner, demo_natural_polynomial_mul_truncated_ref_ref);
    register_demo!(runner, demo_natural_polynomial_mul_truncated_assign);
    register_demo!(runner, demo_natural_polynomial_mul_truncated_assign_ref);

    register_bench!(
        runner,
        benchmark_natural_polynomial_mul_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mul_truncated_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mul_truncated_assign_evaluation_strategy
    );
}

fn demo_natural_polynomial_mul_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mul_truncated({q_old}, {len}) = {}",
            p.mul_truncated(q, len)
        );
    }
}

fn demo_natural_polynomial_mul_truncated_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mul_truncated(&({q}), {len}) = {}",
            p.mul_truncated(&q, len)
        );
    }
}

fn demo_natural_polynomial_mul_truncated_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!(
            "(&({p})).mul_truncated({q_old}, {len}) = {}",
            (&p).mul_truncated(q, len)
        );
    }
}

fn demo_natural_polynomial_mul_truncated_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mul_truncated(&({q}), {len}) = {}",
            (&p).mul_truncated(&q, len)
        );
    }
}

fn demo_natural_polynomial_mul_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, len) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mul_truncated_assign(q, len);
        println!("p := {p_old}; p.mul_truncated_assign({q_old}, {len}); p = {p}");
    }
}

fn demo_natural_polynomial_mul_truncated_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, len) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mul_truncated_assign(&q, len);
        println!("p := {p_old}; p.mul_truncated_assign(&({q}), {len}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mul_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mul_truncated(NaturalPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "NaturalPolynomial.mul_truncated(NaturalPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!(p.mul_truncated(q, len));
                },
            ),
            (
                "NaturalPolynomial.mul_truncated(&NaturalPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!(p.mul_truncated(&q, len));
                },
            ),
            (
                "(&NaturalPolynomial).mul_truncated(NaturalPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!((&p).mul_truncated(q, len));
                },
            ),
            (
                "(&NaturalPolynomial).mul_truncated(&NaturalPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!((&p).mul_truncated(&q, len));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mul_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mul_truncated(NaturalPolynomial, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, len)| {
                no_out!(p.mul_truncated(q, len));
            }),
            ("naive", &mut |(p, q, len)| {
                no_out!(mul_truncated_naive(&p, &q, len));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mul_truncated_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mul_truncated_assign(NaturalPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "NaturalPolynomial.mul_truncated_assign(NaturalPolynomial, u64)",
                &mut |(mut p, q, len)| p.mul_truncated_assign(q, len),
            ),
            (
                "NaturalPolynomial.mul_truncated_assign(&NaturalPolynomial, u64)",
                &mut |(mut p, q, len)| p.mul_truncated_assign(&q, len),
            ),
        ],
    );
}
