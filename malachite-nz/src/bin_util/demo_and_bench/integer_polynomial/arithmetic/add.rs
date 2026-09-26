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
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen;
use malachite_nz::test_util::integer_polynomial::arithmetic::add::add_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_add);
    register_demo!(runner, demo_integer_polynomial_add_val_ref);
    register_demo!(runner, demo_integer_polynomial_add_ref_val);
    register_demo!(runner, demo_integer_polynomial_add_ref_ref);
    register_demo!(runner, demo_integer_polynomial_add_assign);
    register_demo!(runner, demo_integer_polynomial_add_assign_ref);

    register_bench!(runner, benchmark_integer_polynomial_add_evaluation_strategy);
    register_bench!(runner, benchmark_integer_polynomial_add_algorithms);
    register_bench!(
        runner,
        benchmark_integer_polynomial_add_assign_evaluation_strategy
    );
}

fn demo_integer_polynomial_add(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}) + ({q_old}) = {}", p + q);
    }
}

fn demo_integer_polynomial_add_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("({p_old}) + &({q}) = {}", p + &q);
    }
}

fn demo_integer_polynomial_add_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let q_old = q.clone();
        println!("&({p}) + ({q_old}) = {}", &p + q);
    }
}

fn demo_integer_polynomial_add_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        println!("&({p}) + &({q}) = {}", &p + &q);
    }
}

fn demo_integer_polynomial_add_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        p += q;
        println!("p := {p_old}; p += {q_old}; p = {p}");
    }
}

fn demo_integer_polynomial_add_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p += &q;
        println!("p := {p_old}; p += &{q}; p = {p}");
    }
}

fn benchmark_integer_polynomial_add_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial + IntegerPolynomial",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("IntegerPolynomial + IntegerPolynomial", &mut |(p, q)| {
                no_out!(p + q);
            }),
            ("IntegerPolynomial + &IntegerPolynomial", &mut |(p, q)| {
                no_out!(p + &q);
            }),
            ("&IntegerPolynomial + IntegerPolynomial", &mut |(p, q)| {
                no_out!(&p + q);
            }),
            ("&IntegerPolynomial + &IntegerPolynomial", &mut |(p, q)| {
                no_out!(&p + &q);
            }),
        ],
    );
}

fn benchmark_integer_polynomial_add_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial + IntegerPolynomial",
        BenchmarkType::Algorithms,
        integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| no_out!(p + q)),
            ("naive", &mut |(p, q)| no_out!(add_naive(&p, &q))),
        ],
    );
}

fn benchmark_integer_polynomial_add_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial += IntegerPolynomial",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "IntegerPolynomial += IntegerPolynomial",
                &mut |(mut p, q)| p += q,
            ),
            (
                "IntegerPolynomial += &IntegerPolynomial",
                &mut |(mut p, q)| p += &q,
            ),
        ],
    );
}
