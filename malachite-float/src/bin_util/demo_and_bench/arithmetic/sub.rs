// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::arithmetic::add::add_prec_round_naive;
use malachite_float::test_util::arithmetic::sub::sub_rational_prec_round_naive;
use malachite_float::test_util::arithmetic::sub::{
    rug_sub, rug_sub_prec, rug_sub_prec_round, rug_sub_rational, rug_sub_rational_prec,
    rug_sub_rational_prec_round, rug_sub_rational_round, rug_sub_round,
};
use malachite_float::test_util::bench::bucketers::{
    pair_2_pair_float_max_complexity_bucketer, pair_2_pair_float_rational_max_complexity_bucketer,
    pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    pair_2_quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer,
    pair_2_triple_1_2_float_max_complexity_bucketer,
    pair_2_triple_1_2_float_rational_max_complexity_bucketer,
    pair_2_triple_float_float_primitive_int_max_complexity_bucketer,
    pair_2_triple_float_rational_primitive_int_max_complexity_bucketer,
    pair_float_max_complexity_bucketer, pair_float_rational_max_complexity_bucketer,
    quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer,
    triple_1_2_float_max_complexity_bucketer, triple_1_2_float_rational_max_complexity_bucketer,
    triple_float_float_primitive_int_max_complexity_bucketer,
    triple_float_rational_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_1_rm, float_float_rounding_mode_triple_gen_var_2,
    float_float_rounding_mode_triple_gen_var_30,
    float_float_unsigned_rounding_mode_quadruple_gen_var_2,
    float_float_unsigned_rounding_mode_quadruple_gen_var_2_rm,
    float_float_unsigned_rounding_mode_quadruple_gen_var_6, float_float_unsigned_triple_gen_var_1,
    float_float_unsigned_triple_gen_var_1_rm, float_float_unsigned_triple_gen_var_2,
    float_pair_gen, float_pair_gen_rm, float_pair_gen_var_10, float_rational_pair_gen,
    float_rational_pair_gen_rm, float_rational_pair_gen_var_2,
    float_rational_rounding_mode_triple_gen_var_2,
    float_rational_rounding_mode_triple_gen_var_3_rm,
    float_rational_rounding_mode_triple_gen_var_8,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_2,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_2_rm,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_7,
    float_rational_unsigned_triple_gen_var_1, float_rational_unsigned_triple_gen_var_1_rm,
    float_rational_unsigned_triple_gen_var_2,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};
