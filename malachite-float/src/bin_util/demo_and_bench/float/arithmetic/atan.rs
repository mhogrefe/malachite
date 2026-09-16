// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Atan, AtanAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    pair_1_primitive_float_bucketer, primitive_float_bucketer, quadruple_3_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_unsigned_pair_gen_var_1,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::atan::{
    primitive_float_atan, primitive_float_atan_pi, primitive_float_atan_pi_rational,
    primitive_float_atan_rational, primitive_float_atan_with_period,
    primitive_float_atan_with_period_rational,
};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    quadruple_1_float_complexity_bucketer, triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_1_float_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::atan::{rug_atan, rug_atan_prec_round};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_2, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
    float_unsigned_rounding_mode_triple_gen_var_40,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_19,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_20,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_10,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_6,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, pair_rational_bit_u64_max_bucketer, rational_bit_bucketer,
    triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::{
    rational_gen, rational_unsigned_pair_gen_var_1, rational_unsigned_pair_gen_var_3,
};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_primitive_float_atan);
    register_primitive_float_benches!(runner, benchmark_primitive_float_atan);
    register_demo!(runner, demo_float_atan);
    register_demo!(runner, demo_float_atan_debug);
    register_demo!(runner, demo_float_atan_extreme);
    register_demo!(runner, demo_float_atan_extreme_debug);
    register_demo!(runner, demo_float_atan_ref);
    register_demo!(runner, demo_float_atan_ref_debug);
    register_demo!(runner, demo_float_atan_assign);
    register_demo!(runner, demo_float_atan_assign_debug);
    register_demo!(runner, demo_float_atan_prec);
    register_demo!(runner, demo_float_atan_prec_debug);
    register_demo!(runner, demo_float_atan_prec_extreme);
    register_demo!(runner, demo_float_atan_prec_ref);
    register_demo!(runner, demo_float_atan_prec_assign);
    register_demo!(runner, demo_float_atan_round);
    register_demo!(runner, demo_float_atan_round_debug);
    register_demo!(runner, demo_float_atan_round_ref);
    register_demo!(runner, demo_float_atan_round_assign);
    register_demo!(runner, demo_float_atan_prec_round);
    register_demo!(runner, demo_float_atan_prec_round_debug);
    register_demo!(runner, demo_float_atan_prec_round_ref);
    register_demo!(runner, demo_float_atan_prec_round_assign);
    register_bench!(runner, benchmark_float_atan_evaluation_strategy);
    register_bench!(runner, benchmark_float_atan_library_comparison);
    register_bench!(runner, benchmark_float_atan_assign);
    register_bench!(runner, benchmark_float_atan_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_atan_prec_round_library_comparison);
    register_demo!(runner, demo_float_atan_rational_prec);
    register_demo!(runner, demo_float_atan_rational_prec_debug);
    register_demo!(runner, demo_float_atan_rational_prec_ref);
    register_demo!(runner, demo_float_atan_rational_prec_ref_debug);
    register_demo!(runner, demo_float_atan_rational_prec_round);
    register_demo!(runner, demo_float_atan_rational_prec_round_debug);
    register_demo!(runner, demo_float_atan_rational_prec_round_ref);
    register_demo!(runner, demo_float_atan_rational_prec_round_ref_debug);
    register_bench!(
        runner,
        benchmark_float_atan_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_atan_rational_prec_round_evaluation_strategy
    );
    register_primitive_float_demos!(runner, demo_primitive_float_atan_rational);
    register_primitive_float_benches!(runner, benchmark_primitive_float_atan_rational);
    register_demo!(runner, demo_float_atan_with_period_prec_round);
    register_demo!(runner, demo_float_atan_with_period_prec_round_debug);
    register_demo!(runner, demo_float_atan_with_period_prec_round_extreme);
    register_demo!(runner, demo_float_atan_with_period_prec_round_ref);
    register_demo!(runner, demo_float_atan_with_period_prec_round_assign);
    register_demo!(runner, demo_float_atan_with_period_prec);
    register_demo!(runner, demo_float_atan_with_period_prec_debug);
    register_demo!(runner, demo_float_atan_with_period_prec_ref);
    register_demo!(runner, demo_float_atan_with_period_prec_assign);
    register_demo!(runner, demo_float_atan_with_period_round);
    register_demo!(runner, demo_float_atan_with_period_round_debug);
    register_demo!(runner, demo_float_atan_with_period_round_ref);
    register_demo!(runner, demo_float_atan_with_period_round_assign);
    register_bench!(
        runner,
        benchmark_float_atan_with_period_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_atan_with_period_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_atan_with_period_round_evaluation_strategy
    );
    register_demo!(runner, demo_float_atan_with_period);
    register_demo!(runner, demo_float_atan_with_period_debug);
    register_demo!(runner, demo_float_atan_with_period_ref);
    register_demo!(runner, demo_float_atan_with_period_assign);
    register_bench!(runner, benchmark_float_atan_with_period_evaluation_strategy);
    register_primitive_float_demos!(runner, demo_primitive_float_atan_with_period);
    register_primitive_float_benches!(runner, benchmark_primitive_float_atan_with_period);
    register_demo!(runner, demo_float_atan_with_period_rational_prec_round);
    register_demo!(
        runner,
        demo_float_atan_with_period_rational_prec_round_debug
    );
    register_demo!(runner, demo_float_atan_with_period_rational_prec_round_ref);
    register_demo!(runner, demo_float_atan_with_period_rational_prec);
    register_demo!(runner, demo_float_atan_with_period_rational_prec_debug);
    register_demo!(runner, demo_float_atan_with_period_rational_prec_ref);
    register_bench!(
        runner,
        benchmark_float_atan_with_period_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_atan_with_period_rational_prec_evaluation_strategy
    );
    register_primitive_float_demos!(runner, demo_primitive_float_atan_with_period_rational);
    register_primitive_float_benches!(runner, benchmark_primitive_float_atan_with_period_rational);
    register_demo!(runner, demo_float_atan_pi_prec_round);
    register_demo!(runner, demo_float_atan_pi_prec_round_debug);
    register_demo!(runner, demo_float_atan_pi_prec);
    register_demo!(runner, demo_float_atan_pi_round);
    register_demo!(runner, demo_float_atan_pi_prec_round_assign);
    register_demo!(runner, demo_float_atan_pi_rational_prec_round);
    register_demo!(runner, demo_float_atan_pi_rational_prec);
    register_primitive_float_demos!(runner, demo_primitive_float_atan_pi);
    register_primitive_float_demos!(runner, demo_primitive_float_atan_pi_rational);
    register_demo!(runner, demo_float_atan_pi);
    register_demo!(runner, demo_float_atan_pi_debug);
    register_demo!(runner, demo_float_atan_pi_ref);
    register_demo!(runner, demo_float_atan_pi_assign);
    register_bench!(runner, benchmark_float_atan_pi_evaluation_strategy);
}
fn demo_float_atan(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).atan() = {}", x_old, x.atan());
    }
}

