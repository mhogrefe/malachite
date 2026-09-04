// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_1_gaussian_rational_bit_bucketer;
use malachite_q::test_util::gaussian_rational::arithmetic::pow::*;
use malachite_q::test_util::generators::{
    gaussian_rational_signed_pair_gen_var_1, gaussian_rational_unsigned_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_pow_u64);
    register_demo!(runner, demo_gaussian_rational_pow_u64_ref);
    register_demo!(runner, demo_gaussian_rational_pow_assign_u64);
    register_demo!(runner, demo_gaussian_rational_pow_i64);
    register_demo!(runner, demo_gaussian_rational_pow_i64_ref);
    register_demo!(runner, demo_gaussian_rational_pow_assign_i64);

    register_bench!(
        runner,
        benchmark_gaussian_rational_pow_u64_evaluation_strategy
    );
    register_bench!(runner, benchmark_gaussian_rational_pow_u64_algorithms);
    register_bench!(runner, benchmark_gaussian_rational_pow_u64_assign);
    register_bench!(
        runner,
        benchmark_gaussian_rational_pow_i64_evaluation_strategy
    );
    register_bench!(runner, benchmark_gaussian_rational_pow_i64_assign);
}

fn demo_gaussian_rational_pow_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in gaussian_rational_unsigned_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({x_old}).pow({exp}) = {}", x.pow(exp));
    }
}

fn demo_gaussian_rational_pow_u64_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in gaussian_rational_unsigned_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{x}).pow({exp}) = {}", (&x).pow(exp));
    }
}

fn demo_gaussian_rational_pow_assign_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp) in gaussian_rational_unsigned_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.pow_assign(exp);
        println!("x := {x_old}; x.pow_assign({exp}); x = {x}");
    }
}

fn demo_gaussian_rational_pow_i64(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in gaussian_rational_signed_pair_gen_var_1::<i64>()
        .get(gm, config)
        .take(limit)
    {
        if x == 0u32 && exp < 0 {
            continue;
        }
        let x_old = x.clone();
        println!("({x_old}).pow({exp}) = {}", x.pow(exp));
    }
}

fn demo_gaussian_rational_pow_i64_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in gaussian_rational_signed_pair_gen_var_1::<i64>()
        .get(gm, config)
        .take(limit)
    {
        if x == 0u32 && exp < 0 {
            continue;
        }
        println!("(&{x}).pow({exp}) = {}", (&x).pow(exp));
    }
}

fn demo_gaussian_rational_pow_assign_i64(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp) in gaussian_rational_signed_pair_gen_var_1::<i64>()
        .get(gm, config)
        .take(limit)
    {
        if x == 0u32 && exp < 0 {
            continue;
        }
        let x_old = x.clone();
        x.pow_assign(exp);
        println!("x := {x_old}; x.pow_assign({exp}); x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_pow_u64_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_unsigned_pair_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [
            ("GaussianRational.pow(u64)", &mut |(x, exp)| {
                no_out!(x.pow(exp));
            }),
            ("(&GaussianRational).pow(u64)", &mut |(x, exp)| {
                no_out!((&x).pow(exp));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_pow_u64_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.pow(u64)",
        BenchmarkType::Algorithms,
        gaussian_rational_unsigned_pair_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, exp)| {
                no_out!((&x).pow(exp));
            }),
            ("binary", &mut |(x, exp)| {
                no_out!(gaussian_rational_pow_binary(&x, exp));
            }),
            ("naive", &mut |(x, exp)| {
                no_out!(gaussian_rational_pow_naive(&x, i64::try_from(exp).unwrap()));
            }),
        ],
    );
}

fn benchmark_gaussian_rational_pow_u64_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.pow_assign(u64)",
        BenchmarkType::Single,
        gaussian_rational_unsigned_pair_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, exp)| x.pow_assign(exp))],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_pow_i64_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.pow(i64)",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_signed_pair_gen_var_1::<i64>()
            .get(gm, config)
            .filter(|(x, exp)| *x != 0u32 || *exp >= 0),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [
            ("GaussianRational.pow(i64)", &mut |(x, exp)| {
                no_out!(x.pow(exp));
            }),
            ("(&GaussianRational).pow(i64)", &mut |(x, exp)| {
                no_out!((&x).pow(exp));
            }),
        ],
    );
}

fn benchmark_gaussian_rational_pow_i64_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.pow_assign(i64)",
        BenchmarkType::Single,
        gaussian_rational_signed_pair_gen_var_1::<i64>()
            .get(gm, config)
            .filter(|(x, exp)| *x != 0u32 || *exp >= 0),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, exp)| x.pow_assign(exp))],
    );
}
