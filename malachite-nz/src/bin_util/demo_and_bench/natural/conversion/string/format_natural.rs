// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::gmp_format;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::conversion::string::format_natural::format_natural_str;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_string_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_format_natural_str);
    register_demo!(runner, demo_gmp_format_natural);
    register_bench!(runner, benchmark_format_natural_str);
    register_bench!(runner, benchmark_gmp_format_natural_algorithms);
}

fn demo_gmp_format_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, fmt) in natural_string_pair_gen_var_1().get(gm, config).take(limit) {
        println!("gmp_format!({fmt:?}, {x}) = {:?}", gmp_format!(&*fmt, x));
    }
}

// Compares the multi-argument walker with the single-value entry point on the same one-conversion
// templates, measuring the walker's dispatch overhead.
fn benchmark_gmp_format_natural_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "gmp_format!(&str, Natural)",
        BenchmarkType::Algorithms,
        natural_string_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("gmp_format!", &mut |(x, fmt)| {
                no_out!(gmp_format!(&*fmt, x));
            }),
            ("format_natural_str", &mut |(x, fmt)| {
                no_out!(format_natural_str(&x, &fmt));
            }),
        ],
    );
}

fn demo_format_natural_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, fmt) in natural_string_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "format_natural_str({x}, {fmt:?}) = {:?}",
            format_natural_str(&x, &fmt)
        );
    }
}

fn benchmark_format_natural_str(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "format_natural_str(&Natural, &str)",
        BenchmarkType::Single,
        natural_string_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, fmt)| {
            no_out!(format_natural_str(&x, &fmt));
        })],
    );
}
