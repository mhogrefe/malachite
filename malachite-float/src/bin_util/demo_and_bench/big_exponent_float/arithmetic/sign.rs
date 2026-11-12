// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::big_exponent_float_complexity_bucketer;
use malachite_float::test_util::generators::big_exponent_float_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_sign);
    register_bench!(runner, benchmark_big_exponent_float_sign);
}

fn demo_big_exponent_float_sign(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen_var_3().get(gm, config).take(limit) {
        println!("{x:#x}.sign() = {:?}", x.sign());
    }
}

fn benchmark_big_exponent_float_sign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sign()",
        BenchmarkType::Single,
        big_exponent_float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(x.sign());
        })],
    );
}
