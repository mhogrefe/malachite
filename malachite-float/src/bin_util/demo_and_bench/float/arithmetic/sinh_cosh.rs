// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Cosh, Sinh, SinhCosh, SinhCoshAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NaN;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::sinh_cosh::{
    primitive_float_sinh_cosh, primitive_float_sinh_cosh_rational,
};
use malachite_float::float::arithmetic::{cosh, sinh};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::sinh_cosh::{
    rug_sinh_cosh, rug_sinh_cosh_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::test_util::bench::bucketers::{
    pair_rational_bit_u64_max_bucketer, rational_bit_bucketer,
    triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_sinh_cosh_rational_prec_round);
    register_demo!(runner, demo_float_sinh_cosh_rational_prec_round_debug);
    register_demo!(runner, demo_float_sinh_cosh_rational_prec_round_ref);
    register_demo!(runner, demo_float_sinh_cosh_rational_prec);
    register_demo!(runner, demo_float_sinh_cosh_rational_prec_ref);
    register_primitive_float_demos!(runner, demo_primitive_float_sinh_cosh_rational);
    register_bench!(
        runner,
        benchmark_float_sinh_cosh_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sinh_cosh_rational_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_float_sinh_cosh_rational_prec_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_sinh_cosh_rational);
    register_demo!(runner, demo_float_sinh_cosh);
    register_demo!(runner, demo_float_sinh_cosh_debug);
    register_demo!(runner, demo_float_sinh_cosh_extreme);
    register_demo!(runner, demo_float_sinh_cosh_ref);
    register_demo!(runner, demo_float_sinh_cosh_assign);
    register_demo!(runner, demo_float_sinh_cosh_prec);
    register_demo!(runner, demo_float_sinh_cosh_prec_ref);
    register_demo!(runner, demo_float_sinh_cosh_prec_assign);
    register_demo!(runner, demo_float_sinh_cosh_round);
    register_demo!(runner, demo_float_sinh_cosh_round_ref);
    register_demo!(runner, demo_float_sinh_cosh_round_assign);
    register_demo!(runner, demo_float_sinh_cosh_prec_round);
    register_demo!(runner, demo_float_sinh_cosh_prec_round_debug);
    register_demo!(runner, demo_float_sinh_cosh_prec_round_ref);
    register_demo!(runner, demo_float_sinh_cosh_prec_round_assign);
    register_primitive_float_demos!(runner, demo_primitive_float_sinh_cosh);

    register_bench!(runner, benchmark_float_sinh_cosh_evaluation_strategy);
    register_bench!(runner, benchmark_float_sinh_cosh_library_comparison);
    register_bench!(runner, benchmark_float_sinh_cosh_algorithms);
    register_bench!(runner, benchmark_float_sinh_cosh_assign);
    register_bench!(
        runner,
        benchmark_float_sinh_cosh_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sinh_cosh_prec_round_library_comparison
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_sinh_cosh);
    register_primitive_float_benches!(runner, benchmark_primitive_float_sinh_cosh_algorithms);
}

fn demo_float_sinh_cosh(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).sinh_cosh() = {:?}", x_old, x.sinh_cosh());
    }
}

fn demo_float_sinh_cosh_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (s, c) = x.sinh_cosh();
        println!(
            "({:#x}).sinh_cosh() = ({:#x}, {:#x})",
            ComparableFloat(x_old),
            ComparableFloat(s),
            ComparableFloat(c)
        );
    }
}

fn demo_float_sinh_cosh_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).sinh_cosh() = {:?}", x_old, x.sinh_cosh());
    }
}

fn demo_float_sinh_cosh_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).sinh_cosh() = {:?}", x, (&x).sinh_cosh());
    }
}

fn demo_float_sinh_cosh_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let mut c = Float::NAN;
        x.sinh_cosh_assign(&mut c);
        println!("x := {x_old}; x.sinh_cosh_assign(&mut c); x = {x}; c = {c}");
    }
}

fn demo_float_sinh_cosh_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).sinh_cosh_prec({}) = {:?}",
            x_old,
            prec,
            x.sinh_cosh_prec(prec)
        );
    }
}

fn demo_float_sinh_cosh_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).sinh_cosh_prec_ref({}) = {:?}",
            x,
            prec,
            x.sinh_cosh_prec_ref(prec)
        );
    }
}

fn demo_float_sinh_cosh_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let mut c = Float::NAN;
        let o = x.sinh_cosh_prec_assign(&mut c, prec);
        println!("x := {x_old}; x.sinh_cosh_prec_assign(&mut c, {prec}) = {o:?}; x = {x}; c = {c}");
    }
}

fn demo_float_sinh_cosh_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sinh_cosh_round({}) = {:?}",
            x_old,
            rm,
            x.sinh_cosh_round(rm)
        );
    }
}

fn demo_float_sinh_cosh_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sinh_cosh_round_ref({}) = {:?}",
            x,
            rm,
            x.sinh_cosh_round_ref(rm)
        );
    }
}

fn demo_float_sinh_cosh_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let mut c = Float::NAN;
        let o = x.sinh_cosh_round_assign(&mut c, rm);
        println!("x := {x_old}; x.sinh_cosh_round_assign(&mut c, {rm}) = {o:?}; x = {x}; c = {c}");
    }
}

fn demo_float_sinh_cosh_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sinh_cosh_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.sinh_cosh_prec_round(prec, rm)
        );
    }
}

