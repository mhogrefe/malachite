// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{LogBase2, LogBase2Assign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::arithmetic::log_base_2::{
    rug_log_base_2, rug_log_base_2_prec, rug_log_base_2_prec_round, rug_log_base_2_round,
};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_pair_1_float_complexity_bucketer,
    pair_2_pair_float_primitive_int_max_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    pair_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_38,
    float_rounding_mode_pair_gen_var_38_rm, float_rounding_mode_pair_gen_var_39,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_1_rm, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_23,
    float_unsigned_rounding_mode_triple_gen_var_23_rm,
    float_unsigned_rounding_mode_triple_gen_var_24,
    rational_unsigned_rounding_mode_triple_gen_var_7,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::test_util::bench::bucketers::{
    pair_rational_bit_u64_max_bucketer, triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::rational_unsigned_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_log_base_2);
    register_demo!(runner, demo_float_log_base_2_debug);
    register_demo!(runner, demo_float_log_base_2_extreme);
    register_demo!(runner, demo_float_log_base_2_extreme_debug);
    register_demo!(runner, demo_float_log_base_2_ref);
    register_demo!(runner, demo_float_log_base_2_ref_debug);
    register_demo!(runner, demo_float_log_base_2_assign);
    register_demo!(runner, demo_float_log_base_2_assign_debug);
    register_demo!(runner, demo_float_log_base_2_prec);
    register_demo!(runner, demo_float_log_base_2_prec_debug);
    register_demo!(runner, demo_float_log_base_2_prec_extreme);
    register_demo!(runner, demo_float_log_base_2_prec_extreme_debug);
    register_demo!(runner, demo_float_log_base_2_prec_ref);
    register_demo!(runner, demo_float_log_base_2_prec_ref_debug);
    register_demo!(runner, demo_float_log_base_2_prec_assign);
    register_demo!(runner, demo_float_log_base_2_prec_assign_debug);
    register_demo!(runner, demo_float_log_base_2_round);
    register_demo!(runner, demo_float_log_base_2_round_debug);
    register_demo!(runner, demo_float_log_base_2_round_extreme);
    register_demo!(runner, demo_float_log_base_2_round_extreme_debug);
    register_demo!(runner, demo_float_log_base_2_round_ref);
    register_demo!(runner, demo_float_log_base_2_round_ref_debug);
    register_demo!(runner, demo_float_log_base_2_round_assign);
    register_demo!(runner, demo_float_log_base_2_round_assign_debug);
    register_demo!(runner, demo_float_log_base_2_prec_round);
    register_demo!(runner, demo_float_log_base_2_prec_round_debug);
    register_demo!(runner, demo_float_log_base_2_prec_round_extreme);
    register_demo!(runner, demo_float_log_base_2_prec_round_extreme_debug);
    register_demo!(runner, demo_float_log_base_2_prec_round_ref);
    register_demo!(runner, demo_float_log_base_2_prec_round_ref_debug);
    register_demo!(runner, demo_float_log_base_2_prec_round_assign);
    register_demo!(runner, demo_float_log_base_2_prec_round_assign_debug);
    register_demo!(runner, demo_float_log_base_2_rational_prec);
    register_demo!(runner, demo_float_log_base_2_rational_prec_debug);
    register_demo!(runner, demo_float_log_base_2_rational_prec_ref);
    register_demo!(runner, demo_float_log_base_2_rational_prec_ref_debug);
    register_demo!(runner, demo_float_log_base_2_rational_prec_round);
    register_demo!(runner, demo_float_log_base_2_rational_prec_round_debug);
    register_demo!(runner, demo_float_log_base_2_rational_prec_round_ref);
    register_demo!(runner, demo_float_log_base_2_rational_prec_round_ref_debug);

    register_bench!(runner, benchmark_float_log_base_2_evaluation_strategy);
    register_bench!(runner, benchmark_float_log_base_2_library_comparison);
    register_bench!(runner, benchmark_float_log_base_2_assign);
    register_bench!(runner, benchmark_float_log_base_2_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_log_base_2_prec_library_comparison);
    register_bench!(runner, benchmark_float_log_base_2_prec_assign);
    register_bench!(runner, benchmark_float_log_base_2_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_log_base_2_round_library_comparison);
    register_bench!(runner, benchmark_float_log_base_2_round_assign);
    register_bench!(
        runner,
        benchmark_float_log_base_2_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_2_prec_round_library_comparison
    );
    register_bench!(runner, benchmark_float_log_base_2_prec_round_assign);
    register_bench!(
        runner,
        benchmark_float_log_base_2_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_2_rational_prec_round_evaluation_strategy
    );
}

fn demo_float_log_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).log_base_2() = {}", x_old, x.log_base_2());
    }
}

fn demo_float_log_base_2_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).log_base_2() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.log_base_2())
        );
    }
}

fn demo_float_log_base_2_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).log_base_2() = {}", x_old, x.log_base_2());
    }
}

