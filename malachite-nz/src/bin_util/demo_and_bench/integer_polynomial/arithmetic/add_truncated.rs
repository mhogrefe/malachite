// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{AddTruncated, AddTruncatedAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_2_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::add_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_add_truncated);
    register_demo!(runner, demo_integer_polynomial_add_truncated_val_ref);
    register_demo!(runner, demo_integer_polynomial_add_truncated_ref_val);
    register_demo!(runner, demo_integer_polynomial_add_truncated_ref_ref);
    register_demo!(runner, demo_integer_polynomial_add_truncated_assign);
    register_demo!(runner, demo_integer_polynomial_add_truncated_assign_ref);

    register_bench!(
        runner,
        benchmark_integer_polynomial_add_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_add_truncated_algorithms
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_add_truncated_assign_evaluation_strategy
    );
}

fn demo_integer_polynomial_add_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).add_truncated({q_old}, {len}) = {}",
            p.add_truncated(q, len)
        );
    }
}

fn demo_integer_polynomial_add_truncated_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).add_truncated(&({q}), {len}) = {}",
            p.add_truncated(&q, len)
        );
    }
}

fn demo_integer_polynomial_add_truncated_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!(
            "(&({p})).add_truncated({q_old}, {len}) = {}",
            (&p).add_truncated(q, len)
        );
    }
}

fn demo_integer_polynomial_add_truncated_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).add_truncated(&({q}), {len}) = {}",
            (&p).add_truncated(&q, len)
        );
    }
}

fn demo_integer_polynomial_add_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.add_truncated_assign(q, len);
        println!("p := {p_old}; p.add_truncated_assign({q_old}, {len}); p = {p}");
    }
}

fn demo_integer_polynomial_add_truncated_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.add_truncated_assign(&q, len);
        println!("p := {p_old}; p.add_truncated_assign(&({q}), {len}); p = {p}");
    }
}

fn benchmark_integer_polynomial_add_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.add_truncated(IntegerPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "IntegerPolynomial.add_truncated(IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!(p.add_truncated(q, len));
                },
            ),
            (
                "IntegerPolynomial.add_truncated(&IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!(p.add_truncated(&q, len));
                },
            ),
            (
                "(&IntegerPolynomial).add_truncated(IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!((&p).add_truncated(q, len));
                },
            ),
            (
                "(&IntegerPolynomial).add_truncated(&IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!((&p).add_truncated(&q, len));
                },
            ),
        ],
    );
}

fn benchmark_integer_polynomial_add_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.add_truncated(IntegerPolynomial, u64)",
        BenchmarkType::Algorithms,
        integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, len)| {
                no_out!(p.add_truncated(q, len));
            }),
            ("naive", &mut |(p, q, len)| {
                no_out!(add_truncated_naive(&p, &q, len));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_add_truncated_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.add_truncated_assign(IntegerPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "IntegerPolynomial.add_truncated_assign(IntegerPolynomial, u64)",
                &mut |(mut p, q, len)| p.add_truncated_assign(q, len),
            ),
            (
                "IntegerPolynomial.add_truncated_assign(&IntegerPolynomial, u64)",
                &mut |(mut p, q, len)| p.add_truncated_assign(&q, len),
            ),
        ],
    );
}
