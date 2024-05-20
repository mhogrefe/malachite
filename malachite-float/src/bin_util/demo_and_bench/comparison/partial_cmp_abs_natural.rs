// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::pair_float_natural_max_complexity_bucketer;
use malachite_float::test_util::generators::float_natural_pair_gen;
use malachite_float::ComparableFloatRef;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_partial_cmp_abs_natural);
    register_demo!(runner, demo_float_partial_cmp_abs_natural_debug);
    register_demo!(runner, demo_natural_partial_cmp_abs_float);
    register_demo!(runner, demo_natural_partial_cmp_abs_float_debug);

    register_bench!(runner, benchmark_float_partial_cmp_abs_natural);
    register_bench!(runner, benchmark_natural_partial_cmp_abs_float);
}

fn demo_float_partial_cmp_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_natural_pair_gen().get(gm, config).take(limit) {
        match x.partial_cmp_abs(&y) {
            None => println!("|{x}| and |{y}| are incomparable"),
            Some(Less) => println!("|{x}| < |{y}|"),
            Some(Equal) => println!("|{x}| = |{y}|"),
            Some(Greater) => println!("|{x}| > |{y}|"),
        }
    }
}

fn demo_float_partial_cmp_abs_natural_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_natural_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        match x.partial_cmp_abs(&y) {
            None => println!("|{cx:#x}| and {y:#x} are incomparable"),
            Some(Less) => println!("|{cx:#x}| < |{y:#x}|"),
            Some(Equal) => println!("|{cx:#x}| = |{y:#x}|"),
            Some(Greater) => println!("|{cx:#x}| > |{y:#x}|"),
        }
    }
}

fn demo_natural_partial_cmp_abs_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_natural_pair_gen().get(gm, config).take(limit) {
        match x.partial_cmp_abs(&y) {
            None => println!("|{x}| and |{y}| are incomparable"),
            Some(Less) => println!("|{x}| < |{y}|"),
            Some(Equal) => println!("|{x}| = |{y}|"),
            Some(Greater) => println!("|{x}| > |{y}|"),
        }
    }
}

fn demo_natural_partial_cmp_abs_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_natural_pair_gen().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        match x.partial_cmp_abs(&y) {
            None => println!("|{x:#x}| and |{cy:#x}| are incomparable"),
            Some(Less) => println!("|{x:#x}| < |{cy:#x}|"),
            Some(Equal) => println!("|{x:#x}| = |{cy:#x}|"),
            Some(Greater) => println!("|{x:#x}| > |{cy:#x}|"),
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_float_partial_cmp_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.partial_cmp_abs(&Natural)",
        BenchmarkType::Single,
        float_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_natural_max_complexity_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_partial_cmp_abs_float(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.partial_cmp_abs(&Float)",
        BenchmarkType::Single,
        float_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_natural_max_complexity_bucketer("y", "x"),
        &mut [("Malachite", &mut |(y, x)| no_out!(x == y))],
    );
}
