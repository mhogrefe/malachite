// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ExtendedGcd, Gcd, GcdAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_rational_max_bit_bucketer;
use malachite_q::test_util::generators::rational_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_gcd);
    register_demo!(runner, demo_rational_gcd_ref);
    register_demo!(runner, demo_rational_gcd_assign);
    register_demo!(runner, demo_rational_extended_gcd);
    register_demo!(runner, demo_rational_extended_gcd_ref);

    register_bench!(runner, benchmark_rational_gcd_evaluation_strategy);
    register_bench!(runner, benchmark_rational_extended_gcd_evaluation_strategy);
}

fn demo_rational_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{x_old}.gcd({y_old}) = {}", x.gcd(y));
    }
}

fn demo_rational_gcd_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        println!("(&{x}).gcd(&{y}) = {}", (&x).gcd(&y));
    }
}

fn demo_rational_gcd_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.gcd_assign(y);
        println!("x := {x_old}; x.gcd_assign({y_old}); x = {x}");
    }
}

fn demo_rational_extended_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{x_old}.extended_gcd({y_old}) = {:?}", x.extended_gcd(y));
    }
}

fn demo_rational_extended_gcd_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        println!("(&{x}).extended_gcd(&{y}) = {:?}", (&x).extended_gcd(&y));
    }
}

fn benchmark_rational_gcd_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.gcd(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("Rational.gcd(Rational)", &mut |(x, y)| {
                no_out!(x.gcd(y));
            }),
            ("Rational.gcd(&Rational)", &mut |(x, y)| {
                no_out!(x.gcd(&y));
            }),
            ("(&Rational).gcd(Rational)", &mut |(x, y)| {
                no_out!((&x).gcd(y));
            }),
            ("(&Rational).gcd(&Rational)", &mut |(x, y)| {
                no_out!((&x).gcd(&y));
            }),
        ],
    );
}

fn benchmark_rational_extended_gcd_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.extended_gcd(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("Rational.extended_gcd(Rational)", &mut |(x, y)| {
                no_out!(x.extended_gcd(y));
            }),
            ("Rational.extended_gcd(&Rational)", &mut |(x, y)| {
                no_out!(x.extended_gcd(&y));
            }),
            ("(&Rational).extended_gcd(Rational)", &mut |(x, y)| {
                no_out!((&x).extended_gcd(y));
            }),
            ("(&Rational).extended_gcd(&Rational)", &mut |(x, y)| {
                no_out!((&x).extended_gcd(&y));
            }),
        ],
    );
}
