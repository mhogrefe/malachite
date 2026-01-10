// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Agm, AgmAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::pair_max_primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::arithmetic::agm::primitive_float_agm;
use malachite_float::test_util::arithmetic::agm::{
    rug_agm, rug_agm_prec, rug_agm_prec_round, rug_agm_round,
};
use malachite_float::test_util::bench::bucketers::{
    pair_2_pair_float_max_complexity_bucketer,
    pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    pair_2_triple_1_2_float_max_complexity_bucketer,
    pair_2_triple_float_float_primitive_int_max_complexity_bucketer,
    pair_float_max_complexity_bucketer,
    quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_max_complexity_bucketer,
    triple_float_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_33, float_float_rounding_mode_triple_gen_var_33_rm,
    float_float_rounding_mode_triple_gen_var_34,
    float_float_unsigned_rounding_mode_quadruple_gen_var_9,
    float_float_unsigned_rounding_mode_quadruple_gen_var_9_rm,
    float_float_unsigned_rounding_mode_quadruple_gen_var_10, float_float_unsigned_triple_gen_var_1,
    float_float_unsigned_triple_gen_var_1_rm, float_float_unsigned_triple_gen_var_2,
    float_pair_gen, float_pair_gen_rm, float_pair_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_agm);
    register_demo!(runner, demo_float_agm_debug);
    register_demo!(runner, demo_float_agm_extreme);
    register_demo!(runner, demo_float_agm_extreme_debug);
    register_demo!(runner, demo_float_agm_val_ref);
    register_demo!(runner, demo_float_agm_val_ref_debug);
    register_demo!(runner, demo_float_agm_ref_val);
    register_demo!(runner, demo_float_agm_ref_val_debug);
    register_demo!(runner, demo_float_agm_ref_ref);
    register_demo!(runner, demo_float_agm_ref_ref_debug);
    register_demo!(runner, demo_float_agm_assign);
    register_demo!(runner, demo_float_agm_assign_debug);
    register_demo!(runner, demo_float_agm_assign_ref);
    register_demo!(runner, demo_float_agm_assign_ref_debug);
    register_demo!(runner, demo_float_agm_prec);
    register_demo!(runner, demo_float_agm_prec_debug);
    register_demo!(runner, demo_float_agm_prec_extreme);
    register_demo!(runner, demo_float_agm_prec_extreme_debug);
    register_demo!(runner, demo_float_agm_prec_val_ref);
    register_demo!(runner, demo_float_agm_prec_val_ref_debug);
    register_demo!(runner, demo_float_agm_prec_ref_val);
    register_demo!(runner, demo_float_agm_prec_ref_val_debug);
    register_demo!(runner, demo_float_agm_prec_ref_ref);
    register_demo!(runner, demo_float_agm_prec_ref_ref_debug);
    register_demo!(runner, demo_float_agm_prec_assign);
    register_demo!(runner, demo_float_agm_prec_assign_debug);
    register_demo!(runner, demo_float_agm_prec_assign_ref);
    register_demo!(runner, demo_float_agm_prec_assign_ref_debug);
    register_demo!(runner, demo_float_agm_round);
    register_demo!(runner, demo_float_agm_round_debug);
    register_demo!(runner, demo_float_agm_round_extreme);
    register_demo!(runner, demo_float_agm_round_extreme_debug);
    register_demo!(runner, demo_float_agm_round_val_ref);
    register_demo!(runner, demo_float_agm_round_val_ref_debug);
    register_demo!(runner, demo_float_agm_round_ref_val);
    register_demo!(runner, demo_float_agm_round_ref_val_debug);
    register_demo!(runner, demo_float_agm_round_ref_ref);
    register_demo!(runner, demo_float_agm_round_ref_ref_debug);
    register_demo!(runner, demo_float_agm_round_assign);
    register_demo!(runner, demo_float_agm_round_assign_debug);
    register_demo!(runner, demo_float_agm_round_assign_ref);
    register_demo!(runner, demo_float_agm_round_assign_ref_debug);
    register_demo!(runner, demo_float_agm_prec_round);
    register_demo!(runner, demo_float_agm_prec_round_debug);
    register_demo!(runner, demo_float_agm_prec_round_extreme);
    register_demo!(runner, demo_float_agm_prec_round_extreme_debug);
    register_demo!(runner, demo_float_agm_prec_round_val_ref);
    register_demo!(runner, demo_float_agm_prec_round_val_ref_debug);
    register_demo!(runner, demo_float_agm_prec_round_ref_val);
    register_demo!(runner, demo_float_agm_prec_round_ref_val_debug);
    register_demo!(runner, demo_float_agm_prec_round_ref_ref);
    register_demo!(runner, demo_float_agm_prec_round_ref_ref_debug);
    register_demo!(runner, demo_float_agm_prec_round_assign);
    register_demo!(runner, demo_float_agm_prec_round_assign_debug);
    register_demo!(runner, demo_float_agm_prec_round_assign_ref);
    register_demo!(runner, demo_float_agm_prec_round_assign_ref_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_agm);

    register_bench!(runner, benchmark_float_agm_evaluation_strategy);
    register_bench!(runner, benchmark_float_agm_library_comparison);
    register_bench!(runner, benchmark_float_agm_assign_evaluation_strategy);
    register_bench!(runner, benchmark_float_agm_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_agm_prec_library_comparison);
    register_bench!(runner, benchmark_float_agm_prec_assign_evaluation_strategy);
    register_bench!(runner, benchmark_float_agm_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_agm_round_library_comparison);
    register_bench!(runner, benchmark_float_agm_round_assign_evaluation_strategy);
    register_bench!(runner, benchmark_float_agm_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_agm_prec_round_library_comparison);
    register_bench!(
        runner,
        benchmark_float_agm_prec_round_assign_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_agm);
}

