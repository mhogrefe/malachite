// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloatRef;
use malachite_float::Float;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::{
    float_primitive_float_pair_gen, float_primitive_float_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_float_eq_abs_primitive_float);
    register_primitive_float_demos!(runner, demo_float_eq_abs_primitive_float_debug);
    register_primitive_float_demos!(runner, demo_float_eq_abs_primitive_float_extreme);
    register_primitive_float_demos!(runner, demo_float_eq_abs_primitive_float_extreme_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_eq_abs_float);
    register_primitive_float_demos!(runner, demo_primitive_float_eq_abs_float_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_eq_abs_float_extreme);
    register_primitive_float_demos!(runner, demo_primitive_float_eq_abs_float_extreme_debug);

    register_primitive_float_benches!(runner, benchmark_float_eq_abs_primitive_float_algorithms);
    register_primitive_float_benches!(runner, benchmark_primitive_float_eq_abs_float_algorithms);
}

fn demo_float_eq_abs_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: EqAbs<T>,
{
    for (x, y) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_abs(&y) {
            println!("|{}| = |{}|", x, NiceFloat(y));
        } else {
            println!("|{}| ≠ |{}|", x, NiceFloat(y));
        }
    }
}

fn demo_float_eq_abs_primitive_float_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: EqAbs<T>,
{
    for (x, y) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        if x.eq_abs(&y) {
            println!("|{:#x}| = |{}|", cx, NiceFloat(y));
        } else {
            println!("|{:#x}| ≠ |{}|", cx, NiceFloat(y));
        }
    }
}

fn demo_float_eq_abs_primitive_float_extreme<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: EqAbs<T>,
{
    for (x, y) in float_primitive_float_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_abs(&y) {
            println!("|{}| = |{}|", x, NiceFloat(y));
        } else {
            println!("|{}| ≠ |{}|", x, NiceFloat(y));
        }
    }
}

fn demo_float_eq_abs_primitive_float_extreme_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: EqAbs<T>,
{
    for (x, y) in float_primitive_float_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        if x.eq_abs(&y) {
            println!("|{:#x}| = |{}|", cx, NiceFloat(y));
        } else {
            println!("|{:#x}| ≠ |{}|", cx, NiceFloat(y));
        }
    }
}

fn demo_primitive_float_eq_abs_float<T: EqAbs<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_abs(&y) {
            println!("|{}| = |{}|", NiceFloat(x), y);
        } else {
            println!("|{}| ≠ |{}|", NiceFloat(x), y);
        }
    }
}

fn demo_primitive_float_eq_abs_float_debug<T: EqAbs<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cy = ComparableFloatRef(&y);
        if x.eq_abs(&y) {
            println!("|{}| = |{:#x}|", NiceFloat(x), cy);
        } else {
            println!("|{}| ≠ |{:#x}|", NiceFloat(x), cy);
        }
    }
}

fn demo_primitive_float_eq_abs_float_extreme<T: EqAbs<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_primitive_float_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_abs(&y) {
            println!("|{}| = |{}|", NiceFloat(x), y);
        } else {
            println!("|{}| ≠ |{}|", NiceFloat(x), y);
        }
    }
}

fn demo_primitive_float_eq_abs_float_extreme_debug<T: EqAbs<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_primitive_float_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cy = ComparableFloatRef(&y);
        if x.eq_abs(&y) {
            println!("|{}| = |{:#x}|", NiceFloat(x), cy);
        } else {
            println!("|{}| ≠ |{:#x}|", NiceFloat(x), cy);
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_eq_abs_primitive_float_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: EqAbs<T> + PartialEq<T>,
{
    run_benchmark(
        &format!("Float.eq_abs(&{})", T::NAME),
        BenchmarkType::Algorithms,
        float_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.eq_abs(&y))),
            ("using abs", &mut |(x, y)| no_out!(x.abs() == y.abs())),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_primitive_float_eq_abs_float_algorithms<
    T: EqAbs<Float> + PartialEq<Float> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.eq_abs(&Float)", T::NAME),
        BenchmarkType::Algorithms,
        float_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_float_max_complexity_bucketer("y", "x"),
        &mut [
            ("default", &mut |(y, x)| no_out!(x.eq_abs(&y))),
            ("using abs", &mut |(y, x)| no_out!(x.abs() == y.abs())),
        ],
    );
}
