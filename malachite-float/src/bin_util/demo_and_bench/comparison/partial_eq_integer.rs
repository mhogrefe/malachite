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
use malachite_float::test_util::bench::bucketers::pair_2_pair_float_integer_max_complexity_bucketer;
use malachite_float::test_util::generators::{
    float_integer_pair_gen, float_integer_pair_gen_rm, float_integer_pair_gen_var_2,
};
use malachite_float::ComparableFloatRef;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_partial_eq_integer);
    register_demo!(runner, demo_float_partial_eq_integer_debug);
    register_demo!(runner, demo_float_partial_eq_integer_extreme);
    register_demo!(runner, demo_float_partial_eq_integer_extreme_debug);
    register_demo!(runner, demo_integer_partial_eq_float);
    register_demo!(runner, demo_integer_partial_eq_float_debug);
    register_demo!(runner, demo_integer_partial_eq_float_extreme);
    register_demo!(runner, demo_integer_partial_eq_float_extreme_debug);

    register_bench!(
        runner,
        benchmark_float_partial_eq_integer_library_comparison
    );
    register_bench!(
        runner,
        benchmark_integer_partial_eq_float_library_comparison
    );
}

fn demo_float_partial_eq_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_integer_pair_gen().get(gm, config).take(limit) {
        if x == y {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_float_partial_eq_integer_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_integer_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        if x == y {
            println!("{cx:#x} = {y:#x}");
        } else {
            println!("{cx:#x} ≠ {y:#x}");
        }
    }
}

fn demo_float_partial_eq_integer_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_integer_pair_gen_var_2().get(gm, config).take(limit) {
        if x == y {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_float_partial_eq_integer_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_integer_pair_gen_var_2().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        if x == y {
            println!("{cx:#x} = {y:#x}");
        } else {
            println!("{cx:#x} ≠ {y:#x}");
        }
    }
}

fn demo_integer_partial_eq_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_integer_pair_gen().get(gm, config).take(limit) {
        if x == y {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_integer_partial_eq_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_integer_pair_gen().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        if x == y {
            println!("{x:#x} = {cy:#x}");
        } else {
            println!("{x:#x} ≠ {cy:#x}");
        }
    }
}

fn demo_integer_partial_eq_float_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_integer_pair_gen_var_2().get(gm, config).take(limit) {
        if x == y {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_integer_partial_eq_float_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_integer_pair_gen_var_2().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        if x == y {
            println!("{x:#x} = {cy:#x}");
        } else {
            println!("{x:#x} ≠ {cy:#x}");
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_partial_eq_integer_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float == Integer",
        BenchmarkType::LibraryComparison,
        float_integer_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_integer_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x == y)),
            ("rug", &mut |((x, y), _)| no_out!(x == y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_partial_eq_float_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer == Float",
        BenchmarkType::LibraryComparison,
        float_integer_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_integer_max_complexity_bucketer("y", "x"),
        &mut [
            ("Malachite", &mut |(_, (y, x))| no_out!(x == y)),
            ("rug", &mut |((y, x), _)| no_out!(x == y)),
        ],
    );
}
