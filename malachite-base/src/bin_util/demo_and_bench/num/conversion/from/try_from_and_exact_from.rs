// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    primitive_float_bucketer, signed_bit_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_gen_var_13, primitive_float_gen_var_14, signed_gen,
    signed_gen_var_2, signed_gen_var_7, unsigned_gen, unsigned_gen_var_18,
};
use malachite_base::test_util::runner::Runner;
use std::fmt::{Debug, Display};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_primitive_float_demos!(
        runner,
        demo_primitive_int_try_from_primitive_float
    );
    register_unsigned_primitive_float_demos!(runner, demo_primitive_float_try_from_unsigned);
    register_signed_primitive_float_demos!(runner, demo_primitive_float_try_from_signed);

    register_primitive_int_unsigned_demos!(runner, demo_primitive_int_exact_from_unsigned);
    register_primitive_int_signed_demos!(runner, demo_primitive_int_exact_from_signed);
    register_unsigned_primitive_float_demos!(runner, demo_unsigned_exact_from_primitive_float);
    register_signed_primitive_float_demos!(runner, demo_signed_exact_from_primitive_float);
    register_primitive_float_unsigned_demos!(runner, demo_primitive_float_exact_from_unsigned);
    register_primitive_float_signed_demos!(runner, demo_primitive_float_exact_from_signed);

    register_primitive_int_primitive_float_benches!(
        runner,
        benchmark_primitive_int_try_from_primitive_float
    );
    register_primitive_float_unsigned_benches!(runner, benchmark_primitive_float_try_from_unsigned);
    register_primitive_float_signed_benches!(runner, benchmark_primitive_float_try_from_signed);

    register_primitive_int_unsigned_benches!(runner, benchmark_primitive_int_exact_from_unsigned);
    register_primitive_int_signed_benches!(runner, benchmark_primitive_int_exact_from_signed);
    register_unsigned_primitive_float_benches!(
        runner,
        benchmark_unsigned_exact_from_primitive_float
    );
    register_signed_primitive_float_benches!(runner, benchmark_signed_exact_from_primitive_float);
    register_primitive_float_unsigned_benches!(
        runner,
        benchmark_primitive_float_exact_from_unsigned
    );
    register_primitive_float_signed_benches!(runner, benchmark_primitive_float_exact_from_signed);
}

fn demo_primitive_int_try_from_primitive_float<
    T: TryFrom<NiceFloat<U>> + Debug + Named,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    <T as TryFrom<NiceFloat<U>>>::Error: Debug,
{
    for f in primitive_float_gen::<U>().get(gm, config).take(limit) {
        let f = NiceFloat(f);
        println!("{}::try_from({}) = {:?}", T::NAME, f, T::try_from(f));
    }
}

fn demo_primitive_float_try_from_unsigned<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NiceFloat<U>: TryFrom<T>,
    <NiceFloat<U> as TryFrom<T>>::Error: Debug,
{
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!(
            "{}::try_from({}) = {:?}",
            U::NAME,
            u,
            NiceFloat::<U>::try_from(u)
        );
    }
}

fn demo_primitive_float_try_from_signed<T: PrimitiveSigned, U: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NiceFloat<U>: TryFrom<T>,
    <NiceFloat<U> as TryFrom<T>>::Error: Debug,
{
    for u in signed_gen::<T>().get(gm, config).take(limit) {
        println!(
            "{}::try_from({}) = {:?}",
            U::NAME,
            u,
            NiceFloat::<U>::try_from(u)
        );
    }
}

fn demo_primitive_int_exact_from_unsigned<T: TryFrom<U> + Display + Named, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<U>().get(gm, config).take(limit) {
        println!("{}::exact_from({}) = {}", T::NAME, u, T::exact_from(u));
    }
}

fn demo_primitive_int_exact_from_signed<T: TryFrom<U> + Display + Named, U: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for i in signed_gen_var_2::<U>().get(gm, config).take(limit) {
        println!("{}::exact_from({}) = {}", T::NAME, i, T::exact_from(i));
    }
}

fn demo_unsigned_exact_from_primitive_float<
    T: TryFrom<NiceFloat<U>> + PrimitiveUnsigned,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NiceFloat<U>: TryFrom<T>,
{
    for f in primitive_float_gen_var_13::<U, T>()
        .get(gm, config)
        .take(limit)
    {
        let f = NiceFloat(f);
        println!("{}::exact_from({}) = {}", T::NAME, f, T::exact_from(f));
    }
}

fn demo_signed_exact_from_primitive_float<
    T: TryFrom<NiceFloat<U>> + PrimitiveSigned + RoundingFrom<U>,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NiceFloat<U>: TryFrom<T>,
{
    for f in primitive_float_gen_var_14::<U, T>()
        .get(gm, config)
        .take(limit)
    {
        let f = NiceFloat(f);
        println!("{}::exact_from({}) = {}", T::NAME, f, T::exact_from(f));
    }
}

fn demo_primitive_float_exact_from_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned + RoundingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NiceFloat<T>: TryFrom<U>,
{
    for u in unsigned_gen_var_18::<U, T>().get(gm, config).take(limit) {
        println!(
            "{}::exact_from({}) = {}",
            T::NAME,
            u,
            NiceFloat::<T>::exact_from(u)
        );
    }
}

fn demo_primitive_float_exact_from_signed<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned + RoundingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NiceFloat<T>: TryFrom<U>,
{
    for i in signed_gen_var_7::<U, T>().get(gm, config).take(limit) {
        println!(
            "{}::exact_from({}) = {}",
            T::NAME,
            i,
            NiceFloat::<T>::exact_from(i)
        );
    }
}

fn benchmark_primitive_int_try_from_primitive_float<
    T: TryFrom<NiceFloat<U>> + Named,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.try_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [(
            "Malachite",
            &mut |n| no_out!(T::try_from(NiceFloat(n)).ok()),
        )],
    );
}

fn benchmark_primitive_float_try_from_unsigned<T: PrimitiveFloat, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NiceFloat<T>: TryFrom<U>,
{
    run_benchmark(
        &format!("{}.try_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(NiceFloat::<T>::try_from(n).ok())
        })],
    );
}

fn benchmark_primitive_float_try_from_signed<T: PrimitiveFloat, U: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NiceFloat<T>: TryFrom<U>,
{
    run_benchmark(
        &format!("{}.try_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(NiceFloat::<T>::try_from(n).ok())
        })],
    );
}

fn benchmark_primitive_int_exact_from_unsigned<T: TryFrom<U> + Named, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(n)))],
    );
}

fn benchmark_primitive_int_exact_from_signed<T: TryFrom<U> + Named, U: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen_var_2::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(n)))],
    );
}

fn benchmark_unsigned_exact_from_primitive_float<
    T: TryFrom<NiceFloat<U>> + PrimitiveUnsigned,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NiceFloat<U>: TryFrom<T>,
{
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_13::<U, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(NiceFloat(n))))],
    );
}

fn benchmark_signed_exact_from_primitive_float<
    T: TryFrom<NiceFloat<U>> + PrimitiveSigned,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NiceFloat<U>: TryFrom<T>,
{
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_14::<U, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(NiceFloat(n))))],
    );
}

fn benchmark_primitive_float_exact_from_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned + RoundingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NiceFloat<T>: TryFrom<U>,
{
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_18::<U, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(NiceFloat::<T>::exact_from(n)))],
    );
}

fn benchmark_primitive_float_exact_from_signed<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned + RoundingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NiceFloat<T>: TryFrom<U>,
{
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen_var_7::<U, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(NiceFloat::<T>::exact_from(n)))],
    );
}
