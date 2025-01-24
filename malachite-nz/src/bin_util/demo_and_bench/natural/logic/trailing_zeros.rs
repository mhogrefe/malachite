// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen_var_2;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::logic::trailing_zeros::limbs_trailing_zeros;
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::logic::trailing_zeros::natural_trailing_zeros_alt;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_trailing_zeros);
    register_demo!(runner, demo_natural_trailing_zeros);

    register_bench!(runner, benchmark_limbs_trailing_zeros);
    register_bench!(runner, benchmark_natural_trailing_zeros_algorithms);
}

fn demo_limbs_trailing_zeros(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_2().get(gm, config).take(limit) {
        println!(
            "limbs_trailing_zeros({:?}) = {}",
            xs,
            limbs_trailing_zeros(&xs)
        );
    }
}

fn demo_natural_trailing_zeros(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

fn benchmark_limbs_trailing_zeros(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_trailing_zeros(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(limbs_trailing_zeros(&xs)))],
    );
}

fn benchmark_natural_trailing_zeros_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.trailing_zeros()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |n| no_out!(n.trailing_zeros())),
            ("using bits explicitly", &mut |n| {
                no_out!(natural_trailing_zeros_alt(&n))
            }),
        ],
    );
}
