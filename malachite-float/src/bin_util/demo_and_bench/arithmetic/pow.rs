// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::Exact;
use malachite_base::test_util::bench::bucketers::{
    pair_2_bucketer, pair_max_primitive_float_bucketer, quadruple_3_bucketer, triple_3_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_pair_gen, primitive_float_unsigned_pair_gen_var_1,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::Float;
use malachite_float::arithmetic::pow::{
    primitive_float_pow, primitive_float_pow_integer, primitive_float_pow_u,
    primitive_float_rational_pow,
};
use malachite_float::test_util::arithmetic::pow::{
    rug_pow, rug_pow_prec, rug_pow_prec_round, rug_pow_round,
};
use malachite_float::test_util::bench::bucketers::{
    pair_2_pair_float_max_complexity_bucketer,
    pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    pair_2_triple_1_2_float_max_complexity_bucketer,
    pair_2_triple_float_float_primitive_int_max_complexity_bucketer,
    pair_float_integer_max_complexity_bucketer, pair_float_max_complexity_bucketer,
    pair_float_signed_max_complexity_bucketer, pair_float_unsigned_max_complexity_bucketer,
    quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    quadruple_1_2_float_integer_max_complexity_bucketer, quadruple_1_float_complexity_bucketer,
    triple_1_2_float_integer_max_complexity_bucketer, triple_1_2_float_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_float_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_33, float_float_rounding_mode_triple_gen_var_33_rm,
    float_float_rounding_mode_triple_gen_var_34,
    float_float_unsigned_rounding_mode_quadruple_gen_var_9,
    float_float_unsigned_rounding_mode_quadruple_gen_var_9_rm,
    float_float_unsigned_rounding_mode_quadruple_gen_var_10, float_float_unsigned_triple_gen_var_1,
    float_float_unsigned_triple_gen_var_1_rm, float_float_unsigned_triple_gen_var_2,
    float_integer_pair_gen, float_integer_unsigned_rounding_mode_quadruple_gen_var_1,
    float_integer_unsigned_triple_gen_var_1, float_pair_gen, float_pair_gen_rm,
    float_pair_gen_var_10, float_rational_unsigned_rounding_mode_quadruple_gen_var_1,
    float_rational_unsigned_triple_gen_var_1, float_signed_pair_gen,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_11,
    float_signed_unsigned_triple_gen_var_1, float_unsigned_pair_gen,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9,
    float_unsigned_unsigned_triple_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};
use malachite_nz::test_util::bench::bucketers::pair_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_primitive_float_pair_gen;
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::rational_primitive_float_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_rational_pow_prec_round);
    register_demo!(runner, demo_float_rational_pow_prec);
    register_primitive_float_demos!(runner, demo_primitive_float_pow);
    register_primitive_float_demos!(runner, demo_primitive_float_rational_pow);
    register_primitive_float_demos!(runner, demo_primitive_float_pow_integer);
    register_primitive_float_demos!(runner, demo_primitive_float_pow_u);
    register_demo!(runner, demo_float_pow);
    register_demo!(runner, demo_float_pow_s_prec_round);
    register_demo!(runner, demo_float_pow_s_prec_round_debug);
    register_demo!(runner, demo_float_pow_s_prec);
    register_demo!(runner, demo_float_pow_s_prec_debug);
    register_demo!(runner, demo_float_pow_s_round);
    register_demo!(runner, demo_float_pow_s_round_debug);
    register_demo!(runner, demo_float_pow_s);
    register_demo!(runner, demo_float_pow_s_debug);
    register_demo!(runner, demo_float_pow_s_assign);
    register_bench!(runner, benchmark_float_pow_s_prec_round);
    register_bench!(runner, benchmark_float_pow_s_prec);
    register_bench!(runner, benchmark_float_pow_s_evaluation_strategy);
    register_demo!(runner, demo_float_pow_u_prec_round);
    register_demo!(runner, demo_float_pow_u_prec_round_debug);
    register_demo!(runner, demo_float_pow_u_prec);
    register_demo!(runner, demo_float_pow_u_prec_debug);
    register_demo!(runner, demo_float_pow_u_round);
    register_demo!(runner, demo_float_pow_u_round_debug);
    register_demo!(runner, demo_float_pow_u);
    register_demo!(runner, demo_float_pow_u_debug);
    register_demo!(runner, demo_float_pow_u_assign);
    register_bench!(runner, benchmark_float_pow_u_prec_round);
    register_bench!(runner, benchmark_float_pow_u_prec);
    register_bench!(runner, benchmark_float_pow_u_evaluation_strategy);
    register_demo!(runner, demo_float_pow_integer_prec_round);
    register_demo!(runner, demo_float_pow_integer_prec_round_debug);
    register_demo!(runner, demo_float_pow_integer_prec);
    register_demo!(runner, demo_float_pow_integer_prec_debug);
    register_demo!(runner, demo_float_pow_integer_round);
    register_demo!(runner, demo_float_pow_integer_round_debug);
    register_demo!(runner, demo_float_pow_integer);
    register_demo!(runner, demo_float_pow_integer_debug);
    register_demo!(runner, demo_float_pow_integer_assign);
    register_bench!(runner, benchmark_float_pow_integer_prec_round);
    register_bench!(runner, benchmark_float_pow_integer_prec);
    register_bench!(runner, benchmark_float_pow_integer_evaluation_strategy);
    register_demo!(runner, demo_float_pow_debug);
    register_demo!(runner, demo_float_pow_extreme);
    register_demo!(runner, demo_float_pow_extreme_debug);
    register_demo!(runner, demo_float_pow_val_ref);
    register_demo!(runner, demo_float_pow_val_ref_debug);
    register_demo!(runner, demo_float_pow_ref_val);
    register_demo!(runner, demo_float_pow_ref_val_debug);
    register_demo!(runner, demo_float_pow_ref_ref);
    register_demo!(runner, demo_float_pow_ref_ref_debug);
    register_demo!(runner, demo_float_pow_assign);
    register_demo!(runner, demo_float_pow_assign_debug);
    register_demo!(runner, demo_float_pow_assign_ref);
    register_demo!(runner, demo_float_pow_assign_ref_debug);
    register_demo!(runner, demo_float_pow_prec);
    register_demo!(runner, demo_float_pow_prec_debug);
    register_demo!(runner, demo_float_pow_prec_extreme);
    register_demo!(runner, demo_float_pow_prec_extreme_debug);
    register_demo!(runner, demo_float_pow_prec_val_ref);
    register_demo!(runner, demo_float_pow_prec_val_ref_debug);
    register_demo!(runner, demo_float_pow_prec_ref_val);
    register_demo!(runner, demo_float_pow_prec_ref_val_debug);
    register_demo!(runner, demo_float_pow_prec_ref_ref);
    register_demo!(runner, demo_float_pow_prec_ref_ref_debug);
    register_demo!(runner, demo_float_pow_prec_assign);
    register_demo!(runner, demo_float_pow_prec_assign_debug);
    register_demo!(runner, demo_float_pow_prec_assign_ref);
    register_demo!(runner, demo_float_pow_prec_assign_ref_debug);
    register_demo!(runner, demo_float_pow_round);
    register_demo!(runner, demo_float_pow_round_debug);
    register_demo!(runner, demo_float_pow_round_extreme);
    register_demo!(runner, demo_float_pow_round_extreme_debug);
    register_demo!(runner, demo_float_pow_round_val_ref);
    register_demo!(runner, demo_float_pow_round_val_ref_debug);
    register_demo!(runner, demo_float_pow_round_ref_val);
    register_demo!(runner, demo_float_pow_round_ref_val_debug);
    register_demo!(runner, demo_float_pow_round_ref_ref);
    register_demo!(runner, demo_float_pow_round_ref_ref_debug);
    register_demo!(runner, demo_float_pow_round_assign);
    register_demo!(runner, demo_float_pow_round_assign_debug);
    register_demo!(runner, demo_float_pow_round_assign_ref);
    register_demo!(runner, demo_float_pow_round_assign_ref_debug);
    register_demo!(runner, demo_float_pow_prec_round);
    register_demo!(runner, demo_float_pow_prec_round_debug);
    register_demo!(runner, demo_float_pow_prec_round_extreme);
    register_demo!(runner, demo_float_pow_prec_round_extreme_debug);
    register_demo!(runner, demo_float_pow_prec_round_val_ref);
    register_demo!(runner, demo_float_pow_prec_round_val_ref_debug);
    register_demo!(runner, demo_float_pow_prec_round_ref_val);
    register_demo!(runner, demo_float_pow_prec_round_ref_val_debug);
    register_demo!(runner, demo_float_pow_prec_round_ref_ref);
    register_demo!(runner, demo_float_pow_prec_round_ref_ref_debug);
    register_demo!(runner, demo_float_pow_prec_round_assign);
    register_demo!(runner, demo_float_pow_prec_round_assign_debug);
    register_demo!(runner, demo_float_pow_prec_round_assign_ref);
    register_demo!(runner, demo_float_pow_prec_round_assign_ref_debug);

    register_bench!(runner, benchmark_float_rational_pow_prec_round);
    register_bench!(runner, benchmark_float_rational_pow_prec);
    register_primitive_float_benches!(runner, benchmark_primitive_float_pow);
    register_primitive_float_benches!(runner, benchmark_primitive_float_rational_pow);
    register_primitive_float_benches!(runner, benchmark_primitive_float_pow_integer);
    register_primitive_float_benches!(runner, benchmark_primitive_float_pow_u);
    register_bench!(runner, benchmark_float_pow_evaluation_strategy);
    register_bench!(runner, benchmark_float_pow_library_comparison);
    register_bench!(runner, benchmark_float_pow_assign_evaluation_strategy);
    register_bench!(runner, benchmark_float_pow_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_pow_prec_library_comparison);
    register_bench!(runner, benchmark_float_pow_prec_assign_evaluation_strategy);
    register_bench!(runner, benchmark_float_pow_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_pow_round_library_comparison);
    register_bench!(runner, benchmark_float_pow_round_assign_evaluation_strategy);
    register_bench!(runner, benchmark_float_pow_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_pow_prec_round_library_comparison);
    register_bench!(
        runner,
        benchmark_float_pow_prec_round_assign_evaluation_strategy
    );
}

