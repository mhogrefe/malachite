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
use malachite_nz::test_util::bench::bucketers::pair_gaussian_integer_max_bit_bucketer;
use malachite_nz::test_util::gaussian_integer::arithmetic::mul::gaussian_integer_mul_naive;
use malachite_nz::test_util::generators::gaussian_integer_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_mul);
    register_demo!(runner, demo_gaussian_integer_mul_val_ref);
    register_demo!(runner, demo_gaussian_integer_mul_ref_val);
    register_demo!(runner, demo_gaussian_integer_mul_ref_ref);
    register_demo!(runner, demo_gaussian_integer_mul_assign);
    register_demo!(runner, demo_gaussian_integer_mul_assign_ref);

    register_bench!(runner, benchmark_gaussian_integer_mul_algorithms);
    register_bench!(runner, benchmark_gaussian_integer_mul_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_gaussian_integer_mul_assign_evaluation_strategy
    );
}

fn demo_gaussian_integer_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({x_old}) * ({y_old}) = {}", x * y);
    }
}

fn demo_gaussian_integer_mul_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({x_old}) * (&{y}) = {}", x * &y);
    }
}

fn demo_gaussian_integer_mul_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{x}) * ({y_old}) = {}", &x * y);
    }
}

fn demo_gaussian_integer_mul_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen().get(gm, config).take(limit) {
        println!("(&{x}) * (&{y}) = {}", &x * &y);
    }
}

fn demo_gaussian_integer_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in gaussian_integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x *= y;
        println!("x := {x_old}; x *= {y_old}; x = {x}");
    }
}

fn demo_gaussian_integer_mul_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in gaussian_integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x *= &y;
        println!("x := {x_old}; x *= &{y}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger * GaussianInteger",
        BenchmarkType::Algorithms,
        gaussian_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!(&x * &y);
            }),
            ("naive", &mut |(x, y)| {
                no_out!(gaussian_integer_mul_naive(&x, &y));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger * GaussianInteger",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("GaussianInteger * GaussianInteger", &mut |(x, y)| {
                no_out!(x * y);
            }),
            ("GaussianInteger * &GaussianInteger", &mut |(x, y)| {
                no_out!(x * &y);
            }),
            ("&GaussianInteger * GaussianInteger", &mut |(x, y)| {
                no_out!(&x * y);
            }),
            ("&GaussianInteger * &GaussianInteger", &mut |(x, y)| {
                no_out!(&x * &y);
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger *= GaussianInteger",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("GaussianInteger *= GaussianInteger", &mut |(mut x, y)| {
                no_out!(x *= y);
            }),
            ("GaussianInteger *= &GaussianInteger", &mut |(mut x, y)| {
                no_out!(x *= &y);
            }),
        ],
    );
}
