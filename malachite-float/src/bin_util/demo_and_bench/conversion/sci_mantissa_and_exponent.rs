// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, SciMantissaAndExponent};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::pair_1_primitive_float_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_signed_pair_gen_var_3;
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen_var_13, float_gen_var_3, float_rounding_mode_pair_gen,
    float_rounding_mode_pair_gen_var_21, float_signed_pair_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_sci_mantissa_and_exponent_round);
    register_primitive_float_demos!(runner, demo_sci_mantissa_and_exponent_round_debug);
    register_primitive_float_demos!(runner, demo_sci_mantissa_and_exponent_round_extreme);
    register_primitive_float_demos!(runner, demo_sci_mantissa_and_exponent_round_extreme_debug);
    register_demo!(runner, demo_float_sci_mantissa_and_exponent_float);
    register_demo!(runner, demo_float_sci_mantissa_and_exponent_float_debug);
    register_demo!(runner, demo_float_sci_mantissa_and_exponent_float_extreme);
    register_demo!(
        runner,
        demo_float_sci_mantissa_and_exponent_float_extreme_debug
    );
    register_demo!(runner, demo_float_sci_mantissa_and_exponent_float_ref);
    register_demo!(runner, demo_float_sci_mantissa_and_exponent_float_ref_debug);
    register_demo!(runner, demo_float_sci_mantissa_float);
    register_demo!(runner, demo_float_sci_mantissa_float_debug);
    register_demo!(runner, demo_float_sci_mantissa_float_extreme);
    register_demo!(runner, demo_float_sci_mantissa_float_extreme_debug);
    register_demo!(runner, demo_float_sci_mantissa_float_ref);
    register_demo!(runner, demo_float_sci_mantissa_float_ref_debug);
    register_demo!(runner, demo_float_sci_exponent_float);
    register_demo!(runner, demo_float_sci_exponent_float_debug);
    register_demo!(runner, demo_float_sci_exponent_float_extreme);
    register_demo!(runner, demo_float_sci_exponent_float_extreme_debug);
    register_demo!(runner, demo_float_sci_exponent_float_ref);
    register_demo!(runner, demo_float_sci_exponent_float_ref_debug);
    register_demo!(runner, demo_float_from_sci_mantissa_and_exponent_float);
    register_demo!(
        runner,
        demo_float_from_sci_mantissa_and_exponent_float_debug
    );
    register_demo!(runner, demo_float_from_sci_mantissa_and_exponent_float_ref);
    register_demo!(
        runner,
        demo_float_from_sci_mantissa_and_exponent_float_ref_debug
    );

    register_primitive_float_demos!(runner, demo_float_sci_mantissa_and_exponent_primitive_float);
    register_primitive_float_demos!(
        runner,
        demo_float_sci_mantissa_and_exponent_primitive_float_debug
    );
    register_primitive_float_demos!(
        runner,
        demo_float_sci_mantissa_and_exponent_primitive_float_extreme
    );
    register_primitive_float_demos!(
        runner,
        demo_float_sci_mantissa_and_exponent_primitive_float_extreme_debug
    );
    register_primitive_float_demos!(runner, demo_float_sci_mantissa_primitive_float);
    register_primitive_float_demos!(runner, demo_float_sci_mantissa_primitive_float_debug);
    register_primitive_float_demos!(runner, demo_float_sci_mantissa_primitive_float_extreme);
    register_primitive_float_demos!(
        runner,
        demo_float_sci_mantissa_primitive_float_extreme_debug
    );
    register_primitive_float_demos!(runner, demo_float_sci_exponent_primitive_float);
    register_primitive_float_demos!(runner, demo_float_sci_exponent_primitive_float_debug);
    register_primitive_float_demos!(runner, demo_float_sci_exponent_primitive_float_extreme);
    register_primitive_float_demos!(
        runner,
        demo_float_sci_exponent_primitive_float_extreme_debug
    );
    register_primitive_float_demos!(
        runner,
        demo_float_from_sci_mantissa_and_exponent_primitive_float
    );
    register_primitive_float_demos!(
        runner,
        demo_float_from_sci_mantissa_and_exponent_primitive_float_debug
    );

    register_primitive_float_benches!(runner, benchmark_sci_mantissa_and_exponent_round);
    register_bench!(
        runner,
        benchmark_float_sci_mantissa_and_exponent_float_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sci_mantissa_float_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sci_exponent_float_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_from_sci_mantissa_and_exponent_float_evaluation_strategy
    );

    register_primitive_float_benches!(
        runner,
        benchmark_float_sci_mantissa_and_exponent_primitive_float
    );
    register_primitive_float_benches!(runner, benchmark_float_sci_mantissa_primitive_float);
    register_primitive_float_benches!(runner, benchmark_float_sci_exponent_primitive_float);
    register_primitive_float_benches!(
        runner,
        benchmark_float_from_sci_mantissa_and_exponent_primitive_float
    );
}