use std::cmp::max;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_sub);
    register_demo!(runner, demo_float_sub_debug);
    register_demo!(runner, demo_float_sub_extreme);
    register_demo!(runner, demo_float_sub_extreme_debug);
    register_demo!(runner, demo_float_sub_val_ref);
    register_demo!(runner, demo_float_sub_val_ref_debug);
    register_demo!(runner, demo_float_sub_ref_val);
    register_demo!(runner, demo_float_sub_ref_val_debug);
    register_demo!(runner, demo_float_sub_ref_ref);
    register_demo!(runner, demo_float_sub_ref_ref_debug);
    register_demo!(runner, demo_float_sub_assign);
    register_demo!(runner, demo_float_sub_assign_debug);
    register_demo!(runner, demo_float_sub_assign_ref);
    register_demo!(runner, demo_float_sub_assign_ref_debug);
    register_demo!(runner, demo_float_sub_prec);
    register_demo!(runner, demo_float_sub_prec_debug);
    register_demo!(runner, demo_float_sub_prec_extreme);
    register_demo!(runner, demo_float_sub_prec_extreme_debug);
    register_demo!(runner, demo_float_sub_prec_val_ref);
    register_demo!(runner, demo_float_sub_prec_val_ref_debug);
    register_demo!(runner, demo_float_sub_prec_ref_val);
    register_demo!(runner, demo_float_sub_prec_ref_val_debug);
    register_demo!(runner, demo_float_sub_prec_ref_ref);
    register_demo!(runner, demo_float_sub_prec_ref_ref_debug);
    register_demo!(runner, demo_float_sub_prec_assign);
    register_demo!(runner, demo_float_sub_prec_assign_debug);
    register_demo!(runner, demo_float_sub_prec_assign_ref);
    register_demo!(runner, demo_float_sub_prec_assign_ref_debug);
    register_demo!(runner, demo_float_sub_round);
    register_demo!(runner, demo_float_sub_round_debug);
    register_demo!(runner, demo_float_sub_round_extreme);
    register_demo!(runner, demo_float_sub_round_extreme_debug);
    register_demo!(runner, demo_float_sub_round_val_ref);
    register_demo!(runner, demo_float_sub_round_val_ref_debug);
    register_demo!(runner, demo_float_sub_round_ref_val);
    register_demo!(runner, demo_float_sub_round_ref_val_debug);
    register_demo!(runner, demo_float_sub_round_ref_ref);
    register_demo!(runner, demo_float_sub_round_ref_ref_debug);
    register_demo!(runner, demo_float_sub_round_assign);
    register_demo!(runner, demo_float_sub_round_assign_debug);
    register_demo!(runner, demo_float_sub_round_assign_ref);
    register_demo!(runner, demo_float_sub_round_assign_ref_debug);
    register_demo!(runner, demo_float_sub_prec_round);
    register_demo!(runner, demo_float_sub_prec_round_debug);
    register_demo!(runner, demo_float_sub_prec_round_extreme);
    register_demo!(runner, demo_float_sub_prec_round_extreme_debug);
    register_demo!(runner, demo_float_sub_prec_round_val_ref);
    register_demo!(runner, demo_float_sub_prec_round_val_ref_debug);
    register_demo!(runner, demo_float_sub_prec_round_ref_val);
    register_demo!(runner, demo_float_sub_prec_round_ref_val_debug);
    register_demo!(runner, demo_float_sub_prec_round_ref_ref);
    register_demo!(runner, demo_float_sub_prec_round_ref_ref_debug);
    register_demo!(runner, demo_float_sub_prec_round_assign);
    register_demo!(runner, demo_float_sub_prec_round_assign_debug);
    register_demo!(runner, demo_float_sub_prec_round_assign_ref);
    register_demo!(runner, demo_float_sub_prec_round_assign_ref_debug);
    register_demo!(runner, demo_float_sub_rational);
    register_demo!(runner, demo_float_sub_rational_debug);
    register_demo!(runner, demo_float_sub_rational_extreme);
    register_demo!(runner, demo_float_sub_rational_extreme_debug);
    register_demo!(runner, demo_float_sub_rational_val_ref);
    register_demo!(runner, demo_float_sub_rational_val_ref_debug);
    register_demo!(runner, demo_float_sub_rational_ref_val);
    register_demo!(runner, demo_float_sub_rational_ref_val_debug);
    register_demo!(runner, demo_float_sub_rational_ref_ref);
    register_demo!(runner, demo_float_sub_rational_ref_ref_debug);
    register_demo!(runner, demo_float_sub_rational_assign);
    register_demo!(runner, demo_float_sub_rational_assign_debug);
    register_demo!(runner, demo_float_sub_rational_assign_ref);
    register_demo!(runner, demo_float_sub_rational_assign_ref_debug);
    register_demo!(runner, demo_rational_sub_float);
    register_demo!(runner, demo_rational_sub_float_debug);
    register_demo!(runner, demo_rational_sub_float_extreme);
    register_demo!(runner, demo_rational_sub_float_extreme_debug);
    register_demo!(runner, demo_rational_sub_float_val_ref);
    register_demo!(runner, demo_rational_sub_float_val_ref_debug);
    register_demo!(runner, demo_rational_sub_float_ref_val);
    register_demo!(runner, demo_rational_sub_float_ref_val_debug);
    register_demo!(runner, demo_rational_sub_float_ref_ref);
    register_demo!(runner, demo_rational_sub_float_ref_ref_debug);
    register_demo!(runner, demo_float_sub_rational_prec);
    register_demo!(runner, demo_float_sub_rational_prec_debug);
    register_demo!(runner, demo_float_sub_rational_prec_extreme);
    register_demo!(runner, demo_float_sub_rational_prec_extreme_debug);
    register_demo!(runner, demo_float_sub_rational_prec_val_ref);
    register_demo!(runner, demo_float_sub_rational_prec_val_ref_debug);
    register_demo!(runner, demo_float_sub_rational_prec_ref_val);
    register_demo!(runner, demo_float_sub_rational_prec_ref_val_debug);
    register_demo!(runner, demo_float_sub_rational_prec_ref_ref);
    register_demo!(runner, demo_float_sub_rational_prec_ref_ref_debug);
    register_demo!(runner, demo_float_sub_rational_prec_assign);
    register_demo!(runner, demo_float_sub_rational_prec_assign_debug);
    register_demo!(runner, demo_float_sub_rational_prec_assign_ref);
    register_demo!(runner, demo_float_sub_rational_prec_assign_ref_debug);
    register_demo!(runner, demo_float_sub_rational_round);
    register_demo!(runner, demo_float_sub_rational_round_debug);
    register_demo!(runner, demo_float_sub_rational_round_extreme);
    register_demo!(runner, demo_float_sub_rational_round_extreme_debug);
    register_demo!(runner, demo_float_sub_rational_round_val_ref);
    register_demo!(runner, demo_float_sub_rational_round_val_ref_debug);
    register_demo!(runner, demo_float_sub_rational_round_ref_val);
    register_demo!(runner, demo_float_sub_rational_round_ref_val_debug);
    register_demo!(runner, demo_float_sub_rational_round_ref_ref);
    register_demo!(runner, demo_float_sub_rational_round_ref_ref_debug);
    register_demo!(runner, demo_float_sub_rational_round_assign);
    register_demo!(runner, demo_float_sub_rational_round_assign_debug);
    register_demo!(runner, demo_float_sub_rational_round_assign_ref);
    register_demo!(runner, demo_float_sub_rational_round_assign_ref_debug);
    register_demo!(runner, demo_float_sub_rational_prec_round);
    register_demo!(runner, demo_float_sub_rational_prec_round_debug);
    register_demo!(runner, demo_float_sub_rational_prec_round_extreme);
    register_demo!(runner, demo_float_sub_rational_prec_round_extreme_debug);
    register_demo!(runner, demo_float_sub_rational_prec_round_val_ref);
    register_demo!(runner, demo_float_sub_rational_prec_round_val_ref_debug);
    register_demo!(runner, demo_float_sub_rational_prec_round_ref_val);
    register_demo!(runner, demo_float_sub_rational_prec_round_ref_val_debug);
    register_demo!(runner, demo_float_sub_rational_prec_round_ref_ref);
    register_demo!(runner, demo_float_sub_rational_prec_round_ref_ref_debug);
    register_demo!(runner, demo_float_sub_rational_prec_round_assign);
    register_demo!(runner, demo_float_sub_rational_prec_round_assign_debug);
    register_demo!(runner, demo_float_sub_rational_prec_round_assign_ref);
    register_demo!(runner, demo_float_sub_rational_prec_round_assign_ref_debug);

    register_bench!(runner, benchmark_float_sub_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_library_comparison);
    register_bench!(runner, benchmark_float_sub_algorithms);
    register_bench!(runner, benchmark_float_sub_assign_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_prec_library_comparison);
    register_bench!(runner, benchmark_float_sub_prec_algorithms);
    register_bench!(runner, benchmark_float_sub_prec_assign_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_round_library_comparison);
    register_bench!(runner, benchmark_float_sub_round_algorithms);
    register_bench!(runner, benchmark_float_sub_round_assign_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_prec_round_library_comparison);
    register_bench!(runner, benchmark_float_sub_prec_round_algorithms);
    register_bench!(
        runner,
        benchmark_float_sub_prec_round_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_sub_rational_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_rational_library_comparison);
    register_bench!(runner, benchmark_float_sub_rational_algorithms);
    register_bench!(
        runner,
        benchmark_float_sub_rational_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_sub_float_evaluation_strategy);
    register_bench!(runner, benchmark_rational_sub_float_library_comparison);
    register_bench!(
        runner,
        benchmark_float_sub_rational_prec_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_sub_rational_prec_library_comparison);
    register_bench!(runner, benchmark_float_sub_rational_prec_algorithms);
    register_bench!(
        runner,
        benchmark_float_sub_rational_prec_assign_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sub_rational_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sub_rational_round_library_comparison
    );
    register_bench!(runner, benchmark_float_sub_rational_round_algorithms);
    register_bench!(
        runner,
        benchmark_float_sub_rational_round_assign_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sub_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sub_rational_prec_round_library_comparison
    );
    register_bench!(runner, benchmark_float_sub_rational_prec_round_algorithms);
    register_bench!(
        runner,
        benchmark_float_sub_rational_prec_round_assign_evaluation_strategy
    );
}

