// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::test_util::bench::bucketers::usize_convertible_pair_max_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_pair_gen_var_28, unsigned_pair_gen_var_56};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_signed_demos!(runner, demo_stirling_number_of_the_first_kind);
    register_signed_demos!(runner, demo_checked_stirling_number_of_the_first_kind);
    register_signed_benches!(runner, benchmark_stirling_number_of_the_first_kind);
    register_signed_benches!(runner, benchmark_checked_stirling_number_of_the_first_kind);
}

fn demo_stirling_number_of_the_first_kind<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in unsigned_pair_gen_var_56::<T>().get(gm, config).take(limit) {
        println!(
            "s({}, {}) = {}",
            n,
            k,
            T::stirling_number_of_the_first_kind(n, k)
        );
    }
}

fn demo_checked_stirling_number_of_the_first_kind<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in unsigned_pair_gen_var_28::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "s({}, {}) = {:?}",
            n,
            k,
            T::checked_stirling_number_of_the_first_kind(n, k)
        );
    }
}

fn benchmark_stirling_number_of_the_first_kind<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::stirling_number_of_the_first_kind(u64, u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_56::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "k"),
        &mut [("Malachite", &mut |(n, k)| {
            no_out!(T::stirling_number_of_the_first_kind(n, k));
        })],
    );
}

fn benchmark_checked_stirling_number_of_the_first_kind<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::checked_stirling_number_of_the_first_kind(u64, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_pair_gen_var_28::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "k"),
        &mut [("Malachite", &mut |(n, k)| {
            no_out!(T::checked_stirling_number_of_the_first_kind(n, k));
        })],
    );
}
