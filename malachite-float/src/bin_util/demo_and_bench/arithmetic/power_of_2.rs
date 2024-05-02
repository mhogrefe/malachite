// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::test_util::bench::bucketers::{
    abs_pair_usize_convertible_max_bucketer, signed_abs_bucketer, unsigned_direct_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_gen_var_5, signed_unsigned_pair_gen_var_19, unsigned_gen_var_5,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::{ComparableFloat, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_power_of_2_prec);
    register_demo!(runner, demo_float_power_of_2_prec_debug);
    register_demo!(runner, demo_float_power_of_2_u64);
    register_demo!(runner, demo_float_power_of_2_u64_debug);
    register_demo!(runner, demo_float_power_of_2_i64);
    register_demo!(runner, demo_float_power_of_2_i64_debug);

    register_bench!(runner, benchmark_float_power_of_2_prec);
    register_bench!(runner, benchmark_float_power_of_2_u64);
    register_bench!(runner, benchmark_float_power_of_2_i64);
}

fn demo_float_power_of_2_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in signed_unsigned_pair_gen_var_19::<i64, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_prec({}, {}) = {}",
            x,
            prec,
            Float::power_of_2_prec(x, prec)
        );
    }
}

fn demo_float_power_of_2_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in signed_unsigned_pair_gen_var_19::<i64, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_prec({}, {}) = {:#x}",
            x,
            prec,
            ComparableFloat(Float::power_of_2_prec(x, prec))
        );
    }
}

fn demo_float_power_of_2_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen_var_5::<u64>().get(gm, config).take(limit) {
        println!("Float::power_of_2({}) = {}", x, Float::power_of_2(x));
    }
}

fn demo_float_power_of_2_u64_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen_var_5::<u64>().get(gm, config).take(limit) {
        println!(
            "Float::power_of_2({}) = {:#x}",
            x,
            ComparableFloat(Float::power_of_2(x))
        );
    }
}

fn demo_float_power_of_2_i64(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in signed_gen_var_5::<i64>().get(gm, config).take(limit) {
        println!("Float::power_of_2({}) = {}", x, Float::power_of_2(x));
    }
}

fn demo_float_power_of_2_i64_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in signed_gen_var_5::<i64>().get(gm, config).take(limit) {
        println!(
            "Float::power_of_2({}) = {:#x}",
            x,
            ComparableFloat(Float::power_of_2(x))
        );
    }
}

fn benchmark_float_power_of_2_prec(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.power_of_2_prec(i64, u64)",
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_19::<i64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &abs_pair_usize_convertible_max_bucketer("i", "p"),
        &mut [("Malachite", &mut |(i, p)| {
            no_out!(Float::power_of_2_prec(i, p))
        })],
    );
}

fn benchmark_float_power_of_2_u64(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.power_of_2(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(Float::power_of_2(u)))],
    );
}

fn benchmark_float_power_of_2_i64(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.power_of_2(i64)",
        BenchmarkType::Single,
        signed_gen_var_5::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_abs_bucketer("x"),
        &mut [("Malachite", &mut |i| no_out!(Float::power_of_2(i)))],
    );
}