fn demo_sci_mantissa_and_exponent_round<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, rm) in float_rounding_mode_pair_gen().get(gm, config).take(limit) {
        println!(
            "{}.sci_mantissa_and_exponent_round({}) = {:?}",
            n.clone(),
            rm,
            n.sci_mantissa_and_exponent_round::<T>(rm)
        );
    }
}

fn demo_sci_mantissa_and_exponent_round_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, rm) in float_rounding_mode_pair_gen().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_mantissa_and_exponent_round({}) = {:?}",
            ComparableFloat(n.clone()),
            rm,
            n.sci_mantissa_and_exponent_round::<T>(rm)
        );
    }
}

fn demo_sci_mantissa_and_exponent_round_extreme<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, rm) in float_rounding_mode_pair_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.sci_mantissa_and_exponent_round({}) = {:?}",
            n.clone(),
            rm,
            n.sci_mantissa_and_exponent_round::<T>(rm)
        );
    }
}

fn demo_sci_mantissa_and_exponent_round_extreme_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, rm) in float_rounding_mode_pair_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{:#x}.sci_mantissa_and_exponent_round({}) = {:?}",
            ComparableFloat(n.clone()),
            rm,
            n.sci_mantissa_and_exponent_round::<T>(rm)
        );
    }
}

fn demo_float_sci_mantissa_and_exponent_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{}.sci_mantissa_and_exponent() = {:?}",
            n.clone(),
            SciMantissaAndExponent::<Float, _, _>::sci_mantissa_and_exponent(n)
        );
    }
}

fn demo_float_sci_mantissa_and_exponent_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_mantissa_and_exponent() = {:?}",
            ComparableFloat(n.clone()),
            SciMantissaAndExponent::<Float, _, _>::sci_mantissa_and_exponent(n)
        );
    }
}

fn demo_float_sci_mantissa_and_exponent_float_extreme(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in float_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{}.sci_mantissa_and_exponent() = {:?}",
            n.clone(),
            SciMantissaAndExponent::<Float, _, _>::sci_mantissa_and_exponent(n)
        );
    }
}

fn demo_float_sci_mantissa_and_exponent_float_extreme_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in float_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_mantissa_and_exponent() = {:?}",
            ComparableFloat(n.clone()),
            SciMantissaAndExponent::<Float, _, _>::sci_mantissa_and_exponent(n)
        );
    }
}

fn demo_float_sci_mantissa_and_exponent_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "(&{}).sci_mantissa_and_exponent() = {:?}",
            n,
            SciMantissaAndExponent::<Float, _, _>::sci_mantissa_and_exponent(&n)
        );
    }
}

fn demo_float_sci_mantissa_and_exponent_float_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).sci_mantissa_and_exponent() = {:?}",
            ComparableFloatRef(&n),
            SciMantissaAndExponent::<Float, _, _>::sci_mantissa_and_exponent(&n)
        );
    }
}

fn demo_float_sci_mantissa_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{}.sci_mantissa() = {}",
            n.clone(),
            SciMantissaAndExponent::<Float, _, _>::sci_mantissa(n)
        );
    }
}

fn demo_float_sci_mantissa_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_mantissa() = {:#x}",
            ComparableFloat(n.clone()),
            ComparableFloat(SciMantissaAndExponent::<Float, _, _>::sci_mantissa(n))
        );
    }
}