fn demo_float_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_float_sub_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{:#x} - {:#x} = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x - y)
        );
    }
}

fn demo_float_sub_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_float_sub_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{:#x} - {:#x} = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x - y)
        );
    }
}

fn demo_float_sub_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

fn demo_float_sub_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "{:#x} - &{:#x} = {:#x}",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            ComparableFloat(x - &y)
        );
    }
}

fn demo_float_sub_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

fn demo_float_sub_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "&{:#x} - {:#x} = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat(y_old),
            ComparableFloat(&x - y)
        );
    }
}

fn demo_float_sub_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

fn demo_float_sub_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        println!(
            "&{:#x} - &{:#x} = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloatRef(&y),
            ComparableFloat(&x - &y)
        );
    }
}

fn demo_float_sub_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x += y.clone();
        println!("x := {x_old}; x += {y}; x = {x}");
    }
}

fn demo_float_sub_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x += y.clone();
        println!(
            "x := {:#x}; x += {:#x}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x += &y;
        println!("x := {x_old}; x += &{y}; x = {x}");
    }
}

fn demo_float_sub_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x += &y;
        println!(
            "x := {:#x}; x += &{:#x}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.sub_prec(y, prec)
        );
    }
}

fn demo_float_sub_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_prec(y, prec);
        println!(
            "({:#x}).sub_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.sub_prec(y, prec)
        );
    }
}

fn demo_float_sub_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_prec(y, prec);
        println!(
            "({:#x}).sub_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_prec_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sub_prec_val_ref(&{}, {}) = {:?}",
            x_old,
            y,
            prec,
            x.sub_prec_val_ref(&y, prec)
        );
    }
}

fn demo_float_sub_prec_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (diff, o) = x.sub_prec_val_ref(&y, prec);
        println!(
            "({:#x}).sub_prec_val_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_prec_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).sub_prec_ref_val({}, {}) = {:?}",
            x,
            y_old,
            prec,
            x.sub_prec_ref_val(y, prec)
        );
    }
}

fn demo_float_sub_prec_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (diff, o) = x.sub_prec_ref_val(y, prec);
        println!(
            "(&{:#x}).sub_prec_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_prec_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_prec_ref_ref(&{}, {}) = {:?}",
            x,
            y,
            prec,
            x.sub_prec_ref_ref(&y, prec)
        );
    }
}

fn demo_float_sub_prec_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (diff, o) = x.sub_prec_ref_ref(&y, prec);
        println!(
            "(&{:#x}).sub_prec_ref_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y),
            prec,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.sub_prec_assign(y, prec);
        println!("x := {x_old}; x.sub_prec_assign({y_old}, {prec}); x = {x}");
    }
}

fn demo_float_sub_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_prec_assign(y, prec);
        println!(
            "x := {:#x}; x.sub_prec_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_prec_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.sub_prec_assign_ref(&y, prec);
        println!("x := {x_old}; x.sub_prec_assign({y}, {prec}); x = {x}");
    }
}

fn demo_float_sub_prec_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sub_prec_assign_ref(&y, prec);
        println!(
            "x := {:#x}; x.sub_prec_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.sub_round(y, rm)
        );
    }
}

fn demo_float_sub_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_round(y, rm);
        println!(
            "({:#x}).sub_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_30()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.sub_round(y, rm)
        );
    }
}

fn demo_float_sub_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_30()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_round(y, rm);
        println!(
            "({:#x}).sub_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sub_round_val_ref(&{}, {}) = {:?}",
            x_old,
            y,
            rm,
            x.sub_round_val_ref(&y, rm)
        );
    }
}

