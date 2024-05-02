// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{SaturatingSub, SaturatingSubAssign};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_saturating_sub);
    register_demo!(runner, demo_natural_saturating_sub_val_ref);
    register_demo!(runner, demo_natural_saturating_sub_ref_val);
    register_demo!(runner, demo_natural_saturating_sub_ref_ref);
    register_demo!(runner, demo_natural_saturating_sub_assign);
    register_demo!(runner, demo_natural_saturating_sub_assign_ref);

    register_bench!(
        runner,
        benchmark_natural_saturating_sub_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_saturating_sub_evaluation_strategy);
}

fn demo_natural_saturating_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.saturating_sub({}) = {:?}",
            x_old,
            y_old,
            x.saturating_sub(y)
        );
    }
}

fn demo_natural_saturating_sub_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.saturating_sub(&{}) = {:?}",
            x_old,
            y,
            x.saturating_sub(&y)
        );
    }
}

fn demo_natural_saturating_sub_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).saturating_sub({}) = {:?}",
            x,
            y_old,
            (&x).saturating_sub(y)
        );
    }
}

fn demo_natural_saturating_sub_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).saturating_sub(&{}) = {:?}",
            x,
            y,
            (&x).saturating_sub(&y)
        );
    }
}

fn demo_natural_saturating_sub_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.saturating_sub_assign(y);
        println!("x := {x_old}; x.saturating_sub_assign({y_old}); x = {x}");
    }
}

fn demo_natural_saturating_sub_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.saturating_sub_assign(&y);
        println!("x := {x_old}; x.saturating_sub_assign(&{y}); x = {x}");
    }
}

fn benchmark_natural_saturating_sub_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            (
                "Natural.saturating_sub_assign(Natural)",
                &mut |(mut x, y)| x.saturating_sub_assign(y),
            ),
            (
                "Natural.saturating_sub_assign(&Natural)",
                &mut |(mut x, y)| x.saturating_sub_assign(&y),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.saturating_sub(Natural)", &mut |(x, y)| {
                no_out!(x.saturating_sub(y))
            }),
            ("Natural.saturating_sub(&Natural)", &mut |(x, y)| {
                no_out!(x.saturating_sub(&y))
            }),
            ("&Natural.saturating_sub(Natural)", &mut |(x, y)| {
                no_out!((&x).saturating_sub(y))
            }),
            ("&Natural.saturating_sub(&Natural)", &mut |(x, y)| {
                no_out!((&x).saturating_sub(&y))
            }),
        ],
    );
}
