// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::SplitInHalf;
use malachite_base::test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_generic_demos!(runner, demo_upper_half, u16, u32, u64, u128);
    register_generic_benches!(runner, benchmark_upper_half, u16, u32, u64, u128);
}

fn demo_upper_half<T: PrimitiveUnsigned + SplitInHalf>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    T::Half: PrimitiveUnsigned,
{
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("{}.upper_half() = {}", u, u.upper_half());
    }
}

fn benchmark_upper_half<T: PrimitiveUnsigned + SplitInHalf>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.upper_half()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.upper_half()))],
    );
}
