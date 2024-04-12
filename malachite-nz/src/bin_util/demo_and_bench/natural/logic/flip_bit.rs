// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::bench::bucketers::pair_2_pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_4, natural_unsigned_pair_gen_var_4_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_flip_bit);
    register_bench!(runner, benchmark_natural_flip_bit_library_comparison);
}

fn demo_natural_flip_bit(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, index) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.flip_bit(index);
        println!("x := {n_old}; x.flip_bit({index}); x = {n}");
    }
}

fn benchmark_natural_flip_bit_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.flip_bit(u64)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_4_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_2_bucketer("index"),
        &mut [
            ("Malachite", &mut |(_, (mut n, index))| n.flip_bit(index)),
            ("rug", &mut |((mut n, index), _)| {
                no_out!(n.toggle_bit(u32::exact_from(index)))
            }),
        ],
    );
}
