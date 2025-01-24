// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    pair_1_bit_bucketer, pair_1_primitive_float_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_rounding_mode_pair_gen_var_3, signed_rounding_mode_pair_gen_var_4,
    unsigned_rounding_mode_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_primitive_float_demos!(
        runner,
        demo_primitive_int_rounding_from_primitive_float
    );
    register_primitive_float_unsigned_demos!(runner, demo_primitive_float_rounding_from_unsigned);
    register_primitive_float_signed_demos!(runner, demo_primitive_float_rounding_from_signed);

    register_primitive_int_primitive_float_benches!(
        runner,
        benchmark_primitive_int_rounding_from_primitive_float
    );
    register_primitive_float_unsigned_benches!(
        runner,
        benchmark_primitive_float_rounding_from_unsigned
    );
    register_primitive_float_signed_benches!(
        runner,
        benchmark_primitive_float_rounding_from_signed
    );
}

fn demo_primitive_int_rounding_from_primitive_float<
    T: ConvertibleFrom<U> + PrimitiveInt + RoundingFrom<U>,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, rm) in primitive_float_rounding_mode_pair_gen_var_3::<U, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::rounding_from({}, {}) = {:?}",
            T::NAME,
            NiceFloat(f),
            rm,
            T::rounding_from(f, rm)
        );
    }
}

fn demo_primitive_float_rounding_from_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, rm) in unsigned_rounding_mode_pair_gen_var_2::<U, T>()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = T::rounding_from(u, rm);
        println!(
            "{}::rounding_from({}, {}) = {:?}",
            T::NAME,
            u,
            rm,
            (NiceFloat(x), o)
        );
    }
}

fn demo_primitive_float_rounding_from_signed<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (i, rm) in signed_rounding_mode_pair_gen_var_4::<U, T>()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = T::rounding_from(i, rm);
        println!(
            "{}::rounding_from({}, {}) = {:?}",
            T::NAME,
            i,
            rm,
            (NiceFloat(x), o)
        );
    }
}

fn benchmark_primitive_int_rounding_from_primitive_float<
    T: ConvertibleFrom<U> + PrimitiveInt + RoundingFrom<U>,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rounding_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        primitive_float_rounding_mode_pair_gen_var_3::<U, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |(f, rm)| no_out!(T::rounding_from(f, rm)))],
    );
}

fn benchmark_primitive_float_rounding_from_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rounding_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_rounding_mode_pair_gen_var_2::<U, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("u"),
        &mut [("Malachite", &mut |(u, rm)| no_out!(T::rounding_from(u, rm)))],
    );
}

fn benchmark_primitive_float_rounding_from_signed<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rounding_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_rounding_mode_pair_gen_var_4::<U, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("i"),
        &mut [("Malachite", &mut |(i, rm)| no_out!(T::rounding_from(i, rm)))],
    );
}