fn demo_float_sub_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (diff, o) = x.sub_round_val_ref(&y, rm);
        println!(
            "({:#x}).sub_round_val_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y),
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).sub_round_ref_val({}, {}) = {:?}",
            x,
            y_old,
            rm,
            x.sub_round_ref_val(y, rm)
        );
    }
}

fn demo_float_sub_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (diff, o) = x.sub_round_ref_val(y, rm);
        println!(
            "(&{:#x}).sub_round_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_round_ref_ref(&{}, {}) = {:?}",
            x,
            y,
            rm,
            x.sub_round_ref_ref(&y, rm)
        );
    }
}

fn demo_float_sub_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (diff, o) = x.sub_round_ref_ref(&y, rm);
        println!(
            "(&{:#x}).sub_round_ref_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y),
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.sub_round_assign(y, rm);
        println!("x := {x_old}; x.sub_round_assign({y_old}, {rm}); x = {x}");
    }
}

fn demo_float_sub_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_round_assign(y, rm);
        println!(
            "x := {:#x}; x.sub_round_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.sub_round_assign_ref(&y, rm);
        println!("x := {x_old}; x.sub_round_assign({y}, {rm}); x = {x}");
    }
}

fn demo_float_sub_round_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sub_round_assign_ref(&y, rm);
        println!(
            "x := {:#x}; x.sub_round_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.sub_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_sub_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_prec_round(y, prec, rm);
        println!(
            "({:#x}).sub_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.sub_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_sub_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_prec_round(y, prec, rm);
        println!(
            "({:#x}).sub_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_prec_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sub_prec_round(&{}, {}, {}) = {:?}",
            x_old,
            y,
            prec,
            rm,
            x.sub_prec_round_val_ref(&y, prec, rm)
        );
    }
}

fn demo_float_sub_prec_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (diff, o) = x.sub_prec_round_val_ref(&y, prec, rm);
        println!(
            "({:#x}).sub_prec_round_val_ref(&{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_prec_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).sub_prec_round_ref_val({}, {}, {}) = {:?}",
            x,
            y_old,
            prec,
            rm,
            x.sub_prec_round_ref_val(y, prec, rm)
        );
    }
}

fn demo_float_sub_prec_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (diff, o) = x.sub_prec_round_ref_val(y, prec, rm);
        println!(
            "(&{:#x}).sub_prec_round_ref_val({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_prec_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_prec_round({}, {}, {}) = {:?}",
            x,
            y,
            prec,
            rm,
            x.sub_prec_round_ref_ref(&y, prec, rm)
        );
    }
}

fn demo_float_sub_prec_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (diff, o) = x.sub_prec_round_ref_ref(&y, prec, rm);
        println!(
            "({:#x}).sub_prec_round_ref_ref(&{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y),
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_prec_round_assign(y, prec, rm);
        println!("x := {x_old}; x.sub_prec_round({y_old}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_sub_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_prec_round_assign(y, prec, rm);
        println!(
            "x := {:#x}; x.sub_prec_round({:#x}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_prec_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sub_prec_round_assign_ref(&y, prec, rm);
        println!("x := {x_old}; x.sub_prec_round_ref(&{y}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_sub_prec_round_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sub_prec_round_assign_ref(&y, prec, rm);
        println!(
            "x := {:#x}; x.sub_prec_round_ref(&{:#x}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_float_sub_rational_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{:#x} - {} = {:#x}",
            ComparableFloat(x_old),
            y_old,
            ComparableFloat(x - y)
        );
    }
}

fn demo_float_sub_rational_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_float_sub_rational_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{:#x} - {} = {:#x}",
            ComparableFloat(x_old),
            y_old,
            ComparableFloat(x - y)
        );
    }
}

fn demo_float_sub_rational_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

fn demo_float_sub_rational_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "{:#x} - {} = {:#x}",
            ComparableFloat(x_old),
            y,
            ComparableFloat(x - &y)
        );
    }
}

fn demo_float_sub_rational_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

fn demo_float_sub_rational_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "&{:#x} - {} = {:#x}",
            ComparableFloatRef(&x),
            y_old,
            ComparableFloat(&x - y)
        );
    }
}

fn demo_float_sub_rational_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

fn demo_float_sub_rational_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        println!(
            "&{:#x} - &{} = {:#x}",
            ComparableFloatRef(&x),
            y,
            ComparableFloat(&x - &y)
        );
    }
}

fn demo_float_sub_rational_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x += y.clone();
        println!("x := {x_old}; x += {y}; x = {x}");
    }
}

fn demo_float_sub_rational_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x += y.clone();
        println!(
            "x := {:#x}; x += {}; x = {:#x}",
            ComparableFloat(x_old),
            y,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_rational_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x += &y;
        println!("x := {x_old}; x += &{y}; x = {x}");
    }
}

fn demo_float_sub_rational_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x += &y;
        println!(
            "x := {:#x}; x += &{}; x = {:#x}",
            ComparableFloat(x_old),
            y,
            ComparableFloat(x)
        );
    }
}

fn demo_rational_sub_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_rational_sub_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{} - {:#x} = {:#x}",
            x_old,
            ComparableFloat(y_old),
            ComparableFloat(x - y)
        );
    }
}

fn demo_rational_sub_float_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_rational_sub_float_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{} - {:#x} = {:#x}",
            x_old,
            ComparableFloat(y_old),
            ComparableFloat(x - y)
        );
    }
}

