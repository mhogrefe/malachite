// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::{
    float_primitive_float_pair_gen, float_primitive_float_pair_gen_var_1,
};
use malachite_float::ComparableFloatRef;
use malachite_float::Float;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_float_partial_cmp_abs_primitive_float);
    register_primitive_float_demos!(runner, demo_float_partial_cmp_abs_primitive_float_debug);
    register_primitive_float_demos!(runner, demo_float_partial_cmp_abs_primitive_float_extreme);
    register_primitive_float_demos!(
        runner,
        demo_float_partial_cmp_abs_primitive_float_extreme_debug
    );
    register_primitive_float_demos!(runner, demo_primitive_float_partial_cmp_abs_float);
    register_primitive_float_demos!(runner, demo_primitive_float_partial_cmp_abs_float_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_partial_cmp_abs_float_extreme);
    register_primitive_float_demos!(
        runner,
        demo_primitive_float_partial_cmp_abs_float_extreme_debug
    );

    register_primitive_float_benches!(runner, benchmark_float_partial_cmp_abs_primitive_float);
    register_primitive_float_benches!(runner, benchmark_primitive_float_partial_cmp_abs_float);
}

fn demo_float_partial_cmp_abs_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrdAbs<T>,
{
    for (x, y) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        match x.partial_cmp_abs(&y) {
            None => println!("|{}| and |{}| are incomparable", x, NiceFloat(y)),
            Some(Less) => println!("|{}| < |{}|", x, NiceFloat(y)),
            Some(Equal) => println!("|{}| = |{}|", x, NiceFloat(y)),
            Some(Greater) => println!("|{}| > |{}|", x, NiceFloat(y)),
        }
    }
}

fn demo_float_partial_cmp_abs_primitive_float_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrdAbs<T>,
{
    for (x, y) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        match x.partial_cmp_abs(&y) {
            None => println!("{:#x} and |{}| are incomparable", cx, NiceFloat(y)),
            Some(Less) => println!("{:#x} < |{}|", cx, NiceFloat(y)),
            Some(Equal) => println!("{:#x} = |{}|", cx, NiceFloat(y)),
            Some(Greater) => println!("{:#x} > |{}|", cx, NiceFloat(y)),
        }
    }
}

fn demo_float_partial_cmp_abs_primitive_float_extreme<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrdAbs<T>,
{
    for (x, y) in float_primitive_float_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        match x.partial_cmp_abs(&y) {
            None => println!("|{}| and |{}| are incomparable", x, NiceFloat(y)),
            Some(Less) => println!("|{}| < |{}|", x, NiceFloat(y)),
            Some(Equal) => println!("|{}| = |{}|", x, NiceFloat(y)),
            Some(Greater) => println!("|{}| > |{}|", x, NiceFloat(y)),
        }
    }
}

fn demo_float_partial_cmp_abs_primitive_float_extreme_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrdAbs<T>,
{
    for (x, y) in float_primitive_float_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        match x.partial_cmp_abs(&y) {
            None => println!("{:#x} and |{}| are incomparable", cx, NiceFloat(y)),
            Some(Less) => println!("{:#x} < |{}|", cx, NiceFloat(y)),
            Some(Equal) => println!("{:#x} = |{}|", cx, NiceFloat(y)),
            Some(Greater) => println!("{:#x} > |{}|", cx, NiceFloat(y)),
        }
    }
}

fn demo_primitive_float_partial_cmp_abs_float<T: PartialOrdAbs<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        match x.partial_cmp_abs(&y) {
            None => println!("|{}| and |{}| are incomparable", NiceFloat(x), y),
            Some(Less) => println!("|{}| < |{}|", NiceFloat(x), y),
            Some(Equal) => println!("|{}| = |{}|", NiceFloat(x), y),
            Some(Greater) => println!("|{}| > |{}|", NiceFloat(x), y),
        }
    }
}

fn demo_primitive_float_partial_cmp_abs_float_debug<T: PartialOrdAbs<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cy = ComparableFloatRef(&y);
        match x.partial_cmp_abs(&y) {
            None => println!("|{}| and {:#x} are incomparable", NiceFloat(x), cy),
            Some(Less) => println!("|{}| < {:#x}", NiceFloat(x), cy),
            Some(Equal) => println!("|{}| = {:#x}", NiceFloat(x), cy),
            Some(Greater) => println!("|{}| > {:#x}", NiceFloat(x), cy),
        }
    }
}

fn demo_primitive_float_partial_cmp_abs_float_extreme<T: PartialOrdAbs<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_primitive_float_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        match x.partial_cmp_abs(&y) {
            None => println!("|{}| and |{}| are incomparable", NiceFloat(x), y),
            Some(Less) => println!("|{}| < |{}|", NiceFloat(x), y),
            Some(Equal) => println!("|{}| = |{}|", NiceFloat(x), y),
            Some(Greater) => println!("|{}| > |{}|", NiceFloat(x), y),
        }
    }
}

fn demo_primitive_float_partial_cmp_abs_float_extreme_debug<
    T: PartialOrdAbs<Float> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_primitive_float_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cy = ComparableFloatRef(&y);
        match x.partial_cmp_abs(&y) {
            None => println!("|{}| and {:#x} are incomparable", NiceFloat(x), cy),
            Some(Less) => println!("|{}| < {:#x}", NiceFloat(x), cy),
            Some(Equal) => println!("|{}| = {:#x}", NiceFloat(x), cy),
            Some(Greater) => println!("|{}| > {:#x}", NiceFloat(x), cy),
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_partial_cmp_abs_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Float.partial_cmp_abs(&{})", T::NAME),
        BenchmarkType::Single,
        float_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_float_max_complexity_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_primitive_float_partial_cmp_abs_float<T: PartialOrdAbs<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp_abs(&Float)", T::NAME),
        BenchmarkType::Single,
        float_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_float_max_complexity_bucketer("y", "x"),
        &mut [("Malachite", &mut |(y, x)| no_out!(x.partial_cmp_abs(&y)))],
    );
}
