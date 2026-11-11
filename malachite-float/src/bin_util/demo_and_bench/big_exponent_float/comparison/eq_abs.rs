// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::{
    big_exponent_float_pair_gen, big_exponent_float_pair_gen_var_1_float,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_eq_abs);

    register_bench!(runner, benchmark_big_exponent_float_eq_abs_float_comparison);
}

fn demo_big_exponent_float_eq_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x:#x}| = |{y:#x}|");
        } else {
            println!("|{x:#x}| ≠ |{y:#x}|");
        }
    }
}

fn benchmark_big_exponent_float_eq_abs_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.eq_abs(&BigExponentFloat)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_pair_gen_var_1_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_big_exponent_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y))| {
                no_out!(x.eq_abs(&y));
            }),
            ("Float", &mut |((x, y), _)| {
                no_out!(x.eq_abs(&y));
            }),
        ],
    );
}