fn demo_float_atan_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).atan() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.atan())
        );
    }
}

fn demo_float_atan_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).atan() = {}", x_old, x.atan());
    }
}

fn demo_float_atan_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).atan() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.atan())
        );
    }
}

fn demo_float_atan_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).atan() = {}", x, (&x).atan());
    }
}

fn demo_float_atan_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).atan() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).atan())
        );
    }
}

fn demo_float_atan_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.atan_assign();
        println!("x := {x_old}; x.atan_assign(); x = {x}");
    }
}

fn demo_float_atan_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.atan_assign();
        println!(
            "x := {:#x}; x.atan_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_atan_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).atan_prec({}) = {:?}", x_old, prec, x.atan_prec(prec));
    }
}

fn demo_float_atan_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (c, o) = x.atan_prec(prec);
        println!(
            "({:#x}).atan_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_atan_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).atan_prec({}) = {:?}", x_old, prec, x.atan_prec(prec));
    }
}

fn demo_float_atan_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).atan_prec_ref({}) = {:?}",
            x,
            prec,
            x.atan_prec_ref(prec)
        );
    }
}

fn demo_float_atan_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.atan_prec_assign(prec);
        println!("x := {x_old}; x.atan_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_atan_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).atan_round({}) = {:?}", x_old, rm, x.atan_round(rm));
    }
}

fn demo_float_atan_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.atan_round(rm);
        println!(
            "({:#x}).atan_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_atan_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).atan_round_ref({}) = {:?}",
            x,
            rm,
            x.atan_round_ref(rm)
        );
    }
}

fn demo_float_atan_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.atan_round_assign(rm);
        println!("x := {x_old}; x.atan_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_atan_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).atan_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.atan_prec_round(prec, rm)
        );
    }
}

fn demo_float_atan_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.atan_prec_round(prec, rm);
        println!(
            "({:#x}).atan_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_atan_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).atan_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.atan_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_atan_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.atan_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.atan_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_float_atan_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atan()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.atan()", &mut |x| no_out!(x.atan())),
            ("(&Float).atan()", &mut |x| no_out!((&x).atan())),
        ],
    );
}

