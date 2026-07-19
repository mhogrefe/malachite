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
use malachite_float::conversion::string::format_float::format_float_str;
use malachite_float::test_util::bench::bucketers::pair_1_float_complexity_bucketer;
use malachite_float::test_util::generators::float_string_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_format_float_str);
    register_demo!(runner, demo_format_float_str_debug);
    register_bench!(runner, benchmark_format_float_str);
}

fn demo_format_float_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, s) in float_string_pair_gen_var_1().get(gm, config).take(limit) {
        match format_float_str(&x, &s) {
            Some(t) => println!("format_float_str({x}, {s:?}) = {t:?}"),
            None => println!("format_float_str({x}, {s:?}) = None"),
        }
    }
}

fn demo_format_float_str_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, s) in float_string_pair_gen_var_1().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        match format_float_str(&x, &s) {
            Some(t) => println!("format_float_str({cx:#x}, {s:?}) = {t:?}"),
            None => println!("format_float_str({cx:#x}, {s:?}) = None"),
        }
    }
}

fn benchmark_format_float_str(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "format_float_str(&Float, &str)",
        BenchmarkType::Single,
        float_string_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, s)| {
            no_out!(format_float_str(&x, &s));
        })],
    );
}
