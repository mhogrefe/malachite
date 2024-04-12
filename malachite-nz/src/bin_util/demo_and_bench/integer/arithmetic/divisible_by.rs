// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_integer_bit_bucketer, triple_3_pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_pair_gen, integer_pair_gen_nrm};
use malachite_nz::test_util::integer::arithmetic::divisible_by::num_divisible_by;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_divisible_by);
    register_demo!(runner, demo_integer_divisible_by_val_ref);
    register_demo!(runner, demo_integer_divisible_by_ref_val);
    register_demo!(runner, demo_integer_divisible_by_ref_ref);

    register_bench!(runner, benchmark_integer_divisible_by_library_comparison);
    register_bench!(runner, benchmark_integer_divisible_by_algorithms);
    register_bench!(runner, benchmark_integer_divisible_by_evaluation_strategy);
}

fn demo_integer_divisible_by(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        if x.divisible_by(y) {
            println!("{x_old} is divisible by {y_old}");
        } else {
            println!("{x_old} is not divisible by {y_old}");
        }
    }
}

fn demo_integer_divisible_by_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        if x.divisible_by(&y) {
            println!("{x_old} is divisible by {y}");
        } else {
            println!("{x_old} is not divisible by {y}");
        }
    }
}

fn demo_integer_divisible_by_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        if (&x).divisible_by(y) {
            println!("{x} is divisible by {y_old}");
        } else {
            println!("{x} is not divisible by {y_old}");
        }
    }
}

fn demo_integer_divisible_by_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        if (&x).divisible_by(&y) {
            println!("{x} is divisible by {y}");
        } else {
            println!("{x} is not divisible by {y}");
        }
    }
}

fn benchmark_integer_divisible_by_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.divisible_by(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            (
                "Malachite",
                &mut |(_, _, (x, y))| no_out!(x.divisible_by(y)),
            ),
            ("num", &mut |((x, y), _, _)| {
                no_out!(num_divisible_by(&x, &y))
            }),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.is_divisible(&y))),
        ],
    );
}

#[allow(clippy::no_effect, clippy::short_circuit_statement, unused_must_use)]
fn benchmark_integer_divisible_by_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.divisible_by(Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.divisible_by(y))),
            ("using %", &mut |(x, y)| {
                no_out!(x == 0 || y != 0 && x % y == 0)
            }),
        ],
    );
}

fn benchmark_integer_divisible_by_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.divisible_by(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.divisible_by(Integer)", &mut |(x, y)| {
                no_out!(x.divisible_by(y))
            }),
            ("Integer.divisible_by(&Integer)", &mut |(x, y)| {
                no_out!(x.divisible_by(&y))
            }),
            ("(&Integer).divisible_by(Integer)", &mut |(x, y)| {
                no_out!((&x).divisible_by(y))
            }),
            ("(&Integer).divisible_by(&Integer)", &mut |(x, y)| {
                no_out!((&x).divisible_by(&y))
            }),
        ],
    );
}
