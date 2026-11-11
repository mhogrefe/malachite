// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::pair_1_big_exponent_float_complexity_bucketer;
use malachite_float::test_util::generators::big_exponent_float_gaussian_integer_pair_gen;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_big_exponent_float_partial_cmp_abs_gaussian_integer
    );
    register_demo!(
        runner,
        demo_gaussian_integer_partial_cmp_abs_big_exponent_float
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_partial_cmp_abs_gaussian_integer
    );
}

fn demo_big_exponent_float_partial_cmp_abs_gaussian_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y) in big_exponent_float_gaussian_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        match x.partial_cmp_abs(&y) {
            None => println!("|{x:#x}| and |{y}| are incomparable"),
            Some(Less) => println!("|{x:#x}| < |{y}|"),
            Some(Equal) => println!("|{x:#x}| = |{y}|"),
            Some(Greater) => println!("|{x:#x}| > |{y}|"),
        }
    }
}

fn demo_gaussian_integer_partial_cmp_abs_big_exponent_float(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y) in big_exponent_float_gaussian_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        match y.partial_cmp_abs(&x) {
            None => println!("|{y}| and |{x:#x}| are incomparable"),
            Some(Less) => println!("|{y}| < |{x:#x}|"),
            Some(Equal) => println!("|{y}| = |{x:#x}|"),
            Some(Greater) => println!("|{y}| > |{x:#x}|"),
        }
    }
}

fn benchmark_big_exponent_float_partial_cmp_abs_gaussian_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.partial_cmp_abs(&GaussianInteger)",
        BenchmarkType::Single,
        big_exponent_float_gaussian_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(x.partial_cmp_abs(&y));
        })],
    );
}
