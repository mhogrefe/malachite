// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::test_util::bench::bucketers::{
    abs_pair_usize_convertible_max_bucketer, abs_triple_1_2_usize_convertible_max_bucketer,
    signed_abs_bucketer, unsigned_direct_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_gen_var_5, signed_unsigned_pair_gen_var_19, unsigned_gen_var_5,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::arithmetic::power_of_2::{
    power_of_2_i64_naive, power_of_2_prec_naive, power_of_2_prec_round_naive, power_of_2_u64_naive,
};
use malachite_float::test_util::generators::signed_unsigned_rounding_mode_triple_gen_var_5;
use malachite_float::{ComparableFloat, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_power_of_2_prec_round);
    register_demo!(runner, demo_float_power_of_2_prec_round_debug);
    register_demo!(runner, demo_float_power_of_2_prec);
    register_demo!(runner, demo_float_power_of_2_prec_debug);
    register_demo!(runner, demo_float_power_of_2_u64);
    register_demo!(runner, demo_float_power_of_2_u64_debug);
    register_demo!(runner, demo_float_power_of_2_i64);
    register_demo!(runner, demo_float_power_of_2_i64_debug);

    register_bench!(runner, benchmark_float_power_of_2_prec_round_algorithms);
    register_bench!(runner, benchmark_float_power_of_2_prec_algorithms);
    register_bench!(runner, benchmark_float_power_of_2_u64_algorithms);
    register_bench!(runner, benchmark_float_power_of_2_i64_algorithms);
}

fn demo_float_power_of_2_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in signed_unsigned_rounding_mode_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_prec_round({}, {}, {:?}) = {:?}",
            x,
            prec,
            rm,
            Float::power_of_2_prec_round(x, prec, rm)
        );
    }
}

fn demo_float_power_of_2_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in signed_unsigned_rounding_mode_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let (p, o) = Float::power_of_2_prec_round(x, prec, rm);
        println!(
            "Float::power_of_2_prec_round({}, {}, {:?}) = ({:#x}, {:?})",
            x,
            prec,
            rm,
            ComparableFloat(p),
            o
        );
    }
}

fn demo_float_power_of_2_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in signed_unsigned_pair_gen_var_19::<i64, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_prec({}, {}) = {:?}",
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
        let (p, o) = Float::power_of_2_prec(x, prec);
        println!(
            "Float::power_of_2_prec({}, {}) = ({:#x}, {:?})",
            x,
            prec,
            ComparableFloat(p),
            o
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

fn benchmark_float_power_of_2_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        signed_unsigned_rounding_mode_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &abs_triple_1_2_usize_convertible_max_bucketer("i", "p"),
        &mut [
            ("default", &mut |(i, p, rm)| {
                no_out!(Float::power_of_2_prec_round(i, p, rm))
            }),
            ("naive", &mut |(i, p, rm)| {
                no_out!(power_of_2_prec_round_naive(i, p, rm))
            }),
        ],
    );
}

fn benchmark_float_power_of_2_prec_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_prec(i64, u64)",
        BenchmarkType::Algorithms,
        signed_unsigned_pair_gen_var_19::<i64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &abs_pair_usize_convertible_max_bucketer("i", "p"),
        &mut [
            ("default", &mut |(i, p)| {
                no_out!(Float::power_of_2_prec(i, p))
            }),
            ("naive", &mut |(i, p)| no_out!(power_of_2_prec_naive(i, p))),
        ],
    );
}

fn benchmark_float_power_of_2_u64_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |u| no_out!(Float::power_of_2(u))),
            ("naive", &mut |u| no_out!(power_of_2_u64_naive(u))),
        ],
    );
}

fn benchmark_float_power_of_2_i64_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2(i64)",
        BenchmarkType::Algorithms,
        signed_gen_var_5::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_abs_bucketer("x"),
        &mut [
            ("default", &mut |i| no_out!(Float::power_of_2(i))),
            ("naive", &mut |i| no_out!(power_of_2_i64_naive(i))),
        ],
    );
}
