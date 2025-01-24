// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_integer_bit_bucketer, pair_2_pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_unsigned_pair_gen_var_2, integer_unsigned_pair_gen_var_2_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_divisible_by_power_of_2);

    register_bench!(
        runner,
        benchmark_integer_divisible_by_power_of_2_library_comparison
    );
    register_bench!(runner, benchmark_integer_divisible_by_power_of_2_algorithms);
}

fn demo_integer_divisible_by_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        if n.divisible_by_power_of_2(pow) {
            println!("{n} is divisible by 2^{pow}");
        } else {
            println!("{n} is not divisible by 2^{pow}");
        }
    }
}

fn benchmark_integer_divisible_by_power_of_2_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.divisible_by_power_of_2(u64)",
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_var_2_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (n, pow))| {
                no_out!(n.divisible_by_power_of_2(pow))
            }),
            ("rug", &mut |((n, pow), _)| {
                n.is_divisible_2pow(u32::exact_from(pow));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_divisible_by_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.divisible_by_power_of_2(u64)",
        BenchmarkType::Algorithms,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.divisible_by_power_of_2(u64)", &mut |(n, pow)| {
                no_out!(n.divisible_by_power_of_2(pow))
            }),
            (
                "Integer.trailing_zeros().map_or(true, |z| z >= u64)",
                &mut |(n, pow)| no_out!(n.trailing_zeros().map_or(true, |z| z >= pow)),
            ),
        ],
    );
}
