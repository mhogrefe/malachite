// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::conversion::primitive_float_from_float::FloatFromFloatError;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_20,
    float_rounding_mode_pair_gen_var_6,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_primitive_float_rounding_from_float);
    register_primitive_float_demos!(runner, demo_primitive_float_rounding_from_float_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_rounding_from_float_extreme);
    register_primitive_float_demos!(
        runner,
        demo_primitive_float_rounding_from_float_extreme_debug
    );
    register_primitive_float_demos!(runner, demo_primitive_float_rounding_from_float_ref);
    register_primitive_float_demos!(runner, demo_primitive_float_rounding_from_float_ref_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_try_from_float);
    register_primitive_float_demos!(runner, demo_primitive_float_try_from_float_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_try_from_float_extreme);
    register_primitive_float_demos!(runner, demo_primitive_float_try_from_float_extreme_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_try_from_float_ref);
    register_primitive_float_demos!(runner, demo_primitive_float_try_from_float_ref_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_convertible_from_float);
    register_primitive_float_demos!(runner, demo_primitive_float_convertible_from_float_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_convertible_from_float_extreme);
    register_primitive_float_demos!(
        runner,
        demo_primitive_float_convertible_from_float_extreme_debug
    );

    register_primitive_float_benches!(runner, benchmark_primitive_float_try_from_float);
    register_primitive_float_benches!(runner, benchmark_primitive_float_convertible_from_float);
    register_primitive_float_benches!(runner, benchmark_primitive_float_rounding_from_float);
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_rounding_from_float<T: PrimitiveFloat + RoundingFrom<Float>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (x_out, o) = T::rounding_from(x.clone(), rm);
        println!(
            "{}::rounding_from({}, {}) = ({}, {:?})",
            T::NAME,
            x,
            rm,
            NiceFloat(x_out),
            o
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_rounding_from_float_debug<T: PrimitiveFloat + RoundingFrom<Float>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (x_out, o) = T::rounding_from(x.clone(), rm);
        println!(
            "{}::rounding_from({:#x}, {}) = ({}, {:?})",
            T::NAME,
            ComparableFloat(x),
            rm,
            NiceFloat(x_out),
            o
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_rounding_from_float_extreme<T: PrimitiveFloat + RoundingFrom<Float>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_20::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (x_out, o) = T::rounding_from(x.clone(), rm);
        println!(
            "{}::rounding_from({}, {}) = ({}, {:?})",
            T::NAME,
            x,
            rm,
            NiceFloat(x_out),
            o
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_rounding_from_float_extreme_debug<T: PrimitiveFloat + RoundingFrom<Float>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_20::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (x_out, o) = T::rounding_from(x.clone(), rm);
        println!(
            "{}::rounding_from({:#x}, {}) = ({}, {:?})",
            T::NAME,
            ComparableFloat(x),
            rm,
            NiceFloat(x_out),
            o
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_rounding_from_float_ref<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (x_out, o) = T::rounding_from(&x, rm);
        println!(
            "{}::rounding_from(&{}, {}) = ({}, {:?})",
            T::NAME,
            x,
            rm,
            NiceFloat(x_out),
            o
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_rounding_from_float_ref_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (x_out, o) = T::rounding_from(&x, rm);
        println!(
            "{}::rounding_from(&{:#x}, {}) = ({}, {:?})",
            T::NAME,
            ComparableFloatRef(&x),
            rm,
            NiceFloat(x_out),
            o
        );
    }
}

fn demo_primitive_float_try_from_float<
    T: TryFrom<Float, Error = FloatFromFloatError> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({}) = {:?}",
            T::NAME,
            x.clone(),
            T::try_from(x).map(NiceFloat)
        );
    }
}

fn demo_primitive_float_try_from_float_debug<
    T: TryFrom<Float, Error = FloatFromFloatError> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({:#x}) = {:?}",
            T::NAME,
            ComparableFloat(x.clone()),
            T::try_from(x).map(NiceFloat)
        );
    }
}

fn demo_primitive_float_try_from_float_extreme<
    T: TryFrom<Float, Error = FloatFromFloatError> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "{}::try_from({}) = {:?}",
            T::NAME,
            x.clone(),
            T::try_from(x).map(NiceFloat)
        );
    }
}

fn demo_primitive_float_try_from_float_extreme_debug<
    T: TryFrom<Float, Error = FloatFromFloatError> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "{}::try_from({:#x}) = {:?}",
            T::NAME,
            ComparableFloat(x.clone()),
            T::try_from(x).map(NiceFloat)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_try_from_float_ref<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: TryFrom<&'a Float, Error = FloatFromFloatError>,
{
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({}) = {:?}",
            T::NAME,
            x,
            T::try_from(&x).map(NiceFloat)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_try_from_float_ref_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: TryFrom<&'a Float, Error = FloatFromFloatError>,
{
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({:#x}) = {:?}",
            T::NAME,
            ComparableFloatRef(&x),
            T::try_from(&x).map(NiceFloat)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_convertible_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to an {}",
            f,
            if T::convertible_from(&f) { "" } else { "not " },
            T::NAME
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_convertible_from_float_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{:#x} is {}convertible to an {}",
            ComparableFloatRef(&f),
            if T::convertible_from(&f) { "" } else { "not " },
            T::NAME
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_convertible_from_float_extreme<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for f in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to an {}",
            f,
            if T::convertible_from(&f) { "" } else { "not " },
            T::NAME
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_convertible_from_float_extreme_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for f in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "{:#x} is {}convertible to an {}",
            ComparableFloatRef(&f),
            if T::convertible_from(&f) { "" } else { "not " },
            T::NAME
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_primitive_float_try_from_float<T: TryFrom<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::try_from(Float)", T::NAME),
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(T::try_from(x)))],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_convertible_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    run_benchmark(
        &format!("{}::convertible_from(Float)", T::NAME),
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(T::convertible_from(&x)))],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_rounding_from_float<T: RoundingFrom<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    run_benchmark(
        &format!("{}::rounding_from(Float, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        float_rounding_mode_pair_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, rm)| no_out!(T::rounding_from(x, rm)))],
    );
}
