// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    bool_vec_gen_var_1, bool_vec_gen_var_2, bool_vec_gen_var_3, bool_vec_gen_var_4,
};
use malachite_base::test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_asc_signed_naive, from_bits_asc_unsigned_naive, from_bits_desc_alt,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_from_bits_asc_unsigned);
    register_signed_demos!(runner, demo_from_bits_asc_signed);
    register_unsigned_demos!(runner, demo_from_bits_desc_unsigned);
    register_signed_demos!(runner, demo_from_bits_desc_signed);

    register_unsigned_benches!(runner, benchmark_from_bits_asc_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_from_bits_asc_algorithms_signed);
    register_unsigned_benches!(runner, benchmark_from_bits_desc_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_from_bits_desc_algorithms_signed);
}

fn demo_from_bits_asc_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for bs in bool_vec_gen_var_1::<T>().get(gm, config).take(limit) {
        println!(
            "{}::from_bits_asc({:?}) = {}",
            T::NAME,
            bs,
            T::from_bits_asc(bs.iter().cloned())
        );
    }
}

fn demo_from_bits_asc_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for bs in bool_vec_gen_var_2::<T>().get(gm, config).take(limit) {
        println!(
            "{}::from_bits_asc({:?}) = {}",
            T::NAME,
            bs,
            T::from_bits_asc(bs.iter().cloned())
        );
    }
}

fn demo_from_bits_desc_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for bs in bool_vec_gen_var_3::<T>().get(gm, config).take(limit) {
        println!(
            "{}::from_bits_desc({:?}) = {}",
            T::NAME,
            bs,
            T::from_bits_desc(bs.iter().cloned())
        );
    }
}

fn demo_from_bits_desc_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for bs in bool_vec_gen_var_4::<T>().get(gm, config).take(limit) {
        println!(
            "{}::from_bits_desc({:?}) = {}",
            T::NAME,
            bs,
            T::from_bits_desc(bs.iter().cloned())
        );
    }
}

fn benchmark_from_bits_asc_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_bits_asc<I: Iterator<Item=bool>>(I)", T::NAME),
        BenchmarkType::Algorithms,
        bool_vec_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |ref bs| {
                no_out!(T::from_bits_asc(bs.iter().cloned()))
            }),
            ("alt", &mut |ref bs| {
                no_out!(from_bits_asc_alt::<T, _>(bs.iter().cloned()))
            }),
            ("naive", &mut |ref bs| {
                no_out!(from_bits_asc_unsigned_naive::<T, _>(bs.iter().cloned()))
            }),
        ],
    );
}

fn benchmark_from_bits_asc_algorithms_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_bits_asc<I: Iterator<Item=bool>>(I)", T::NAME),
        BenchmarkType::Algorithms,
        bool_vec_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |ref bs| {
                no_out!(T::from_bits_asc(bs.iter().cloned()))
            }),
            ("alt", &mut |ref bs| {
                no_out!(from_bits_asc_alt::<T, _>(bs.iter().cloned()))
            }),
            ("naive", &mut |ref bs| {
                no_out!(from_bits_asc_signed_naive::<T, _>(bs.iter().cloned()))
            }),
        ],
    );
}

fn benchmark_from_bits_desc_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_bits_desc<I: Iterator<Item=bool>>(I)", T::NAME),
        BenchmarkType::Algorithms,
        bool_vec_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |ref bs| {
                no_out!(T::from_bits_desc(bs.iter().cloned()))
            }),
            ("alt", &mut |ref bs| {
                no_out!(from_bits_desc_alt::<T, _>(bs.iter().cloned()))
            }),
        ],
    );
}

fn benchmark_from_bits_desc_algorithms_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_bits_desc<I: Iterator<Item=bool>>(I)", T::NAME),
        BenchmarkType::Algorithms,
        bool_vec_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |ref bs| {
                no_out!(T::from_bits_desc(bs.iter().cloned()))
            }),
            ("alt", &mut |ref bs| {
                no_out!(from_bits_desc_alt::<T, _>(bs.iter().cloned()))
            }),
        ],
    );
}