fn demo_float_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("pow({}, {}) = {}", x_old, y_old, x.pow(y));
    }
}

fn demo_float_pow_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "pow({:#x}, {:#x}) = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x.pow(y))
        );
    }
}

fn demo_float_pow_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("pow({}, {}) = {}", x_old, y_old, x.pow(y));
    }
}

fn demo_float_pow_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "pow({:#x}, {:#x}) = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x.pow(y))
        );
    }
}

fn demo_float_pow_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("pow({}, &{}) = {}", x_old, y, x.pow(&y));
    }
}

fn demo_float_pow_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "pow({:#x}, &{:#x}) = {:#x}",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            ComparableFloat(x.pow(&y))
        );
    }
}

fn demo_float_pow_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("pow(&{}, {}) = {}", x, y_old, (&x).pow(y));
    }
}

fn demo_float_pow_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "pow(&{:#x}, {:#x}) = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat(y_old),
            ComparableFloat((&x).pow(y))
        );
    }
}

fn demo_float_pow_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        println!("pow(&{}, &{}) = {}", x, y, (&x).pow(&y));
    }
}

fn demo_float_pow_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        println!(
            "pow(&{:#x}, &{:#x}) = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloatRef(&y),
            ComparableFloat((&x).pow(&y))
        );
    }
}