fn demo_float_agm(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("agm({}, {}) = {}", x_old, y_old, x.agm(y));
    }
}

fn demo_float_agm_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "agm({:#x}, {:#x}) = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x.agm(y))
        );
    }
}

fn demo_float_agm_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("agm({}, {}) = {}", x_old, y_old, x.agm(y));
    }
}

fn demo_float_agm_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "agm({:#x}, {:#x}) = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x.agm(y))
        );
    }
}

fn demo_float_agm_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("agm({}, &{}) = {}", x_old, y, x.agm(&y));
    }
}

fn demo_float_agm_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "agm({:#x}, &{:#x}) = {:#x}",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            ComparableFloat(x.agm(&y))
        );
    }
}

fn demo_float_agm_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("agm(&{}, {}) = {}", x, y_old, (&x).agm(y));
    }
}

fn demo_float_agm_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "agm(&{:#x}, {:#x}) = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat(y_old),
            ComparableFloat((&x).agm(y))
        );
    }
}

fn demo_float_agm_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        println!("agm(&{}, &{}) = {}", x, y, (&x).agm(&y));
    }
}

fn demo_float_agm_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        println!(
            "agm(&{:#x}, &{:#x}) = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloatRef(&y),
            ComparableFloat((&x).agm(&y))
        );
    }
}

fn demo_float_agm_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.agm_assign(y.clone());
        println!("x := {x_old}; x.agm_assign({y}); x = {x}");
    }
}

fn demo_float_agm_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.agm_assign(y.clone());
        println!(
            "x := {:#x}; x.agm_assign({:#x}); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            ComparableFloat(x)
        );
    }
}

fn demo_float_agm_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.agm_assign(&y);
        println!("x := {x_old}; x.agm_assign(&{y}); x = {x}");
    }
}

fn demo_float_agm_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.agm_assign(&y);
        println!(
            "x := {:#x}; x.agm_assign(&{:#x}); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            ComparableFloat(x)
        );
    }
}

fn demo_float_agm_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).agm_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.agm_prec(y, prec)
        );
    }
}

fn demo_float_agm_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (sum, o) = x.agm_prec(y, prec);
        println!(
            "({:#x}).agm_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).agm_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.agm_prec(y, prec)
        );
    }
}

fn demo_float_agm_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (sum, o) = x.agm_prec(y, prec);
        println!(
            "({:#x}).agm_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_prec_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).agm_prec_val_ref(&{}, {}) = {:?}",
            x_old,
            y,
            prec,
            x.agm_prec_val_ref(&y, prec)
        );
    }
}

fn demo_float_agm_prec_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.agm_prec_val_ref(&y, prec);
        println!(
            "({:#x}).agm_prec_val_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_prec_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).agm_prec_ref_val({}, {}) = {:?}",
            x,
            y_old,
            prec,
            x.agm_prec_ref_val(y, prec)
        );
    }
}

fn demo_float_agm_prec_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (sum, o) = x.agm_prec_ref_val(y, prec);
        println!(
            "(&{:#x}).agm_prec_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_prec_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).agm_prec_ref_ref(&{}, {}) = {:?}",
            x,
            y,
            prec,
            x.agm_prec_ref_ref(&y, prec)
        );
    }
}

fn demo_float_agm_prec_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.agm_prec_ref_ref(&y, prec);
        println!(
            "(&{:#x}).agm_prec_ref_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.agm_prec_assign(y, prec);
        println!("x := {x_old}; x.agm_prec_assign({y_old}, {prec}); x = {x}");
    }
}

fn demo_float_agm_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.agm_prec_assign(y, prec);
        println!(
            "x := {:#x}; x.agm_prec_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_agm_prec_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.agm_prec_assign_ref(&y, prec);
        println!("x := {x_old}; x.agm_prec_assign({y}, {prec}); x = {x}");
    }
}

