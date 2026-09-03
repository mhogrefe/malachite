// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{MulIPow, MulIPowAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_1_gaussian_rational_bit_bucketer;
use malachite_q::test_util::generators::gaussian_rational_unsigned_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_mul_i_pow_assign);
    register_demo!(runner, demo_gaussian_rational_mul_i_pow);
    register_demo!(runner, demo_gaussian_rational_mul_i_pow_ref);

    register_bench!(runner, benchmark_gaussian_rational_mul_i_pow_assign);
    register_bench!(
        runner,
        benchmark_gaussian_rational_mul_i_pow_evaluation_strategy
    );
}

fn demo_gaussian_rational_mul_i_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, k) in gaussian_rational_unsigned_pair_gen::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mul_i_pow_assign(k);
        println!("x := {x_old}; x.mul_i_pow_assign({k}); x = {x}");
    }
}

fn demo_gaussian_rational_mul_i_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k) in gaussian_rational_unsigned_pair_gen::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({x_old}).mul_i_pow({k}) = {}", x.mul_i_pow(k));
    }
}

fn demo_gaussian_rational_mul_i_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k) in gaussian_rational_unsigned_pair_gen::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{x}).mul_i_pow({k}) = {}", (&x).mul_i_pow(k));
    }
}

fn benchmark_gaussian_rational_mul_i_pow_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.mul_i_pow_assign(u64)",
        BenchmarkType::Single,
        gaussian_rational_unsigned_pair_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, k)| x.mul_i_pow_assign(k))],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_mul_i_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.mul_i_pow(u64)",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_unsigned_pair_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [
            ("GaussianRational.mul_i_pow(u64)", &mut |(x, k)| {
                no_out!(x.mul_i_pow(k));
            }),
            ("(&GaussianRational).mul_i_pow(u64)", &mut |(x, k)| {
                no_out!((&x).mul_i_pow(k));
            }),
        ],
    );
}
