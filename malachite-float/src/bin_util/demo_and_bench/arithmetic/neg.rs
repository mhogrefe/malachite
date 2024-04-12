// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{float_gen, float_gen_rm};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_neg);
    register_demo!(runner, demo_float_neg_debug);
    register_demo!(runner, demo_float_neg_ref);
    register_demo!(runner, demo_float_neg_ref_debug);
    register_demo!(runner, demo_float_neg_assign);
    register_demo!(runner, demo_float_neg_assign_debug);

    register_bench!(runner, benchmark_float_neg_library_comparison);
    register_bench!(runner, benchmark_float_neg_evaluation_strategy);
    register_bench!(runner, benchmark_float_neg_assign);
}

fn demo_float_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!("-({}) = {}", n.clone(), -n);
    }
}

fn demo_float_neg_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!(
            "-({:#x}) = {:#x}",
            ComparableFloat(n.clone()),
            ComparableFloat(-n)
        );
    }
}

fn demo_float_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!("-(&{}) = {}", n, -&n);
    }
}

fn demo_float_neg_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!(
            "-(&{:#x}) = {:#x}",
            ComparableFloatRef(&n),
            ComparableFloat(-&n)
        );
    }
}

fn demo_float_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in float_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.neg_assign();
        println!("n := {n_old}; n.neg_assign(); n = {n}");
    }
}

fn demo_float_neg_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in float_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.neg_assign();
        println!(
            "n := {:#x}; n.neg_assign(); n = {:#x}",
            ComparableFloat(n_old),
            ComparableFloat(n)
        );
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_float_neg_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-Float",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(_, n)| no_out!(-n)), ("rug", &mut |(n, _)| no_out!(-n))],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-Float",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Float.neg()", &mut |n| no_out!(-n)), ("(&Float).neg()", &mut |n| no_out!(-&n))],
    );
}

fn benchmark_float_neg_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.neg_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut n| n.neg_assign())],
    );
}
