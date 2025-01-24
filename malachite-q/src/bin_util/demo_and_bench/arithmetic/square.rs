// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_square);
    register_demo!(runner, demo_rational_square_ref);
    register_demo!(runner, demo_rational_square_assign);

    register_bench!(runner, benchmark_rational_square_evaluation_strategy);
    register_bench!(runner, benchmark_rational_square_algorithms);
    register_bench!(runner, benchmark_rational_square_assign);
}

fn demo_rational_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen().get(gm, config).take(limit) {
        println!("{} ^ 2 = {}", n.clone(), n.square());
    }
}

fn demo_rational_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen().get(gm, config).take(limit) {
        println!("&{} ^ 2 = {}", n, (&n).square());
    }
}

fn demo_rational_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in rational_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.square_assign();
        println!("n := {n_old}; n.square_assign(); n = {n}");
    }
}

fn benchmark_rational_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.square()",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("Rational.square()", &mut |n| no_out!(n.square())),
            ("(&Rational).square()", &mut |n| no_out!((&n).square())),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.square()",
        BenchmarkType::Algorithms,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("standard", &mut |ref n| no_out!(n.square())),
            ("using *", &mut |ref n| no_out!(n * n)),
        ],
    );
}

fn benchmark_rational_square_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.square_assign()",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut n| n.square_assign())],
    );
}