fn demo_float_sinh_cosh_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (s, c, o_s, o_c) = x.sinh_cosh_prec_round(prec, rm);
        println!(
            "({:#x}).sinh_cosh_prec_round({}, {}) = ({:#x}, {:#x}, {:?}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(s),
            ComparableFloat(c),
            o_s,
            o_c
        );
    }
}

fn demo_float_sinh_cosh_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sinh_cosh_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.sinh_cosh_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_sinh_cosh_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let mut c = Float::NAN;
        let o = x.sinh_cosh_prec_round_assign(&mut c, prec, rm);
        println!(
            "x := {x_old}; x.sinh_cosh_prec_round_assign(&mut c, {prec}, {rm}) = {o:?}; x = {x}; \
             c = {c}"
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_float_sinh_cosh_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sinh_cosh()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.sinh_cosh()", &mut |x| no_out!(x.sinh_cosh())),
            ("(&Float).sinh_cosh()", &mut |x| no_out!((&x).sinh_cosh())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sinh_cosh_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sinh_cosh()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.sinh_cosh())),
            ("rug", &mut |(x, _)| no_out!(rug_sinh_cosh(&x))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sinh_cosh_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sinh_cosh()",
        BenchmarkType::Algorithms,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("together", &mut |x| no_out!(x.sinh_cosh())),
            ("separately", &mut |x| no_out!(((&x).sinh(), x.cosh()))),
        ],
    );
}

fn benchmark_float_sinh_cosh_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sinh_cosh_assign(&mut Float)",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| {
            let mut c = Float::NAN;
            x.sinh_cosh_assign(&mut c);
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sinh_cosh_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sinh_cosh_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.sinh_cosh_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.sinh_cosh_prec_round(prec, rm)),
            ),
            (
                "(&Float).sinh_cosh_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.sinh_cosh_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sinh_cosh_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sinh_cosh_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.sinh_cosh_prec_round(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_sinh_cosh_prec_round(&x, prec, rm));
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_sinh_cosh<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        let (s, c) = primitive_float_sinh_cosh(x);
        println!(
            "primitive_float_sinh_cosh({}) = ({}, {})",
            NiceFloat(x),
            NiceFloat(s),
            NiceFloat(c)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_sinh_cosh<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_sinh_cosh({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_sinh_cosh(x));
        })],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_sinh_cosh_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_sinh_cosh({})", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [
            ("together", &mut |x| {
                no_out!(primitive_float_sinh_cosh(x));
            }),
            ("separately", &mut |x| {
                no_out!((sinh::primitive_float_sinh(x), cosh::primitive_float_cosh(x)));
            }),
        ],
    );
}

fn demo_float_sinh_cosh_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sinh_cosh_rational_prec_round({}, {}, {}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::sinh_cosh_rational_prec_round(n, p, rm)
        );
    }
}

fn demo_float_sinh_cosh_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (s, c, o_s, o_c) = Float::sinh_cosh_rational_prec_round(n.clone(), p, rm);
        println!(
            "Float::sinh_cosh_rational_prec_round({}, {}, {}) = ({:#x}, {:#x}, {:?}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(s),
            ComparableFloat(c),
            o_s,
            o_c
        );
    }
}

fn demo_float_sinh_cosh_rational_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sinh_cosh_rational_prec_round_ref(&{}, {}, {}) = {:?}",
            n,
            p,
            rm,
            Float::sinh_cosh_rational_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_sinh_cosh_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sinh_cosh_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::sinh_cosh_rational_prec(n, p)
        );
    }
}

fn demo_float_sinh_cosh_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sinh_cosh_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::sinh_cosh_rational_prec_ref(&n, p)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_float_sinh_cosh_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sinh_cosh_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::sinh_cosh_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(n, p, rm)| no_out!(Float::sinh_cosh_rational_prec_round(n, p, rm)),
            ),
            (
                "Float::sinh_cosh_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(n, p, rm)| no_out!(Float::sinh_cosh_rational_prec_round_ref(&n, p, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sinh_cosh_rational_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sinh_cosh_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("together", &mut |(n, p, rm)| {
                no_out!(Float::sinh_cosh_rational_prec_round(n, p, rm));
            }),
            ("separately", &mut |(n, p, rm)| {
                no_out!((
                    Float::sinh_rational_prec_round_ref(&n, p, rm),
                    Float::cosh_rational_prec_round(n, p, rm),
                ));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sinh_cosh_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sinh_cosh_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::sinh_cosh_rational_prec(Rational, u64)",
                &mut |(n, p)| no_out!(Float::sinh_cosh_rational_prec(n, p)),
            ),
            (
                "Float::sinh_cosh_rational_prec_ref(&Rational, u64)",
                &mut |(n, p)| no_out!(Float::sinh_cosh_rational_prec_ref(&n, p)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_sinh_cosh_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        let (s, c) = primitive_float_sinh_cosh_rational::<T>(&x);
        println!(
            "primitive_float_sinh_cosh_rational({}) = ({}, {})",
            x,
            NiceFloat(s),
            NiceFloat(c)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_sinh_cosh_rational<T: PrimitiveFloat>(
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
            "primitive_float_sinh_cosh_rational::<{}>(&Rational)",
            T::NAME
        ),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_sinh_cosh_rational::<T>(&x));
        })],
    );
}
