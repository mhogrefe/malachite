// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::arithmetic::traits::{RemPrecomputed, RemPrecomputedAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen_var_6;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_rem_precomputed);
    register_demo!(runner, demo_integer_polynomial_rem_precomputed_ref_ref);
    register_demo!(runner, demo_integer_polynomial_rem_precomputed_assign);
    register_bench!(
        runner,
        benchmark_integer_polynomial_rem_precomputed_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_rem_precomputed_algorithms
    );
}

fn demo_integer_polynomial_rem_precomputed(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let data = IntegerPolynomial::precompute_rem_data(&q);
        println!(
            "({p_old}).rem_precomputed({q_old}, &data) = {}",
            p.rem_precomputed(q, &data)
        );
    }
}

fn demo_integer_polynomial_rem_precomputed_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let data = IntegerPolynomial::precompute_rem_data(&q);
        println!(
            "(&({p})).rem_precomputed(&({q}), &data) = {}",
            (&p).rem_precomputed(&q, &data)
        );
    }
}

fn demo_integer_polynomial_rem_precomputed_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let data = IntegerPolynomial::precompute_rem_data(&q);
        p.rem_precomputed_assign(q, &data);
        println!("p := {p_old}; p.rem_precomputed_assign({q_old}, &data); p = {p}");
    }
}

fn benchmark_integer_polynomial_rem_precomputed_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.rem_precomputed(IntegerPolynomial, &data)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen_var_6()
            .get(gm, config)
            .map(|(p, q)| {
                let data = IntegerPolynomial::precompute_rem_data(&q);
                (p, q, data)
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.rem_precomputed(q, &data)", &mut |(p, q, data)| {
                no_out!(p.rem_precomputed(q, &data));
            }),
            ("p.rem_precomputed(&q, &data)", &mut |(p, q, data)| {
                no_out!(p.rem_precomputed(&q, &data));
            }),
            ("(&p).rem_precomputed(q, &data)", &mut |(p, q, data)| {
                no_out!((&p).rem_precomputed(q, &data));
            }),
            ("(&p).rem_precomputed(&q, &data)", &mut |(p, q, data)| {
                no_out!((&p).rem_precomputed(&q, &data));
            }),
            ("p.rem_precomputed_assign(q, &data)", &mut |(
                mut p,
                q,
                data,
            )| {
                no_out!(p.rem_precomputed_assign(q, &data));
            }),
            ("p.rem_precomputed_assign(&q, &data)", &mut |(
                mut p,
                q,
                data,
            )| {
                no_out!(p.rem_precomputed_assign(&q, &data));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_rem_precomputed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.rem_precomputed(IntegerPolynomial, &data)",
        BenchmarkType::Algorithms,
        integer_polynomial_pair_gen_var_6()
            .get(gm, config)
            .map(|(p, q)| {
                let data = IntegerPolynomial::precompute_rem_data(&q);
                (p, q, data)
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("precomputed", &mut |(p, q, data)| {
                no_out!((&p).rem_precomputed(&q, &data));
            }),
            ("precomputed, including precomputation", &mut |(p, q, _)| {
                let data = IntegerPolynomial::precompute_rem_data(&q);
                no_out!((&p).rem_precomputed(&q, &data));
            }),
            ("not precomputed", &mut |(p, q, _)| {
                no_out!(&p % &q);
            }),
        ],
    );
}
