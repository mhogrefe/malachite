// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{IsGaussianInteger, IsInteger, IsReal};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::big_exponent_float_complexity_bucketer;
use malachite_float::test_util::generators::big_exponent_float_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_is_integer);
    register_demo!(runner, demo_big_exponent_float_is_real);
    register_demo!(runner, demo_big_exponent_float_is_gaussian_integer);

    register_bench!(runner, benchmark_big_exponent_float_is_integer);
    register_bench!(runner, benchmark_big_exponent_float_is_real);
    register_bench!(runner, benchmark_big_exponent_float_is_gaussian_integer);
}

fn demo_big_exponent_float_is_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("({x:#x}).is_integer() = {}", x.is_integer());
    }
}

fn demo_big_exponent_float_is_real(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("({x:#x}).is_real() = {}", x.is_real());
    }
}

fn demo_big_exponent_float_is_gaussian_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "({x:#x}).is_gaussian_integer() = {}",
            x.is_gaussian_integer()
        );
    }
}

fn benchmark_big_exponent_float_is_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_integer()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(x.is_integer());
        })],
    );
}

fn benchmark_big_exponent_float_is_real(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_real()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(x.is_real());
        })],
    );
}

fn benchmark_big_exponent_float_is_gaussian_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_gaussian_integer()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(x.is_gaussian_integer());
        })],
    );
}
