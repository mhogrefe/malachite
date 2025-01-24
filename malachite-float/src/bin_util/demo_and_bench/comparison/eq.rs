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
    pair_2_pair_float_max_complexity_bucketer, pair_float_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_pair_gen, float_pair_gen_rm, float_pair_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_eq);
    register_demo!(runner, demo_float_eq_debug);
    register_demo!(runner, demo_float_eq_extreme);
    register_demo!(runner, demo_float_eq_extreme_debug);
    register_demo!(runner, demo_comparable_float_eq);
    register_demo!(runner, demo_comparable_float_eq_debug);
    register_demo!(runner, demo_comparable_float_eq_extreme);
    register_demo!(runner, demo_comparable_float_eq_extreme_debug);
    register_demo!(runner, demo_comparable_float_ref_eq);
    register_demo!(runner, demo_comparable_float_ref_eq_debug);

    register_bench!(runner, benchmark_float_eq_library_comparison);
    register_bench!(runner, benchmark_comparable_float_eq);
    register_bench!(runner, benchmark_comparable_float_ref_eq);
}

fn demo_float_eq(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        if x == y {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_float_eq_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        if x == y {
            println!("{cx:#x} = {cy:#x}");
        } else {
            println!("{cx:#x} ≠ {cy:#x}");
        }
    }
}

fn demo_float_eq_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        if x == y {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_float_eq_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        if x == y {
            println!("{cx:#x} = {cy:#x}");
        } else {
            println!("{cx:#x} ≠ {cy:#x}");
        }
    }
}

fn demo_comparable_float_eq(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloat(x.clone());
        let cy = ComparableFloat(y.clone());
        if cx == cy {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_comparable_float_eq_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloat(x.clone());
        let cy = ComparableFloat(y.clone());
        if cx == cy {
            println!("{cx:#x} = {cy:#x}");
        } else {
            println!("{cx:#x} ≠ {cy:#x}");
        }
    }
}

fn demo_comparable_float_eq_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let cx = ComparableFloat(x.clone());
        let cy = ComparableFloat(y.clone());
        if cx == cy {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_comparable_float_eq_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let cx = ComparableFloat(x.clone());
        let cy = ComparableFloat(y.clone());
        if cx == cy {
            println!("{cx:#x} = {cy:#x}");
        } else {
            println!("{cx:#x} ≠ {cy:#x}");
        }
    }
}

fn demo_comparable_float_ref_eq(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        if cx == cy {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_comparable_float_ref_eq_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        if cx == cy {
            println!("{cx:#x} = {cy:#x}");
        } else {
            println!("{cx:#x} ≠ {cy:#x}");
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_eq_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float == Float",
        BenchmarkType::LibraryComparison,
        float_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x == y)),
            ("rug", &mut |((x, y), _)| no_out!(x == y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_comparable_float_eq(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "ComparableFloat == ComparableFloat",
        BenchmarkType::Single,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(ComparableFloat(x) == ComparableFloat(y))
        })],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_comparable_float_ref_eq(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "ComparableFloatRef == ComparableFloatRef",
        BenchmarkType::Single,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(ComparableFloatRef(&x) == ComparableFloatRef(&y))
        })],
    );
}