fn demo_float_sci_mantissa_float_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{}.sci_mantissa() = {}",
            n.clone(),
            SciMantissaAndExponent::<Float, _, _>::sci_mantissa(n)
        );
    }
}

fn demo_float_sci_mantissa_float_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_mantissa() = {:#x}",
            ComparableFloat(n.clone()),
            ComparableFloat(SciMantissaAndExponent::<Float, _, _>::sci_mantissa(n))
        );
    }
}

fn demo_float_sci_mantissa_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "(&{}).sci_mantissa() = {}",
            n,
            SciMantissaAndExponent::<Float, _, _>::sci_mantissa(&n)
        );
    }
}

fn demo_float_sci_mantissa_float_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).sci_mantissa() = {:#x}",
            ComparableFloatRef(&n),
            ComparableFloat(SciMantissaAndExponent::<Float, _, _>::sci_mantissa(&n))
        );
    }
}

fn demo_float_sci_exponent_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{}.sci_exponent() = {}",
            n.clone(),
            SciMantissaAndExponent::<Float, _, _>::sci_exponent(n)
        );
    }
}

fn demo_float_sci_exponent_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_exponent() = {}",
            ComparableFloat(n.clone()),
            SciMantissaAndExponent::<Float, _, _>::sci_exponent(n)
        );
    }
}

fn demo_float_sci_exponent_float_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{}.sci_exponent() = {}",
            n.clone(),
            SciMantissaAndExponent::<Float, _, _>::sci_exponent(n)
        );
    }
}

fn demo_float_sci_exponent_float_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_exponent() = {}",
            ComparableFloat(n.clone()),
            SciMantissaAndExponent::<Float, _, _>::sci_exponent(n)
        );
    }
}

fn demo_float_sci_exponent_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{}.sci_exponent() = {}",
            n,
            SciMantissaAndExponent::<Float, _, _>::sci_exponent(&n)
        );
    }
}

fn demo_float_sci_exponent_float_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_exponent() = {}",
            ComparableFloatRef(&n),
            SciMantissaAndExponent::<Float, _, _>::sci_exponent(&n)
        );
    }
}

fn demo_float_from_sci_mantissa_and_exponent_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mantissa, exponent) in float_signed_pair_gen_var_1::<i32>()
        .get(gm, config)
        .take(limit)
    {
        let n =
            <Float as SciMantissaAndExponent<Float, i32, Float>>::from_sci_mantissa_and_exponent(
                mantissa.clone(),
                exponent,
            );
        println!(
            "Float::from_sci_mantissa_and_exponent({}, {}) = {}",
            mantissa,
            exponent,
            n.unwrap()
        );
    }
}

fn demo_float_from_sci_mantissa_and_exponent_float_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in float_signed_pair_gen_var_1::<i32>()
        .get(gm, config)
        .take(limit)
    {
        let n =
            <Float as SciMantissaAndExponent<Float, i32, Float>>::from_sci_mantissa_and_exponent(
                mantissa.clone(),
                exponent,
            );
        println!(
            "Float::from_sci_mantissa_and_exponent({:#x}, {}) = {:#x}",
            mantissa,
            exponent,
            ComparableFloat(n.unwrap())
        );
    }
}

fn demo_float_from_sci_mantissa_and_exponent_float_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in float_signed_pair_gen_var_1::<i32>()
        .get(gm, config)
        .take(limit)
    {
        let n =
            <&Float as SciMantissaAndExponent<Float, i32, Float>>::from_sci_mantissa_and_exponent(
                mantissa.clone(),
                exponent,
            );
        println!(
            "Float::from_sci_mantissa_and_exponent({}, {}) = {}",
            mantissa,
            exponent,
            n.unwrap()
        );
    }
}

fn demo_float_from_sci_mantissa_and_exponent_float_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in float_signed_pair_gen_var_1::<i32>()
        .get(gm, config)
        .take(limit)
    {
        let n =
            <&Float as SciMantissaAndExponent<Float, i32, Float>>::from_sci_mantissa_and_exponent(
                mantissa.clone(),
                exponent,
            );
        println!(
            "Float::from_sci_mantissa_and_exponent({:#x}, {}) = {:#x}",
            mantissa,
            exponent,
            ComparableFloat(n.unwrap())
        );
    }
}

