// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPowerOf2IsReduced, PowerOf2};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_power_of_2_is_reduced);

    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_is_reduced_algorithms
    );
}

fn demo_natural_mod_power_of_2_is_reduced(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, log_base) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        if n.mod_power_of_2_is_reduced(log_base) {
            println!("{n} is reduced mod 2^{log_base}");
        } else {
            println!("{n} is not reduced mod 2^{log_base}");
        }
    }
}

fn benchmark_natural_mod_power_of_2_is_reduced_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_add_limb(&[Limb], Limb, u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |(n, log_base)| {
                no_out!(n.mod_power_of_2_is_reduced(log_base))
            }),
            ("using mod_is_reduced", &mut |(n, log_base)| {
                no_out!(n.mod_is_reduced(&Natural::power_of_2(log_base)))
            }),
        ],
    );
}
