// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{DeflatePowerOfX, DeflatePowerOfXAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_unsigned_pair_gen_var_2;
use malachite_nz::test_util::integer_polynomial::arithmetic::deflate_power_of_x::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_deflate_power_of_x);
    register_demo!(runner, demo_integer_polynomial_deflate_power_of_x_ref);
    register_demo!(runner, demo_integer_polynomial_deflate_power_of_x_assign);

    register_bench!(
        runner,
        benchmark_integer_polynomial_deflate_power_of_x_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_deflate_power_of_x_algorithms
    );
}

fn demo_integer_polynomial_deflate_power_of_x(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, n) in integer_polynomial_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).deflate_power_of_x({n}) = {}",
            p.deflate_power_of_x(n)
        );
    }
}

fn demo_integer_polynomial_deflate_power_of_x_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, n) in integer_polynomial_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).deflate_power_of_x({n}) = {}",
            (&p).deflate_power_of_x(n)
        );
    }
}

fn demo_integer_polynomial_deflate_power_of_x_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, n) in integer_polynomial_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.deflate_power_of_x_assign(n);
        println!("p := {p_old}; p.deflate_power_of_x_assign({n}); p = {p}");
    }
}

fn benchmark_integer_polynomial_deflate_power_of_x_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.deflate_power_of_x(n)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("p.deflate_power_of_x(n)", &mut |(p, n)| {
                no_out!(p.deflate_power_of_x(n));
            }),
            ("(&p).deflate_power_of_x(n)", &mut |(p, n)| {
                no_out!((&p).deflate_power_of_x(n));
            }),
            ("p.deflate_power_of_x_assign(n)", &mut |(mut p, n)| {
                p.deflate_power_of_x_assign(n);
            }),
        ],
    );
}

fn benchmark_integer_polynomial_deflate_power_of_x_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.deflate_power_of_x(n)",
        BenchmarkType::Algorithms,
        integer_polynomial_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, n)| no_out!((&p).deflate_power_of_x(n))),
            ("naive", &mut |(p, n)| {
                no_out!(deflate_power_of_x_naive(&p, n));
            }),
        ],
    );
}