fn demo_rational_sub_float_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

fn demo_rational_sub_float_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "{} - &{:#x} = {:#x}",
            x_old,
            ComparableFloatRef(&y),
            ComparableFloat(x - &y)
        );
    }
}

fn demo_rational_sub_float_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

fn demo_rational_sub_float_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "&{} - {:#x} = {:#x}",
            x,
            ComparableFloat(y_old),
            ComparableFloat(&x - y)
        );
    }
}

fn demo_rational_sub_float_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

fn demo_rational_sub_float_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        println!(
            "&{} - &{:#x} = {:#x}",
            x,
            ComparableFloatRef(&y),
            ComparableFloat(&x - &y)
        );
    }
}

fn demo_float_sub_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_rational_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.sub_rational_prec(y, prec)
        );
    }
}

fn demo_float_sub_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_rational_prec(y, prec);
        println!(
            "({:#x}).sub_rational_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y_old,
            prec,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_rational_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_rational_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.sub_rational_prec(y, prec)
        );
    }
}

fn demo_float_sub_rational_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_rational_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_rational_prec(y, prec);
        println!(
            "({:#x}).sub_rational_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y_old,
            prec,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_prec_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sub_rational_prec({}, &{}) = {:?}",
            x_old,
            y,
            prec,
            x.sub_rational_prec_val_ref(&y, prec)
        );
    }
}

fn demo_float_sub_rational_prec_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (diff, o) = x.sub_rational_prec_val_ref(&y, prec);
        println!(
            "({:#x}).sub_rational_prec_val_ref(&{}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y,
            prec,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_prec_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).sub_rational_prec_ref_val({}, {}) = {:?}",
            x,
            y_old,
            prec,
            x.sub_rational_prec_ref_val(y, prec)
        );
    }
}

fn demo_float_sub_rational_prec_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (diff, o) = x.sub_rational_prec_ref_val(y, prec);
        println!(
            "(&{:#x}).sub_rational_prec_ref_val({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            y_old,
            prec,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_prec_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_rational_prec_ref_ref(&{}, {}) = {:?}",
            x,
            y,
            prec,
            x.sub_rational_prec_ref_ref(&y, prec)
        );
    }
}

fn demo_float_sub_rational_prec_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (diff, o) = x.sub_rational_prec_ref_ref(&y, prec);
        println!(
            "(&{:#x}).sub_rational_prec_ref_ref(&{}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            y,
            prec,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_rational_prec_assign(y, prec);
        println!("x := {x_old}; x.sub_rational_prec_assign({y_old}, {prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_sub_rational_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_rational_prec_assign(y, prec);
        println!(
            "x := {:#x}; x.sub_rational_prec_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            y_old,
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_rational_prec_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sub_rational_prec_assign_ref(&y, prec);
        println!("x := {x_old}; x.sub_rational_prec_assign_ref({y}, &{prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_sub_rational_prec_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sub_rational_prec_assign_ref(&y, prec);
        println!(
            "x := {:#x}; x.sub_rational_prec_assign(&{}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            y,
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_rational_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_rational_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.sub_rational_round(y, rm)
        );
    }
}

fn demo_float_sub_rational_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_rational_round(y, rm);
        println!(
            "({:#x}).sub_rational_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y_old,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_rational_rounding_mode_triple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_rational_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.sub_rational_round(y, rm)
        );
    }
}

fn demo_float_sub_rational_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_rational_rounding_mode_triple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_rational_round(y, rm);
        println!(
            "({:#x}).sub_rational_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y_old,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sub_rational_round_val_ref(&{}, {}) = {:?}",
            x_old,
            y,
            rm,
            x.sub_rational_round_val_ref(&y, rm)
        );
    }
}

fn demo_float_sub_rational_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (diff, o) = x.sub_rational_round_val_ref(&y, rm);
        println!(
            "({:#x}).sub_rational_round_val_ref(&{}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).sub_rational_round_ref_val(&{}, {}) = {:?}",
            x,
            y_old,
            rm,
            x.sub_rational_round_ref_val(y, rm)
        );
    }
}

fn demo_float_sub_rational_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (diff, o) = x.sub_rational_round_ref_val(y, rm);
        println!(
            "(&{:#x}).sub_rational_round_ref_val({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            y_old,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_rational_round_ref_ref(&{}, {}) = {:?}",
            x,
            y,
            rm,
            x.sub_rational_round_ref_ref(&y, rm)
        );
    }
}

fn demo_float_sub_rational_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (diff, o) = x.sub_rational_round_ref_ref(&y, rm);
        println!(
            "(&{:#x}).sub_rational_round_ref_ref(&{}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            y,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_rational_round_assign(y, rm);
        println!("x := {x_old}; x.sub_rational_round_assign({y_old}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_sub_rational_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_rational_round_assign(y, rm);
        println!(
            "x := {:#x}; x.sub_rational_round_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            y_old,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_rational_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sub_rational_round_assign_ref(&y, rm);
        println!("x := {x_old}; x.sub_rational_round_assign_ref(&{y}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_sub_rational_round_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_rational_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_rational_round_assign_ref(&y, rm);
        println!(
            "x := {:#x}; x.sub_rational_round_assign_ref(&{}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            y_old,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_rational_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.sub_rational_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_sub_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_rational_prec_round(y, prec, rm);
        println!(
            "({:#x}).sub_rational_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y_old,
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).sub_rational_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.sub_rational_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_sub_rational_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (diff, o) = x.sub_rational_prec_round(y, prec, rm);
        println!(
            "({:#x}).sub_rational_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y_old,
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_prec_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sub_rational_prec_round_val_ref(&{}, {}, {}) = {:?}",
            x_old,
            y,
            prec,
            rm,
            x.sub_rational_prec_round_val_ref(&y, prec, rm)
        );
    }
}

fn demo_float_sub_rational_prec_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (diff, o) = x.sub_rational_prec_round_val_ref(&y, prec, rm);
        println!(
            "({:#x}).sub_rational_prec_round_val_ref(&{}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y,
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_prec_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).sub_rational_prec_round_ref_val({}, {}, {}) = {:?}",
            x,
            y_old,
            prec,
            rm,
            x.sub_rational_prec_round_ref_val(y, prec, rm)
        );
    }
}