fn demo_float_log_base_2_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).log_base_2() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.log_base_2())
        );
    }
}

fn demo_float_log_base_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).log_base_2() = {}", x, (&x).log_base_2());
    }
}

fn demo_float_log_base_2_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).log_base_2() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).log_base_2())
        );
    }
}

fn demo_float_log_base_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.log_base_2_assign();
        println!("x := {x_old}; x.log_base_2_assign(); x = {x}");
    }
}

fn demo_float_log_base_2_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.log_base_2_assign();
        println!(
            "x := {:#x}; x.log_base_2_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_log_base_2_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).log_base_2_prec({}) = {:?}",
            x_old,
            prec,
            x.log_base_2_prec(prec)
        );
    }
}

fn demo_float_log_base_2_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (log_base_2, o) = x.log_base_2_prec(prec);
        println!(
            "({:#x}).log_base_2_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(log_base_2),
            o
        );
    }
}

fn demo_float_log_base_2_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).log_base_2_prec({}) = {:?}",
            x_old,
            prec,
            x.log_base_2_prec(prec)
        );
    }
}

fn demo_float_log_base_2_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (log_base_2, o) = x.log_base_2_prec(prec);
        println!(
            "({:#x}).log_base_2_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(log_base_2),
            o
        );
    }
}

fn demo_float_log_base_2_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).log_base_2_prec_ref({}) = {:?}",
            x,
            prec,
            x.log_base_2_prec_ref(prec)
        );
    }
}

fn demo_float_log_base_2_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (log_base_2, o) = x.log_base_2_prec_ref(prec);
        println!(
            "(&{:#x}).log_base_2_prec_ref({}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            ComparableFloat(log_base_2),
            o
        );
    }
}

fn demo_float_log_base_2_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.log_base_2_prec_assign(prec);
        println!("x := {x_old}; x.log_base_2_prec_assign({prec}); x = {x}");
    }
}

fn demo_float_log_base_2_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.log_base_2_prec_assign(prec);
        println!(
            "x := {:#x}; x.log_base_2_prec_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_log_base_2_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_38()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_2_round({}) = {:?}",
            x_old,
            rm,
            x.log_base_2_round(rm)
        );
    }
}

fn demo_float_log_base_2_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_38()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log_base_2, o) = x.log_base_2_round(rm);
        println!(
            "({:#x}).log_base_2_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(log_base_2),
            o
        );
    }
}

fn demo_float_log_base_2_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_2_round({}) = {:?}",
            x_old,
            rm,
            x.log_base_2_round(rm)
        );
    }
}

fn demo_float_log_base_2_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log_base_2, o) = x.log_base_2_round(rm);
        println!(
            "({:#x}).log_base_2_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(log_base_2),
            o
        );
    }
}

fn demo_float_log_base_2_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_38()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).log_base_2_round_ref({}) = {:?}",
            x,
            rm,
            x.log_base_2_round_ref(rm)
        );
    }
}

fn demo_float_log_base_2_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_38()
        .get(gm, config)
        .take(limit)
    {
        let (log_base_2, o) = x.log_base_2_round_ref(rm);
        println!(
            "(&{:#x}).log_base_2_round_ref({}) = ({:#x}, {:?})",
            ComparableFloat(x),
            rm,
            ComparableFloat(log_base_2),
            o
        );
    }
}

fn demo_float_log_base_2_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_38()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_2_round_assign(rm);
        println!("x := {x_old}; x.log_base_2_round_assign({rm}); x = {x}");
    }
}

fn demo_float_log_base_2_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_38()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_2_round_assign(rm);
        println!(
            "x := {:#x}; x.log_base_2_round_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_log_base_2_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_23()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_2_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.log_base_2_prec_round(prec, rm)
        );
    }
}

fn demo_float_log_base_2_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_23()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log_base_2, o) = x.log_base_2_prec_round(prec, rm);
        println!(
            "({:#x}).log_base_2_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(log_base_2),
            o
        );
    }
}

fn demo_float_log_base_2_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_2_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.log_base_2_prec_round(prec, rm)
        );
    }
}

fn demo_float_log_base_2_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log_base_2, o) = x.log_base_2_prec_round(prec, rm);
        println!(
            "({:#x}).log_base_2_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(log_base_2),
            o
        );
    }
}

fn demo_float_log_base_2_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_23()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_2_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.log_base_2_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_log_base_2_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_23()
        .get(gm, config)
        .take(limit)
    {
        let (log_base_2, o) = x.log_base_2_prec_round_ref(prec, rm);
        println!(
            "({:#x}).log_base_2_prec_round_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            rm,
            ComparableFloat(log_base_2),
            o
        );
    }
}

fn demo_float_log_base_2_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_23()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_2_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.log_base_2_prec_round({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_log_base_2_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_23()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_2_prec_round_assign(prec, rm);
        println!(
            "x := {:#x}; x.log_base_2_prec_round({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_log_base_2_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::log_base_2_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::log_base_2_rational_prec(n, p)
        );
    }
}

