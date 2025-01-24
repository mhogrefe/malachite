// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, unsigned_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_get_bit_unsigned);
    register_signed_demos!(runner, demo_get_bit_signed);
    register_unsigned_benches!(runner, benchmark_get_bit_unsigned);
    register_signed_benches!(runner, benchmark_get_bit_signed);
}

fn demo_get_bit_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, index) in unsigned_pair_gen_var_2::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

fn demo_get_bit_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, index) in signed_unsigned_pair_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

fn benchmark_get_bit_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.get_bit(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_2::<T, _>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(n, index)| no_out!(n.get_bit(index)))],
    );
}

fn benchmark_get_bit_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.get_bit(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_1::<T, _>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(n, index)| no_out!(n.get_bit(index)))],
    );
}
