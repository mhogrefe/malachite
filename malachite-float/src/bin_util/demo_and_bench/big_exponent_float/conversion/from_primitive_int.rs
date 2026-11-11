// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{
    signed_bit_bucketer, triple_1_2_primitive_int_bit_u64_max_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::generators::{
    signed_unsigned_rounding_mode_triple_gen_var_3,
    unsigned_unsigned_rounding_mode_triple_gen_var_5,
};
use malachite_nz::platform::{Limb, SignedLimb};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_from_u64);
    register_demo!(runner, demo_big_exponent_float_from_i64);
    register_demo!(runner, demo_big_exponent_float_const_from_unsigned);
    register_demo!(runner, demo_big_exponent_float_const_from_signed);
    register_demo!(runner, demo_big_exponent_float_from_unsigned_prec_round);
    register_demo!(runner, demo_big_exponent_float_from_signed_prec_round);

    register_bench!(runner, benchmark_big_exponent_float_from_u64);
    register_bench!(runner, benchmark_big_exponent_float_from_i64);
    register_bench!(runner, benchmark_big_exponent_float_const_from_unsigned);
    register_bench!(
        runner,
        benchmark_big_exponent_float_from_unsigned_prec_round
    );
    register_bench!(runner, benchmark_big_exponent_float_from_signed_prec_round);
}

fn demo_big_exponent_float_from_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen::<u64>().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::from({n}) = {:#x}",
            BigExponentFloat::from(n)
        );
    }
}

fn demo_big_exponent_float_from_i64(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in signed_gen::<i64>().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::from({n}) = {:#x}",
            BigExponentFloat::from(n)
        );
    }
}

fn demo_big_exponent_float_const_from_unsigned(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen::<Limb>().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::from({n}) = {:#x}",
            BigExponentFloat::const_from_unsigned(n)
        );
    }
}

fn demo_big_exponent_float_const_from_signed(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in signed_gen::<SignedLimb>().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::from({n}) = {:#x}",
            BigExponentFloat::const_from_signed(n)
        );
    }
}

fn demo_big_exponent_float_from_unsigned_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, prec, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_5::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = BigExponentFloat::from_unsigned_prec_round(n, prec, rm);
        println!("BigExponentFloat::from_unsigned_prec_round({n}, {prec}, {rm}) = ({x:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_from_signed_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, prec, rm) in signed_unsigned_rounding_mode_triple_gen_var_3::<i64>()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = BigExponentFloat::from_signed_prec_round(n, prec, rm);
        println!("BigExponentFloat::from_signed_prec_round({n}, {prec}, {rm}) = ({x:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_from_u64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from(u64)",
        BenchmarkType::Single,
        unsigned_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(BigExponentFloat::from(n));
        })],
    );
}

fn benchmark_big_exponent_float_from_i64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from(i64)",
        BenchmarkType::Single,
        signed_gen::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(BigExponentFloat::from(n));
        })],
    );
}

fn benchmark_big_exponent_float_const_from_unsigned(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::const_from_unsigned(Limb)",
        BenchmarkType::Single,
        unsigned_gen::<Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(BigExponentFloat::const_from_unsigned(n));
        })],
    );
}

fn benchmark_big_exponent_float_from_unsigned_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_unsigned_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::Single,
        unsigned_unsigned_rounding_mode_triple_gen_var_5::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_primitive_int_bit_u64_max_bucketer("n", "prec"),
        &mut [("Malachite", &mut |(n, prec, rm)| {
            no_out!(BigExponentFloat::from_unsigned_prec_round(n, prec, rm));
        })],
    );
}

fn benchmark_big_exponent_float_from_signed_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_signed_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::Single,
        signed_unsigned_rounding_mode_triple_gen_var_3::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_primitive_int_bit_u64_max_bucketer("n", "prec"),
        &mut [("Malachite", &mut |(n, prec, rm)| {
            no_out!(BigExponentFloat::from_signed_prec_round(n, prec, rm));
        })],
    );
}
