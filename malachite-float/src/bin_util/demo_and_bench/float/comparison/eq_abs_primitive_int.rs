// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloatRef;
use malachite_float::Float;
use malachite_float::test_util::bench::bucketers::{
    pair_float_signed_max_complexity_bucketer, pair_float_unsigned_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_signed_pair_gen, float_signed_pair_gen_var_4, float_unsigned_pair_gen,
    float_unsigned_pair_gen_var_5,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_float_eq_abs_unsigned);
    register_unsigned_demos!(runner, demo_float_eq_abs_unsigned_debug);
    register_unsigned_demos!(runner, demo_float_eq_abs_unsigned_extreme);
    register_unsigned_demos!(runner, demo_float_eq_abs_unsigned_extreme_debug);
    register_unsigned_demos!(runner, demo_unsigned_eq_abs_float);
    register_unsigned_demos!(runner, demo_unsigned_eq_abs_float_debug);
    register_unsigned_demos!(runner, demo_unsigned_eq_abs_float_extreme);
    register_unsigned_demos!(runner, demo_unsigned_eq_abs_float_extreme_debug);
    register_signed_demos!(runner, demo_float_eq_abs_signed);
    register_signed_demos!(runner, demo_float_eq_abs_signed_debug);
    register_signed_demos!(runner, demo_float_eq_abs_signed_extreme);
    register_signed_demos!(runner, demo_float_eq_abs_signed_extreme_debug);
    register_signed_demos!(runner, demo_signed_eq_abs_float);
    register_signed_demos!(runner, demo_signed_eq_abs_float_debug);
    register_signed_demos!(runner, demo_signed_eq_abs_float_extreme);
    register_signed_demos!(runner, demo_signed_eq_abs_float_extreme_debug);

    register_unsigned_benches!(runner, benchmark_float_eq_abs_unsigned_algorithms);
    register_unsigned_benches!(runner, benchmark_unsigned_eq_abs_float_algorithms);
    register_signed_benches!(runner, benchmark_float_eq_abs_signed_algorithms);
    register_signed_benches!(runner, benchmark_signed_eq_abs_float_algorithms);
}

fn demo_float_eq_abs_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: EqAbs<T>,
{
    for (x, y) in float_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_float_eq_abs_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: EqAbs<T>,
{
    for (x, y) in float_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        if x.eq_abs(&y) {
            println!("|{cx:#x}| = |{y:x}|");
        } else {
            println!("|{cx:#x}| ≠ |{y:x}|");
        }
    }
}

fn demo_float_eq_abs_unsigned_extreme<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: EqAbs<T>,
{
    for (x, y) in float_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_float_eq_abs_unsigned_extreme_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: EqAbs<T>,
{
    for (x, y) in float_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        if x.eq_abs(&y) {
            println!("|{cx:#x}| = |{y:x}|");
        } else {
            println!("|{cx:#x}| ≠ |{y:x}|");
        }
    }
}

fn demo_unsigned_eq_abs_float<T: EqAbs<Float> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_unsigned_eq_abs_float_debug<T: EqAbs<Float> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        if x.eq_abs(&y) {
            println!("|{x:x}| = |{cy:#x}|");
        } else {
            println!("|{x:x}| ≠ |{cy:#x}|");
        }
    }
}

fn demo_unsigned_eq_abs_float_extreme<T: EqAbs<Float> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_unsigned_eq_abs_float_extreme_debug<T: EqAbs<Float> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cy = ComparableFloatRef(&y);
        if x.eq_abs(&y) {
            println!("|{x:x}| = |{cy:#x}|");
        } else {
            println!("|{x:x}| ≠ |{cy:#x}|");
        }
    }
}

fn demo_float_eq_abs_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: EqAbs<T>,
{
    for (x, y) in float_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_float_eq_abs_signed_debug<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: EqAbs<T>,
{
    for (x, y) in float_signed_pair_gen::<T>().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        if x.eq_abs(&y) {
            println!("|{cx:#x}| = |{y:x}|");
        } else {
            println!("|{cx:#x}| ≠ |{y:x}|");
        }
    }
}

fn demo_float_eq_abs_signed_extreme<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: EqAbs<T>,
{
    for (x, y) in float_signed_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_float_eq_abs_signed_extreme_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: EqAbs<T>,
{
    for (x, y) in float_signed_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        if x.eq_abs(&y) {
            println!("|{cx:#x}| = |{y:x}|");
        } else {
            println!("|{cx:#x}| ≠ |{y:x}|");
        }
    }
}

fn demo_signed_eq_abs_float<T: EqAbs<Float> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_signed_eq_abs_float_debug<T: EqAbs<Float> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_signed_pair_gen::<T>().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        if x.eq_abs(&y) {
            println!("|{x:x}| = |{cy:#x}|");
        } else {
            println!("|{x:x}| ≠ |{cy:#x}|");
        }
    }
}

fn demo_signed_eq_abs_float_extreme<T: EqAbs<Float> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_signed_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_signed_eq_abs_float_extreme_debug<T: EqAbs<Float> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x) in float_signed_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let cy = ComparableFloatRef(&y);
        if x.eq_abs(&y) {
            println!("|{x:x}| = |{cy:#x}|");
        } else {
            println!("|{x:x}| ≠ |{cy:#x}|");
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_eq_abs_unsigned_algorithms<T: PrimitiveUnsigned>(
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
        float_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_unsigned_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.eq_abs(&y))),
            ("using abs", &mut |(x, y)| no_out!(x.abs() == y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_unsigned_eq_abs_float_algorithms<
    T: EqAbs<Float> + PartialEq<Float> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.eq_abs(&Float)", T::NAME),
        BenchmarkType::Algorithms,
        float_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_unsigned_max_complexity_bucketer("y", "x"),
        &mut [
            ("default", &mut |(y, x)| no_out!(x.eq_abs(&y))),
            ("using abs", &mut |(y, x)| no_out!(x == y.abs())),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_eq_abs_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: EqAbs<T> + PartialEq<<T as UnsignedAbs>::Output>,
{
    run_benchmark(
        &format!("Float.eq_abs(&{})", T::NAME),
        BenchmarkType::Algorithms,
        float_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_signed_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.eq_abs(&y))),
            ("using abs", &mut |(x, y)| {
                no_out!(x.abs() == y.unsigned_abs());
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_signed_eq_abs_float_algorithms<T: EqAbs<Float> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    <T as UnsignedAbs>::Output: PartialEq<Float>,
{
    run_benchmark(
        &format!("{}.eq_abs(&Float)", T::NAME),
        BenchmarkType::Algorithms,
        float_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_signed_max_complexity_bucketer("y", "x"),
        &mut [
            ("default", &mut |(y, x)| no_out!(x.eq_abs(&y))),
            ("using abs", &mut |(y, x)| {
                no_out!(x.unsigned_abs() == y.abs());
            }),
        ],
    );
}