fn demo_float_agm_prec_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.agm_prec_assign_ref(&y, prec);
        println!(
            "x := {:#x}; x.agm_prec_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_agm_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).agm_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.agm_round(y, rm)
        );
    }
}

fn demo_float_agm_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (sum, o) = x.agm_round(y, rm);
        println!(
            "({:#x}).agm_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_34()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).agm_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.agm_round(y, rm)
        );
    }
}

fn demo_float_agm_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_34()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (sum, o) = x.agm_round(y, rm);
        println!(
            "({:#x}).agm_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).agm_round_val_ref(&{}, {}) = {:?}",
            x_old,
            y,
            rm,
            x.agm_round_val_ref(&y, rm)
        );
    }
}

fn demo_float_agm_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.agm_round_val_ref(&y, rm);
        println!(
            "({:#x}).agm_round_val_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).agm_round_ref_val({}, {}) = {:?}",
            x,
            y_old,
            rm,
            x.agm_round_ref_val(y, rm)
        );
    }
}

fn demo_float_agm_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (sum, o) = x.agm_round_ref_val(y, rm);
        println!(
            "(&{:#x}).agm_round_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).agm_round_ref_ref(&{}, {}) = {:?}",
            x,
            y,
            rm,
            x.agm_round_ref_ref(&y, rm)
        );
    }
}

fn demo_float_agm_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.agm_round_ref_ref(&y, rm);
        println!(
            "(&{:#x}).agm_round_ref_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.agm_round_assign(y, rm);
        println!("x := {x_old}; x.agm_round_assign({y_old}, {rm}); x = {x}");
    }
}

fn demo_float_agm_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.agm_round_assign(y, rm);
        println!(
            "x := {:#x}; x.agm_round_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_agm_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.agm_round_assign_ref(&y, rm);
        println!("x := {x_old}; x.agm_round_assign({y}, {rm}); x = {x}");
    }
}

fn demo_float_agm_round_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.agm_round_assign_ref(&y, rm);
        println!(
            "x := {:#x}; x.agm_round_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_agm_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).agm_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.agm_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_agm_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (sum, o) = x.agm_prec_round(y, prec, rm);
        println!(
            "({:#x}).agm_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).agm_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.agm_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_agm_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (sum, o) = x.agm_prec_round(y, prec, rm);
        println!(
            "({:#x}).agm_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_prec_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).agm_prec_round(&{}, {}, {}) = {:?}",
            x_old,
            y,
            prec,
            rm,
            x.agm_prec_round_val_ref(&y, prec, rm)
        );
    }
}

fn demo_float_agm_prec_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.agm_prec_round_val_ref(&y, prec, rm);
        println!(
            "({:#x}).agm_prec_round_val_ref(&{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_prec_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).agm_prec_round_ref_val({}, {}, {}) = {:?}",
            x,
            y_old,
            prec,
            rm,
            x.agm_prec_round_ref_val(y, prec, rm)
        );
    }
}

fn demo_float_agm_prec_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (sum, o) = x.agm_prec_round_ref_val(y, prec, rm);
        println!(
            "(&{:#x}).agm_prec_round_ref_val({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_prec_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).agm_prec_round({}, {}, {}) = {:?}",
            x,
            y,
            prec,
            rm,
            x.agm_prec_round_ref_ref(&y, prec, rm)
        );
    }
}

fn demo_float_agm_prec_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.agm_prec_round_ref_ref(&y, prec, rm);
        println!(
            "({:#x}).agm_prec_round_ref_ref(&{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_agm_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.agm_prec_round_assign(y, prec, rm);
        println!("x := {x_old}; x.agm_prec_round({y_old}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_agm_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.agm_prec_round_assign(y, prec, rm);
        println!(
            "x := {:#x}; x.agm_prec_round({:#x}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_agm_prec_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.agm_prec_round_assign_ref(&y, prec, rm);
        println!("x := {x_old}; x.agm_prec_round_ref(&{y}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_agm_prec_round_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.agm_prec_round_assign_ref(&y, prec, rm);
        println!(
            "x := {:#x}; x.agm_prec_round_ref(&{:#x}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_agm<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, y) in primitive_float_pair_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_agm({}, {}) = {}",
            NiceFloat(x),
            NiceFloat(y),
            NiceFloat(primitive_float_agm(x, y))
        );
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_agm_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm(Float)",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.agm(Float)", &mut |(x, y)| no_out!(x.agm(y))),
            ("Float.agm(&Float)", &mut |(x, y)| no_out!(x.agm(&y))),
            ("(&Float).agm(Float)", &mut |(x, y)| no_out!((&x).agm(y))),
            ("(&Float).agm(&Float)", &mut |(x, y)| no_out!((&x).agm(&y))),
        ],
    );
}

