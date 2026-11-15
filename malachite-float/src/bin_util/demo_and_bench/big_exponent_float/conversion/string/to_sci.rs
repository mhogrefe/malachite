// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::string::options::SciSizeOptions;
use malachite_base::num::conversion::traits::ToSci;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::pair_1_big_exponent_float_complexity_bucketer;
use malachite_float::test_util::big_exponent_float::big_exponent_float_exact_string;
use malachite_float::test_util::generators::big_exponent_float_to_sci_options_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_to_sci_with_options);

    register_bench!(runner, benchmark_big_exponent_float_to_sci_with_options);
}

fn demo_big_exponent_float_to_sci_with_options(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in big_exponent_float_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let size = match options.get_size_options() {
            SciSizeOptions::Precision(p) => format!("precision {p}"),
            SciSizeOptions::Scale(s) => format!("scale {s}"),
            SciSizeOptions::Complete => "complete".to_string(),
        };
        println!(
            "({}).to_sci_with_options({}, {size}, {}) = {}",
            big_exponent_float_exact_string(&x),
            options.get_base(),
            options.get_rounding_mode(),
            x.to_sci_with_options(options)
        );
    }
}

fn benchmark_big_exponent_float_to_sci_with_options(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.to_sci_with_options(ToSciOptions)",
        BenchmarkType::Single,
        big_exponent_float_to_sci_options_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(x.to_sci_with_options(options).to_string());
        })],
    );
}
