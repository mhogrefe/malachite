// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::CountOnes;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::logic::count_ones::limbs_count_ones;
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::logic::count_ones::{
    natural_count_ones_alt_1, natural_count_ones_alt_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_count_ones);
    register_demo!(runner, demo_natural_count_ones);

    register_bench!(runner, benchmark_limbs_count_ones);
    register_bench!(runner, benchmark_natural_count_ones_algorithms);
}

fn demo_limbs_count_ones(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen().get(gm, config).take(limit) {
        println!("limbs_count_ones({:?}) = {}", xs, limbs_count_ones(&xs));
    }
}

fn demo_natural_count_ones(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("count_ones({}) = {}", n, n.count_ones());
    }
}

fn benchmark_limbs_count_ones(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_count_ones(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(limbs_count_ones(&xs)))],
    );
}

fn benchmark_natural_count_ones_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.count_ones()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |n| no_out!(n.count_ones())),
            ("using bits explicitly", &mut |n| {
                no_out!(natural_count_ones_alt_1(&n))
            }),
            ("using limbs explicitly", &mut |n| {
                no_out!(natural_count_ones_alt_2(&n))
            }),
        ],
    );
}
