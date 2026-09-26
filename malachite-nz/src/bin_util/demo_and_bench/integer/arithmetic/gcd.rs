// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign, UnsignedAbs};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_gcd);
    register_demo!(runner, demo_integer_gcd_val_ref);
    register_demo!(runner, demo_integer_gcd_ref_val);
    register_demo!(runner, demo_integer_gcd_ref_ref);
    register_demo!(runner, demo_integer_gcd_assign);
    register_demo!(runner, demo_integer_gcd_assign_ref);

    register_bench!(runner, benchmark_integer_gcd_evaluation_strategy);
    register_bench!(runner, benchmark_integer_gcd_algorithms);
    register_bench!(runner, benchmark_integer_gcd_assign_evaluation_strategy);
}

fn demo_integer_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).gcd({}) = {}", x_old, y_old, x.gcd(y));
    }
}

fn demo_integer_gcd_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).gcd(&{}) = {}", x_old, y, x.gcd(&y));
    }
}

fn demo_integer_gcd_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).gcd({}) = {}", x, y_old, (&x).gcd(y));
    }
}

fn demo_integer_gcd_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        println!("(&{}).gcd(&{}) = {}", x, y, (&x).gcd(&y));
    }
}

fn demo_integer_gcd_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.gcd_assign(y);
        println!("x := {x_old}; x.gcd_assign({y_old}); x = {x}");
    }
}

fn demo_integer_gcd_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.gcd_assign(&y);
        println!("x := {x_old}; x.gcd_assign(&{y}); x = {x}");
    }
}

fn benchmark_integer_gcd_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.gcd(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Integer.gcd(Integer)", &mut |(x, y)| no_out!(x.gcd(y))),
            ("Integer.gcd(&Integer)", &mut |(x, y)| no_out!(x.gcd(&y))),
            ("(&Integer).gcd(Integer)", &mut |(x, y)| {
                no_out!((&x).gcd(y));
            }),
            ("(&Integer).gcd(&Integer)", &mut |(x, y)| {
                no_out!((&x).gcd(&y));
            }),
        ],
    );
}

fn benchmark_integer_gcd_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.gcd(Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.gcd(y))),
            ("via absolute values", &mut |(x, y)| {
                no_out!(x.unsigned_abs().gcd(y.unsigned_abs()));
            }),
        ],
    );
}

fn benchmark_integer_gcd_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.gcd_assign(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Integer.gcd_assign(Integer)", &mut |(mut x, y)| {
                x.gcd_assign(y);
            }),
            ("Integer.gcd_assign(&Integer)", &mut |(mut x, y)| {
                x.gcd_assign(&y);
            }),
        ],
    );
}