fn benchmark_float_atan_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atan()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.atan())),
            ("rug", &mut |(x, _)| no_out!(rug_atan(&x))),
        ],
    );
}

fn benchmark_float_atan_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.atan_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.atan_assign())],
    );
}

fn benchmark_float_atan_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atan_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.atan_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.atan_prec_round(prec, rm)),
            ),
            (
                "(&Float).atan_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.atan_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_atan_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atan_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.atan_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_atan_prec_round(&x, prec, rm));
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_atan<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_atan({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_atan(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_atan<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_atan({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_atan(x));
        })],
    );
}

fn demo_float_atan_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::atan_rational_prec(n, p)
        );
    }
}

fn demo_float_atan_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::atan_rational_prec(n.clone(), p);
        println!(
            "Float::atan_rational_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_atan_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::atan_rational_prec_ref(&n, p)
        );
    }
}

fn demo_float_atan_rational_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::atan_rational_prec_ref(&n, p);
        println!(
            "Float::atan_rational_prec_ref(&{}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_atan_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan_rational_prec_round({}, {}, {}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::atan_rational_prec_round(n, p, rm)
        );
    }
}

fn demo_float_atan_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::atan_rational_prec_round(n.clone(), p, rm);
        println!(
            "Float::atan_rational_prec_round({}, {}, {}) = ({:#x}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_atan_rational_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan_rational_prec_round_ref(&{}, {}, {}) = {:?}",
            n,
            p,
            rm,
            Float::atan_rational_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_atan_rational_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::atan_rational_prec_round_ref(&n, p, rm);
        println!(
            "Float::atan_rational_prec_round_ref(&{}, {}, {}) = ({:#x}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(f),
            o
        );
    }
}

fn benchmark_float_atan_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::atan_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::atan_rational_prec(Rational, u64)",
                &mut |(n, prec)| no_out!(Float::atan_rational_prec(n, prec)),
            ),
            (
                "Float::atan_rational_prec_ref(&Rational, u64)",
                &mut |(n, prec)| no_out!(Float::atan_rational_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_atan_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::atan_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::atan_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::atan_rational_prec_round(n, prec, rm)),
            ),
            (
                "Float::atan_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::atan_rational_prec_round_ref(&n, prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_atan_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_atan_rational({}) = {:?}",
            x,
            NiceFloat(primitive_float_atan_rational::<T>(&x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_atan_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_atan_rational::<{}>(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_atan_rational::<T>(&x));
        })],
    );
}

fn demo_float_atan_with_period_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).atan_with_period_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            rm,
            x.atan_with_period_prec_round(u, prec, rm)
        );
    }
}

fn demo_float_atan_with_period_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = x.clone().atan_with_period_prec_round(u, prec, rm);
        println!(
            "({:#x}).atan_with_period_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            u,
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_atan_with_period_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).atan_with_period_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            rm,
            x.atan_with_period_prec_round(u, prec, rm)
        );
    }
}

fn demo_float_atan_with_period_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).atan_with_period_prec_round_ref({}, {}, {}) = {:?}",
            x,
            u,
            prec,
            rm,
            x.atan_with_period_prec_round_ref(u, prec, rm)
        );
    }
}

fn demo_float_atan_with_period_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.atan_with_period_prec_round_assign(u, prec, rm);
        println!(
            "x := {x_old}; x.atan_with_period_prec_round_assign({u}, {prec}, {rm}) = {o:?}; x = {x}"
        );
    }
}

fn demo_float_atan_with_period_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).atan_with_period_prec({}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            x.atan_with_period_prec(u, prec)
        );
    }
}

fn demo_float_atan_with_period_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = x.clone().atan_with_period_prec(u, prec);
        println!(
            "({:#x}).atan_with_period_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            u,
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_atan_with_period_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).atan_with_period_prec_ref({}, {}) = {:?}",
            x,
            u,
            prec,
            x.atan_with_period_prec_ref(u, prec)
        );
    }
}

fn demo_float_atan_with_period_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.atan_with_period_prec_assign(u, prec);
        println!("x := {x_old}; x.atan_with_period_prec_assign({u}, {prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_atan_with_period_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, rm) in float_unsigned_rounding_mode_triple_gen_var_40()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).atan_with_period_round({}, {}) = {:?}",
            x.clone(),
            u,
            rm,
            x.atan_with_period_round(u, rm)
        );
    }
}

