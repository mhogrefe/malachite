// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::log_base::{ceiling_log_base_naive, checked_log_base_naive};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_24;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_floor_log_base);
    register_unsigned_demos!(runner, demo_ceiling_log_base);
    register_unsigned_demos!(runner, demo_checked_log_base);
    register_unsigned_benches!(runner, benchmark_floor_log_base);
    register_unsigned_benches!(runner, benchmark_ceiling_log_base_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_log_base_algorithms);
}

fn demo_floor_log_base<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, b) in unsigned_pair_gen_var_24::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.floor_log_base({}) = {}", n, b, n.floor_log_base(b));
    }
}

fn demo_ceiling_log_base<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, b) in unsigned_pair_gen_var_24::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.ceiling_log_base({}) = {}", n, b, n.ceiling_log_base(b));
    }
}

fn demo_checked_log_base<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, b) in unsigned_pair_gen_var_24::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.checked_log_base({}) = {:?}",
            n,
            b,
            n.checked_log_base(b)
        );
    }
}

fn benchmark_floor_log_base<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_log_base({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_24::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [(
            "Malachite",
            &mut |(n, base)| no_out!(n.floor_log_base(base)),
        )],
    );
}

fn benchmark_ceiling_log_base_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_log_base({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_24::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [
            (
                "default",
                &mut |(n, base)| no_out!(n.ceiling_log_base(base)),
            ),
            ("naive", &mut |(n, base)| {
                no_out!(ceiling_log_base_naive(n, base))
            }),
        ],
    );
}

fn benchmark_checked_log_base_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_log_base({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_24::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [
            (
                "default",
                &mut |(n, base)| no_out!(n.checked_log_base(base)),
            ),
            ("naive", &mut |(n, base)| {
                no_out!(checked_log_base_naive(n, base))
            }),
        ],
    );
}
