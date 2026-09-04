// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedRoot;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::bench::bucketers::pair_1_gaussian_rational_bit_bucketer;
use malachite_q::test_util::generators::gaussian_rational_unsigned_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_checked_root);
    register_demo!(runner, demo_gaussian_rational_checked_root_ref);
    register_demo!(runner, demo_gaussian_rational_checked_roots);

    register_bench!(
        runner,
        benchmark_gaussian_rational_checked_root_evaluation_strategy
    );
    register_bench!(runner, benchmark_gaussian_rational_checked_roots);
}

// `Option` and `Vec` print their contents in debug form, which for a `GaussianRational` is the
// struct; these spell out the display form instead.
fn root_string(root: Option<GaussianRational>) -> String {
    root.map_or_else(|| "None".to_string(), |r| format!("Some({r})"))
}

fn roots_string(roots: &[GaussianRational]) -> String {
    format!(
        "[{}]",
        roots
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn demo_gaussian_rational_checked_root(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in gaussian_rational_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).checked_root({}) = {}",
            x,
            exp,
            root_string(x.clone().checked_root(exp))
        );
    }
}

fn demo_gaussian_rational_checked_root_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in gaussian_rational_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).checked_root({}) = {}",
            x,
            exp,
            root_string((&x).checked_root(exp))
        );
    }
}

fn demo_gaussian_rational_checked_roots(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in gaussian_rational_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).checked_roots({}) = {}",
            x,
            exp,
            roots_string(&x.checked_roots(exp))
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_checked_root_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.checked_root(u64)",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_unsigned_pair_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [
            ("GaussianRational.checked_root(u64)", &mut |(x, exp)| {
                no_out!(x.checked_root(exp));
            }),
            ("(&GaussianRational).checked_root(u64)", &mut |(x, exp)| {
                no_out!((&x).checked_root(exp));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_checked_roots(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.checked_roots(u64)",
        BenchmarkType::Single,
        gaussian_rational_unsigned_pair_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, exp)| {
            no_out!(x.checked_roots(exp));
        })],
    );
}
