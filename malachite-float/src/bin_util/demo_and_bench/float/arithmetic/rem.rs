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
use malachite_float::test_util::bench::bucketers::quadruple_1_float_complexity_bucketer;
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_40,
    float_float_unsigned_rounding_mode_quadruple_gen_var_18,
    float_float_unsigned_rounding_mode_quadruple_gen_var_19, float_float_unsigned_triple_gen_var_1,
    float_float_unsigned_triple_gen_var_2, float_pair_gen, float_pair_gen_var_10,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_rem);
    register_demo!(runner, demo_float_rem_debug);
    register_demo!(runner, demo_float_rem_extreme_debug);
    register_demo!(runner, demo_float_rem_prec);
    register_demo!(runner, demo_float_rem_prec_debug);
    register_demo!(runner, demo_float_rem_prec_extreme_debug);
    register_demo!(runner, demo_float_rem_round);
    register_demo!(runner, demo_float_rem_round_debug);
    register_demo!(runner, demo_float_rem_prec_round);
    register_demo!(runner, demo_float_rem_prec_round_debug);
    register_demo!(runner, demo_float_rem_prec_round_extreme_debug);

    register_bench!(runner, benchmark_float_rem_prec_round);
}

fn demo_float_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} % {} = {}", x_old, y_old, x % y);
    }
}

fn demo_float_rem_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{:#x} % {:#x} = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x % y)
        );
    }
}

fn demo_float_rem_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{:#x} % {:#x} = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x % y)
        );
    }
}

fn demo_float_rem_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).rem_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.rem_prec(y, prec)
        );
    }
}

fn demo_float_rem_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (r, o) = x.rem_prec(y, prec);
        println!(
            "({:#x}).rem_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(r),
            o
        );
    }
}

fn demo_float_rem_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (r, o) = x.rem_prec(y, prec);
        println!(
            "({:#x}).rem_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(r),
            o
        );
    }
}

fn demo_float_rem_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_40()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).rem_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.rem_round(y, rm)
        );
    }
}

fn demo_float_rem_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_40()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (r, o) = x.rem_round(y, rm);
        println!(
            "({:#x}).rem_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(r),
            o
        );
    }
}

fn demo_float_rem_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).rem_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.rem_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_rem_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (r, o) = x.rem_prec_round(y, prec, rm);
        println!(
            "({:#x}).rem_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(r),
            o
        );
    }
}

fn demo_float_rem_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (r, o) = x.rem_prec_round(y, prec, rm);
        println!(
            "({:#x}).rem_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(r),
            o
        );
    }
}

fn benchmark_float_rem_prec_round(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.rem_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::Single,
        float_float_unsigned_rounding_mode_quadruple_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, y, prec, rm)| {
            no_out!(x.rem_prec_round(y, prec, rm));
        })],
    );
}
