// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::string_gen;
use malachite_base::test_util::runner::Runner;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::bench::bucketers::gaussian_rational_bit_bucketer;
use malachite_q::test_util::generators::gaussian_rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_from_str);
    register_demo!(runner, demo_gaussian_rational_from_str_targeted);
    register_bench!(runner, benchmark_gaussian_rational_from_str);
}

fn demo_gaussian_rational_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        println!(
            "GaussianRational::from_str({:?}) = {:?}",
            s,
            GaussianRational::from_str(&s)
        );
    }
}

fn demo_gaussian_rational_from_str_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        let s = x.to_string();
        println!(
            "GaussianRational::from_str({:?}) = {:?}",
            s,
            GaussianRational::from_str(&s)
        );
    }
}

// The string is produced inside the benchmarked function, so the measurement includes the to_string
// time; the benchmark is a round trip.
fn benchmark_gaussian_rational_from_str(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational::from_str(&GaussianRational.to_string())",
        BenchmarkType::Single,
        gaussian_rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(GaussianRational::from_str(&x.to_string()).unwrap());
        })],
    );
}
