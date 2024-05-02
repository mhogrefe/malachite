// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::BitScan;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_16;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::logic::bit_scan::limbs_index_of_next_true_bit;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;
use malachite_nz::test_util::natural::logic::index_of_next_true_bit::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_index_of_next_true_bit);
    register_demo!(runner, demo_natural_index_of_next_true_bit);

    register_bench!(runner, benchmark_limbs_index_of_next_true_bit);
    register_bench!(runner, benchmark_natural_index_of_next_true_bit_algorithms);
}

fn demo_limbs_index_of_next_true_bit(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, u) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_index_of_next_true_bit({:?}, {}) = {:?}",
            xs,
            u,
            limbs_index_of_next_true_bit(&xs, u)
        );
    }
}

fn demo_natural_index_of_next_true_bit(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "index_of_next_true_bit({}, {}) = {:?}",
            n,
            u,
            n.index_of_next_true_bit(u)
        );
    }
}

fn benchmark_limbs_index_of_next_true_bit(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_index_of_next_true_bit(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, u)| {
            no_out!(limbs_index_of_next_true_bit(&xs, u))
        })],
    );
}

fn benchmark_natural_index_of_next_true_bit_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.index_of_next_true_bit(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            (
                "default",
                &mut |(n, u)| no_out!(n.index_of_next_true_bit(u)),
            ),
            ("using bits explicitly", &mut |(n, u)| {
                no_out!(natural_index_of_next_true_bit_alt(&n, u))
            }),
        ],
    );
}