fn demo_float_atan_with_period_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, rm) in float_unsigned_rounding_mode_triple_gen_var_40()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = x.clone().atan_with_period_round(u, rm);
        println!(
            "({:#x}).atan_with_period_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            u,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_atan_with_period_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, rm) in float_unsigned_rounding_mode_triple_gen_var_40()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).atan_with_period_round_ref({}, {}) = {:?}",
            x,
            u,
            rm,
            x.atan_with_period_round_ref(u, rm)
        );
    }
}

fn demo_float_atan_with_period_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, u, rm) in float_unsigned_rounding_mode_triple_gen_var_40()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.atan_with_period_round_assign(u, rm);
        println!("x := {x_old}; x.atan_with_period_round_assign({u}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_float_atan_with_period_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atan_with_period_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float.atan_with_period_prec_round(u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| no_out!(x.atan_with_period_prec_round(u, prec, rm)),
            ),
            (
                "(&Float).atan_with_period_prec_round_ref(u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| no_out!(x.atan_with_period_prec_round_ref(u, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_atan_with_period_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atan_with_period_prec(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float.atan_with_period_prec(u64, u64)",
                &mut |(x, u, prec)| {
                    no_out!(x.atan_with_period_prec(u, prec));
                },
            ),
            (
                "(&Float).atan_with_period_prec_ref(u64, u64)",
                &mut |(x, u, prec)| {
                    no_out!(x.atan_with_period_prec_ref(u, prec));
                },
            ),
        ],
    );
}

fn benchmark_float_atan_with_period_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atan_with_period_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_40().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float.atan_with_period_round(u64, RoundingMode)",
                &mut |(x, u, rm)| {
                    no_out!(x.atan_with_period_round(u, rm));
                },
            ),
            (
                "(&Float).atan_with_period_round_ref(u64, RoundingMode)",
                &mut |(x, u, rm)| no_out!(x.atan_with_period_round_ref(u, rm)),
            ),
        ],
    );
}

fn demo_float_atan_with_period(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u) in float_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).atan_with_period({}) = {}",
            x_old,
            u,
            x.atan_with_period(u)
        );
    }
}

fn demo_float_atan_with_period_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u) in float_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let s = x.atan_with_period(u);
        println!(
            "({:#x}).atan_with_period({}) = {:#x}",
            ComparableFloat(x_old),
            u,
            ComparableFloat(s)
        );
    }
}

fn demo_float_atan_with_period_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u) in float_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).atan_with_period_ref({}) = {}",
            x,
            u,
            x.atan_with_period_ref(u)
        );
    }
}

fn demo_float_atan_with_period_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, u) in float_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.atan_with_period_assign(u);
        println!("x := {x_old}; x.atan_with_period_assign({u}); x = {x}");
    }
}

fn benchmark_float_atan_with_period_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atan_with_period(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Float.atan_with_period(u64)", &mut |(x, u)| {
                no_out!(x.atan_with_period(u));
            }),
            ("(&Float).atan_with_period_ref(u64)", &mut |(x, u)| {
                no_out!(x.atan_with_period_ref(u));
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_atan_with_period<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, u) in primitive_float_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_atan_with_period({}, {}) = {}",
            NiceFloat(x),
            u,
            NiceFloat(primitive_float_atan_with_period(x, u))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_atan_with_period<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_atan_with_period({}, u64)", T::NAME),
        BenchmarkType::Single,
        primitive_float_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("x"),
        &mut [("malachite", &mut |(x, u)| {
            no_out!(primitive_float_atan_with_period(x, u));
        })],
    );
}

fn demo_float_atan_with_period_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan_with_period_rational_prec_round({}, {}, {}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            rm,
            Float::atan_with_period_rational_prec_round(x, u, prec, rm)
        );
    }
}

fn demo_float_atan_with_period_rational_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, u, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let (t, o) = Float::atan_with_period_rational_prec_round(x.clone(), u, prec, rm);
        println!(
            "Float::atan_with_period_rational_prec_round({}, {}, {}, {}) = ({:#x}, {:?})",
            x,
            u,
            prec,
            rm,
            ComparableFloat(t),
            o
        );
    }
}

fn demo_float_atan_with_period_rational_prec_round_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, u, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan_with_period_rational_prec_round_ref(&{}, {}, {}, {}) = {:?}",
            x,
            u,
            prec,
            rm,
            Float::atan_with_period_rational_prec_round_ref(&x, u, prec, rm)
        );
    }
}

