// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen_var_8;
use malachite_base::test_util::runner::Runner;
use malachite_q::conversion::from_primitive_float::RationalFromPrimitiveFloatError;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_rational_try_from_float_simplest);
    register_primitive_float_benches!(runner, benchmark_rational_try_from_float_simplest);
}

fn demo_rational_try_from_float_simplest<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Rational: TryFrom<T, Error = RationalFromPrimitiveFloatError>,
{
    for f in primitive_float_gen_var_8::<T>().get(gm, config).take(limit) {
        println!(
            "Rational::try_from_float_simplest({}) = {:?}",
            NiceFloat(f),
            Rational::try_from_float_simplest(f)
        );
    }
}

fn benchmark_rational_try_from_float_simplest<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: TryFrom<T, Error = RationalFromPrimitiveFloatError>,
{
    run_benchmark(
        &format!("Rational::try_from_float_simplest({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_8::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| {
            no_out!(Rational::try_from_float_simplest(f).ok())
        })],
    );
}
