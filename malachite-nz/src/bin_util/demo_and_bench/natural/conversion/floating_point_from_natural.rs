// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::from::UnsignedFromFloatError;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::conversion::primitive_float_from_natural::PrimitiveFloatFromNaturalError;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_gen, natural_gen_var_3, natural_rounding_mode_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_float_rounding_from_natural);
    register_primitive_float_demos!(runner, demo_float_try_from_natural);
    register_primitive_float_demos!(runner, demo_float_exact_from_natural);
    register_primitive_float_demos!(runner, demo_float_convertible_from_natural);

    register_primitive_float_benches!(runner, benchmark_float_rounding_from_natural);
    register_primitive_float_benches!(runner, benchmark_float_try_from_natural);
    register_primitive_float_benches!(runner, benchmark_float_exact_from_natural);
    register_primitive_float_benches!(runner, benchmark_float_convertible_from_natural);
}

fn demo_float_rounding_from_natural<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat + for<'a> RoundingFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, rm) in natural_rounding_mode_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = T::rounding_from(&n, rm);
        println!(
            "{}::rounding_from(&{}, {}) = {:?}",
            T::NAME,
            n,
            rm,
            (NiceFloat(f), o)
        );
    }
}

fn demo_float_try_from_natural<
    T: for<'a> TryFrom<&'a Natural, Error = PrimitiveFloatFromNaturalError> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from(&{}) = {:?}",
            T::NAME,
            n.clone(),
            T::try_from(&n).map(NiceFloat)
        );
    }
}

fn demo_float_exact_from_natural<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
{
    for n in natural_gen_var_3::<T>().get(gm, config).take(limit) {
        println!(
            "{}::exact_from(&{}) = {}",
            T::NAME,
            n.clone(),
            NiceFloat(T::exact_from(&n))
        );
    }
}

fn demo_float_convertible_from_natural<T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, config).take(limit) {
        if T::convertible_from(&n) {
            println!("{} is convertible to an {}", n, T::NAME);
        } else {
            println!("{} is not convertible to an {}", n, T::NAME);
        }
    }
}

fn benchmark_float_rounding_from_natural<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat + for<'a> RoundingFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::rounding_from(Natural, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        natural_rounding_mode_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, rm)| {
            no_out!(T::rounding_from(&n, rm))
        })],
    );
}

fn benchmark_float_try_from_natural<T: for<'a> TryFrom<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::try_from(Natural)", T::NAME),
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(T::try_from(&n).ok()))],
    );
}

fn benchmark_float_exact_from_natural<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
{
    run_benchmark(
        &format!("{}::exact_from(Natural)", T::NAME),
        BenchmarkType::Single,
        natural_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(&n)))],
    );
}

fn benchmark_float_convertible_from_natural<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::convertible_from(Natural)", T::NAME),
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(T::convertible_from(&n)))],
    );
}