fn demo_float_atan_with_period_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan_with_period_rational_prec({}, {}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            Float::atan_with_period_rational_prec(x, u, prec)
        );
    }
}

fn demo_float_atan_with_period_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let (t, o) = Float::atan_with_period_rational_prec(x.clone(), u, prec);
        println!(
            "Float::atan_with_period_rational_prec({}, {}, {}) = ({:#x}, {:?})",
            x,
            u,
            prec,
            ComparableFloat(t),
            o
        );
    }
}

fn demo_float_atan_with_period_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan_with_period_rational_prec_ref(&{}, {}, {}) = {:?}",
            x,
            u,
            prec,
            Float::atan_with_period_rational_prec_ref(&x, u, prec)
        );
    }
}

fn benchmark_float_atan_with_period_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::atan_with_period_rational_prec_round(Rational, u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "Float::atan_with_period_rational_prec_round(Rational, u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| {
                    no_out!(Float::atan_with_period_rational_prec_round(x, u, prec, rm));
                },
            ),
            (
                "Float::atan_with_period_rational_prec_round_ref(&Rational, u64, u64, \
                 RoundingMode)",
                &mut |(x, u, prec, rm)| {
                    no_out!(Float::atan_with_period_rational_prec_round_ref(
                        &x, u, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_float_atan_with_period_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::atan_with_period_rational_prec(Rational, u64, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "Float::atan_with_period_rational_prec(Rational, u64, u64)",
                &mut |(x, u, prec, _)| no_out!(Float::atan_with_period_rational_prec(x, u, prec)),
            ),
            (
                "Float::atan_with_period_rational_prec_ref(&Rational, u64, u64)",
                &mut |(x, u, prec, _)| {
                    no_out!(Float::atan_with_period_rational_prec_ref(&x, u, prec));
                },
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_atan_with_period_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, u) in rational_unsigned_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_atan_with_period_rational({}, {}) = {:?}",
            x,
            u,
            NiceFloat(primitive_float_atan_with_period_rational::<T>(&x, u))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_atan_with_period_rational<T: PrimitiveFloat>(
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
            "primitive_float_atan_with_period_rational::<{}>(&Rational, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        rational_unsigned_pair_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("malachite", &mut |(x, u)| {
            no_out!(primitive_float_atan_with_period_rational::<T>(&x, u));
        })],
    );
}

fn demo_float_atan_pi_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).atan_pi_prec_round({}, {}) = {:?}",
            x.clone(),
            prec,
            rm,
            x.atan_pi_prec_round(prec, rm)
        );
    }
}

fn demo_float_atan_pi_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = x.clone().atan_pi_prec_round(prec, rm);
        println!(
            "({:#x}).atan_pi_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_atan_pi_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "({}).atan_pi_prec({}) = {:?}",
            x.clone(),
            prec,
            x.atan_pi_prec(prec)
        );
    }
}

fn demo_float_atan_pi_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).atan_pi_round({}) = {:?}",
            x.clone(),
            rm,
            x.atan_pi_round(rm)
        );
    }
}

fn demo_float_atan_pi_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.atan_pi_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.atan_pi_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_atan_pi_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan_pi_rational_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            prec,
            rm,
            Float::atan_pi_rational_prec_round(x, prec, rm)
        );
    }
}

fn demo_float_atan_pi_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan_pi_rational_prec({}, {}) = {:?}",
            x.clone(),
            prec,
            Float::atan_pi_rational_prec(x, prec)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_atan_pi<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_atan_pi({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_atan_pi(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_atan_pi_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_atan_pi_rational({}) = {:?}",
            x,
            NiceFloat(primitive_float_atan_pi_rational::<T>(&x))
        );
    }
}

fn demo_float_atan_pi(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).atan_pi() = {}", x_old, x.atan_pi());
    }
}

fn demo_float_atan_pi_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let s = x.atan_pi();
        println!(
            "({:#x}).atan_pi() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(s)
        );
    }
}

fn demo_float_atan_pi_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).atan_pi_ref() = {}", x, x.atan_pi_ref());
    }
}

fn demo_float_atan_pi_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.atan_pi_assign();
        println!("x := {x_old}; x.atan_pi_assign(); x = {x}");
    }
}

fn benchmark_float_atan_pi_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atan_pi()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.atan_pi()", &mut |x| no_out!(x.atan_pi())),
            ("(&Float).atan_pi_ref()", &mut |x| no_out!(x.atan_pi_ref())),
        ],
    );
}