fn demo_float_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.pow_assign(y.clone());
        println!("x := {x_old}; x.pow_assign({y}); x = {x}");
    }
}

fn demo_float_pow_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.pow_assign(y.clone());
        println!(
            "x := {:#x}; x.pow_assign({:#x}); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            ComparableFloat(x)
        );
    }
}

fn demo_float_pow_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.pow_assign(&y);
        println!("x := {x_old}; x.pow_assign(&{y}); x = {x}");
    }
}

fn demo_float_pow_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.pow_assign(&y);
        println!(
            "x := {:#x}; x.pow_assign(&{:#x}); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            ComparableFloat(x)
        );
    }
}

fn demo_float_pow_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).pow_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.pow_prec(y, prec)
        );
    }
}

fn demo_float_pow_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (pow, o) = x.pow_prec(y, prec);
        println!(
            "({:#x}).pow_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).pow_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.pow_prec(y, prec)
        );
    }
}

fn demo_float_pow_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (pow, o) = x.pow_prec(y, prec);
        println!(
            "({:#x}).pow_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_prec_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).pow_prec_val_ref(&{}, {}) = {:?}",
            x_old,
            y,
            prec,
            x.pow_prec_val_ref(&y, prec)
        );
    }
}

fn demo_float_pow_prec_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (pow, o) = x.pow_prec_val_ref(&y, prec);
        println!(
            "({:#x}).pow_prec_val_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_prec_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).pow_prec_ref_val({}, {}) = {:?}",
            x,
            y_old,
            prec,
            x.pow_prec_ref_val(y, prec)
        );
    }
}

