// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen_var_1;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::is_power_of_2::limbs_is_power_of_2;
use malachite_nz::test_util::bench::bucketers::pair_2_natural_bit_bucketer;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_is_power_of_2);
    register_demo!(runner, demo_natural_is_power_of_2);

    register_bench!(runner, benchmark_limbs_is_power_of_2);
    register_bench!(runner, benchmark_natural_is_power_of_2_library_comparison);
}

fn demo_limbs_is_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        println!(
            "limbs_is_power_of_2({:?}) = {:?}",
            xs,
            limbs_is_power_of_2(&xs)
        );
    }
}

fn demo_natural_is_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        if n.is_power_of_2() {
            println!("{n} is a power of 2");
        } else {
            println!("{n} is not a power of 2");
        }
    }
}

fn benchmark_limbs_is_power_of_2(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_is_power_of_2(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(limbs_is_power_of_2(&xs)))],
    );
}

fn benchmark_natural_is_power_of_2_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.is_power_of_2()",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(n.is_power_of_2())),
            ("rug", &mut |(n, _)| no_out!(n.is_power_of_two())),
        ],
    );
}
