// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{NextPowerOf2, NextPowerOf2Assign};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::rational_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_next_power_of_2);
    register_demo!(runner, demo_rational_next_power_of_2_ref);
    register_demo!(runner, demo_rational_next_power_of_2_assign);

    register_bench!(
        runner,
        benchmark_rational_next_power_of_2_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_next_power_of_2_assign);
}

fn demo_rational_next_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen_var_2().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("next_power_of_2({}) = {}", x_old, x.next_power_of_2());
    }
}

fn demo_rational_next_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen_var_2().get(gm, config).take(limit) {
        println!("next_power_of_2({}) = {}", x, (&x).next_power_of_2());
    }
}

fn demo_rational_next_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in rational_gen_var_2().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.next_power_of_2_assign();
        println!("x := {old_x}; x.next_power_of_2_assign(); x = {x}");
    }
}

fn benchmark_rational_next_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.next_power_of_2()",
        BenchmarkType::EvaluationStrategy,
        rational_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("Rational.next_power_of_2()", &mut |x| {
                no_out!(x.next_power_of_2())
            }),
            ("(&Rational).next_power_of_2()", &mut |x| {
                no_out!((&x).next_power_of_2())
            }),
        ],
    );
}

fn benchmark_rational_next_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.next_power_of_2_assign()",
        BenchmarkType::Single,
        rational_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.next_power_of_2_assign())],
    );
}