fn demo_float_sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_3().get(gm, config).take(limit) {
        let (m, e) = SciMantissaAndExponent::<T, _, _>::sci_mantissa_and_exponent(&n);
        println!(
            "{}.sci_mantissa_and_exponent() = {:?}",
            n,
            (NiceFloat(m), e)
        );
    }
}

fn demo_float_sci_mantissa_and_exponent_primitive_float_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_3().get(gm, config).take(limit) {
        let (m, e) = SciMantissaAndExponent::<T, _, _>::sci_mantissa_and_exponent(&n);
        println!(
            "{:#x}.sci_mantissa_and_exponent() = {:?}",
            ComparableFloatRef(&n),
            (NiceFloat(m), e)
        );
    }
}

fn demo_float_sci_mantissa_and_exponent_primitive_float_extreme<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_13().get(gm, config).take(limit) {
        let (m, e) = SciMantissaAndExponent::<T, _, _>::sci_mantissa_and_exponent(&n);
        println!(
            "{}.sci_mantissa_and_exponent() = {:?}",
            n,
            (NiceFloat(m), e)
        );
    }
}

fn demo_float_sci_mantissa_and_exponent_primitive_float_extreme_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_13().get(gm, config).take(limit) {
        let (m, e) = SciMantissaAndExponent::<T, _, _>::sci_mantissa_and_exponent(&n);
        println!(
            "{:#x}.sci_mantissa_and_exponent() = {:?}",
            ComparableFloatRef(&n),
            (NiceFloat(m), e)
        );
    }
}

fn demo_float_sci_mantissa_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{}.sci_mantissa() = {}",
            n,
            NiceFloat(SciMantissaAndExponent::<T, _, _>::sci_mantissa(&n))
        );
    }
}

fn demo_float_sci_mantissa_primitive_float_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_mantissa() = {}",
            ComparableFloatRef(&n),
            NiceFloat(SciMantissaAndExponent::<T, _, _>::sci_mantissa(&n))
        );
    }
}

fn demo_float_sci_mantissa_primitive_float_extreme<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{}.sci_mantissa() = {}",
            n,
            NiceFloat(SciMantissaAndExponent::<T, _, _>::sci_mantissa(&n))
        );
    }
}

fn demo_float_sci_mantissa_primitive_float_extreme_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_mantissa() = {}",
            ComparableFloatRef(&n),
            NiceFloat(SciMantissaAndExponent::<T, _, _>::sci_mantissa(&n))
        );
    }
}

fn demo_float_sci_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{}.sci_exponent() = {:?}",
            n,
            SciMantissaAndExponent::<T, _, _>::sci_exponent(&n)
        );
    }
}

fn demo_float_sci_exponent_primitive_float_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_exponent() = {:?}",
            ComparableFloatRef(&n),
            SciMantissaAndExponent::<T, _, _>::sci_exponent(&n)
        );
    }
}

fn demo_float_sci_exponent_primitive_float_extreme<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{}.sci_exponent() = {:?}",
            n,
            SciMantissaAndExponent::<T, _, _>::sci_exponent(&n)
        );
    }
}

fn demo_float_sci_exponent_primitive_float_extreme_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for n in float_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{:#x}.sci_exponent() = {:?}",
            ComparableFloatRef(&n),
            SciMantissaAndExponent::<T, _, _>::sci_exponent(&n)
        );
    }
}

fn demo_float_from_sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for (mantissa, exponent) in primitive_float_signed_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let exponent = i32::exact_from(exponent);
        let n = <&Float as SciMantissaAndExponent<T, i32, Float>>::from_sci_mantissa_and_exponent(
            mantissa, exponent,
        );
        println!(
            "Float::from_sci_mantissa_and_exponent({}, {}) = {}",
            NiceFloat(mantissa),
            exponent,
            n.unwrap()
        );
    }
}

fn demo_float_from_sci_mantissa_and_exponent_primitive_float_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    for (mantissa, exponent) in primitive_float_signed_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let exponent = i32::exact_from(exponent);
        let n = <&Float as SciMantissaAndExponent<T, i32, Float>>::from_sci_mantissa_and_exponent(
            mantissa, exponent,
        );
        println!(
            "Float::from_sci_mantissa_and_exponent({}, {}) = {:#x}",
            NiceFloat(mantissa),
            exponent,
            ComparableFloat(n.unwrap())
        );
    }
}