fn demo_float_pow_prec_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (pow, o) = x.pow_prec_ref_val(y, prec);
        println!(
            "(&{:#x}).pow_prec_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_prec_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).pow_prec_ref_ref(&{}, {}) = {:?}",
            x,
            y,
            prec,
            x.pow_prec_ref_ref(&y, prec)
        );
    }
}

fn demo_float_pow_prec_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (pow, o) = x.pow_prec_ref_ref(&y, prec);
        println!(
            "(&{:#x}).pow_prec_ref_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y),
            prec,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.pow_prec_assign(y, prec);
        println!("x := {x_old}; x.pow_prec_assign({y_old}, {prec}); x = {x}");
    }
}

fn demo_float_pow_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.pow_prec_assign(y, prec);
        println!(
            "x := {:#x}; x.pow_prec_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_pow_prec_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.pow_prec_assign_ref(&y, prec);
        println!("x := {x_old}; x.pow_prec_assign({y}, {prec}); x = {x}");
    }
}

fn demo_float_pow_prec_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.pow_prec_assign_ref(&y, prec);
        println!(
            "x := {:#x}; x.pow_prec_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_pow_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).pow_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.pow_round(y, rm)
        );
    }
}

fn demo_float_pow_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (pow, o) = x.pow_round(y, rm);
        println!(
            "({:#x}).pow_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_34()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).pow_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.pow_round(y, rm)
        );
    }
}

fn demo_float_pow_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_34()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (pow, o) = x.pow_round(y, rm);
        println!(
            "({:#x}).pow_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).pow_round_val_ref(&{}, {}) = {:?}",
            x_old,
            y,
            rm,
            x.pow_round_val_ref(&y, rm)
        );
    }
}

fn demo_float_pow_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (pow, o) = x.pow_round_val_ref(&y, rm);
        println!(
            "({:#x}).pow_round_val_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y),
            rm,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).pow_round_ref_val({}, {}) = {:?}",
            x,
            y_old,
            rm,
            x.pow_round_ref_val(y, rm)
        );
    }
}

fn demo_float_pow_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (pow, o) = x.pow_round_ref_val(y, rm);
        println!(
            "(&{:#x}).pow_round_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).pow_round_ref_ref(&{}, {}) = {:?}",
            x,
            y,
            rm,
            x.pow_round_ref_ref(&y, rm)
        );
    }
}

fn demo_float_pow_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let (pow, o) = x.pow_round_ref_ref(&y, rm);
        println!(
            "(&{:#x}).pow_round_ref_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y),
            rm,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.pow_round_assign(y, rm);
        println!("x := {x_old}; x.pow_round_assign({y_old}, {rm}); x = {x}");
    }
}

fn demo_float_pow_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.pow_round_assign(y, rm);
        println!(
            "x := {:#x}; x.pow_round_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_pow_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.pow_round_assign_ref(&y, rm);
        println!("x := {x_old}; x.pow_round_assign({y}, {rm}); x = {x}");
    }
}

fn demo_float_pow_round_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_33()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.pow_round_assign_ref(&y, rm);
        println!(
            "x := {:#x}; x.pow_round_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_pow_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).pow_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.pow_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_pow_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (pow, o) = x.pow_prec_round(y, prec, rm);
        println!(
            "({:#x}).pow_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).pow_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.pow_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_pow_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (pow, o) = x.pow_prec_round(y, prec, rm);
        println!(
            "({:#x}).pow_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_prec_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).pow_prec_round(&{}, {}, {}) = {:?}",
            x_old,
            y,
            prec,
            rm,
            x.pow_prec_round_val_ref(&y, prec, rm)
        );
    }
}

fn demo_float_pow_prec_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (pow, o) = x.pow_prec_round_val_ref(&y, prec, rm);
        println!(
            "({:#x}).pow_prec_round_val_ref(&{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y),
            prec,
            rm,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_prec_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).pow_prec_round_ref_val({}, {}, {}) = {:?}",
            x,
            y_old,
            prec,
            rm,
            x.pow_prec_round_ref_val(y, prec, rm)
        );
    }
}

