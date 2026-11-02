// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_gen, unsigned_gen_var_35};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_bell_number);
    register_unsigned_demos!(runner, demo_checked_bell_number);
    register_unsigned_benches!(runner, benchmark_bell_number);
    register_unsigned_benches!(runner, benchmark_checked_bell_number);
}

fn demo_bell_number<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_35::<T>().get(gm, config).take(limit) {
        println!("B({}) = {}", n, T::bell_number(n));
    }
}

fn demo_checked_bell_number<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen().get(gm, config).take(limit) {
        println!("B({}) = {:?}", n, T::checked_bell_number(n));
    }
}

fn benchmark_bell_number<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::bell_number(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_35::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::bell_number(n)))],
    );
}

fn benchmark_checked_bell_number<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_bell_number(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_bell_number(n)))],
    );
}
