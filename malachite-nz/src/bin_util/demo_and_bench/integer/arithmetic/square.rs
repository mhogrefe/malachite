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
use malachite_nz::test_util::bench::bucketers::integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_square_assign);
    register_demo!(runner, demo_integer_square);
    register_demo!(runner, demo_integer_square_ref);

    register_bench!(runner, benchmark_integer_square_assign);
    register_bench!(runner, benchmark_integer_square_algorithms);
    register_bench!(runner, benchmark_integer_square_evaluation_strategy);
}

fn demo_integer_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in integer_gen().get(gm, config).take(limit) {
        let old_n = n.clone();
        n.square_assign();
        println!("n := {n}; n.square_assign(); n = {old_n}");
    }
}

fn demo_integer_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("({}) ^ 2 = {}", n.clone(), n.square());
    }
}

fn demo_integer_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("&{} ^ 2 = {}", n, (&n).square());
    }
}

fn benchmark_integer_square_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.square_assign()",
        BenchmarkType::Single,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |mut n| n.square_assign())],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.square()",
        BenchmarkType::Algorithms,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("standard", &mut |ref n| no_out!(n.square())),
            ("using *", &mut |ref n| no_out!(n * n)),
        ],
    );
}

fn benchmark_integer_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.square()",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("Integer.square()", &mut |n| no_out!(n.square())),
            ("(&Integer).square()", &mut |n| no_out!((&n).square())),
        ],
    );
}
