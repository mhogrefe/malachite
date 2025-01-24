// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    primitive_float_bucketer, signed_bit_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_unsigned_demos!(runner, demo_primitive_int_convertible_from_unsigned);
    register_primitive_int_signed_demos!(runner, demo_primitive_int_convertible_from_signed);
    register_primitive_int_primitive_float_demos!(
        runner,
        demo_primitive_int_convertible_from_primitive_float
    );
    register_primitive_float_unsigned_demos!(
        runner,
        demo_primitive_float_convertible_from_unsigned
    );
    register_primitive_float_signed_demos!(runner, demo_primitive_float_convertible_from_signed);

    register_primitive_int_unsigned_benches!(
        runner,
        benchmark_primitive_int_convertible_from_unsigned
    );
    register_primitive_int_signed_benches!(runner, benchmark_primitive_int_convertible_from_signed);
    register_primitive_int_primitive_float_benches!(
        runner,
        benchmark_primitive_int_convertible_from_primitive_float
    );
    register_primitive_float_unsigned_benches!(
        runner,
        benchmark_primitive_float_convertible_from_unsigned
    );
    register_primitive_float_signed_benches!(
        runner,
        benchmark_primitive_float_convertible_from_signed
    );
}

fn demo_primitive_int_convertible_from_unsigned<
    T: ConvertibleFrom<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<U>().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            u,
            if T::convertible_from(u) { "" } else { "not " },
            T::NAME,
        );
    }
}

fn demo_primitive_int_convertible_from_signed<T: ConvertibleFrom<U> + Named, U: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for i in signed_gen::<U>().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            i,
            if T::convertible_from(i) { "" } else { "not " },
            T::NAME,
        );
    }
}

fn demo_primitive_int_convertible_from_primitive_float<
    T: ConvertibleFrom<U> + PrimitiveInt,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen::<U>().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            NiceFloat(x),
            if T::convertible_from(x) { "" } else { "not " },
            T::NAME,
        );
    }
}

fn demo_primitive_float_convertible_from_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<U>().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            u,
            if T::convertible_from(u) { "" } else { "not " },
            T::NAME,
        );
    }
}

fn demo_primitive_float_convertible_from_signed<
    T: ConvertibleFrom<U> + PrimitiveFloat,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for i in signed_gen::<U>().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            i,
            if T::convertible_from(i) { "" } else { "not " },
            T::NAME,
        );
    }
}

fn benchmark_primitive_int_convertible_from_unsigned<
    T: ConvertibleFrom<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.convertible_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::convertible_from(n)))],
    );
}

fn benchmark_primitive_int_convertible_from_signed<
    T: ConvertibleFrom<U> + Named,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.convertible_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::convertible_from(n)))],
    );
}

fn benchmark_primitive_int_convertible_from_primitive_float<
    T: ConvertibleFrom<U> + PrimitiveInt,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.convertible_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(T::convertible_from(n)))],
    );
}

fn benchmark_primitive_float_convertible_from_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.convertible_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::convertible_from(n)))],
    );
}

fn benchmark_primitive_float_convertible_from_signed<
    T: ConvertibleFrom<U> + PrimitiveFloat,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.convertible_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::convertible_from(n)))],
    );
}
