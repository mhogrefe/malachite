// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloatRef;
use malachite_float::test_util::bench::bucketers::pair_1_float_complexity_bucketer;
use malachite_float::test_util::generators::float_ordering_pair_gen;
use malachite_float::{test_overflow, test_underflow};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_test_overflow);
    register_demo!(runner, demo_float_test_overflow_debug);
    register_demo!(runner, demo_float_test_underflow);
    register_demo!(runner, demo_float_test_underflow_debug);

    register_bench!(runner, benchmark_float_test_overflow);
    register_bench!(runner, benchmark_float_test_underflow);
}

fn demo_float_test_overflow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, o) in float_ordering_pair_gen().get(gm, config).take(limit) {
        println!("test_overflow({}, {:?}) = {}", x, o, test_overflow(&x, o));
    }
}

fn demo_float_test_overflow_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, o) in float_ordering_pair_gen().get(gm, config).take(limit) {
        println!(
            "test_overflow({:#x}, {:?}) = {}",
            ComparableFloatRef(&x),
            o,
            test_overflow(&x, o)
        );
    }
}

fn demo_float_test_underflow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, o) in float_ordering_pair_gen().get(gm, config).take(limit) {
        println!("test_underflow({}, {:?}) = {}", x, o, test_underflow(&x, o));
    }
}

fn demo_float_test_underflow_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, o) in float_ordering_pair_gen().get(gm, config).take(limit) {
        println!(
            "test_underflow({:#x}, {:?}) = {}",
            ComparableFloatRef(&x),
            o,
            test_underflow(&x, o)
        );
    }
}

fn benchmark_float_test_overflow(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.ulp()",
        BenchmarkType::Single,
        float_ordering_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, o)| no_out!(test_overflow(&x, o)))],
    );
}

fn benchmark_float_test_underflow(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.ulp()",
        BenchmarkType::Single,
        float_ordering_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, o)| no_out!(test_overflow(&x, o)))],
    );
}
