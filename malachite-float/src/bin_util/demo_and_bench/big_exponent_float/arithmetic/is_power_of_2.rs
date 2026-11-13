// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::pair_2_big_exponent_float_complexity_bucketer;
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1_float,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_is_power_of_2);

    register_bench!(
        runner,
        benchmark_big_exponent_float_is_power_of_2_float_comparison
    );
}

fn demo_big_exponent_float_is_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        if x.is_power_of_2() {
            println!("{x:#x} is a power of 2");
        } else {
            println!("{x:#x} is not a power of 2");
        }
    }
}

fn benchmark_big_exponent_float_is_power_of_2_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_power_of_2()",
        BenchmarkType::LibraryComparison,
        big_exponent_float_gen_var_1_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("BigExponentFloat", &mut |(_, x)| {
                no_out!(x.is_power_of_2());
            }),
            ("Float", &mut |(x, _)| {
                no_out!(x.is_power_of_2());
            }),
        ],
    );
}