fn demo_float_sub_rational_prec_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (diff, o) = x.sub_rational_prec_round_ref_val(y, prec, rm);
        println!(
            "(&{:#x}).sub_rational_prec_round_ref_val({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            y_old,
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_prec_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_rational_prec_round_ref_ref(&{}, {}, {}) = {:?}",
            x,
            y,
            prec,
            rm,
            x.sub_rational_prec_round_ref_ref(&y, prec, rm)
        );
    }
}

fn demo_float_sub_rational_prec_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (diff, o) = x.sub_rational_prec_round_ref_ref(&y, prec, rm);
        println!(
            "(&{:#x}).sub_rational_prec_round_ref_ref(&{}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            y,
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_rational_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_rational_prec_round_assign(y, prec, rm);
        println!(
            "x := {x_old}; x.sub_rational_prec_round_assign({y_old}, {prec}, {rm}) = {o:?}; \
            x = {x}",
        );
    }
}

fn demo_float_sub_rational_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.sub_rational_prec_round_assign(y, prec, rm);
        println!(
            "x := {:#x}; x.sub_rational_prec_round_assign({}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            y_old,
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_rational_prec_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sub_rational_prec_round_assign_ref(&y, prec, rm);
        println!(
            "x := {x_old}; x.sub_rational_prec_round_assign_ref(&{y}, {prec}, {rm}) = {o:?}; \
            x = {x}",
        );
    }
}

fn demo_float_sub_rational_prec_round_assign_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sub_rational_prec_round_assign_ref(&y, prec, rm);
        println!(
            "x := {:#x}; x.sub_rational_prec_round_assign_ref(&{}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            y,
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float - Float",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float - Float", &mut |(x, y)| no_out!(x - y)),
            ("Float - &Float", &mut |(x, y)| no_out!(x - &y)),
            ("&Float - Float", &mut |(x, y)| no_out!(&x - y)),
            ("&Float - &Float", &mut |(x, y)| no_out!(&x - &y)),
        ],
    );
}

#[allow(unused_must_use, clippy::no_effect)]
fn benchmark_float_sub_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float - Float",
        BenchmarkType::LibraryComparison,
        float_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(&x - &y)),
            ("rug", &mut |((x, y), _)| no_out!(rug_sub(&x, &y))),
        ],
    );
}

#[allow(unused_must_use, clippy::no_effect)]
fn benchmark_float_sub_algorithms(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float - Float",
        BenchmarkType::Algorithms,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x - y)),
            ("naive", &mut |(x, y)| {
                let xsb = x.significant_bits();
                let ysb = y.significant_bits();
                no_out!(add_prec_round_naive(x, -y, max(xsb, ysb), Nearest).0)
            }),
        ],
    );
}

fn benchmark_float_sub_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float -= Float",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float -= Float", &mut |(mut x, y)| x -= y),
            ("Float -= &Float", &mut |(mut x, y)| x -= &y),
        ],
    );
}

fn benchmark_float_sub_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_prec(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float.sub_prec(Float, u64)", &mut |(x, y, prec)| {
                no_out!(x.sub_prec(y, prec))
            }),
            ("Float.sub_prec_val_ref(&Float, u64)", &mut |(
                x,
                y,
                prec,
            )| {
                no_out!(x.sub_prec_val_ref(&y, prec))
            }),
            (
                "(&Float).sub_prec_ref_val(Float, u64)",
                &mut |(x, y, prec)| no_out!(x.sub_prec_ref_val(y, prec)),
            ),
            (
                "(&Float).sub_prec_ref_ref(&Float, u64)",
                &mut |(x, y, prec)| no_out!(x.sub_prec_ref_ref(&y, prec)),
            ),
        ],
    );
}

fn benchmark_float_sub_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_prec(Float, u64)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec))| {
                no_out!(x.sub_prec_ref_ref(&y, prec))
            }),
            ("rug", &mut |((x, y, prec), _)| {
                no_out!(rug_sub_prec(&x, &y, prec))
            }),
        ],
    );
}

fn benchmark_float_sub_prec_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_prec(Float, u64)",
        BenchmarkType::Algorithms,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("default", &mut |(x, y, prec)| no_out!(x.sub_prec(y, prec))),
            ("naive", &mut |(x, y, prec)| {
                no_out!(add_prec_round_naive(x, -y, prec, Nearest))
            }),
        ],
    );
}

fn benchmark_float_sub_prec_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_prec_assign(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float.sub_prec_assign(Float, u64)", &mut |(
                mut x,
                y,
                prec,
            )| {
                no_out!(x.sub_prec_assign(y, prec))
            }),
            (
                "Float.sub_prec_assign_ref(&Float, u64)",
                &mut |(mut x, y, prec)| no_out!(x.sub_prec_assign_ref(&y, prec)),
            ),
        ],
    );
}