fn demo_float_log_base_2_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::log_base_2_rational_prec(n.clone(), p);
        println!(
            "Float::log_base_2_rational_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_log_base_2_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::log_base_2_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::log_base_2_rational_prec_ref(&n, p)
        );
    }
}

fn demo_float_log_base_2_rational_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::log_base_2_rational_prec_ref(&n, p);
        println!(
            "Float::log_base_2_rational_prec_ref(&{}, {}) = {:x?}",
            n,
            p,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_log_base_2_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::log_base_2_rational_prec_round({}, {}, {:?}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::log_base_2_rational_prec_round(n, p, rm)
        );
    }
}

fn demo_float_log_base_2_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::log_base_2_rational_prec_round(n.clone(), p, rm);
        println!(
            "Float::log_base_2_rational_prec_round({}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_log_base_2_rational_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::log_base_2_rational_prec_round_ref(&{}, {}, {:?}) = {:?}",
            n,
            p,
            rm,
            Float::log_base_2_rational_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_log_base_2_rational_prec_round_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::log_base_2_rational_prec_round_ref(&n, p, rm);
        println!(
            "Float::log_base_2_rational_prec_round_ref(&{}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_log_base_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.log_base_2()", &mut |x| no_out!(x.log_base_2())),
            ("(&Float).log_base_2()", &mut |x| {
                no_out!((&x).log_base_2());
            }),
        ],
    );
}

fn benchmark_float_log_base_2_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!((&x).log_base_2())),
            ("rug", &mut |(x, _)| no_out!(rug_log_base_2(&x))),
        ],
    );
}

fn benchmark_float_log_base_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Float.log_base_2_assign()", &mut |mut x| {
            x.log_base_2_assign();
        })],
    );
}

fn benchmark_float_log_base_2_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Float.log_base_2_prec(u64)", &mut |(x, prec)| {
                no_out!(x.log_base_2_prec(prec));
            }),
            ("(&Float).log_base_2_prec_ref(u64)", &mut |(x, prec)| {
                no_out!(x.log_base_2_prec_ref(prec));
            }),
        ],
    );
}

fn benchmark_float_log_base_2_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2_prec(u64)",
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec))| {
                no_out!(x.log_base_2_prec_ref(prec));
            }),
            ("rug", &mut |((x, prec), _)| {
                no_out!(rug_log_base_2_prec(&x, prec));
            }),
        ],
    );
}

fn benchmark_float_log_base_2_prec_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2_prec_assign(u64)",
        BenchmarkType::Single,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [("Float.log_base_2_prec_assign(u64)", &mut |(mut x, prec)| {
            no_out!(x.log_base_2_prec_assign(prec));
        })],
    );
}

fn benchmark_float_log_base_2_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2_round(RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_38().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Float.log_base_2_round(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.log_base_2_round(rm));
            }),
            (
                "(&Float).log_base_2_round_ref(RoundingMode)",
                &mut |(x, rm)| no_out!(x.log_base_2_round_ref(rm)),
            ),
        ],
    );
}

fn benchmark_float_log_base_2_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2_round(RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_rounding_mode_pair_gen_var_38_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, rm))| {
                no_out!(x.log_base_2_round_ref(rm));
            }),
            ("rug", &mut |((x, rm), _)| {
                no_out!(rug_log_base_2_round(&x, rm));
            }),
        ],
    );
}

fn benchmark_float_log_base_2_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2_round_assign(RoundingMode)",
        BenchmarkType::Single,
        float_rounding_mode_pair_gen_var_38().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [(
            "Float.log_base_2_round_assign(RoundingMode)",
            &mut |(mut x, rm)| no_out!(x.log_base_2_round_assign(rm)),
        )],
    );
}

fn benchmark_float_log_base_2_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_23().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.log_base_2_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.log_base_2_prec_round(prec, rm)),
            ),
            (
                "(&Float).log_base_2_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.log_base_2_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_log_base_2_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_23_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.log_base_2_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_log_base_2_prec_round(&x, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_log_base_2_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_2_prec_round_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_23().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [(
            "Float.log_base_2_prec_round_assign(u64, RoundingMode)",
            &mut |(mut x, prec, rm)| no_out!(x.log_base_2_prec_round_assign(prec, rm)),
        )],
    );
}

fn benchmark_float_log_base_2_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::log_base_2_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::log_base_2_rational_prec(Rational, u64)",
                &mut |(n, prec)| no_out!(Float::log_base_2_rational_prec(n, prec)),
            ),
            (
                "Float::log_base_2_rational_prec_ref(&Rational, u64)",
                &mut |(n, prec)| no_out!(Float::log_base_2_rational_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_log_base_2_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::log_base_2_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::log_base_2_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::log_base_2_rational_prec_round(n, prec, rm)),
            ),
            (
                "Float::log_base_2_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| {
                    no_out!(Float::log_base_2_rational_prec_round_ref(&n, prec, rm));
                },
            ),
        ],
    );
}
