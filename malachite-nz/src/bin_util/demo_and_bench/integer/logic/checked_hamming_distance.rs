// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::CheckedHammingDistance;
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_vec_max_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen_var_8, unsigned_vec_unsigned_pair_gen_var_19,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::logic::checked_hamming_distance::{
    limbs_hamming_distance_limb_neg, limbs_hamming_distance_neg,
};
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_integer_max_bit_bucketer, pair_integer_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_pair_gen, integer_pair_gen_rm};
use malachite_nz::test_util::integer::logic::checked_hamming_distance::rug_checked_hamming_distance;
use malachite_nz::test_util::integer::logic::checked_hamming_distance::{
    integer_checked_hamming_distance_alt_1, integer_checked_hamming_distance_alt_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_hamming_distance_limb_neg);
    register_demo!(runner, demo_limbs_hamming_distance_neg);
    register_demo!(runner, demo_integer_checked_hamming_distance);

    register_bench!(runner, benchmark_limbs_hamming_distance_limb_neg);
    register_bench!(runner, benchmark_limbs_hamming_distance_neg);
    register_bench!(
        runner,
        benchmark_integer_checked_hamming_distance_library_comparison
    );
    register_bench!(
        runner,
        benchmark_integer_checked_hamming_distance_algorithms
    );
}

fn demo_limbs_hamming_distance_limb_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_hamming_distance_limb_neg({:?}, {}) = {}",
            xs,
            y,
            limbs_hamming_distance_limb_neg(&xs, y)
        );
    }
}

fn demo_limbs_hamming_distance_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, config).take(limit) {
        println!(
            "limbs_hamming_distance_neg({:?}, {:?}) = {}",
            xs,
            ys,
            limbs_hamming_distance_neg(&xs, &ys)
        );
    }
}

fn demo_integer_checked_hamming_distance(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            x,
            y,
            x.checked_hamming_distance(&y)
        );
    }
}

fn benchmark_limbs_hamming_distance_limb_neg(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_hamming_distance_limb_neg(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| {
            no_out!(limbs_hamming_distance_limb_neg(&xs, y))
        })],
    );
}

fn benchmark_limbs_hamming_distance_neg(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_hamming_distance_neg(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref xs, ref ys)| {
            no_out!(limbs_hamming_distance_neg(xs, ys))
        })],
    );
}

fn benchmark_integer_checked_hamming_distance_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.checked_hamming_distance(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(x.checked_hamming_distance(&y))
            }),
            ("rug", &mut |((x, y), _)| {
                no_out!(rug_checked_hamming_distance(&x, &y))
            }),
        ],
    );
}

fn benchmark_integer_checked_hamming_distance_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.checked_hamming_distance(&Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(n, other)| {
                no_out!(n.checked_hamming_distance(&other))
            }),
            ("using bits explicitly", &mut |(n, other)| {
                no_out!(integer_checked_hamming_distance_alt_1(&n, &other))
            }),
            ("using limbs explicitly", &mut |(n, other)| {
                no_out!(integer_checked_hamming_distance_alt_2(&n, &other))
            }),
        ],
    );
}
