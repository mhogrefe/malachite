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
use malachite_q::test_util::bench::bucketers::gaussian_rational_bit_bucketer;
use malachite_q::test_util::gaussian_rational::arithmetic::square::gaussian_rational_square_naive;
use malachite_q::test_util::generators::gaussian_rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_square);
    register_demo!(runner, demo_gaussian_rational_square_ref);
    register_demo!(runner, demo_gaussian_rational_square_assign);

    register_bench!(runner, benchmark_gaussian_rational_square_algorithms);
    register_bench!(
        runner,
        benchmark_gaussian_rational_square_evaluation_strategy
    );
}

fn demo_gaussian_rational_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        println!("({}) ^ 2 = {}", x.clone(), x.square());
    }
}

fn demo_gaussian_rational_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        println!("(&{x}) ^ 2 = {}", (&x).square());
    }
}

fn demo_gaussian_rational_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in gaussian_rational_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.square_assign();
        println!("x := {old_x}; x.square_assign(); x = {x}");
    }
}

fn benchmark_gaussian_rational_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.square()",
        BenchmarkType::Algorithms,
        gaussian_rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_rational_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| {
                no_out!((&x).square());
            }),
            ("naive", &mut |x| {
                no_out!(gaussian_rational_square_naive(&x));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.square()",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_rational_bit_bucketer("x"),
        &mut [
            ("GaussianRational.square()", &mut |x| {
                no_out!(x.square());
            }),
            ("(&GaussianRational).square()", &mut |x| {
                no_out!((&x).square());
            }),
        ],
    );
}
