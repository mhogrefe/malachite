// Copyright © 2024 Mikhail Hogrefe
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
use malachite_float::test_util::generators::{float_pair_gen, float_pair_gen_rm};
use malachite_float::{ComparableFloat, ComparableFloatRef};
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_partial_cmp);
    register_demo!(runner, demo_float_partial_cmp_debug);
    register_demo!(runner, demo_comparable_float_partial_cmp);
    register_demo!(runner, demo_comparable_float_partial_cmp_debug);
    register_demo!(runner, demo_comparable_float_ref_partial_cmp);
    register_demo!(runner, demo_comparable_float_ref_partial_cmp_debug);

    register_bench!(runner, benchmark_float_partial_cmp_library_comparison);
    register_bench!(runner, benchmark_comparable_float_cmp);
    register_bench!(runner, benchmark_comparable_float_ref_cmp);
}

fn demo_float_partial_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_float_partial_cmp_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        match x.partial_cmp(&y) {
            None => println!("{cx:#x} and {cy:#x} are incomparable"),
            Some(Less) => println!("{cx:#x} < {cy:#x}"),
            Some(Equal) => println!("{cx:#x} = {cy:#x}"),
            Some(Greater) => println!("{cx:#x} > {cy:#x}"),
        }
    }
}

fn demo_comparable_float_partial_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloat(x.clone());
        let cy = ComparableFloat(y.clone());
        match cx.partial_cmp(&cy) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_comparable_float_partial_cmp_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloat(x);
        let cy = ComparableFloat(y);
        match cx.partial_cmp(&cy) {
            None => println!("{cx} and {cy} are incomparable"),
            Some(Less) => println!("{cx:#x} < {cy:#x}"),
            Some(Equal) => println!("{cx:#x} = {cy:#x}"),
            Some(Greater) => println!("{cx:#x} > {cy:#x}"),
        }
    }
}

fn demo_comparable_float_ref_partial_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        match cx.partial_cmp(&cy) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_comparable_float_ref_partial_cmp_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        match cx.partial_cmp(&cy) {
            None => println!("{cx} and {cy} are incomparable"),
            Some(Less) => println!("{cx:#x} < {cy:#x}"),
            Some(Equal) => println!("{cx:#x} = {cy:#x}"),
            Some(Greater) => println!("{cx:#x} > {cy:#x}"),
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_float_partial_cmp_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.cmp(&Integer)",
        BenchmarkType::LibraryComparison,
        float_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.partial_cmp(&y))),
            ("rug", &mut |((x, y), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_comparable_float_cmp(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "ComparableFloat.cmp(&ComparableFloat)",
        BenchmarkType::Single,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(ComparableFloat(x).cmp(&ComparableFloat(y)))
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_comparable_float_ref_cmp(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "ComparableFloatRef.cmp(&ComparableFloatRef)",
        BenchmarkType::Single,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(ComparableFloatRef(&x).cmp(&ComparableFloatRef(&y)))
        })],
    );
}
