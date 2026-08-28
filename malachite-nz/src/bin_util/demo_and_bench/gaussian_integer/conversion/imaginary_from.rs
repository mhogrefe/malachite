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
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::bench::bucketers::{integer_bit_bucketer, natural_bit_bucketer};
use malachite_nz::test_util::generators::{integer_gen, natural_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_imaginary_from_integer);
    register_demo!(runner, demo_gaussian_integer_imaginary_from_natural);
    register_bench!(runner, benchmark_gaussian_integer_imaginary_from_integer);
    register_bench!(runner, benchmark_gaussian_integer_imaginary_from_natural);
}

fn demo_gaussian_integer_imaginary_from_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in integer_gen().get(gm, config).take(limit) {
        println!(
            "GaussianInteger::imaginary_from({}) = {}",
            x.clone(),
            GaussianInteger::imaginary_from(x)
        );
    }
}

fn benchmark_gaussian_integer_imaginary_from_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger::imaginary_from(Integer)",
        BenchmarkType::Single,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(GaussianInteger::imaginary_from(x));
        })],
    );
}

fn demo_gaussian_integer_imaginary_from_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!(
            "GaussianInteger::imaginary_from({}) = {}",
            x.clone(),
            GaussianInteger::imaginary_from(x)
        );
    }
}

fn benchmark_gaussian_integer_imaginary_from_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger::imaginary_from(Natural)",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(GaussianInteger::imaginary_from(x));
        })],
    );
}