fn benchmark_sci_mantissa_and_exponent_round<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sci_mantissa_and_exponent_round(RoundingMode)",
        BenchmarkType::Single,
        float_rounding_mode_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, rm)| {
            no_out!(x.sci_mantissa_and_exponent_round::<T>(rm))
        })],
    );
}

fn benchmark_float_sci_mantissa_and_exponent_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sci_mantissa_and_exponent()",
        BenchmarkType::EvaluationStrategy,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.sci_mantissa_and_exponent()", &mut |x| {
                no_out!(SciMantissaAndExponent::<Float, _, _>::sci_mantissa_and_exponent(x))
            }),
            ("(&Float).sci_mantissa_and_exponent()", &mut |x| {
                no_out!(
                    <&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa_and_exponent(&x)
                )
            }),
        ],
    );
}

fn benchmark_float_sci_mantissa_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sci_mantissa()",
        BenchmarkType::EvaluationStrategy,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.sci_mantissa()", &mut |x| {
                no_out!(SciMantissaAndExponent::<Float, _, _>::sci_mantissa(x))
            }),
            ("(&Float).sci_mantissa()", &mut |x| {
                no_out!(<&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa(&x))
            }),
        ],
    );
}

fn benchmark_float_sci_exponent_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sci_exponent()",
        BenchmarkType::EvaluationStrategy,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.sci_exponent()", &mut |x| {
                no_out!(SciMantissaAndExponent::<Float, _, _>::sci_exponent(x))
            }),
            ("(&Float).sci_exponent()", &mut |x| {
                no_out!(<&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&x))
            }),
        ],
    );
}

fn benchmark_float_from_sci_mantissa_and_exponent_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_sci_mantissa_and_exponent(Float, i32)",
        BenchmarkType::EvaluationStrategy,
        float_signed_pair_gen_var_1::<i32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float::from_sci_mantissa_and_exponent(Float, i32)",
                &mut |(mantissa, exponent)| {
                    no_out!(<Float as SciMantissaAndExponent::<
                        Float,
                        i32,
                        Float,
                    >>::from_sci_mantissa_and_exponent(
                        mantissa, exponent
                    ))
                },
            ),
            (
                "(&Float)::from_sci_mantissa_and_exponent(Float, i32)",
                &mut |(mantissa, exponent)| {
                    no_out!(<&Float as SciMantissaAndExponent::<
                        Float,
                        i32,
                        Float,
                    >>::from_sci_mantissa_and_exponent(
                        mantissa, exponent
                    ))
                },
            ),
        ],
    );
}

fn benchmark_float_sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    run_benchmark(
        "Float.sci_mantissa_and_exponent()",
        BenchmarkType::Single,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(SciMantissaAndExponent::<T, _, _>::sci_mantissa_and_exponent(&x))
        })],
    );
}

fn benchmark_float_sci_mantissa_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    run_benchmark(
        "Float.sci_mantissa()",
        BenchmarkType::Single,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(SciMantissaAndExponent::<T, _, _>::sci_mantissa(&x))
        })],
    );
}

fn benchmark_float_sci_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    run_benchmark(
        "Float.sci_exponent()",
        BenchmarkType::Single,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(SciMantissaAndExponent::<T, _, _>::sci_exponent(&x))
        })],
    );
}

fn benchmark_float_from_sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    run_benchmark(
        "Float::from_sci_mantissa_and_exponent(Float, i32)",
        BenchmarkType::Single,
        primitive_float_signed_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("mantissa"),
        &mut [(
            &format!("Float::from_sci_mantissa_and_exponent({}, i32)", T::NAME),
            &mut |(mantissa, exponent)| {
                let exponent = i32::exact_from(exponent);
                no_out!(<&Float as SciMantissaAndExponent::<
                        T,
                        i32,
                        Float,
                    >>::from_sci_mantissa_and_exponent(
                        mantissa, exponent
                    )
                )
            },
        )],
    );
}
