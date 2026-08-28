// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::IsReal;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloat;
use malachite_float::test_util::bench::bucketers::float_complexity_bucketer;
use malachite_float::test_util::generators::float_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_is_real);
    register_demo!(runner, demo_float_is_real_debug);

    register_bench!(runner, benchmark_float_is_real);
}

fn demo_float_is_real(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        if n.is_real() {
            println!("{n} is a real number");
        } else {
            println!("{n} is not a real number");
        }
    }
}

fn demo_float_is_real_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        if n.is_real() {
            println!("{:#x} is a real number", ComparableFloat(n));
        } else {
            println!("{:#x} is not a real number", ComparableFloat(n));
        }
    }
}

fn benchmark_float_is_real(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.is_real()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_real()))],
    );
}