fn demo_float_pow_prec_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (pow, o) = x.pow_prec_round_ref_val(y, prec, rm);
        println!(
            "(&{:#x}).pow_prec_round_ref_val({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_prec_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).pow_prec_round({}, {}, {}) = {:?}",
            x,
            y,
            prec,
            rm,
            x.pow_prec_round_ref_ref(&y, prec, rm)
        );
    }
}

fn demo_float_pow_prec_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let (pow, o) = x.pow_prec_round_ref_ref(&y, prec, rm);
        println!(
            "({:#x}).pow_prec_round_ref_ref(&{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y),
            prec,
            rm,
            ComparableFloat(pow),
            o
        );
    }
}

fn demo_float_pow_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.pow_prec_round_assign(y, prec, rm);
        println!("x := {x_old}; x.pow_prec_round({y_old}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_pow_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.pow_prec_round_assign(y, prec, rm);
        println!(
            "x := {:#x}; x.pow_prec_round({:#x}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_pow_prec_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.pow_prec_round_assign_ref(&y, prec, rm);
        println!("x := {x_old}; x.pow_prec_round_ref(&{y}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_pow_prec_round_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.pow_prec_round_assign_ref(&y, prec, rm);
        println!(
            "x := {:#x}; x.pow_prec_round_ref(&{:#x}, {}, {}) = {:?}; x = {:#x}",
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
#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow(Float)",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.pow(Float)", &mut |(x, y)| no_out!(x.pow(y))),
            ("Float.pow(&Float)", &mut |(x, y)| no_out!(x.pow(&y))),
            ("(&Float).pow(Float)", &mut |(x, y)| no_out!((&x).pow(y))),
            ("(&Float).pow(&Float)", &mut |(x, y)| no_out!((&x).pow(&y))),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_pow_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow(Float)",
        BenchmarkType::LibraryComparison,
        float_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!((&x).pow(&y))),
            ("rug", &mut |((x, y), _)| no_out!(rug_pow(&x, &y))),
        ],
    );
}

fn benchmark_float_pow_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_assign(Float)",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.pow_assign(Float)", &mut |(mut x, y)| x.pow_assign(y)),
            ("Float.pow_assign(&Float)", &mut |(mut x, y)| {
                x.pow_assign(&y);
            }),
        ],
    );
}

fn benchmark_float_pow_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_prec(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float.pow_prec(Float, u64)", &mut |(x, y, prec)| {
                no_out!(x.pow_prec(y, prec));
            }),
            ("Float.pow_prec_val_ref(&Float, u64)", &mut |(
                x,
                y,
                prec,
            )| {
                no_out!(x.pow_prec_val_ref(&y, prec));
            }),
            (
                "(&Float).pow_prec_ref_val(Float, u64)",
                &mut |(x, y, prec)| no_out!(x.pow_prec_ref_val(y, prec)),
            ),
            (
                "(&Float).pow_prec_ref_ref(&Float, u64)",
                &mut |(x, y, prec)| no_out!(x.pow_prec_ref_ref(&y, prec)),
            ),
        ],
    );
}

fn benchmark_float_pow_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_prec(Float, u64)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec))| {
                no_out!(x.pow_prec_ref_ref(&y, prec));
            }),
            ("rug", &mut |((x, y, prec), _)| {
                no_out!(rug_pow_prec(&x, &y, prec));
            }),
        ],
    );
}

fn benchmark_float_pow_prec_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_prec_assign(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float.pow_prec_assign(Float, u64)", &mut |(
                mut x,
                y,
                prec,
            )| {
                no_out!(x.pow_prec_assign(y, prec));
            }),
            (
                "Float.pow_prec_assign_ref(&Float, u64)",
                &mut |(mut x, y, prec)| no_out!(x.pow_prec_assign_ref(&y, prec)),
            ),
        ],
    );
}

fn benchmark_float_pow_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_round(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_33().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.pow_round(Float, RoundingMode)", &mut |(x, y, rm)| {
                no_out!(x.pow_round(y, rm));
            }),
            (
                "Float.pow_round_val_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.pow_round_val_ref(&y, rm)),
            ),
            (
                "(&Float).pow_round_ref_val(Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.pow_round_ref_val(y, rm)),
            ),
            (
                "(&Float).pow_round_ref_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.pow_round_ref_ref(&y, rm)),
            ),
        ],
    );
}

