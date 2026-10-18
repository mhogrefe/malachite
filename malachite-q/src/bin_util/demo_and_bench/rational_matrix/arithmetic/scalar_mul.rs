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
use malachite_q::test_util::bench::bucketers::pair_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_rational_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_mul_scalar);
    register_demo!(runner, demo_rational_matrix_mul_scalar_ref);
    register_demo!(runner, demo_rational_matrix_scalar_mul_ref);
    register_demo!(runner, demo_rational_matrix_mul_scalar_assign);

    register_bench!(
        runner,
        benchmark_rational_matrix_mul_scalar_evaluation_strategy
    );
}

fn demo_rational_matrix_mul_scalar(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, c) in rational_matrix_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old} * {c} = {}", a * &c);
    }
}

fn demo_rational_matrix_mul_scalar_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, c) in rational_matrix_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("&{a} * &{c} = {}", &a * &c);
    }
}

fn demo_rational_matrix_scalar_mul_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, c) in rational_matrix_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("&{c} * &{a} = {}", &c * &a);
    }
}

fn demo_rational_matrix_mul_scalar_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, c) in rational_matrix_rational_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a *= &c;
        println!("a := {a_old}; a *= {c}; a = {a}");
    }
}

fn benchmark_rational_matrix_mul_scalar_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix * Rational",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix * Rational", &mut |(a, c)| {
                no_out!(a * c);
            }),
            ("RationalMatrix * &Rational", &mut |(a, c)| {
                no_out!(a * &c);
            }),
            ("&RationalMatrix * Rational", &mut |(a, c)| {
                no_out!(&a * c);
            }),
            ("&RationalMatrix * &Rational", &mut |(a, c)| {
                no_out!(&a * &c);
            }),
            ("RationalMatrix *= Rational", &mut |(mut a, c)| {
                a *= c;
            }),
            ("RationalMatrix *= &Rational", &mut |(mut a, c)| {
                a *= &c;
            }),
        ],
    );
}
