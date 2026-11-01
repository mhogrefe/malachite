// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, FactoradicDigits};
use malachite_base::test_util::bench::bucketers::{bit_bucketer, vec_len_bucketer};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_gen, unsigned_vec_gen};
use malachite_base::test_util::num::conversion::digits::factoradic_digits::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_to_factoradic_digits_asc);
    register_unsigned_demos!(runner, demo_to_factoradic_digits_desc);
    register_unsigned_demos!(runner, demo_from_factoradic_digits_asc);
    register_unsigned_demos!(runner, demo_from_factoradic_digits_desc);
    register_unsigned_benches!(runner, benchmark_to_factoradic_digits_asc_algorithms);
    register_unsigned_benches!(runner, benchmark_from_factoradic_digits_asc_algorithms);
}

fn demo_to_factoradic_digits_asc<T: FactoradicDigits + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!(
            "{}.to_factoradic_digits_asc() = {:?}",
            x,
            x.to_factoradic_digits_asc()
        );
    }
}

fn demo_to_factoradic_digits_desc<T: FactoradicDigits + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!(
            "{}.to_factoradic_digits_desc() = {:?}",
            x,
            x.to_factoradic_digits_desc()
        );
    }
}

fn demo_from_factoradic_digits_asc<T: FactoradicDigits + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for xs in unsigned_vec_gen::<u8>().get(gm, config).take(limit) {
        let xs: Vec<u64> = xs.into_iter().map(u64::from).collect();
        println!(
            "{}::from_factoradic_digits_asc({:?}) = {:?}",
            T::NAME,
            xs,
            T::from_factoradic_digits_asc(xs.iter().copied())
        );
    }
}

fn demo_from_factoradic_digits_desc<T: FactoradicDigits + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for xs in unsigned_vec_gen::<u8>().get(gm, config).take(limit) {
        let xs: Vec<u64> = xs.into_iter().map(u64::from).collect();
        println!(
            "{}::from_factoradic_digits_desc({:?}) = {:?}",
            T::NAME,
            xs,
            T::from_factoradic_digits_desc(xs.iter().copied())
        );
    }
}

fn benchmark_to_factoradic_digits_asc_algorithms<T: FactoradicDigits + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    u128: ExactFrom<T>,
{
    run_benchmark(
        &format!("{}.to_factoradic_digits_asc()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &bit_bucketer("x"),
        &mut [
            ("default", &mut |x| {
                no_out!(x.to_factoradic_digits_asc());
            }),
            ("naive", &mut |x| {
                no_out!(to_factoradic_digits_asc_naive(x));
            }),
        ],
    );
}

fn benchmark_from_factoradic_digits_asc_algorithms<T: FactoradicDigits + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_factoradic_digits_asc(Iterator<Item = u64>)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vec_gen::<u8>()
            .get(gm, config)
            .map(|xs| xs.into_iter().map(u64::from).collect::<Vec<u64>>()),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |xs| {
                no_out!(T::from_factoradic_digits_asc(xs.into_iter()));
            }),
            ("naive", &mut |xs| {
                no_out!(from_factoradic_digits_asc_naive::<T>(&xs));
            }),
        ],
    );
}
