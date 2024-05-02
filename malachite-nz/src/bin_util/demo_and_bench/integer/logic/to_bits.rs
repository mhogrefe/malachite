// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable};
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{bool_vec_gen, bool_vec_gen_var_5};
use malachite_base::test_util::num::logic::bit_convertible::{to_bits_asc_alt, to_bits_desc_alt};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::logic::bit_convertible::{
    bits_slice_to_twos_complement_bits_negative, bits_to_twos_complement_bits_non_negative,
    bits_vec_to_twos_complement_bits_negative,
};
use malachite_nz::test_util::bench::bucketers::integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_gen;
use malachite_nz::test_util::integer::logic::to_bits::{to_bits_asc_naive, to_bits_desc_naive};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_bits_to_twos_complement_bits_non_negative);
    register_demo!(runner, demo_bits_slice_to_twos_complement_bits_negative);
    register_demo!(runner, demo_bits_vec_to_twos_complement_bits_negative);
    register_demo!(runner, demo_integer_to_bits_asc);
    register_demo!(runner, demo_integer_to_bits_desc);

    register_bench!(runner, benchmark_bits_to_twos_complement_bits_non_negative);
    register_bench!(
        runner,
        benchmark_bits_slice_to_twos_complement_bits_negative
    );
    register_bench!(runner, benchmark_bits_vec_to_twos_complement_bits_negative);
    register_bench!(runner, benchmark_integer_to_bits_asc_evaluation_strategy);
    register_bench!(runner, benchmark_integer_to_bits_asc_algorithms);
    register_bench!(runner, benchmark_integer_to_bits_desc_evaluation_strategy);
    register_bench!(runner, benchmark_integer_to_bits_desc_algorithms);
}

fn demo_bits_to_twos_complement_bits_non_negative(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut bits in bool_vec_gen().get(gm, config).take(limit) {
        let old_bits = bits.clone();
        bits_to_twos_complement_bits_non_negative(&mut bits);
        println!(
            "bits := {old_bits:?}; \
            bits_to_twos_complement_bits_non_negative(&mut bits); bits = {bits:?}",
        );
    }
}

fn demo_bits_slice_to_twos_complement_bits_negative(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut bits in bool_vec_gen().get(gm, config).take(limit) {
        let old_bits = bits.clone();
        let carry = bits_slice_to_twos_complement_bits_negative(&mut bits);
        println!(
            "bits := {old_bits:?}; \
            bits_slice_to_twos_complement_bits_negative(&mut bits) = {carry}; bits = {bits:?}",
        );
    }
}

fn demo_bits_vec_to_twos_complement_bits_negative(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut bits in bool_vec_gen_var_5().get(gm, config).take(limit) {
        let old_bits = bits.clone();
        bits_vec_to_twos_complement_bits_negative(&mut bits);
        println!(
            "bits := {old_bits:?}; \
            bits_vec_to_twos_complement_bits_negative(&mut bits); bits = {bits:?}",
        );
    }
}

fn demo_integer_to_bits_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("to_bits_asc({}) = {:?}", n, n.to_bits_asc());
    }
}

fn demo_integer_to_bits_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("to_bits_desc({}) = {:?}", n, n.to_bits_desc());
    }
}

fn benchmark_bits_to_twos_complement_bits_non_negative(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "bits_to_twos_complement_bits_non_negative(&mut [bool])",
        BenchmarkType::Single,
        bool_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |mut bits| {
            bits_to_twos_complement_bits_non_negative(&mut bits)
        })],
    );
}

fn benchmark_bits_slice_to_twos_complement_bits_negative(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "bits_slice_to_twos_complement_bits_negative(&mut [bool])",
        BenchmarkType::Single,
        bool_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |mut bits| {
            no_out!(bits_slice_to_twos_complement_bits_negative(&mut bits))
        })],
    );
}

fn benchmark_bits_vec_to_twos_complement_bits_negative(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "bits_vec_to_twos_complement_bits_negative(&mut [bool])",
        BenchmarkType::Single,
        bool_vec_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |ref mut bits| {
            bits_vec_to_twos_complement_bits_negative(bits)
        })],
    );
}

fn benchmark_integer_to_bits_asc_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_bits_asc()",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("Integer.to_bits_asc()", &mut |n| no_out!(n.to_bits_asc())),
            ("Integer.bits().collect_vec()", &mut |n| {
                no_out!(n.bits().collect_vec())
            }),
        ],
    );
}

fn benchmark_integer_to_bits_asc_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_bits_asc()",
        BenchmarkType::Algorithms,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("default", &mut |n| no_out!(n.to_bits_asc())),
            ("alt", &mut |n| no_out!(to_bits_asc_alt(&n))),
            ("naive", &mut |n| no_out!(to_bits_asc_naive(&n))),
        ],
    );
}

fn benchmark_integer_to_bits_desc_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_bits_desc()",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("Integer.to_bits_desc()", &mut |n| no_out!(n.to_bits_desc())),
            ("Integer.bits().rev().collect_vec()", &mut |n| {
                no_out!(n.bits().rev().collect_vec())
            }),
        ],
    );
}

fn benchmark_integer_to_bits_desc_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_bits_desc()",
        BenchmarkType::Algorithms,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("default", &mut |n| no_out!(n.to_bits_desc())),
            ("alt", &mut |n| no_out!(to_bits_desc_alt(&n))),
            ("naive", &mut |n| no_out!(to_bits_desc_naive(&n))),
        ],
    );
}
