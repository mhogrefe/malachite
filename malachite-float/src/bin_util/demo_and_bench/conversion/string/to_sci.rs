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
use malachite_float::conversion::string::to_sci::{to_sci_string, to_sci_valid};
use malachite_float::test_util::bench::bucketers::pair_1_float_complexity_bucketer;
use malachite_float::test_util::generators::float_to_sci_options_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_to_sci_string);
    register_demo!(runner, demo_to_sci_string_debug);
    register_demo!(runner, demo_to_sci_valid);
    register_bench!(runner, benchmark_to_sci_string);
}

fn demo_to_sci_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in float_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_sci_string({x}, {options:?}) = {:?}",
            to_sci_string(&x, options)
        );
    }
}

fn demo_to_sci_string_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in float_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        println!(
            "to_sci_string({cx:#x}, {options:?}) = {:?}",
            to_sci_string(&x, options)
        );
    }
}

fn demo_to_sci_valid(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in float_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_sci_valid({x}, {options:?}) = {}",
            to_sci_valid(&x, options)
        );
    }
}

fn benchmark_to_sci_string(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "to_sci_string(&Float, ToSciOptions)",
        BenchmarkType::Single,
        float_to_sci_options_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(to_sci_string(&x, options));
        })],
    );
}
