// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedSqrt;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::gaussian_integer::arithmetic::sqrt::*;
use malachite_nz::test_util::generators::{gaussian_integer_gen, gaussian_integer_gen_var_4};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_checked_sqrt);
    register_demo!(runner, demo_gaussian_integer_checked_sqrt_ref);
    register_demo!(runner, demo_gaussian_integer_checked_sqrts);

    register_bench!(runner, benchmark_gaussian_integer_checked_sqrt_algorithms);
    register_bench!(runner, benchmark_gaussian_integer_checked_sqrts);
    register_bench!(
        runner,
        benchmark_gaussian_integer_checked_sqrt_evaluation_strategy
    );
}

// `Option` and `Vec` print their contents in debug form, which for a `GaussianInteger` is the
// struct; these spell out the display form instead.
fn root_string(root: Option<GaussianInteger>) -> String {
    root.map_or_else(|| "None".to_string(), |r| format!("Some({r})"))
}

fn roots_string(roots: &[GaussianInteger]) -> String {
    format!(
        "[{}]",
        roots
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn demo_gaussian_integer_checked_sqrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!(
            "({}).checked_sqrt() = {}",
            x,
            root_string(x.clone().checked_sqrt())
        );
    }
}

fn demo_gaussian_integer_checked_sqrt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).checked_sqrt() = {}",
            x,
            root_string((&x).checked_sqrt())
        );
    }
}

fn demo_gaussian_integer_checked_sqrts(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!(
            "({}).checked_sqrts() = {}",
            x,
            roots_string(&x.checked_sqrts())
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_checked_sqrts(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.checked_sqrts()",
        BenchmarkType::Single,
        gaussian_integer_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(x.checked_sqrts());
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_checked_sqrt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.checked_sqrt()",
        BenchmarkType::Algorithms,
        gaussian_integer_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| {
                no_out!((&x).checked_sqrt());
            }),
            ("naive", &mut |x| {
                no_out!(gaussian_integer_checked_sqrt_naive(&x));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_checked_sqrt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.checked_sqrt()",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("GaussianInteger.checked_sqrt()", &mut |x| {
                no_out!(x.checked_sqrt());
            }),
            ("(&GaussianInteger).checked_sqrt()", &mut |x| {
                no_out!((&x).checked_sqrt());
            }),
        ],
    );
}