fn benchmark_float_pow_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_round(Float, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_rounding_mode_triple_gen_var_33_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y, rm))| {
                no_out!(x.pow_round_ref_ref(&y, rm));
            }),
            ("rug", &mut |((x, y, rm), _)| {
                no_out!(rug_pow_round(&x, &y, rm));
            }),
        ],
    );
}

fn benchmark_float_pow_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_round_assign(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_33().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            (
                "Float.pow_round_assign(Float, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.pow_round_assign(y, rm)),
            ),
            (
                "Float.pow_round_assign_ref(&Float, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.pow_round_assign_ref(&y, rm)),
            ),
        ],
    );
}

fn benchmark_float_pow_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.pow_prec_round(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.pow_prec_round(y, prec, rm)),
            ),
            (
                "Float.pow_prec_round_val_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.pow_prec_round_val_ref(&y, prec, rm)),
            ),
            (
                "(&Float).pow_prec_round_ref_val(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.pow_prec_round_ref_val(y, prec, rm)),
            ),
            (
                "(&Float).pow_prec_round_ref_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.pow_prec_round_ref_ref(&y, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_pow_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_rounding_mode_quadruple_gen_var_9_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.pow_prec_round_ref_ref(&y, prec, rm));
            }),
            ("rug", &mut |((x, y, prec, rm), _)| {
                no_out!(rug_pow_prec_round(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_pow_prec_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_prec_round_assign(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.pow_prec_round_assign(Float, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| no_out!(x.pow_prec_round_assign(y, prec, rm)),
            ),
            (
                "Float.pow_prec_round_assign_ref(&Float, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| no_out!(x.pow_prec_round_assign_ref(&y, prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_pow<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, y) in primitive_float_pair_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_pow({}, {}) = {}",
            NiceFloat(x),
            NiceFloat(y),
            NiceFloat(primitive_float_pow(x, y))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_pow<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_pow({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_primitive_float_bucketer("x", "y"),
        &mut [("malachite", &mut |(x, y)| {
            no_out!(primitive_float_pow(x, y));
        })],
    );
}

fn demo_float_rational_pow_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (p, o) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, rm);
        println!("rational_pow_prec_round({x}, {y}, {prec}, {rm}) = ({p}, {o:?})");
    }
}

fn demo_float_rational_pow_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec) in float_rational_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let (p, o) = Float::rational_pow_prec_ref_ref(&x, &y, prec);
        println!("rational_pow_prec({x}, {y}, {prec}) = ({p}, {o:?})");
    }
}

fn benchmark_float_rational_pow_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::rational_pow_prec_round(Rational, Float, u64, RoundingMode)",
        BenchmarkType::Single,
        float_rational_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [("Malachite", &mut |(y, x, prec, rm)| {
            no_out!(Float::rational_pow_prec_round(x, y, prec, rm));
        })],
    );
}

fn benchmark_float_rational_pow_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::rational_pow_prec(Rational, Float, u64)",
        BenchmarkType::Single,
        float_rational_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("prec"),
        &mut [("Malachite", &mut |(y, x, prec)| {
            no_out!(Float::rational_pow_prec(x, y, prec));
        })],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_rational_pow<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, y) in rational_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_rational_pow({}, {}) = {}",
            x.clone(),
            NiceFloat(y),
            NiceFloat(primitive_float_rational_pow::<T>(&x, y))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_rational_pow<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_rational_pow(Rational, {})", T::NAME),
        BenchmarkType::Single,
        rational_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("malachite", &mut |(x, y)| {
            no_out!(primitive_float_rational_pow::<T>(&x, y));
        })],
    );
}

fn demo_float_pow_integer_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_integer_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).pow_integer_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.pow_integer_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_pow_integer_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_integer_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (power, o) = x.pow_integer_prec_round(y, prec, rm);
        println!(
            "({:#x}).pow_integer_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y_old,
            prec,
            rm,
            ComparableFloat(power),
            o
        );
    }
}

fn demo_float_pow_integer_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_integer_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).pow_integer_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.pow_integer_prec(y, prec)
        );
    }
}

