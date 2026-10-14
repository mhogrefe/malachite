// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
use malachite_base::test_util::bench::bucketers::triple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::natural_natural_unsigned_triple_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_power_of_2_div_val_val);
    register_demo!(runner, demo_natural_mod_power_of_2_div_val_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_div_ref_val);
    register_demo!(runner, demo_natural_mod_power_of_2_div_ref_ref);
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_div_evaluation_strategy
    );
}

fn demo_natural_mod_power_of_2_div_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{x}.mod_power_of_2_div({y}, {pow}) = {:?}",
            x.clone().mod_power_of_2_div(y.clone(), pow)
        );
    }
}

fn demo_natural_mod_power_of_2_div_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{x}.mod_power_of_2_div(&{y}, {pow}) = {:?}",
            x.clone().mod_power_of_2_div(&y, pow)
        );
    }
}

fn demo_natural_mod_power_of_2_div_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).mod_power_of_2_div({y}, {pow}) = {:?}",
            (&x).mod_power_of_2_div(y.clone(), pow)
        );
    }
}

fn demo_natural_mod_power_of_2_div_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).mod_power_of_2_div(&{y}, {pow}) = {:?}",
            (&x).mod_power_of_2_div(&y, pow)
        );
    }
}

fn benchmark_natural_mod_power_of_2_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_div(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            (
                "Natural.mod_power_of_2_div(Natural, u64)",
                &mut |(x, y, pow)| {
                    no_out!(x.mod_power_of_2_div(y, pow));
                },
            ),
            (
                "Natural.mod_power_of_2_div(&Natural, u64)",
                &mut |(x, y, pow)| {
                    no_out!(x.mod_power_of_2_div(&y, pow));
                },
            ),
            (
                "(&Natural).mod_power_of_2_div(Natural, u64)",
                &mut |(x, y, pow)| {
                    no_out!((&x).mod_power_of_2_div(y, pow));
                },
            ),
            (
                "(&Natural).mod_power_of_2_div(&Natural, u64)",
                &mut |(x, y, pow)| {
                    no_out!((&x).mod_power_of_2_div(&y, pow));
                },
            ),
        ],
    );
}
