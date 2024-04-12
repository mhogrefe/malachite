// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::bench::bucketers::{pair_2_bucketer, triple_3_pair_2_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_16;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::logic::bit_access::limbs_get_bit;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_4, natural_unsigned_pair_gen_var_4_nrm,
};
use malachite_nz::test_util::natural::logic::get_bit::num_get_bit;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_get_bit);
    register_demo!(runner, demo_natural_get_bit);

    register_bench!(runner, benchmark_limbs_get_bit);
    register_bench!(runner, benchmark_natural_get_bit_library_comparison);
}

fn demo_limbs_get_bit(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, index) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_get_bit({:?}, {}) = {}",
            xs,
            index,
            limbs_get_bit(&xs, index)
        );
    }
}

fn demo_natural_get_bit(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, index) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.get_bit({}) = {}", n, index, n.get_bit(index));
    }
}

fn benchmark_limbs_get_bit(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_get_bit(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(xs, index)| {
            no_out!(limbs_get_bit(&xs, index))
        })],
    );
}

fn benchmark_natural_get_bit_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.get_bit(u64)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_4_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_2_bucketer("index"),
        &mut [
            ("Malachite", &mut |(_, _, (n, index))| {
                no_out!(n.get_bit(index))
            }),
            ("num", &mut |((n, index), _, _)| {
                no_out!(num_get_bit(&n, index))
            }),
            ("rug", &mut |(_, (n, index), _)| {
                no_out!(n.get_bit(u32::exact_from(index)))
            }),
        ],
    );
}