#[allow(unused_must_use, clippy::no_effect)]
fn benchmark_float_agm_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm(Float)",
        BenchmarkType::LibraryComparison,
        float_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!((&x).agm(&y))),
            ("rug", &mut |((x, y), _)| no_out!(rug_agm(&x, &y))),
        ],
    );
}

fn benchmark_float_agm_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm_assign(Float)",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.agm_assign(Float)", &mut |(mut x, y)| x.agm_assign(y)),
            ("Float.agm_assign(&Float)", &mut |(mut x, y)| {
                x.agm_assign(&y);
            }),
        ],
    );
}

fn benchmark_float_agm_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm_prec(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float.agm_prec(Float, u64)", &mut |(x, y, prec)| {
                no_out!(x.agm_prec(y, prec));
            }),
            ("Float.agm_prec_val_ref(&Float, u64)", &mut |(
                x,
                y,
                prec,
            )| {
                no_out!(x.agm_prec_val_ref(&y, prec));
            }),
            (
                "(&Float).agm_prec_ref_val(Float, u64)",
                &mut |(x, y, prec)| no_out!(x.agm_prec_ref_val(y, prec)),
            ),
            (
                "(&Float).agm_prec_ref_ref(&Float, u64)",
                &mut |(x, y, prec)| no_out!(x.agm_prec_ref_ref(&y, prec)),
            ),
        ],
    );
}

fn benchmark_float_agm_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm_prec(Float, u64)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec))| {
                no_out!(x.agm_prec_ref_ref(&y, prec));
            }),
            ("rug", &mut |((x, y, prec), _)| {
                no_out!(rug_agm_prec(&x, &y, prec));
            }),
        ],
    );
}

fn benchmark_float_agm_prec_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm_prec_assign(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float.agm_prec_assign(Float, u64)", &mut |(
                mut x,
                y,
                prec,
            )| {
                no_out!(x.agm_prec_assign(y, prec));
            }),
            (
                "Float.agm_prec_assign_ref(&Float, u64)",
                &mut |(mut x, y, prec)| no_out!(x.agm_prec_assign_ref(&y, prec)),
            ),
        ],
    );
}

fn benchmark_float_agm_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm_round(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_33().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.agm_round(Float, RoundingMode)", &mut |(x, y, rm)| {
                no_out!(x.agm_round(y, rm));
            }),
            (
                "Float.agm_round_val_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.agm_round_val_ref(&y, rm)),
            ),
            (
                "(&Float).agm_round_ref_val(Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.agm_round_ref_val(y, rm)),
            ),
            (
                "(&Float).agm_round_ref_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.agm_round_ref_ref(&y, rm)),
            ),
        ],
    );
}

fn benchmark_float_agm_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm_round(Float, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_rounding_mode_triple_gen_var_33_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y, rm))| {
                no_out!(x.agm_round_ref_ref(&y, rm));
            }),
            ("rug", &mut |((x, y, rm), _)| {
                no_out!(rug_agm_round(&x, &y, rm));
            }),
        ],
    );
}

fn benchmark_float_agm_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm_round_assign(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_33().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            (
                "Float.agm_round_assign(Float, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.agm_round_assign(y, rm)),
            ),
            (
                "Float.agm_round_assign_ref(&Float, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.agm_round_assign_ref(&y, rm)),
            ),
        ],
    );
}

fn benchmark_float_agm_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.agm_prec_round(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.agm_prec_round(y, prec, rm)),
            ),
            (
                "Float.agm_prec_round_val_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.agm_prec_round_val_ref(&y, prec, rm)),
            ),
            (
                "(&Float).agm_prec_round_ref_val(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.agm_prec_round_ref_val(y, prec, rm)),
            ),
            (
                "(&Float).agm_prec_round_ref_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.agm_prec_round_ref_ref(&y, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_agm_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_rounding_mode_quadruple_gen_var_9_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.agm_prec_round_ref_ref(&y, prec, rm));
            }),
            ("rug", &mut |((x, y, prec, rm), _)| {
                no_out!(rug_agm_prec_round(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_agm_prec_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.agm_prec_round_assign(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.agm_prec_round_assign(Float, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| no_out!(x.agm_prec_round_assign(y, prec, rm)),
            ),
            (
                "Float.agm_prec_round_assign_ref(&Float, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| no_out!(x.agm_prec_round_assign_ref(&y, prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_agm<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_agm({})", T::NAME),
        BenchmarkType::EvaluationStrategy,
        primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_primitive_float_bucketer("x", "y"),
        &mut [("malachite", &mut |(x, y)| {
            no_out!(primitive_float_agm(x, y));
        })],
    );
}
