// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base::test_util::bench::bucketers::get_bits_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_unsigned_unsigned_triple_gen_var_2, unsigned_triple_gen_var_5,
};
use malachite_base::test_util::num::logic::bit_block_access::get_bits_naive;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_get_bits_unsigned);
    register_unsigned_signed_match_demos!(runner, demo_get_bits_signed);
    register_unsigned_benches!(runner, benchmark_get_bits_algorithms_unsigned);
    register_unsigned_signed_match_benches!(runner, benchmark_get_bits_algorithms_signed);
}

fn demo_get_bits_unsigned<T: BitBlockAccess<Bits = T> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, start, end) in unsigned_triple_gen_var_5::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.get_bits({}, {}) = {}",
            n,
            start,
            end,
            n.get_bits(start, end)
        );
    }
}

fn demo_get_bits_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    S::Bits: PrimitiveUnsigned,
{
    for (n, start, end) in signed_unsigned_unsigned_triple_gen_var_2::<U, S, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).get_bits({}, {}) = {}",
            n,
            start,
            end,
            n.get_bits(start, end)
        );
    }
}

fn benchmark_get_bits_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.get_bits(u64, u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_triple_gen_var_5::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &get_bits_bucketer(),
        &mut [
            ("default", &mut |(n, start, end)| {
                no_out!(n.get_bits(start, end))
            }),
            ("naive", &mut |(n, start, end)| {
                no_out!(get_bits_naive::<T, T>(&n, start, end))
            }),
        ],
    );
}

fn benchmark_get_bits_algorithms_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.get_bits(u64, u64)", S::NAME),
        BenchmarkType::Algorithms,
        signed_unsigned_unsigned_triple_gen_var_2::<U, S, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &get_bits_bucketer(),
        &mut [
            ("default", &mut |(n, start, end)| {
                no_out!(n.get_bits(start, end))
            }),
            ("naive", &mut |(n, start, end)| {
                no_out!(get_bits_naive::<S, U>(&n, start, end))
            }),
        ],
    );
}
