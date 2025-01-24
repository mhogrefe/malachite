// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::arithmetic::traits::SimplestRationalInInterval;
use malachite_q::test_util::arithmetic::simplest_rational_in_interval::*;
use malachite_q::test_util::bench::bucketers::pair_rational_max_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_pair_gen, rational_pair_gen_var_3, rational_pair_gen_var_4, rational_pair_gen_var_5,
    rational_pair_gen_var_6,
};
use malachite_q::Rational;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_cmp_complexity);
    register_demo!(runner, demo_simplest_rational_in_open_interval);
    register_demo!(runner, demo_simplest_rational_in_closed_interval);

    register_bench!(runner, benchmark_rational_cmp_complexity);
    register_bench!(
        runner,
        benchmark_simplest_rational_in_open_interval_algorithms
    );
    register_bench!(
        runner,
        benchmark_simplest_rational_in_open_interval_algorithms_2
    );
    register_bench!(runner, benchmark_simplest_rational_in_closed_interval);
    register_bench!(
        runner,
        benchmark_simplest_rational_in_closed_interval_algorithms
    );
}

fn demo_rational_cmp_complexity(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        match x.cmp_complexity(&y) {
            Less => println!("{x} c< {y}"),
            Equal => println!("{x} c= {y}"),
            Greater => println!("{x} c> {y}"),
        }
    }
}

fn demo_simplest_rational_in_open_interval(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_3().get(gm, config).take(limit) {
        println!(
            "simplest_rational_in_open_interval({}, {}) = {}",
            x,
            y,
            Rational::simplest_rational_in_open_interval(&x, &y)
        );
    }
}

fn demo_simplest_rational_in_closed_interval(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_4().get(gm, config).take(limit) {
        println!(
            "simplest_rational_in_closed_interval({}, {}) = {}",
            x,
            y,
            Rational::simplest_rational_in_closed_interval(&x, &y)
        );
    }
}

fn benchmark_rational_cmp_complexity(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.cmp_complexity(&Rational)",
        BenchmarkType::Single,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.cmp_complexity(&y)))],
    );
}

fn benchmark_simplest_rational_in_open_interval_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "simplest_rational_in_open_interval(&Rational, &Rational)",
        BenchmarkType::Algorithms,
        rational_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!(Rational::simplest_rational_in_open_interval(&x, &y))
            }),
            ("explicit", &mut |(x, y)| {
                no_out!(simplest_rational_in_open_interval_explicit(&x, &y))
            }),
            ("naive", &mut |(x, y)| {
                no_out!(simplest_rational_in_open_interval_naive(&x, &y))
            }),
        ],
    );
}

fn benchmark_simplest_rational_in_open_interval_algorithms_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "simplest_rational_in_open_interval(&Rational, &Rational)",
        BenchmarkType::Algorithms,
        rational_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!(Rational::simplest_rational_in_open_interval(&x, &y))
            }),
            ("explicit", &mut |(x, y)| {
                no_out!(simplest_rational_in_open_interval_explicit(&x, &y))
            }),
        ],
    );
}

fn benchmark_simplest_rational_in_closed_interval(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "simplest_rational_in_closed_interval(&Rational, &Rational)",
        BenchmarkType::Single,
        rational_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(Rational::simplest_rational_in_closed_interval(&x, &y))
        })],
    );
}

fn benchmark_simplest_rational_in_closed_interval_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "simplest_rational_in_closed_interval(&Rational, &Rational)",
        BenchmarkType::Algorithms,
        rational_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!(Rational::simplest_rational_in_closed_interval(&x, &y))
            }),
            ("naive", &mut |(x, y)| {
                no_out!(simplest_rational_in_closed_interval_naive(&x, &y))
            }),
        ],
    );
}
