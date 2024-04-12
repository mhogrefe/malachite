// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::HammingDistance;
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_vec_max_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen_var_6, unsigned_vec_pair_gen_var_7, unsigned_vec_unsigned_pair_gen_var_15,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::logic::hamming_distance::{
    limbs_hamming_distance, limbs_hamming_distance_limb, limbs_hamming_distance_same_length,
};
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_natural_max_bit_bucketer, pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_pair_gen, natural_pair_gen_rm};
use malachite_nz::test_util::natural::logic::hamming_distance::{
    natural_hamming_distance_alt_1, natural_hamming_distance_alt_2, rug_hamming_distance,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_hamming_distance_limb);
    register_demo!(runner, demo_limbs_hamming_distance_same_length);
    register_demo!(runner, demo_limbs_hamming_distance);
    register_demo!(runner, demo_natural_hamming_distance);

    register_bench!(runner, benchmark_limbs_hamming_distance_limb);
    register_bench!(runner, benchmark_limbs_hamming_distance_same_length);
    register_bench!(runner, benchmark_limbs_hamming_distance);
    register_bench!(
        runner,
        benchmark_natural_hamming_distance_library_comparison
    );
    register_bench!(runner, benchmark_natural_hamming_distance_algorithms);
}

fn demo_limbs_hamming_distance_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_hamming_distance_limb({:?}, {}) = {}",
            xs,
            y,
            limbs_hamming_distance_limb(&xs, y)
        );
    }
}

fn demo_limbs_hamming_distance_same_length(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_6().get(gm, config).take(limit) {
        println!(
            "limbs_hamming_distance_same_length({:?}, {:?}) = {}",
            xs,
            ys,
            limbs_hamming_distance_same_length(&xs, &ys),
        );
    }
}

fn demo_limbs_hamming_distance(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_7().get(gm, config).take(limit) {
        println!(
            "limbs_hamming_distance({:?}, {:?}) = {}",
            xs,
            ys,
            limbs_hamming_distance(&xs, &ys)
        );
    }
}

fn demo_natural_hamming_distance(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        println!(
            "hamming_distance({}, {}) = {}",
            x,
            y,
            x.hamming_distance(&y)
        );
    }
}

fn benchmark_limbs_hamming_distance_limb(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_hamming_distance_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| {
            no_out!(limbs_hamming_distance_limb(&xs, y))
        })],
    );
}

fn benchmark_limbs_hamming_distance_same_length(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_hamming_distance_same_length(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, ys)| {
            no_out!(limbs_hamming_distance_same_length(&xs, &ys))
        })],
    );
}

fn benchmark_limbs_hamming_distance(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_hamming_distance(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| {
            no_out!(limbs_hamming_distance(&xs, &ys))
        })],
    );
}

fn benchmark_natural_hamming_distance_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.hamming_distance(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(x.hamming_distance(&y))
            }),
            ("rug", &mut |((x, y), _)| {
                no_out!(rug_hamming_distance(&x, &y))
            }),
        ],
    );
}

fn benchmark_natural_hamming_distance_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.hamming_distance(&Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.hamming_distance(&y))),
            ("using bits explicitly", &mut |(x, y)| {
                no_out!(natural_hamming_distance_alt_1(&x, &y))
            }),
            ("using limbs explicitly", &mut |(x, y)| {
                no_out!(natural_hamming_distance_alt_2(&x, &y))
            }),
        ],
    );
}
