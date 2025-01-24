// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;
use std::fmt::Display;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_unsigned_demos!(runner, demo_primitive_int_wrapping_from_unsigned);
    register_primitive_int_signed_demos!(runner, demo_primitive_int_wrapping_from_signed);
    register_primitive_int_unsigned_benches!(
        runner,
        benchmark_primitive_int_wrapping_from_unsigned
    );
    register_primitive_int_signed_benches!(runner, benchmark_primitive_int_wrapping_from_signed);
}

fn demo_primitive_int_wrapping_from_unsigned<
    T: Display + WrappingFrom<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<U>().get(gm, config).take(limit) {
        println!(
            "{}::wrapping_from({}) = {}",
            T::NAME,
            u,
            T::wrapping_from(u)
        );
    }
}

fn demo_primitive_int_wrapping_from_signed<
    T: Display + WrappingFrom<U> + Named,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for i in signed_gen::<U>().get(gm, config).take(limit) {
        println!(
            "{}::wrapping_from({}) = {}",
            T::NAME,
            i,
            T::wrapping_from(i)
        );
    }
}

fn benchmark_primitive_int_wrapping_from_unsigned<
    T: WrappingFrom<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.wrapping_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::wrapping_from(n)))],
    );
}

fn benchmark_primitive_int_wrapping_from_signed<T: WrappingFrom<U> + Named, U: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.wrapping_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::wrapping_from(n)))],
    );
}
