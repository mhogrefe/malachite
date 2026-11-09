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
use malachite_float::ComparableFloat;
use malachite_float::test_util::bench::bucketers::triple_1_float_complexity_bucketer;
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_53,
    float_rounding_mode_unsigned_rounding_mode_quadruple_gen_var_1, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_56,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_round_to_integer);
    register_demo!(runner, demo_float_round_to_integer_debug);
    register_demo!(runner, demo_float_round_to_integer_prec_debug);
    register_demo!(runner, demo_float_round_to_integer_round_debug);
    register_demo!(runner, demo_float_round_to_integer_prec_round);
    register_demo!(runner, demo_float_round_to_integer_prec_round_debug);
    register_demo!(runner, demo_float_round_to_integer_then_prec_round);
    register_demo!(runner, demo_float_round_to_integer_then_prec_round_debug);

    register_bench!(runner, benchmark_float_round_to_integer_prec_round);
}

fn demo_float_round_to_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).round_to_integer() = {:?}",
            x_old,
            x.round_to_integer()
        );
    }
}

fn demo_float_round_to_integer_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (r, o, is_int) = x.round_to_integer();
        println!(
            "({:#x}).round_to_integer() = ({:#x}, {:?}, {})",
            ComparableFloat(x_old),
            ComparableFloat(r),
            o,
            is_int
        );
    }
}

fn demo_float_round_to_integer_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (r, o, is_int) = x.round_to_integer_prec(prec);
        println!(
            "({:#x}).round_to_integer_prec({}) = ({:#x}, {:?}, {})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(r),
            o,
            is_int
        );
    }
}

fn demo_float_round_to_integer_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_53()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (r, o, is_int) = x.round_to_integer_round(rm);
        println!(
            "({:#x}).round_to_integer_round({}) = ({:#x}, {:?}, {})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(r),
            o,
            is_int
        );
    }
}

fn demo_float_round_to_integer_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_56()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).round_to_integer_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.round_to_integer_prec_round(prec, rm)
        );
    }
}

fn demo_float_round_to_integer_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_56()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (r, o, is_int) = x.round_to_integer_prec_round(prec, rm);
        println!(
            "({:#x}).round_to_integer_prec_round({}, {}) = ({:#x}, {:?}, {})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(r),
            o,
            is_int
        );
    }
}

fn demo_float_round_to_integer_then_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, irm, prec, rm) in float_rounding_mode_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).round_to_integer_then_prec_round({}, {}, {}) = {:?}",
            x_old,
            irm,
            prec,
            rm,
            x.round_to_integer_then_prec_round(irm, prec, rm)
        );
    }
}

fn demo_float_round_to_integer_then_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, irm, prec, rm) in float_rounding_mode_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (r, o) = x.round_to_integer_then_prec_round(irm, prec, rm);
        println!(
            "({:#x}).round_to_integer_then_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            irm,
            prec,
            rm,
            ComparableFloat(r),
            o
        );
    }
}

fn benchmark_float_round_to_integer_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.round_to_integer_prec_round(u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_56().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, prec, rm)| {
            no_out!(x.round_to_integer_prec_round(prec, rm));
        })],
    );
}
