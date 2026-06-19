// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    LogBasePowerOf2Of1PlusX, LogBasePowerOf2Of1PlusXAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::pair_1_primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_signed_pair_gen_var_4;
use malachite_base::test_util::runner::Runner;
use malachite_float::arithmetic::log_base_power_of_2_1_plus_x::*;
use malachite_float::test_util::arithmetic::log_base_power_of_2_1_plus_x::{
    rug_log_base_power_of_2_1_plus_x, rug_log_base_power_of_2_1_plus_x_prec_round,
    rug_log_base_power_of_2_1_plus_x_round,
};
use malachite_float::test_util::bench::bucketers::{
    pair_2_quadruple_1_3_float_primitive_int_max_complexity_bucketer,
    pair_2_triple_1_float_complexity_bucketer,
    quadruple_1_3_float_primitive_int_max_complexity_bucketer, triple_1_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_signed_rounding_mode_triple_gen_var_9, float_signed_rounding_mode_triple_gen_var_9_rm,
    float_signed_rounding_mode_triple_gen_var_10,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_7,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_7_rm,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_8,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x);
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_debug);
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_extreme);
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_extreme_debug
    );
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_ref);
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_ref_debug);
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_assign);
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_assign_debug);
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_prec);
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_prec_debug);
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_prec_ref);
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_prec_ref_debug
    );
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_prec_assign);
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_prec_assign_debug
    );
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_round);
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_round_debug);
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_round_extreme
    );
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_round_extreme_debug
    );
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_round_ref);
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_round_ref_debug
    );
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_round_assign);
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_round_assign_debug
    );
    register_demo!(runner, demo_float_log_base_power_of_2_1_plus_x_prec_round);
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_prec_round_debug
    );
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_prec_round_extreme
    );
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_prec_round_extreme_debug
    );
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_prec_round_ref
    );
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_prec_round_ref_debug
    );
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_prec_round_assign
    );
    register_demo!(
        runner,
        demo_float_log_base_power_of_2_1_plus_x_prec_round_assign_debug
    );
    register_primitive_float_demos!(runner, demo_primitive_float_log_base_power_of_2_1_plus_x);

    register_bench!(
        runner,
        benchmark_float_log_base_power_of_2_1_plus_x_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_power_of_2_1_plus_x_library_comparison
    );
    register_bench!(runner, benchmark_float_log_base_power_of_2_1_plus_x_assign);
    register_bench!(
        runner,
        benchmark_float_log_base_power_of_2_1_plus_x_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_power_of_2_1_plus_x_round_library_comparison
    );
    register_bench!(
        runner,
        benchmark_float_log_base_power_of_2_1_plus_x_round_assign
    );
    register_bench!(
        runner,
        benchmark_float_log_base_power_of_2_1_plus_x_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_power_of_2_1_plus_x_prec_round_library_comparison
    );
    register_bench!(
        runner,
        benchmark_float_log_base_power_of_2_1_plus_x_prec_round_assign
    );
    register_primitive_float_benches!(
        runner,
        benchmark_primitive_float_log_base_power_of_2_1_plus_x
    );
}

