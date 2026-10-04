// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Coth, CothAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::coth::{
    primitive_float_coth, primitive_float_coth_rational,
};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::coth::{rug_coth, rug_coth_prec_round};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::test_util::bench::bucketers::{
    pair_rational_bit_u64_max_bucketer, rational_bit_bucketer,
    triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_coth_rational_prec);
    register_demo!(runner, demo_float_coth_rational_prec_debug);
    register_demo!(runner, demo_float_coth_rational_prec_ref);
    register_demo!(runner, demo_float_coth_rational_prec_ref_debug);
    register_demo!(runner, demo_float_coth_rational_prec_round);
    register_demo!(runner, demo_float_coth_rational_prec_round_debug);
    register_demo!(runner, demo_float_coth_rational_prec_round_ref);
    register_demo!(runner, demo_float_coth_rational_prec_round_ref_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_coth_rational);
    register_bench!(
        runner,
        benchmark_float_coth_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_coth_rational_prec_round_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_coth_rational);
    register_demo!(runner, demo_float_coth);
    register_demo!(runner, demo_float_coth_debug);
    register_demo!(runner, demo_float_coth_extreme);
    register_demo!(runner, demo_float_coth_extreme_debug);
    register_demo!(runner, demo_float_coth_ref);
    register_demo!(runner, demo_float_coth_ref_debug);
    register_demo!(runner, demo_float_coth_assign);
    register_demo!(runner, demo_float_coth_assign_debug);
    register_demo!(runner, demo_float_coth_prec);
    register_demo!(runner, demo_float_coth_prec_debug);
    register_demo!(runner, demo_float_coth_prec_extreme);
    register_demo!(runner, demo_float_coth_prec_ref);
    register_demo!(runner, demo_float_coth_prec_assign);
    register_demo!(runner, demo_float_coth_round);
    register_demo!(runner, demo_float_coth_round_debug);
    register_demo!(runner, demo_float_coth_round_ref);
    register_demo!(runner, demo_float_coth_round_assign);
    register_demo!(runner, demo_float_coth_prec_round);
    register_demo!(runner, demo_float_coth_prec_round_debug);
    register_demo!(runner, demo_float_coth_prec_round_ref);
    register_demo!(runner, demo_float_coth_prec_round_assign);
    register_primitive_float_demos!(runner, demo_primitive_float_coth);

    register_bench!(runner, benchmark_float_coth_evaluation_strategy);
    register_bench!(runner, benchmark_float_coth_library_comparison);
    register_bench!(runner, benchmark_float_coth_assign);
    register_bench!(runner, benchmark_float_coth_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_coth_prec_round_library_comparison);
    register_primitive_float_benches!(runner, benchmark_primitive_float_coth);
}

fn demo_float_coth(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).coth() = {}", x_old, x.coth());
    }
}

fn demo_float_coth_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).coth() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.coth())
        );
    }
}

fn demo_float_coth_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).coth() = {}", x_old, x.coth());
    }
}

fn demo_float_coth_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).coth() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.coth())
        );
    }
}

fn demo_float_coth_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).coth() = {}", x, (&x).coth());
    }
}

fn demo_float_coth_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).coth() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).coth())
        );
    }
}

fn demo_float_coth_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.coth_assign();
        println!("x := {x_old}; x.coth_assign(); x = {x}");
    }
}

fn demo_float_coth_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.coth_assign();
        println!(
            "x := {:#x}; x.coth_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_coth_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).coth_prec({}) = {:?}", x_old, prec, x.coth_prec(prec));
    }
}

fn demo_float_coth_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (c, o) = x.coth_prec(prec);
        println!(
            "({:#x}).coth_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_coth_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).coth_prec({}) = {:?}", x_old, prec, x.coth_prec(prec));
    }
}

fn demo_float_coth_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).coth_prec_ref({}) = {:?}",
            x,
            prec,
            x.coth_prec_ref(prec)
        );
    }
}

fn demo_float_coth_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.coth_prec_assign(prec);
        println!("x := {x_old}; x.coth_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_coth_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).coth_round({}) = {:?}", x_old, rm, x.coth_round(rm));
    }
}

fn demo_float_coth_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.coth_round(rm);
        println!(
            "({:#x}).coth_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_coth_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).coth_round_ref({}) = {:?}",
            x,
            rm,
            x.coth_round_ref(rm)
        );
    }
}

fn demo_float_coth_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.coth_round_assign(rm);
        println!("x := {x_old}; x.coth_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_coth_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).coth_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.coth_prec_round(prec, rm)
        );
    }
}

fn demo_float_coth_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.coth_prec_round(prec, rm);
        println!(
            "({:#x}).coth_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_coth_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).coth_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.coth_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_coth_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.coth_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.coth_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_float_coth_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.coth()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.coth()", &mut |x| no_out!(x.coth())),
            ("(&Float).coth()", &mut |x| no_out!((&x).coth())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_coth_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.coth()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.coth())),
            ("rug", &mut |(x, _)| no_out!(rug_coth(&x))),
        ],
    );
}

fn benchmark_float_coth_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.coth_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.coth_assign())],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_coth_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.coth_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.coth_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.coth_prec_round(prec, rm)),
            ),
            (
                "(&Float).coth_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.coth_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_coth_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.coth_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.coth_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_coth_prec_round(&x, prec, rm));
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_coth<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_coth({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_coth(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_coth<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_coth({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_coth(x));
        })],
    );
}

fn demo_float_coth_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::coth_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::coth_rational_prec(n, p)
        );
    }
}

fn demo_float_coth_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::coth_rational_prec(n.clone(), p);
        println!(
            "Float::coth_rational_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_coth_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::coth_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::coth_rational_prec_ref(&n, p)
        );
    }
}

fn demo_float_coth_rational_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::coth_rational_prec_ref(&n, p);
        println!(
            "Float::coth_rational_prec_ref(&{}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_coth_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::coth_rational_prec_round({}, {}, {}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::coth_rational_prec_round(n, p, rm)
        );
    }
}

fn demo_float_coth_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::coth_rational_prec_round(n.clone(), p, rm);
        println!(
            "Float::coth_rational_prec_round({}, {}, {}) = ({:#x}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_coth_rational_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::coth_rational_prec_round_ref(&{}, {}, {}) = {:?}",
            n,
            p,
            rm,
            Float::coth_rational_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_coth_rational_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::coth_rational_prec_round_ref(&n, p, rm);
        println!(
            "Float::coth_rational_prec_round_ref(&{}, {}, {}) = ({:#x}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(f),
            o
        );
    }
}

fn benchmark_float_coth_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::coth_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::coth_rational_prec(Rational, u64)",
                &mut |(n, prec)| no_out!(Float::coth_rational_prec(n, prec)),
            ),
            (
                "Float::coth_rational_prec_ref(&Rational, u64)",
                &mut |(n, prec)| no_out!(Float::coth_rational_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_coth_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::coth_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::coth_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::coth_rational_prec_round(n, prec, rm)),
            ),
            (
                "Float::coth_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::coth_rational_prec_round_ref(&n, prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_coth_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_coth_rational({}) = {:?}",
            x,
            NiceFloat(primitive_float_coth_rational::<T>(&x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_coth_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_coth_rational::<{}>(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_coth_rational::<T>(&x));
        })],
    );
}
