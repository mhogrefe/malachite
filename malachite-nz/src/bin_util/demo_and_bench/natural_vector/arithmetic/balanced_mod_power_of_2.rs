// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{BalancedMod, BalancedModPowerOf2, PowerOf2};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_balanced_mod_power_of_2);
    register_demo!(runner, demo_natural_vector_balanced_mod_power_of_2_ref);

    register_bench!(
        runner,
        benchmark_natural_vector_balanced_mod_power_of_2_algorithms
    );
}

fn demo_natural_vector_balanced_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, pow) in natural_vector_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.balanced_mod_power_of_2({pow}) = {}",
            v.balanced_mod_power_of_2(pow)
        );
    }
}

fn demo_natural_vector_balanced_mod_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, pow) in natural_vector_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).balanced_mod_power_of_2({pow}) = {}",
            (&v).balanced_mod_power_of_2(pow)
        );
    }
}

fn benchmark_natural_vector_balanced_mod_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.balanced_mod_power_of_2(u64)",
        BenchmarkType::Algorithms,
        natural_vector_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, pow)| {
                no_out!(v.balanced_mod_power_of_2(pow));
            }),
            ("using balanced_mod", &mut |(v, pow)| {
                no_out!(v.balanced_mod(Natural::power_of_2(pow)));
            }),
        ],
    );
}
