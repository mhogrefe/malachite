// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_q::gaussian_rational::GaussianRational;
use std::fmt::Debug;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_gaussian_rational_try_from_primitive_float);
    register_primitive_float_demos!(
        runner,
        demo_gaussian_rational_convertible_from_primitive_float
    );

    register_primitive_float_benches!(runner, benchmark_gaussian_rational_try_from_primitive_float);
    register_primitive_float_benches!(
        runner,
        benchmark_gaussian_rational_convertible_from_primitive_float
    );
}

fn demo_gaussian_rational_try_from_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: TryFrom<T> + ConvertibleFrom<T>,
    <GaussianRational as TryFrom<T>>::Error: Debug,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "GaussianRational::try_from({}) = {:?}",
            NiceFloat(x),
            GaussianRational::try_from(x)
        );
    }
}

fn demo_gaussian_rational_convertible_from_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: ConvertibleFrom<T>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a GaussianRational",
            NiceFloat(x),
            if GaussianRational::convertible_from(x) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_gaussian_rational_try_from_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianRational: TryFrom<T> + ConvertibleFrom<T>,
{
    run_benchmark(
        &format!("GaussianRational::try_from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            let _ = GaussianRational::try_from(x);
        })],
    );
}

fn benchmark_gaussian_rational_convertible_from_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianRational: ConvertibleFrom<T>,
{
    run_benchmark(
        &format!("GaussianRational::convertible_from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(GaussianRational::convertible_from(x));
        })],
    );
}
