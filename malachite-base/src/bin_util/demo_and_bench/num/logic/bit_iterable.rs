// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::{pair_1_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_gen, signed_unsigned_pair_gen_var_1, unsigned_gen, unsigned_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;
use std::ops::Index;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_bits_unsigned);
    register_signed_demos!(runner, demo_bits_signed);
    register_unsigned_demos!(runner, demo_bits_rev_unsigned);
    register_signed_demos!(runner, demo_bits_rev_signed);
    register_unsigned_demos!(runner, demo_bits_size_hint_unsigned);
    register_signed_demos!(runner, demo_bits_index_signed);

    register_unsigned_benches!(runner, benchmark_bits_size_hint_unsigned);
    register_unsigned_benches!(runner, benchmark_bits_get_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_bits_get_algorithms_signed);
}

fn demo_bits_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("bits({}) = {:?}", u, u.bits().collect_vec());
    }
}

fn demo_bits_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!("bits({}) = {:?}", i, i.bits().collect_vec());
    }
}

fn demo_bits_rev_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("bits({}).rev() = {:?}", u, u.bits().rev().collect_vec());
    }
}

fn demo_bits_rev_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!("bits({}).rev() = {:?}", i, i.bits().rev().collect_vec());
    }
}

fn demo_bits_size_hint_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("bits({}).size_hint() = {:?}", u, u.bits().size_hint());
    }
}

fn demo_bits_index_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    T::BitIterator: Index<u64, Output = bool>,
{
    for (n, i) in signed_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("bits({})[{}] = {:?}", n, i, n.bits()[i]);
    }
}

fn benchmark_bits_size_hint_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.bits().size_hint()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [(&format!("{}.bits().size_hint()", T::NAME), &mut |n| {
            no_out!(n.bits().size_hint())
        })],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_bits_get_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    T::BitIterator: Index<u64, Output = bool>,
{
    run_benchmark(
        &format!("{}.bits()[u64]", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_2::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [
            (&format!("{}.bits()[u]", T::NAME), &mut |(n, u)| {
                no_out!(n.bits()[u])
            }),
            (&format!("{}.to_bits_asc()[u]", T::NAME), &mut |(n, u)| {
                let bits = n.to_bits_asc();
                let u = usize::exact_from(u);
                if u >= bits.len() {
                    n < T::ZERO
                } else {
                    bits[u]
                };
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_bits_get_algorithms_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    T::BitIterator: Index<u64, Output = bool>,
{
    run_benchmark(
        &format!("{}.bits()[u64]", T::NAME),
        BenchmarkType::Algorithms,
        signed_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [
            (&format!("{}.bits()[u]", T::NAME), &mut |(n, u)| {
                no_out!(n.bits()[u])
            }),
            (&format!("{}.to_bits_asc()[u]", T::NAME), &mut |(n, u)| {
                let bits = n.to_bits_asc();
                let u = usize::exact_from(u);
                if u >= bits.len() {
                    n < T::ZERO
                } else {
                    bits[u]
                };
            }),
        ],
    );
}