fn demo_float_log_base_power_of_2_1_plus_x(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, pow, _, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_power_of_2_1_plus_x({}) = {}",
            x_old,
            pow,
            x.log_base_power_of_2_1_plus_x(pow)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, pow, _, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({:#x}).log_base_power_of_2_1_plus_x({}) = {:#x}",
            ComparableFloat(x_old),
            pow,
            ComparableFloat(x.log_base_power_of_2_1_plus_x(pow))
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, pow, _, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_power_of_2_1_plus_x({}) = {}",
            x_old,
            pow,
            x.log_base_power_of_2_1_plus_x(pow)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_extreme_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, _, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({:#x}).log_base_power_of_2_1_plus_x({}) = {:#x}",
            ComparableFloat(x_old),
            pow,
            ComparableFloat(x.log_base_power_of_2_1_plus_x(pow))
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, pow, _, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).log_base_power_of_2_1_plus_x({}) = {}",
            x,
            pow,
            (&x).log_base_power_of_2_1_plus_x(pow)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, _, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{:#x}).log_base_power_of_2_1_plus_x({}) = {:#x}",
            ComparableFloatRef(&x),
            pow,
            ComparableFloat((&x).log_base_power_of_2_1_plus_x(pow))
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, pow, _, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_power_of_2_1_plus_x_assign(pow);
        println!("x := {x_old}; x.log_base_power_of_2_1_plus_x_assign({pow}); x = {x}");
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, pow, _, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_power_of_2_1_plus_x_assign(pow);
        println!(
            "x := {:#x}; x.log_base_power_of_2_1_plus_x_assign({}); x = {:#x}",
            ComparableFloat(x_old),
            pow,
            ComparableFloat(x)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, pow, prec, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_power_of_2_1_plus_x_prec({}, {}) = {:?}",
            x_old,
            pow,
            prec,
            x.log_base_power_of_2_1_plus_x_prec(pow, prec)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, prec, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log, o) = x.log_base_power_of_2_1_plus_x_prec(pow, prec);
        println!(
            "({:#x}).log_base_power_of_2_1_plus_x_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            pow,
            prec,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, pow, prec, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).log_base_power_of_2_1_plus_x_prec_ref({}, {}) = {:?}",
            x,
            pow,
            prec,
            x.log_base_power_of_2_1_plus_x_prec_ref(pow, prec)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, prec, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.log_base_power_of_2_1_plus_x_prec_ref(pow, prec);
        println!(
            "(&{:#x}).log_base_power_of_2_1_plus_x_prec_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            pow,
            prec,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, pow, prec, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_power_of_2_1_plus_x_prec_assign(pow, prec);
        println!(
            "x := {x_old}; x.log_base_power_of_2_1_plus_x_prec_assign({pow}, {prec}); x = {x}"
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, pow, prec, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_power_of_2_1_plus_x_prec_assign(pow, prec);
        println!(
            "x := {:#x}; x.log_base_power_of_2_1_plus_x_prec_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            pow,
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, pow, rm) in float_signed_rounding_mode_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_power_of_2_1_plus_x_round({}, {}) = {:?}",
            x_old,
            pow,
            rm,
            x.log_base_power_of_2_1_plus_x_round(pow, rm)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, rm) in float_signed_rounding_mode_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log, o) = x.log_base_power_of_2_1_plus_x_round(pow, rm);
        println!(
            "({:#x}).log_base_power_of_2_1_plus_x_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            pow,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_round_extreme(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, rm) in float_signed_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_power_of_2_1_plus_x_round({}, {}) = {:?}",
            x_old,
            pow,
            rm,
            x.log_base_power_of_2_1_plus_x_round(pow, rm)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_round_extreme_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, rm) in float_signed_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log, o) = x.log_base_power_of_2_1_plus_x_round(pow, rm);
        println!(
            "({:#x}).log_base_power_of_2_1_plus_x_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            pow,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_round_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, rm) in float_signed_rounding_mode_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).log_base_power_of_2_1_plus_x_round_ref({}, {}) = {:?}",
            x,
            pow,
            rm,
            x.log_base_power_of_2_1_plus_x_round_ref(pow, rm)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_round_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, rm) in float_signed_rounding_mode_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.log_base_power_of_2_1_plus_x_round_ref(pow, rm);
        println!(
            "(&{:#x}).log_base_power_of_2_1_plus_x_round_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            pow,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, pow, rm) in float_signed_rounding_mode_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_power_of_2_1_plus_x_round_assign(pow, rm);
        println!("x := {x_old}; x.log_base_power_of_2_1_plus_x_round_assign({pow}, {rm}); x = {x}");
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_round_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, pow, rm) in float_signed_rounding_mode_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_power_of_2_1_plus_x_round_assign(pow, rm);
        println!(
            "x := {:#x}; x.log_base_power_of_2_1_plus_x_round_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            pow,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_power_of_2_1_plus_x_prec_round({}, {}, {}) = {:?}",
            x_old,
            pow,
            prec,
            rm,
            x.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log, o) = x.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
        println!(
            "({:#x}).log_base_power_of_2_1_plus_x_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            pow,
            prec,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_round_extreme(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_power_of_2_1_plus_x_prec_round({}, {}, {}) = {:?}",
            x_old,
            pow,
            prec,
            rm,
            x.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_round_extreme_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log, o) = x.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
        println!(
            "({:#x}).log_base_power_of_2_1_plus_x_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            pow,
            prec,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_round_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).log_base_power_of_2_1_plus_x_prec_round_ref({}, {}, {}) = {:?}",
            x,
            pow,
            prec,
            rm,
            x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, rm)
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_round_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, rm);
        println!(
            "(&{:#x}).log_base_power_of_2_1_plus_x_prec_round_ref({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            pow,
            prec,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, pow, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_power_of_2_1_plus_x_prec_round_assign(pow, prec, rm);
        println!(
            "x := {x_old}; x.log_base_power_of_2_1_plus_x_prec_round_assign({pow}, {prec}, {rm}) = \
             {o:?}; x = {x}"
        );
    }
}

fn demo_float_log_base_power_of_2_1_plus_x_prec_round_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, pow, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_power_of_2_1_plus_x_prec_round_assign(pow, prec, rm);
        println!(
            "x := {:#x}; x.log_base_power_of_2_1_plus_x_prec_round_assign({}, {}, {}) = {:?}; \
             x = {:#x}",
            ComparableFloat(x_old),
            pow,
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_log_base_power_of_2_1_plus_x_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_power_of_2_1_plus_x(i64)",
        BenchmarkType::EvaluationStrategy,
        float_signed_rounding_mode_triple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float.log_base_power_of_2_1_plus_x(i64)",
                &mut |(x, pow, _)| {
                    no_out!(x.log_base_power_of_2_1_plus_x(pow));
                },
            ),
            (
                "(&Float).log_base_power_of_2_1_plus_x(i64)",
                &mut |(x, pow, _)| {
                    no_out!((&x).log_base_power_of_2_1_plus_x(pow));
                },
            ),
        ],
    );
}

