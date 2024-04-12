// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_overflowing_square_unsigned);
    register_signed_demos!(runner, demo_overflowing_square_signed);
    register_unsigned_demos!(runner, demo_overflowing_square_assign_unsigned);
    register_signed_demos!(runner, demo_overflowing_square_assign_signed);

    register_unsigned_benches!(runner, benchmark_overflowing_square_unsigned);
    register_signed_benches!(runner, benchmark_overflowing_square_signed);
    register_unsigned_benches!(runner, benchmark_overflowing_square_assign_unsigned);
    register_signed_benches!(runner, benchmark_overflowing_square_assign_signed);
}

fn demo_overflowing_square_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("{}.overflowing_square() = {:?}", x, x.overflowing_square());
    }
}

fn demo_overflowing_square_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in signed_gen::<T>().get(gm, config).take(limit) {
        println!("{}.overflowing_square() = {:?}", x, x.overflowing_square());
    }
}

fn demo_overflowing_square_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut x in unsigned_gen::<T>().get(gm, config).take(limit) {
        let old_x = x;
        let overflow = x.overflowing_square_assign();
        println!("x := {old_x}; x.overflowing_square_assign() = {overflow}; x = {x}");
    }
}

fn demo_overflowing_square_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut x in signed_gen::<T>().get(gm, config).take(limit) {
        let old_x = x;
        let overflow = x.overflowing_square_assign();
        println!("x := {old_x}; x.overflowing_square_assign() = {overflow}; x = {x}");
    }
}

fn benchmark_overflowing_square_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.overflowing_square()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(x.overflowing_square()))],
    );
}

fn benchmark_overflowing_square_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.overflowing_square()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(x.overflowing_square()))],
    );
}

fn benchmark_overflowing_square_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.overflowing_square_assign()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut x| {
            no_out!(x.overflowing_square_assign())
        })],
    );
}

fn benchmark_overflowing_square_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.overflowing_square_assign()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut x| {
            no_out!(x.overflowing_square_assign())
        })],
    );
}
