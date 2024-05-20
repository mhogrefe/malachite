// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    pair_2_pair_float_signed_max_complexity_bucketer,
    pair_2_pair_float_unsigned_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_signed_pair_gen, float_signed_pair_gen_rm, float_unsigned_pair_gen,
    float_unsigned_pair_gen_rm,
};
use malachite_float::ComparableFloatRef;
use malachite_float::Float;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_float_partial_cmp_unsigned);
    register_unsigned_demos!(runner, demo_float_partial_cmp_unsigned_debug);
    register_unsigned_demos!(runner, demo_unsigned_partial_cmp_float);
    register_unsigned_demos!(runner, demo_unsigned_partial_cmp_float_debug);
    register_signed_demos!(runner, demo_float_partial_cmp_signed);
    register_signed_demos!(runner, demo_float_partial_cmp_signed_debug);
    register_signed_demos!(runner, demo_signed_partial_cmp_float);
    register_signed_demos!(runner, demo_signed_partial_cmp_float_debug);

    register_unsigned_benches!(
        runner,
        benchmark_float_partial_cmp_unsigned_library_comparison
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_partial_cmp_float_library_comparison
    );
    register_signed_benches!(
        runner,
        benchmark_float_partial_cmp_signed_library_comparison
    );
    register_signed_benches!(
        runner,
        benchmark_signed_partial_cmp_float_library_comparison
    );
}

fn demo_float_partial_cmp_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
{
    for (x, y) in float_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_float_partial_cmp_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
{
    for (x, y) in float_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        match x.partial_cmp(&y) {
            None => println!("{cx:#x} and {y:#x} are incomparable"),
            Some(Less) => println!("{cx:#x} < {y:#x}"),
            Some(Equal) => println!("{cx:#x} = {y:#x}"),
            Some(Greater) => println!("{cx:#x} > {y:#x}"),
        }
    }
}

fn demo_unsigned_partial_cmp_float<T: PartialOrd<Float> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_unsigned_partial_cmp_float_debug<T: PartialOrd<Float> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        match x.partial_cmp(&y) {
            None => println!("{x:x} and {cy:#x} are incomparable"),
            Some(Less) => println!("{x:#x} < {cy:#x}"),
            Some(Equal) => println!("{x:#x} = {cy:#x}"),
            Some(Greater) => println!("{x:#x} > {cy:#x}"),
        }
    }
}

fn demo_float_partial_cmp_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: PartialOrd<T>,
{
    for (x, y) in float_signed_pair_gen::<T>().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_float_partial_cmp_signed_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
{
    for (x, y) in float_signed_pair_gen::<T>().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        match x.partial_cmp(&y) {
            None => println!("{cx:#x} and {y:#x} are incomparable"),
            Some(Less) => println!("{cx:#x} < {y:#x}"),
            Some(Equal) => println!("{cx:#x} = {y:#x}"),
            Some(Greater) => println!("{cx:#x} > {y:#x}"),
        }
    }
}

fn demo_signed_partial_cmp_float<T: PartialOrd<Float> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_signed_pair_gen::<T>().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Less) => println!("{x} < {y}"),
            Some(Equal) => println!("{x} = {y}"),
            Some(Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_signed_partial_cmp_float_debug<T: PartialOrd<Float> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_signed_pair_gen::<T>().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        match x.partial_cmp(&y) {
            None => println!("{x:x} and {cy:#x} are incomparable"),
            Some(Less) => println!("{x:#x} < {cy:#x}"),
            Some(Equal) => println!("{x:#x} = {cy:#x}"),
            Some(Greater) => println!("{x:#x} > {cy:#x}"),
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_partial_cmp_unsigned_library_comparison<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: PartialOrd<T>,
    rug::Float: PartialOrd<T>,
{
    run_benchmark(
        &format!("Float.partial_cmp(&{})", T::NAME),
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_unsigned_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.partial_cmp(&y))),
            ("rug", &mut |((x, y), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_unsigned_partial_cmp_float_library_comparison<
    T: PartialOrd<Float> + PartialOrd<rug::Float> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp(&Float)", T::NAME),
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_unsigned_max_complexity_bucketer("y", "x"),
        &mut [
            ("Malachite", &mut |(_, (y, x))| no_out!(x.partial_cmp(&y))),
            ("rug", &mut |((y, x), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_partial_cmp_signed_library_comparison<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: PartialOrd<T>,
    rug::Float: PartialOrd<T>,
{
    run_benchmark(
        &format!("Float.partial_cmp(&{})", T::NAME),
        BenchmarkType::LibraryComparison,
        float_signed_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_signed_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.partial_cmp(&y))),
            ("rug", &mut |((x, y), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_signed_partial_cmp_float_library_comparison<
    T: PartialOrd<Float> + PartialOrd<rug::Float> + PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp(&Float)", T::NAME),
        BenchmarkType::LibraryComparison,
        float_signed_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_signed_max_complexity_bucketer("y", "x"),
        &mut [
            ("Malachite", &mut |(_, (y, x))| no_out!(x.partial_cmp(&y))),
            ("rug", &mut |((y, x), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}
