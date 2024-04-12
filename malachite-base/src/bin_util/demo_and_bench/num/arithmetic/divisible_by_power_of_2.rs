// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, unsigned_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_divisible_by_power_of_2_unsigned);
    register_signed_demos!(runner, demo_divisible_by_power_of_2_signed);

    register_unsigned_benches!(runner, benchmark_divisible_by_power_of_2_unsigned);
    register_signed_benches!(runner, benchmark_divisible_by_power_of_2_signed);
}

fn demo_divisible_by_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, pow) in unsigned_pair_gen_var_2::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        if u.divisible_by_power_of_2(pow) {
            println!("{u} is divisible by 2^{pow}");
        } else {
            println!("{u} is not divisible by 2^{pow}");
        }
    }
}

fn demo_divisible_by_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (i, pow) in signed_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        if i.divisible_by_power_of_2(pow) {
            println!("{i} is divisible by 2^{pow}");
        } else {
            println!("{i} is not divisible by 2^{pow}");
        }
    }
}

fn benchmark_divisible_by_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.divisible_by_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_2::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("u"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(x.divisible_by_power_of_2(y))
        })],
    );
}

fn benchmark_divisible_by_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.divisible_by_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("i"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(x.divisible_by_power_of_2(y))
        })],
    );
}
