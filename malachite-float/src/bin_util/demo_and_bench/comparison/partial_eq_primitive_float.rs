// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::{
    float_primitive_float_pair_gen, float_primitive_float_pair_gen_rm,
};
use malachite_float::ComparableFloatRef;
use malachite_float::Float;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_float_partial_eq_primitive_float);
    register_primitive_float_demos!(runner, demo_float_partial_eq_primitive_float_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_partial_eq_float);
    register_primitive_float_demos!(runner, demo_primitive_float_partial_eq_float_debug);

    register_primitive_float_benches!(
        runner,
        benchmark_float_partial_eq_primitive_float_library_comparison
    );
    register_primitive_float_benches!(
        runner,
        benchmark_primitive_float_partial_eq_float_library_comparison
    );
}

fn demo_float_partial_eq_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialEq<T>,
{
    for (x, y) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x == y {
            println!("{} = {}", x, NiceFloat(y));
        } else {
            println!("{} ≠ {}", x, NiceFloat(y));
        }
    }
}

fn demo_float_partial_eq_primitive_float_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialEq<T>,
{
    for (x, y) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        if x == y {
            println!("{:#x} = {}", cx, NiceFloat(y));
        } else {
            println!("{:#x} ≠ {}", cx, NiceFloat(y));
        }
    }
}

fn demo_primitive_float_partial_eq_float<T: PartialEq<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x == y {
            println!("{} = {}", NiceFloat(x), y);
        } else {
            println!("{} ≠ {}", NiceFloat(x), y);
        }
    }
}

fn demo_primitive_float_partial_eq_float_debug<T: PartialEq<Float> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cy = ComparableFloatRef(&y);
        if x == y {
            println!("{} = {:#x}", NiceFloat(x), cy);
        } else {
            println!("{} ≠ {:#x}", NiceFloat(x), cy);
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_partial_eq_primitive_float_library_comparison<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: PartialEq<T>,
    rug::Float: PartialEq<T>,
{
    run_benchmark(
        &format!("Float == {}", T::NAME),
        BenchmarkType::LibraryComparison,
        float_primitive_float_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_primitive_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x == y)),
            ("rug", &mut |((x, y), _)| no_out!(x == y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_primitive_float_partial_eq_float_library_comparison<
    T: PartialEq<Float> + PartialEq<rug::Float> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{} == Float", T::NAME),
        BenchmarkType::LibraryComparison,
        float_primitive_float_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_primitive_float_max_complexity_bucketer("y", "x"),
        &mut [
            ("Malachite", &mut |(_, (y, x))| no_out!(x == y)),
            ("rug", &mut |((y, x), _)| no_out!(x == y)),
        ],
    );
}