fn benchmark_float_sub_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_round(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.sub_round(Float, RoundingMode)", &mut |(x, y, rm)| {
                no_out!(x.sub_round(y, rm))
            }),
            (
                "Float.sub_round_val_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.sub_round_val_ref(&y, rm)),
            ),
            (
                "(&Float).sub_round_ref_val(Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.sub_round_ref_val(y, rm)),
            ),
            (
                "(&Float).sub_round_ref_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.sub_round_ref_ref(&y, rm)),
            ),
        ],
    );
}

fn benchmark_float_sub_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_round(Float, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_rounding_mode_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y, rm))| {
                no_out!(x.sub_round_ref_ref(&y, rm))
            }),
            ("rug", &mut |((x, y, rm), _)| {
                no_out!(rug_sub_round(&x, &y, rm))
            }),
        ],
    );
}

fn benchmark_float_sub_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_round(Float, RoundingMode)",
        BenchmarkType::Algorithms,
        float_float_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y, rm)| no_out!(x.sub_round(y, rm))),
            ("naive", &mut |(x, y, rm)| {
                let xsb = x.significant_bits();
                let ysb = y.significant_bits();
                add_prec_round_naive(x, -y, max(xsb, ysb), rm);
            }),
        ],
    );
}

fn benchmark_float_sub_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_round_assign(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            (
                "Float.sub_round_assign(Float, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.sub_round_assign(y, rm)),
            ),
            (
                "Float.sub_round_assign_ref(&Float, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.sub_round_assign_ref(&y, rm)),
            ),
        ],
    );
}

fn benchmark_float_sub_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.sub_prec_round(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.sub_prec_round(y, prec, rm)),
            ),
            (
                "Float.sub_prec_round_val_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.sub_prec_round_val_ref(&y, prec, rm)),
            ),
            (
                "(&Float).sub_prec_round_ref_val(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.sub_prec_round_ref_val(y, prec, rm)),
            ),
            (
                "(&Float).sub_prec_round_ref_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.sub_prec_round_ref_ref(&y, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_sub_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_rounding_mode_quadruple_gen_var_2_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.sub_prec_round_ref_ref(&y, prec, rm))
            }),
            ("rug", &mut |((x, y, prec, rm), _)| {
                no_out!(rug_sub_prec_round(&x, &y, prec, rm))
            }),
        ],
    );
}

fn benchmark_float_sub_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_float_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("default", &mut |(x, y, prec, rm)| {
                no_out!(x.sub_prec_round(y, prec, rm))
            }),
            ("naive", &mut |(x, y, prec, rm)| {
                no_out!(add_prec_round_naive(x, -y, prec, rm))
            }),
        ],
    );
}

fn benchmark_float_sub_prec_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_prec_round_assign(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.sub_prec_round_assign(Float, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| no_out!(x.sub_prec_round_assign(y, prec, rm)),
            ),
            (
                "Float.sub_prec_round_assign_ref(&Float, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| no_out!(x.sub_prec_round_assign_ref(&y, prec, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use, clippy::no_effect)]
fn benchmark_float_sub_rational_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float - Rational",
        BenchmarkType::EvaluationStrategy,
        float_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float - Rational", &mut |(x, y)| no_out!(x - y)),
            ("Float - &Rational", &mut |(x, y)| no_out!(x - &y)),
            ("&Float - Rational", &mut |(x, y)| no_out!(&x - y)),
            ("&Float - &Rational", &mut |(x, y)| no_out!(&x - &y)),
        ],
    );
}

#[allow(unused_must_use, clippy::no_effect)]
fn benchmark_float_sub_rational_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float - Rational",
        BenchmarkType::LibraryComparison,
        float_rational_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(&x - &y)),
            ("rug", &mut |((x, y), _)| no_out!(rug_sub_rational(&x, &y))),
        ],
    );
}

#[allow(unused_must_use, clippy::no_effect)]
fn benchmark_float_sub_rational_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float - Rational",
        BenchmarkType::Algorithms,
        float_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x - y)),
            ("naive", &mut |(x, y)| {
                let xsb = x.significant_bits();
                let ysb = y.significant_bits();
                no_out!(sub_rational_prec_round_naive(x, y, max(xsb, ysb), Nearest).0)
            }),
        ],
    );
}

fn benchmark_float_sub_rational_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float -= Rational",
        BenchmarkType::EvaluationStrategy,
        float_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float -= Rational", &mut |(mut x, y)| x -= y),
            ("Float -= &Rational", &mut |(mut x, y)| x -= &y),
        ],
    );
}

#[allow(unused_must_use, clippy::no_effect)]
fn benchmark_rational_sub_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational - Float",
        BenchmarkType::EvaluationStrategy,
        float_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("Rational - Float", &mut |(y, x)| no_out!(x - y)),
            ("Rational - &Float", &mut |(y, x)| no_out!(x - &y)),
            ("&Rational - Float", &mut |(y, x)| no_out!(&x - y)),
            ("&Rational - &Float", &mut |(y, x)| no_out!(&x - &y)),
        ],
    );
}

#[allow(unused_must_use, clippy::no_effect)]
fn benchmark_rational_sub_float_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational - Float",
        BenchmarkType::LibraryComparison,
        float_rational_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (y, x))| no_out!(&x - &y)),
            ("rug", &mut |((y, x), _)| no_out!(-rug_sub_rational(&y, &x))),
        ],
    );
}

