// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::number_theory::traits::StirlingNumberOfTheFirstKind;
use malachite_base::test_util::bench::bucketers::usize_convertible_pair_max_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_28;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_stirling_number_of_the_first_kind);
    register_bench!(runner, benchmark_integer_stirling_number_of_the_first_kind);
}

fn demo_integer_stirling_number_of_the_first_kind(gm: GenMode, config: &GenConfig, limit: usize) {
    // s(n, k) has up to about n log n bits, so the arguments are kept to u8 scale.
    for (n, k) in unsigned_pair_gen_var_28::<u8, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "s({}, {}) = {}",
            n,
            k,
            Integer::stirling_number_of_the_first_kind(u64::from(n), u64::from(k))
        );
    }
}

fn benchmark_integer_stirling_number_of_the_first_kind(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::stirling_number_of_the_first_kind(u64, u64)",
        BenchmarkType::Single,
        unsigned_pair_gen_var_28::<u16, u16>()
            .get(gm, config)
            .map(|(n, k)| (u64::from(n), u64::from(k))),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "k"),
        &mut [("Malachite", &mut |(n, k)| {
            no_out!(Integer::stirling_number_of_the_first_kind(n, k));
        })],
    );
}
