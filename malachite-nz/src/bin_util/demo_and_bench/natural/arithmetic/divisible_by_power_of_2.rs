// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_unsigned_pair_gen_var_20, unsigned_vec_unsigned_pair_gen_var_22,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_natural_bit_bucketer, pair_2_pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_4, natural_unsigned_pair_gen_var_4_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_divisible_by_power_of_2);
    register_demo!(runner, demo_natural_divisible_by_power_of_2);

    register_bench!(runner, benchmark_limbs_divisible_by_power_of_2);
    register_bench!(
        runner,
        benchmark_natural_divisible_by_power_of_2_library_comparison
    );
    register_bench!(runner, benchmark_natural_divisible_by_power_of_2_algorithms);
}

fn demo_limbs_divisible_by_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_divisible_by_power_of_2({:?}, {}) = {:?}",
            xs,
            pow,
            limbs_divisible_by_power_of_2(&xs, pow)
        );
    }
}

fn demo_natural_divisible_by_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_4()
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

fn benchmark_limbs_divisible_by_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_divisible_by_power_of_2(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, pow)| {
            no_out!(limbs_divisible_by_power_of_2(&xs, pow))
        })],
    );
}

fn benchmark_natural_divisible_by_power_of_2_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.divisible_by_power_of_2(u64)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_4_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("n"),
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
fn benchmark_natural_divisible_by_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.divisible_by_power_of_2(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural.divisible_by_power_of_2(u64)", &mut |(n, pow)| {
                no_out!(n.divisible_by_power_of_2(pow))
            }),
            (
                "Natural.trailing_zeros().map_or(true, |z| z >= u64)",
                &mut |(n, pow)| no_out!(n.trailing_zeros().map_or(true, |z| z >= pow)),
            ),
        ],
    );
}
