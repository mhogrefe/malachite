// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_is_square_unsigned);
    register_signed_demos!(runner, demo_is_square_signed);

    register_unsigned_benches!(runner, benchmark_is_square_unsigned_algorithms);
    register_signed_benches!(runner, benchmark_is_square_signed_algorithms);
}

fn demo_is_square_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        if u.is_square() {
            println!("{u} is a perfect square");
        } else {
            println!("{u} is not a perfect square");
        }
    }
}

fn demo_is_square_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        if i.is_square() {
            println!("{i} is a perfect square");
        } else {
            println!("{i} is not a perfect square");
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_is_square_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.is_square()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |u| no_out!(u.is_square())),
            ("naive", &mut |u| no_out!(u.checked_sqrt().is_some())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_is_square_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.is_square()", T::NAME),
        BenchmarkType::Algorithms,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            ("default", &mut |i| no_out!(i.is_square())),
            ("naive", &mut |i| no_out!(i.checked_sqrt().is_some())),
        ],
    );
}
