// Copyright © 2025 Mikhail Hogrefe
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
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_q::conversion::from_primitive_float::RationalFromPrimitiveFloatError;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_rational_convertible_from_primitive_float);
    register_primitive_float_demos!(runner, demo_rational_try_from_primitive_float);

    register_primitive_float_benches!(runner, benchmark_rational_try_from_primitive_float);
    register_bench!(runner, benchmark_rational_try_from_f32_library_comparison);
    register_bench!(runner, benchmark_rational_try_from_f64_library_comparison);
    register_primitive_float_benches!(runner, benchmark_rational_convertible_from_primitive_float);
}

fn demo_rational_try_from_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Rational: TryFrom<T, Error = RationalFromPrimitiveFloatError>,
{
    for f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "Rational::try_from({}) = {:?}",
            NiceFloat(f),
            Rational::try_from(f)
        );
    }
}

fn demo_rational_convertible_from_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Rational: ConvertibleFrom<T>,
{
    for f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        if Rational::convertible_from(f) {
            println!("{} is convertible to a Rational", NiceFloat(f));
        } else {
            println!("{} is not convertible to a Rational", NiceFloat(f));
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_rational_try_from_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: TryFrom<T>,
{
    run_benchmark(
        &format!("Rational::try_from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Rational::try_from(f)))],
    );
}

#[allow(unused_must_use)]
fn benchmark_rational_try_from_f32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::try_from(f32)",
        BenchmarkType::LibraryComparison,
        primitive_float_gen::<f32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("Malachite", &mut |f| no_out!(Rational::try_from(f))),
            ("rug", &mut |f| no_out!(rug::Rational::from_f32(f))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_rational_try_from_f64_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::try_from(f64)",
        BenchmarkType::LibraryComparison,
        primitive_float_gen::<f64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("Malachite", &mut |f| no_out!(Rational::try_from(f))),
            ("rug", &mut |f| no_out!(rug::Rational::from_f64(f))),
        ],
    );
}

fn benchmark_rational_convertible_from_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: ConvertibleFrom<T>,
{
    run_benchmark(
        &format!("Rational::convertible_from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Rational::convertible_from(f)))],
    );
}
