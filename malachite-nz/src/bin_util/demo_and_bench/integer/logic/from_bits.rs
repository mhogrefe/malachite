// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::BitConvertible;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::bool_vec_gen;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_desc_alt,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::integer::logic::from_bits::{
    from_bits_asc_naive, from_bits_desc_naive,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_from_bits_asc);
    register_demo!(runner, demo_integer_from_bits_desc);

    register_bench!(runner, benchmark_integer_from_bits_asc_algorithms);
    register_bench!(runner, benchmark_integer_from_bits_desc_algorithms);
}

fn demo_integer_from_bits_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for bits in bool_vec_gen().get(gm, config).take(limit) {
        println!(
            "from_bits_asc({:?}) = {:?}",
            bits,
            Integer::from_bits_asc(bits.iter().cloned())
        );
    }
}

fn demo_integer_from_bits_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for bits in bool_vec_gen().get(gm, config).take(limit) {
        println!(
            "from_bits_desc({:?}) = {:?}",
            bits,
            Integer::from_bits_desc(bits.iter().cloned())
        );
    }
}

fn benchmark_integer_from_bits_asc_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from_bits_asc<I: Iterator<Item=bool>>(I)",
        BenchmarkType::Algorithms,
        bool_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |bits| {
                no_out!(Integer::from_bits_asc(bits.into_iter()))
            }),
            ("alt", &mut |bits| {
                no_out!(from_bits_asc_alt::<Integer, _>(bits.into_iter()))
            }),
            ("naive", &mut |bits| {
                no_out!(from_bits_asc_naive(bits.into_iter()))
            }),
        ],
    );
}

fn benchmark_integer_from_bits_desc_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from_bits_desc<I: Iterator<Item=bool>>(I)",
        BenchmarkType::Algorithms,
        bool_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |bits| {
                no_out!(Integer::from_bits_desc(bits.into_iter()))
            }),
            ("alt", &mut |bits| {
                no_out!(from_bits_desc_alt::<Integer, _>(bits.into_iter()))
            }),
            ("naive", &mut |bits| {
                no_out!(from_bits_desc_naive(bits.into_iter()))
            }),
        ],
    );
}
