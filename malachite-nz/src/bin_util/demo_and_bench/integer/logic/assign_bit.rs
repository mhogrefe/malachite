// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::bench::bucketers::pair_2_triple_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::{
    integer_unsigned_bool_triple_gen_var_1, integer_unsigned_bool_triple_gen_var_1_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_assign_bit);
    register_bench!(runner, benchmark_integer_assign_bit_library_comparison);
}

fn demo_integer_assign_bit(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, index, bit) in integer_unsigned_bool_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.assign_bit(index, bit);
        println!("x := {n_old}; x.assign_bit({index}, {bit}); x = {n}");
    }
}

fn benchmark_integer_assign_bit_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.assign_bit(u64, bool)",
        BenchmarkType::LibraryComparison,
        integer_unsigned_bool_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_2_bucketer("index"),
        &mut [
            ("Malachite", &mut |(_, (mut n, index, bit))| {
                n.assign_bit(index, bit)
            }),
            ("rug", &mut |((mut n, index, bit), _)| {
                no_out!(n.set_bit(u32::exact_from(index), bit))
            }),
        ],
    );
}
