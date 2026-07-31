// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{BalancedMod, BalancedModAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_integer_bit_bucketer, pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_pair_gen_var_1, natural_pair_gen_var_5};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_balanced_mod);
    register_demo!(runner, demo_integer_balanced_mod);
    register_demo!(runner, demo_integer_balanced_mod_assign);

    register_bench!(runner, benchmark_natural_balanced_mod_evaluation_strategy);
    register_bench!(runner, benchmark_integer_balanced_mod_evaluation_strategy);
}

fn demo_natural_balanced_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).balanced_mod({}) = {}",
            x_old,
            y_old,
            x.balanced_mod(y)
        );
    }
}

fn demo_integer_balanced_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).balanced_mod({}) = {}",
            x_old,
            y_old,
            x.balanced_mod(y)
        );
    }
}

fn demo_integer_balanced_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.balanced_mod_assign(&y);
        println!("x := {x_old}; x.balanced_mod_assign(&{y}); x = {x}");
    }
}

fn benchmark_natural_balanced_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.balanced_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.balanced_mod(Natural)", &mut |(x, y)| {
                no_out!(x.balanced_mod(y));
            }),
            ("(&Natural).balanced_mod(&Natural)", &mut |(x, y)| {
                no_out!((&x).balanced_mod(&y));
            }),
        ],
    );
}

fn benchmark_integer_balanced_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.balanced_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.balanced_mod(Integer)", &mut |(x, y)| {
                no_out!(x.balanced_mod(y));
            }),
            ("(&Integer).balanced_mod(&Integer)", &mut |(x, y)| {
                no_out!((&x).balanced_mod(&y));
            }),
        ],
    );
}