fn demo_float_pow_integer_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_integer_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (power, o) = x.pow_integer_prec(y, prec);
        println!(
            "({:#x}).pow_integer_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y_old,
            prec,
            ComparableFloat(power),
            o
        );
    }
}

fn demo_float_pow_integer_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, _, rm) in float_integer_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).pow_integer_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.pow_integer_round(y, rm)
        );
    }
}

fn demo_float_pow_integer_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, _, rm) in float_integer_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (power, o) = x.pow_integer_round(y, rm);
        println!(
            "({:#x}).pow_integer_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            y_old,
            rm,
            ComparableFloat(power),
            o
        );
    }
}

fn demo_float_pow_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).pow({}) = {}", x_old, y_old, x.pow(y));
    }
}

fn demo_float_pow_integer_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({:#x}).pow({}) = {:#x}",
            ComparableFloat(x_old),
            y_old,
            ComparableFloat(x.pow(y))
        );
    }
}

fn demo_float_pow_integer_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.pow_assign(y);
        println!("x := {x_old}; x.pow_assign({y_old}); x = {x}");
    }
}

fn benchmark_float_pow_integer_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_integer_prec_round(Integer, u64, RoundingMode)",
        BenchmarkType::Single,
        float_integer_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_float_integer_max_complexity_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y, prec, rm)| {
            no_out!(x.pow_integer_prec_round(y, prec, rm));
        })],
    );
}

fn benchmark_float_pow_integer_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_integer_prec(Integer, u64)",
        BenchmarkType::Single,
        float_integer_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_integer_max_complexity_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y, prec)| {
            no_out!(x.pow_integer_prec(y, prec));
        })],
    );
}

fn benchmark_float_pow_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow(Integer)",
        BenchmarkType::EvaluationStrategy,
        float_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_integer_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.pow(Integer)", &mut |(x, y)| no_out!(x.pow(y))),
            ("Float.pow(&Integer)", &mut |(x, y)| no_out!(x.pow(&y))),
            ("(&Float).pow(Integer)", &mut |(x, y)| no_out!((&x).pow(y))),
            ("(&Float).pow(&Integer)", &mut |(x, y)| {
                no_out!((&x).pow(&y));
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_pow_integer<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (y, x) in integer_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_pow_integer({}, {}) = {}",
            NiceFloat(x),
            y.clone(),
            NiceFloat(primitive_float_pow_integer::<T>(x, &y))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_pow_integer<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_pow_integer({}, Integer)", T::NAME),
        BenchmarkType::Single,
        integer_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("n"),
        &mut [("malachite", &mut |(y, x)| {
            no_out!(primitive_float_pow_integer::<T>(x, &y));
        })],
    );
}

fn demo_float_pow_u_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).pow_u_prec_round({}, {}, {}) = {:?}",
            x_old,
            n,
            prec,
            rm,
            x.pow_u_prec_round(n, prec, rm)
        );
    }
}

fn demo_float_pow_u_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (power, o) = x.pow_u_prec_round(n, prec, rm);
        println!(
            "({:#x}).pow_u_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            prec,
            rm,
            ComparableFloat(power),
            o
        );
    }
}

fn demo_float_pow_u_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).pow_u_prec({}, {}) = {:?}",
            x_old,
            n,
            prec,
            x.pow_u_prec(n, prec)
        );
    }
}

fn demo_float_pow_u_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (power, o) = x.pow_u_prec(n, prec);
        println!(
            "({:#x}).pow_u_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            prec,
            ComparableFloat(power),
            o
        );
    }
}

fn demo_float_pow_u_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, _, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).pow_u_round({}, {}) = {:?}",
            x_old,
            n,
            rm,
            x.pow_u_round(n, rm)
        );
    }
}

fn demo_float_pow_u_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, _, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        let (power, o) = x.pow_u_round(n, rm);
        println!(
            "({:#x}).pow_u_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            rm,
            ComparableFloat(power),
            o
        );
    }
}

fn demo_float_pow_u(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in float_unsigned_pair_gen::<u64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).pow({}) = {}", x_old, n, x.pow(n));
    }
}

fn demo_float_pow_u_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in float_unsigned_pair_gen::<u64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).pow({}) = {:#x}",
            ComparableFloat(x_old),
            n,
            ComparableFloat(x.pow(n))
        );
    }
}