fn benchmark_float_log_base_power_of_2_1_plus_x_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_power_of_2_1_plus_x(i64)",
        BenchmarkType::LibraryComparison,
        float_signed_rounding_mode_triple_gen_var_9_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, pow, _))| {
                no_out!((&x).log_base_power_of_2_1_plus_x(pow));
            }),
            ("rug", &mut |((x, pow, _), _)| {
                no_out!(rug_log_base_power_of_2_1_plus_x(&x, pow));
            }),
        ],
    );
}

fn benchmark_float_log_base_power_of_2_1_plus_x_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_power_of_2_1_plus_x_assign(i64)",
        BenchmarkType::Single,
        float_signed_rounding_mode_triple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [(
            "Float.log_base_power_of_2_1_plus_x_assign(i64)",
            &mut |(mut x, pow, _)| {
                x.log_base_power_of_2_1_plus_x_assign(pow);
            },
        )],
    );
}

fn benchmark_float_log_base_power_of_2_1_plus_x_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_power_of_2_1_plus_x_round(i64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_signed_rounding_mode_triple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float.log_base_power_of_2_1_plus_x_round(i64, RoundingMode)",
                &mut |(x, pow, rm)| no_out!(x.log_base_power_of_2_1_plus_x_round(pow, rm)),
            ),
            (
                "(&Float).log_base_power_of_2_1_plus_x_round_ref(i64, RoundingMode)",
                &mut |(x, pow, rm)| no_out!(x.log_base_power_of_2_1_plus_x_round_ref(pow, rm)),
            ),
        ],
    );
}

fn benchmark_float_log_base_power_of_2_1_plus_x_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_power_of_2_1_plus_x_round(i64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_signed_rounding_mode_triple_gen_var_9_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, pow, rm))| {
                no_out!(x.log_base_power_of_2_1_plus_x_round_ref(pow, rm));
            }),
            ("rug", &mut |((x, pow, rm), _)| {
                no_out!(rug_log_base_power_of_2_1_plus_x_round(&x, pow, rm));
            }),
        ],
    );
}

fn benchmark_float_log_base_power_of_2_1_plus_x_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_power_of_2_1_plus_x_round_assign(i64, RoundingMode)",
        BenchmarkType::Single,
        float_signed_rounding_mode_triple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [(
            "Float.log_base_power_of_2_1_plus_x_round_assign(i64, RoundingMode)",
            &mut |(mut x, pow, rm)| no_out!(x.log_base_power_of_2_1_plus_x_round_assign(pow, rm)),
        )],
    );
}

fn benchmark_float_log_base_power_of_2_1_plus_x_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_power_of_2_1_plus_x_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_3_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.log_base_power_of_2_1_plus_x_prec_round(i64, u64, RoundingMode)",
                &mut |(x, pow, prec, rm)| {
                    no_out!(x.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm));
                },
            ),
            (
                "(&Float).log_base_power_of_2_1_plus_x_prec_round_ref(i64, u64, RoundingMode)",
                &mut |(x, pow, prec, rm)| {
                    no_out!(x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_log_base_power_of_2_1_plus_x_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_power_of_2_1_plus_x_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_7_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_3_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, pow, prec, rm))| {
                no_out!(x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, rm));
            }),
            ("rug", &mut |((x, pow, prec, rm), _)| {
                no_out!(rug_log_base_power_of_2_1_plus_x_prec_round(
                    &x, pow, prec, rm
                ));
            }),
        ],
    );
}

fn benchmark_float_log_base_power_of_2_1_plus_x_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_power_of_2_1_plus_x_prec_round_assign(i64, u64, RoundingMode)",
        BenchmarkType::Single,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_3_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [(
            "Float.log_base_power_of_2_1_plus_x_prec_round_assign(i64, u64, RoundingMode)",
            &mut |(mut x, pow, prec, rm)| {
                no_out!(x.log_base_power_of_2_1_plus_x_prec_round_assign(pow, prec, rm));
            },
        )],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_log_base_power_of_2_1_plus_x<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, pow) in primitive_float_signed_pair_gen_var_4::<T, i64>()
        .get(gm, config)
        .filter(|&(_, pow)| pow != 0)
        .take(limit)
    {
        println!(
            "primitive_float_log_base_power_of_2_1_plus_x({}, {}) = {}",
            NiceFloat(x),
            pow,
            NiceFloat(primitive_float_log_base_power_of_2_1_plus_x(x, pow))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_log_base_power_of_2_1_plus_x<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!(
            "primitive_float_log_base_power_of_2_1_plus_x({}, i64)",
            T::NAME
        ),
        BenchmarkType::Single,
        primitive_float_signed_pair_gen_var_4::<T, i64>()
            .get(gm, config)
            .filter(|&(_, pow)| pow != 0),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("x"),
        &mut [("malachite", &mut |(x, pow)| {
            no_out!(primitive_float_log_base_power_of_2_1_plus_x(x, pow));
        })],
    );
}
