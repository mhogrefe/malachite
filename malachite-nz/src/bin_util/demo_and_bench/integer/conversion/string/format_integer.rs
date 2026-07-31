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
use malachite_nz::integer::conversion::string::format_integer::format_integer_str;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_string_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_format_integer_str);
    register_bench!(runner, benchmark_format_integer_str);
}

fn demo_format_integer_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, fmt) in integer_string_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "format_integer_str({x}, {fmt:?}) = {:?}",
            format_integer_str(&x, &fmt)
        );
    }
}

fn benchmark_format_integer_str(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "format_integer_str(&Integer, &str)",
        BenchmarkType::Single,
        integer_string_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, fmt)| {
            no_out!(format_integer_str(&x, &fmt));
        })],
    );
}
