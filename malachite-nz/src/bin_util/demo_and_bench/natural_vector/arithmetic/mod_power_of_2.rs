// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Assign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_power_of_2);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_ref);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_assign);

    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_evaluation_strategy
    );
}

fn demo_natural_vector_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, pow) in natural_vector_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "({v_old}).mod_power_of_2({pow}) = {}",
            v.mod_power_of_2(pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, pow) in natural_vector_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({v})).mod_power_of_2({pow}) = {}",
            (&v).mod_power_of_2(pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, pow) in natural_vector_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_power_of_2_assign(pow);
        println!("v := {v_old}; v.mod_power_of_2_assign({pow}); v = {v}");
    }
}

fn benchmark_natural_vector_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector.mod_power_of_2(u64)", &mut |(v, pow)| {
                no_out!(v.mod_power_of_2(pow));
            }),
            ("(&NaturalVector).mod_power_of_2(u64)", &mut |(v, pow)| {
                no_out!((&v).mod_power_of_2(pow));
            }),
            (
                "NaturalVector.mod_power_of_2_assign(u64)",
                &mut |(mut v, pow)| v.mod_power_of_2_assign(pow),
            ),
        ],
    );
}
