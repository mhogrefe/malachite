// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::{RemovePower, RemovePowerAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_integer_bit_bucketer, pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_pair_gen_var_9, natural_pair_gen_var_16};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_remove_power);
    register_demo!(runner, demo_natural_remove_power_assign);
    register_demo!(runner, demo_integer_remove_power);
    register_demo!(runner, demo_integer_remove_power_assign);

    register_bench!(runner, benchmark_natural_remove_power_evaluation_strategy);
    register_bench!(runner, benchmark_integer_remove_power_library_comparison);
}

fn demo_natural_remove_power(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_16().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).remove_power({}) = {:?}",
            x_old,
            y_old,
            x.remove_power(y)
        );
    }
}

fn demo_natural_remove_power_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_16().get(gm, config).take(limit) {
        let x_old = x.clone();
        let k = x.remove_power_assign(&y);
        println!("x := {x_old}; x.remove_power_assign(&{y}) = {k}; x = {x}");
    }
}

fn demo_integer_remove_power(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_9().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).remove_power({}) = {:?}",
            x_old,
            y_old,
            x.remove_power(y)
        );
    }
}

fn demo_integer_remove_power_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_9().get(gm, config).take(limit) {
        let x_old = x.clone();
        let k = x.remove_power_assign(&y);
        println!("x := {x_old}; x.remove_power_assign(&{y}) = {k}; x = {x}");
    }
}

fn benchmark_natural_remove_power_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.remove_power(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.remove_power(Natural)", &mut |(x, y)| {
                no_out!(x.remove_power(y));
            }),
            ("(&Natural).remove_power(&Natural)", &mut |(x, y)| {
                no_out!((&x).remove_power(&y));
            }),
        ],
    );
}

fn benchmark_integer_remove_power_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.remove_power(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(x, y)| no_out!(x.remove_power(y))),
            ("rug", &mut |(x, y)| {
                no_out!(rug::Integer::from(&x).remove_factor(&rug::Integer::from(&y)));
            }),
        ],
    );
}