fn benchmark_float_sub_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        float_rational_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_rational_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.sub_rational_prec(Rational, u64)",
                &mut |(x, y, prec)| no_out!(x.sub_rational_prec(y, prec)),
            ),
            (
                "Float.sub_rational_prec_val_ref(&Rational, u64)",
                &mut |(x, y, prec)| no_out!(x.sub_rational_prec_val_ref(&y, prec)),
            ),
            (
                "(&Float).sub_rational_prec_ref_val(Rational, u64)",
                &mut |(x, y, prec)| no_out!(x.sub_rational_prec_ref_val(y, prec)),
            ),
            (
                "(&Float).sub_rational_prec_ref_ref(&Rational, u64)",
                &mut |(x, y, prec)| no_out!(x.sub_rational_prec_ref_ref(&y, prec)),
            ),
        ],
    );
}

fn benchmark_float_sub_rational_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_prec(Rational, u64)",
        BenchmarkType::LibraryComparison,
        float_rational_unsigned_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_float_rational_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec))| {
                no_out!(x.sub_rational_prec_ref_ref(&y, prec))
            }),
            ("rug", &mut |((x, y, prec), _)| {
                no_out!(rug_sub_rational_prec(&x, &y, prec))
            }),
        ],
    );
}

fn benchmark_float_sub_rational_prec_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_prec(Rational, u64)",
        BenchmarkType::Algorithms,
        float_rational_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_rational_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("default", &mut |(x, y, prec)| {
                no_out!(x.sub_rational_prec(y, prec))
            }),
            ("naive", &mut |(x, y, prec)| {
                no_out!(sub_rational_prec_round_naive(x, y, prec, Nearest))
            }),
        ],
    );
}

fn benchmark_float_sub_rational_prec_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_prec_assign(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        float_rational_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_rational_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.sub_rational_prec_assign(Rational, u64)",
                &mut |(mut x, y, prec)| no_out!(x.sub_rational_prec_assign(y, prec)),
            ),
            (
                "Float.sub_rational_prec_assign_ref(&Rational, u64)",
                &mut |(mut x, y, prec)| no_out!(x.sub_rational_prec_assign_ref(&y, prec)),
            ),
        ],
    );
}

fn benchmark_float_sub_rational_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_round(Rational, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rational_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            (
                "Float.sub_rational_round(Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.sub_rational_round(y, rm)),
            ),
            (
                "Float.sub_rational_round_val_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.sub_rational_round_val_ref(&y, rm)),
            ),
            (
                "(&Float).sub_rational_round_ref_val(Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.sub_rational_round_ref_val(y, rm)),
            ),
            (
                "(&Float).sub_rational_round_ref_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.sub_rational_round_ref_ref(&y, rm)),
            ),
        ],
    );
}

fn benchmark_float_sub_rational_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_round(Rational, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_rational_rounding_mode_triple_gen_var_3_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y, rm))| {
                no_out!(x.sub_rational_round_ref_ref(&y, rm))
            }),
            ("rug", &mut |((x, y, rm), _)| {
                no_out!(rug_sub_rational_round(&x, &y, rm))
            }),
        ],
    );
}

fn benchmark_float_sub_rational_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_round(Rational, RoundingMode)",
        BenchmarkType::Algorithms,
        float_rational_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y, rm)| {
                no_out!(x.sub_rational_round(y, rm))
            }),
            ("naive", &mut |(x, y, rm)| {
                let ysb = y.significant_bits();
                sub_rational_prec_round_naive(x, y, ysb, rm);
            }),
        ],
    );
}

fn benchmark_float_sub_rational_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_round_assign(Rational, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rational_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            (
                "Float.sub_rational_round_assign(Rational, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.sub_rational_round_assign(y, rm)),
            ),
            (
                "Float.sub_rational_round_assign_ref(&Rational, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.sub_rational_round_assign_ref(&y, rm)),
            ),
        ],
    );
}

fn benchmark_float_sub_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rational_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.sub_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.sub_rational_prec_round(y, prec, rm)),
            ),
            (
                "Float.sub_rational_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.sub_rational_prec_round_val_ref(&y, prec, rm)),
            ),
            (
                "(&Float).sub_rational_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.sub_rational_prec_round_ref_val(y, prec, rm)),
            ),
            (
                "(&Float).sub_rational_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.sub_rational_prec_round_ref_ref(&y, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_sub_rational_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_rational_unsigned_rounding_mode_quadruple_gen_var_2_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.sub_rational_prec_round_ref_ref(&y, prec, rm))
            }),
            ("rug", &mut |((x, y, prec, rm), _)| {
                no_out!(rug_sub_rational_prec_round(&x, &y, prec, rm))
            }),
        ],
    );
}

fn benchmark_float_sub_rational_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_rational_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("default", &mut |(x, y, prec, rm)| {
                no_out!(x.sub_rational_prec_round(y, prec, rm))
            }),
            ("naive", &mut |(x, y, prec, rm)| {
                no_out!(sub_rational_prec_round_naive(x, y, prec, rm))
            }),
        ],
    );
}

fn benchmark_float_sub_rational_prec_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_rational_prec_round_assign(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rational_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.sub_rational_prec_round_assign(Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| no_out!(x.sub_rational_prec_round_assign(y, prec, rm)),
            ),
            (
                "Float.sub_rational_prec_round_assign_ref(&Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.sub_rational_prec_round_assign_ref(&y, prec, rm))
                },
            ),
        ],
    );
}