fn demo_float_pow_u_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, n) in float_unsigned_pair_gen::<u64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.pow_assign(n);
        println!("x := {x_old}; x.pow_assign({n}); x = {x}");
    }
}

fn benchmark_float_pow_u_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_u_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, n, prec, rm)| {
            no_out!(x.pow_u_prec_round(n, prec, rm));
        })],
    );
}

fn benchmark_float_pow_u_prec(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.pow_u_prec(u64, u64)",
        BenchmarkType::Single,
        float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "n"),
        &mut [("Malachite", &mut |(x, n, prec)| {
            no_out!(x.pow_u_prec(n, prec));
        })],
    );
}

fn benchmark_float_pow_u_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_unsigned_max_complexity_bucketer("x", "n"),
        &mut [
            ("Float.pow(u64)", &mut |(x, n)| no_out!(x.pow(n))),
            ("(&Float).pow(u64)", &mut |(x, n)| no_out!((&x).pow(n))),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_pow_u<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, n) in primitive_float_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_pow_u({}, {}) = {}",
            NiceFloat(x),
            n,
            NiceFloat(primitive_float_pow_u::<T>(x, n))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_pow_u<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_pow_u({}, u64)", T::NAME),
        BenchmarkType::Single,
        primitive_float_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("n"),
        &mut [("malachite", &mut |(x, n)| {
            no_out!(primitive_float_pow_u::<T>(x, n));
        })],
    );
}

fn demo_float_pow_s_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).pow_s_prec_round({}, {}, {}) = {:?}",
            x_old,
            n,
            prec,
            rm,
            x.pow_s_prec_round(n, prec, rm)
        );
    }
}

fn demo_float_pow_s_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (power, o) = x.pow_s_prec_round(n, prec, rm);
        println!(
            "({:#x}).pow_s_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            prec,
            rm,
            ComparableFloat(power),
            o
        );
    }
}

fn demo_float_pow_s_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in float_signed_unsigned_triple_gen_var_1::<i64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).pow_s_prec({}, {}) = {:?}",
            x_old,
            n,
            prec,
            x.pow_s_prec(n, prec)
        );
    }
}

fn demo_float_pow_s_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in float_signed_unsigned_triple_gen_var_1::<i64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (power, o) = x.pow_s_prec(n, prec);
        println!(
            "({:#x}).pow_s_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            prec,
            ComparableFloat(power),
            o
        );
    }
}

fn demo_float_pow_s_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, _, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).pow_s_round({}, {}) = {:?}",
            x_old,
            n,
            rm,
            x.pow_s_round(n, rm)
        );
    }
}

fn demo_float_pow_s_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, _, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        let (power, o) = x.pow_s_round(n, rm);
        println!(
            "({:#x}).pow_s_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            rm,
            ComparableFloat(power),
            o
        );
    }
}

fn demo_float_pow_s(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in float_signed_pair_gen::<i64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).pow({}) = {}", x_old, n, x.pow(n));
    }
}

fn demo_float_pow_s_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in float_signed_pair_gen::<i64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).pow({}) = {:#x}",
            ComparableFloat(x_old),
            n,
            ComparableFloat(x.pow(n))
        );
    }
}

fn demo_float_pow_s_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, n) in float_signed_pair_gen::<i64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.pow_assign(n);
        println!("x := {x_old}; x.pow_assign({n}); x = {x}");
    }
}

fn benchmark_float_pow_s_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow_s_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::Single,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, n, prec, rm)| {
            no_out!(x.pow_s_prec_round(n, prec, rm));
        })],
    );
}

fn benchmark_float_pow_s_prec(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.pow_s_prec(i64, u64)",
        BenchmarkType::Single,
        float_signed_unsigned_triple_gen_var_1::<i64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "n"),
        &mut [("Malachite", &mut |(x, n, prec)| {
            no_out!(x.pow_s_prec(n, prec));
        })],
    );
}

fn benchmark_float_pow_s_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.pow(i64)",
        BenchmarkType::EvaluationStrategy,
        float_signed_pair_gen::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_signed_max_complexity_bucketer("x", "n"),
        &mut [
            ("Float.pow(i64)", &mut |(x, n)| no_out!(x.pow(n))),
            ("(&Float).pow(i64)", &mut |(x, n)| no_out!((&x).pow(n))),
        ],
    );
}
