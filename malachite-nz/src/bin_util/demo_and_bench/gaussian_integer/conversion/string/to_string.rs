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
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_gen_var_1, gaussian_integer_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_to_string);
    register_demo!(runner, demo_real_gaussian_integer_to_string);
    register_demo!(runner, demo_imaginary_gaussian_integer_to_string);
    register_bench!(runner, benchmark_gaussian_integer_to_string);
    register_bench!(runner, benchmark_real_gaussian_integer_to_string);
    register_bench!(runner, benchmark_imaginary_gaussian_integer_to_string);
}

fn demo_gaussian_integer_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("{x}");
    }
}

fn demo_real_gaussian_integer_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen_var_1().get(gm, config).take(limit) {
        println!("{x}");
    }
}

fn demo_imaginary_gaussian_integer_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen_var_2().get(gm, config).take(limit) {
        println!("{x}");
    }
}

fn benchmark_gaussian_integer_to_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.to_string()",
        BenchmarkType::Single,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.to_string()))],
    );
}

fn benchmark_real_gaussian_integer_to_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.to_string(), purely real",
        BenchmarkType::Single,
        gaussian_integer_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.to_string()))],
    );
}

fn benchmark_imaginary_gaussian_integer_to_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.to_string(), purely imaginary",
        BenchmarkType::Single,
        gaussian_integer_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.to_string()))],
    );
}
