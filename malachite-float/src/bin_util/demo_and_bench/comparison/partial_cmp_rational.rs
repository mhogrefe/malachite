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
    pair_2_pair_float_rational_max_complexity_bucketer, pair_float_rational_max_complexity_bucketer,
};
use malachite_float::test_util::comparison::partial_cmp_rational::float_partial_cmp_rational_alt;
use malachite_float::test_util::generators::{
    float_rational_pair_gen, float_rational_pair_gen_rm, float_rational_pair_gen_var_2,
};
use malachite_float::ComparableFloatRef;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_partial_cmp_rational);
    register_demo!(runner, demo_float_partial_cmp_rational_debug);
    register_demo!(runner, demo_float_partial_cmp_rational_extreme);
    register_demo!(runner, demo_float_partial_cmp_rational_extreme_debug);
    register_demo!(runner, demo_rational_partial_cmp_float);
    register_demo!(runner, demo_rational_partial_cmp_float_debug);
    register_demo!(runner, demo_rational_partial_cmp_float_extreme);
    register_demo!(runner, demo_rational_partial_cmp_float_extreme_debug);

    register_bench!(
        runner,
        benchmark_float_partial_cmp_rational_library_comparison
    );
    register_bench!(runner, benchmark_float_partial_cmp_rational_algorithms);
    register_bench!(
        runner,
        benchmark_rational_partial_cmp_float_library_comparison
    );
}

fn demo_float_partial_cmp_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_float_partial_cmp_rational_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        match x.partial_cmp(&y) {
            None => println!("{cx:#x} and {y} are incomparable"),
            Some(Less) => println!("{cx:#x} < {y}"),
            Some(Equal) => println!("{cx:#x} = {y}"),
            Some(Greater) => println!("{cx:#x} > {y}"),
        }
    }
}

fn demo_float_partial_cmp_rational_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_float_partial_cmp_rational_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        match x.partial_cmp(&y) {
            None => println!("{cx:#x} and {y} are incomparable"),
            Some(Less) => println!("{cx:#x} < {y}"),
            Some(Equal) => println!("{cx:#x} = {y}"),
            Some(Greater) => println!("{cx:#x} > {y}"),
        }
    }
}

fn demo_rational_partial_cmp_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_rational_partial_cmp_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        match x.partial_cmp(&y) {
            None => println!("{x} and {cy:#x} are incomparable"),
            Some(Less) => println!("{x} < {cy:#x}"),
            Some(Equal) => println!("{x} = {cy:#x}"),
            Some(Greater) => println!("{x} > {cy:#x}"),
        }
    }
}

fn demo_rational_partial_cmp_float_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_rational_partial_cmp_float_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        match x.partial_cmp(&y) {
            None => println!("{x} and {cy:#x} are incomparable"),
            Some(Less) => println!("{x} < {cy:#x}"),
            Some(Equal) => println!("{x} = {cy:#x}"),
            Some(Greater) => println!("{x} > {cy:#x}"),
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_float_partial_cmp_rational_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.partial_cmp(&Rational)",
        BenchmarkType::LibraryComparison,
        float_rational_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.partial_cmp(&y))),
            ("rug", &mut |((x, y), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_partial_cmp_rational_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.partial_cmp(&Rational)",
        BenchmarkType::Algorithms,
        float_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.partial_cmp(&y))),
            ("alt", &mut |(x, y)| {
                no_out!(float_partial_cmp_rational_alt(&x, &y))
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_partial_cmp_float_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.partial_cmp(&Float)",
        BenchmarkType::LibraryComparison,
        float_rational_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_rational_max_complexity_bucketer("y", "x"),
        &mut [
            ("Malachite", &mut |(_, (y, x))| no_out!(x == y)),
            ("rug", &mut |((y, x), _)| no_out!(x == y)),
        ],
    );
}
