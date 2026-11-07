// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::usize_convertible_pair_max_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_pair_gen_var_28, unsigned_pair_gen_var_55};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_stirling_number_of_the_first_kind);
    register_unsigned_demos!(
        runner,
        demo_checked_unsigned_stirling_number_of_the_first_kind
    );
    register_unsigned_benches!(runner, benchmark_unsigned_stirling_number_of_the_first_kind);
    register_unsigned_benches!(
        runner,
        benchmark_checked_unsigned_stirling_number_of_the_first_kind
    );
}

fn demo_unsigned_stirling_number_of_the_first_kind<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in unsigned_pair_gen_var_55::<T>().get(gm, config).take(limit) {
        println!(
            "[{}, {}] = {}",
            n,
            k,
            T::unsigned_stirling_number_of_the_first_kind(n, k)
        );
    }
}

fn demo_checked_unsigned_stirling_number_of_the_first_kind<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in unsigned_pair_gen_var_28::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "[{}, {}] = {:?}",
            n,
            k,
            T::checked_unsigned_stirling_number_of_the_first_kind(n, k)
        );
    }
}

fn benchmark_unsigned_stirling_number_of_the_first_kind<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::unsigned_stirling_number_of_the_first_kind(u64, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_pair_gen_var_55::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "k"),
        &mut [("Malachite", &mut |(n, k)| {
            no_out!(T::unsigned_stirling_number_of_the_first_kind(n, k));
        })],
    );
}

fn benchmark_checked_unsigned_stirling_number_of_the_first_kind<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::checked_unsigned_stirling_number_of_the_first_kind(u64, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_pair_gen_var_28::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "k"),
        &mut [("Malachite", &mut |(n, k)| {
            no_out!(T::checked_unsigned_stirling_number_of_the_first_kind(n, k));
        })],
    );
}
