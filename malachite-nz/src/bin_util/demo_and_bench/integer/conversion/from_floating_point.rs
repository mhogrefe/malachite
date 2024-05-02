// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::from::SignedFromFloatError;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    pair_1_primitive_float_bucketer, primitive_float_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_gen_var_5, primitive_float_rounding_mode_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_integer_rounding_from_float);
    register_primitive_float_demos!(runner, demo_integer_try_from_float);
    register_primitive_float_demos!(runner, demo_integer_exact_from_float);
    register_primitive_float_demos!(runner, demo_integer_convertible_from_float);

    register_primitive_float_benches!(runner, benchmark_integer_rounding_from_float);
    register_primitive_float_benches!(runner, benchmark_integer_try_from_float);
    register_primitive_float_benches!(runner, benchmark_integer_exact_from_float);
    register_primitive_float_benches!(runner, benchmark_integer_convertible_from_float_algorithms);
}

fn demo_integer_rounding_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: RoundingFrom<T>,
{
    for (f, rm) in primitive_float_rounding_mode_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Integer::rounding_from({}, {}) = {:?}",
            NiceFloat(f),
            rm,
            Integer::rounding_from(f, rm)
        );
    }
}

fn demo_integer_try_from_float<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: TryFrom<T, Error = SignedFromFloatError>,
{
    for f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "Integer::try_from({}) = {:?}",
            NiceFloat(f),
            Integer::try_from(f)
        );
    }
}

fn demo_integer_exact_from_float<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: ExactFrom<T>,
{
    for f in primitive_float_gen_var_5::<T>().get(gm, config).take(limit) {
        println!(
            "Integer::exact_from({}) = {}",
            NiceFloat(f),
            Integer::exact_from(f)
        );
    }
}

fn demo_integer_convertible_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: ConvertibleFrom<T>,
{
    for f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        if Integer::convertible_from(f) {
            println!("{} is convertible to a Integer", NiceFloat(f));
        } else {
            println!("{} is not convertible to a Integer", NiceFloat(f));
        }
    }
}

fn benchmark_integer_rounding_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: RoundingFrom<T>,
{
    run_benchmark(
        &format!("Integer::rounding_from({}, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        primitive_float_rounding_mode_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |(f, rm)| {
            no_out!(Integer::rounding_from(f, rm))
        })],
    );
}

fn benchmark_integer_try_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: TryFrom<T>,
{
    run_benchmark(
        &format!("Integer::try_from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Integer::try_from(f).ok()))],
    );
}

fn benchmark_integer_exact_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: ExactFrom<T>,
{
    run_benchmark(
        &format!("Integer::exact_from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Integer::exact_from(f)))],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_convertible_from_float_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: TryFrom<T> + ConvertibleFrom<T>,
{
    run_benchmark(
        &format!("Integer::convertible_from({})", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("standard", &mut |f| no_out!(Integer::convertible_from(f))),
            ("using try_from", &mut |f| {
                no_out!(Integer::try_from(f).is_ok())
            }),
        ],
    );
}
