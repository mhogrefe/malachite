// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen_var_2, natural_unsigned_pair_gen_var_4};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_integer_mantissa_and_exponent);
    register_demo!(runner, demo_natural_integer_mantissa);
    register_demo!(runner, demo_natural_integer_exponent);
    register_demo!(runner, demo_natural_from_integer_mantissa_and_exponent);
    register_bench!(runner, benchmark_natural_integer_mantissa_and_exponent);
    register_bench!(runner, benchmark_natural_integer_mantissa);
    register_bench!(runner, benchmark_natural_integer_exponent);
    register_bench!(runner, benchmark_natural_from_integer_mantissa_and_exponent);
}

fn demo_natural_integer_mantissa_and_exponent(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_2().get(gm, config).take(limit) {
        println!(
            "{}.integer_mantissa_and_exponent() = {:?}",
            n,
            n.integer_mantissa_and_exponent()
        );
    }
}

fn demo_natural_integer_mantissa(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_2().get(gm, config).take(limit) {
        println!("{}.integer_mantissa() = {}", n, n.integer_mantissa());
    }
}

fn demo_natural_integer_exponent(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_2().get(gm, config).take(limit) {
        println!("{}.integer_exponent() = {}", n, n.integer_exponent());
    }
}

fn demo_natural_from_integer_mantissa_and_exponent(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mantissa, exponent) in natural_unsigned_pair_gen_var_4::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let n = <&Natural as IntegerMantissaAndExponent::<Natural, u64, Natural>>
            ::from_integer_mantissa_and_exponent(mantissa.clone(), exponent);
        println!(
            "Natural::from_integer_mantissa_and_exponent({}, {}) = {}",
            mantissa,
            exponent,
            n.unwrap()
        );
    }
}

fn benchmark_natural_integer_mantissa_and_exponent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.integer_mantissa_and_exponent()",
        BenchmarkType::Single,
        natural_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(x.integer_mantissa_and_exponent())
        })],
    );
}

fn benchmark_natural_integer_mantissa(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.integer_mantissa()",
        BenchmarkType::Single,
        natural_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.integer_mantissa()))],
    );
}

fn benchmark_natural_integer_exponent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.integer_exponent()",
        BenchmarkType::Single,
        natural_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.integer_exponent()))],
    );
}

fn benchmark_natural_from_integer_mantissa_and_exponent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_integer_mantissa_and_exponent(Natural, u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_4::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(<&Natural as IntegerMantissaAndExponent::<
                Natural,
                u64,
                Natural,
            >>::from_integer_mantissa_and_exponent(
                mantissa, exponent
            ))
        })],
    );
}
