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
use malachite_q::rational::conversion::string::format_rational::format_rational_str;
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::rational_string_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_format_rational_str);
    register_demo!(runner, demo_gmp_format_rational);
    register_bench!(runner, benchmark_format_rational_str);
}

// The `%Q` templates from the generator sweep every flag subset, conversion, width, and precision
// through the multi-argument walker.
fn demo_gmp_format_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, fmt) in rational_string_pair_gen_var_1().get(gm, config).take(limit) {
        println!("gmp_format!({fmt:?}, {x}) = {:?}", gmp_format!(&*fmt, x));
    }
}

fn demo_format_rational_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, fmt) in rational_string_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "format_rational_str({x}, {fmt:?}) = {:?}",
            format_rational_str(&x, &fmt)
        );
    }
}

fn benchmark_format_rational_str(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "format_rational_str(&Rational, &str)",
        BenchmarkType::Single,
        rational_string_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, fmt)| {
            no_out!(format_rational_str(&x, &fmt));
        })],
    );
}
