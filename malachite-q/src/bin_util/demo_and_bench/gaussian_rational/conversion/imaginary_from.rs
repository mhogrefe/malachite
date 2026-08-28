// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ImaginaryFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_imaginary_from_integer);
    register_demo!(runner, demo_gaussian_rational_imaginary_from_rational);
    register_bench!(runner, benchmark_gaussian_rational_imaginary_from_integer);
    register_bench!(runner, benchmark_gaussian_rational_imaginary_from_rational);
}

fn demo_gaussian_rational_imaginary_from_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in integer_gen().get(gm, config).take(limit) {
        println!(
            "GaussianRational::imaginary_from({}) = {}",
            x.clone(),
            GaussianRational::imaginary_from(x)
        );
    }
}

fn benchmark_gaussian_rational_imaginary_from_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational::imaginary_from(Integer)",
        BenchmarkType::Single,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(GaussianRational::imaginary_from(x));
        })],
    );
}

fn demo_gaussian_rational_imaginary_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "GaussianRational::imaginary_from({}) = {}",
            x.clone(),
            GaussianRational::imaginary_from(x)
        );
    }
}

fn benchmark_gaussian_rational_imaginary_from_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational::imaginary_from(Rational)",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(GaussianRational::imaginary_from(x));
        })],
    );
}
