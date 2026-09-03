// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::test_util::bench::bucketers::pair_1_bits_times_pair_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::gaussian_integer::arithmetic::pow::*;
use malachite_nz::test_util::generators::gaussian_integer_unsigned_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_pow_assign);
    register_demo!(runner, demo_gaussian_integer_pow);
    register_demo!(runner, demo_gaussian_integer_pow_ref);

    register_bench!(runner, benchmark_gaussian_integer_pow_assign);
    register_bench!(runner, benchmark_gaussian_integer_pow_algorithms);
    register_bench!(runner, benchmark_gaussian_integer_pow_evaluation_strategy);
}

fn demo_gaussian_integer_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp) in gaussian_integer_unsigned_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.pow_assign(exp);
        println!("x := {x_old}; x.pow_assign({exp}); x = {x}");
    }
}

fn demo_gaussian_integer_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in gaussian_integer_unsigned_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({x_old}).pow({exp}) = {}", x.pow(exp));
    }
}

fn demo_gaussian_integer_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in gaussian_integer_unsigned_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{x}).pow({exp}) = {}", (&x).pow(exp));
    }
}

fn benchmark_gaussian_integer_pow_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.pow_assign(u64)",
        BenchmarkType::Single,
        gaussian_integer_unsigned_pair_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bits_times_pair_2_bucketer("x", "exp"),
        &mut [("Malachite", &mut |(mut x, exp)| x.pow_assign(exp))],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.pow(u64)",
        BenchmarkType::Algorithms,
        gaussian_integer_unsigned_pair_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bits_times_pair_2_bucketer("x", "exp"),
        &mut [
            ("default", &mut |(x, exp)| {
                no_out!((&x).pow(exp));
            }),
            ("naive", &mut |(x, exp)| {
                no_out!(gaussian_integer_pow_naive(&x, exp));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_unsigned_pair_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bits_times_pair_2_bucketer("x", "exp"),
        &mut [
            ("GaussianInteger.pow(u64)", &mut |(x, exp)| {
                no_out!(x.pow(exp));
            }),
            ("(&GaussianInteger).pow(u64)", &mut |(x, exp)| {
                no_out!((&x).pow(exp));
            }),
        ],
    );
}
