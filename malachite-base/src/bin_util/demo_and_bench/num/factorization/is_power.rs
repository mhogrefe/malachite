// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::factorization::traits::{ExpressAsPower, IsPower};
use malachite_base::test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_generic_demos!(
        runner,
        demo_express_as_power_unsigned,
        u8,
        u16,
        u32,
        u64,
        usize
    );
    register_generic_demos!(
        runner,
        demo_express_as_power_signed,
        i8,
        i16,
        i32,
        i64,
        isize
    );
    register_generic_demos!(runner, demo_is_power_unsigned, u8, u16, u32, u64, usize);
    register_generic_demos!(runner, demo_is_power_signed, i8, i16, i32, i64, isize);

    register_generic_benches!(
        runner,
        benchmark_express_as_power_unsigned,
        u8,
        u16,
        u32,
        u64,
        usize
    );
    register_generic_benches!(
        runner,
        benchmark_express_as_power_signed,
        i8,
        i16,
        i32,
        i64,
        isize
    );
    register_generic_benches!(
        runner,
        benchmark_is_power_unsigned_algorithms,
        u8,
        u16,
        u32,
        u64,
        usize
    );
    register_generic_benches!(
        runner,
        benchmark_is_power_signed_algorithms,
        i8,
        i16,
        i32,
        i64,
        isize
    );
}

fn demo_express_as_power_unsigned<T: PrimitiveUnsigned + ExpressAsPower>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("{}.express_as_power() = {:?}", u, u.express_as_power())
    }
}

fn demo_express_as_power_signed<T: PrimitiveSigned + ExpressAsPower>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!("({}).express_as_power() = {:?}", i, i.express_as_power())
    }
}

fn demo_is_power_unsigned<T: PrimitiveUnsigned + IsPower>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        if u.is_power() {
            println!("{u} is a perfect power");
        } else {
            println!("{u} is not a perfect power");
        }
    }
}

fn demo_is_power_signed<T: PrimitiveSigned + IsPower>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        if i.is_power() {
            println!("{i} is a perfect power");
        } else {
            println!("{i} is not a perfect power");
        }
    }
}

fn benchmark_express_as_power_unsigned<T: PrimitiveUnsigned + ExpressAsPower>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.express_as_power()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.express_as_power()))],
    );
}

fn benchmark_express_as_power_signed<T: PrimitiveSigned + ExpressAsPower>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.express_as_power()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |i| no_out!(i.express_as_power()))],
    );
}

#[allow(unused_must_use)]
fn benchmark_is_power_unsigned_algorithms<T: PrimitiveUnsigned + ExpressAsPower + IsPower>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.is_power()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |u| no_out!(u.is_power())),
            ("using express_as_power", &mut |u| {
                no_out!(u.express_as_power().is_some())
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_is_power_signed_algorithms<T: PrimitiveSigned + ExpressAsPower + IsPower>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.is_power()", T::NAME),
        BenchmarkType::Algorithms,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            ("default", &mut |i| no_out!(i.is_power())),
            ("using express_as_power", &mut |i| {
                no_out!(i.express_as_power().is_some())
            }),
        ],
    );
}
