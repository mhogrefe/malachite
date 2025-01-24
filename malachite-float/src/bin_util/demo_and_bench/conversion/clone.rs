// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    pair_2_float_complexity_bucketer, pair_2_pair_float_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_pair_gen, float_pair_gen_rm,
    float_pair_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_clone);
    register_demo!(runner, demo_float_clone_debug);
    register_demo!(runner, demo_float_clone_extreme);
    register_demo!(runner, demo_float_clone_extreme_debug);
    register_demo!(runner, demo_float_clone_from);
    register_demo!(runner, demo_float_clone_from_debug);
    register_demo!(runner, demo_float_clone_from_extreme);
    register_demo!(runner, demo_float_clone_from_extreme_debug);

    register_bench!(runner, benchmark_float_clone_library_comparison);
    register_bench!(runner, benchmark_float_clone_from_library_comparison);
}

fn demo_float_clone(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!("clone({}) = {}", n, n.clone());
    }
}

fn demo_float_clone_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!(
            "clone({:#x}) = {:#x}",
            ComparableFloatRef(&n),
            ComparableFloat(n.clone())
        );
    }
}

fn demo_float_clone_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_12().get(gm, config).take(limit) {
        println!("clone({}) = {}", n, n.clone());
    }
}

fn demo_float_clone_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "clone({:#x}) = {:#x}",
            ComparableFloatRef(&n),
            ComparableFloat(n.clone())
        );
    }
}

fn demo_float_clone_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {x_old}; x.clone_from({y}); x = {x}");
    }
}

fn demo_float_clone_from_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!(
            "x := {:#x}; x.clone_from({:#x}); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            ComparableFloat(x)
        );
    }
}

fn demo_float_clone_from_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {x_old}; x.clone_from({y}); x = {x}");
    }
}

fn demo_float_clone_from_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!(
            "x := {:#x}; x.clone_from({:#x}); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            ComparableFloat(x)
        );
    }
}

#[allow(clippy::redundant_clone, unused_must_use)]
fn benchmark_float_clone_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.clone()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(n.clone())),
            ("rug", &mut |(n, _)| no_out!(n.clone())),
        ],
    );
}

fn benchmark_float_clone_from_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.clone_from(&Float)",
        BenchmarkType::LibraryComparison,
        float_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (mut x, y))| x.clone_from(&y)),
            ("rug", &mut |((mut x, y), _)| x.clone_from(&y)),
        ],
    );
}
