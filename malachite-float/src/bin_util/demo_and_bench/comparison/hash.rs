// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::hash::hash;
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::float_complexity_bucketer;
use malachite_float::test_util::generators::float_gen;
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_comparable_float_hash);
    register_demo!(runner, demo_comparable_float_hash_debug);
    register_demo!(runner, demo_comparable_float_ref_hash);
    register_demo!(runner, demo_comparable_float_ref_hash_debug);

    register_bench!(runner, benchmark_comparable_float_hash);
    register_bench!(runner, benchmark_comparable_float_ref_hash);
}

fn demo_comparable_float_hash(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("hash({}) = {}", x.clone(), hash(&ComparableFloat(x)));
    }
}

fn demo_comparable_float_hash_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let cx = ComparableFloat(x);
        println!("hash({:#x}) = {}", cx, hash(&cx));
    }
}

fn demo_comparable_float_ref_hash(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("hash({}) = {}", x.clone(), hash(&ComparableFloatRef(&x)));
    }
}

fn demo_comparable_float_ref_hash_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        println!("hash({:#x}) = {}", cx, hash(&cx));
    }
}

fn benchmark_comparable_float_hash(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float hash",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(hash(&ComparableFloat(x))))],
    );
}

fn benchmark_comparable_float_ref_hash(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float hash",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(hash(&ComparableFloatRef(&x))))],
    );
}
