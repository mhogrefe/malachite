// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Add, ModPowerOf2AddAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_vector_unsigned_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_power_of_2_add);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_add_ref_ref);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_add_assign);
    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_add_evaluation_strategy
    );
}

fn demo_natural_vector_mod_power_of_2_add(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, pow) in natural_vector_natural_vector_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (v_old, w_old) = (v.clone(), w.clone());
        println!(
            "{v_old}.mod_power_of_2_add({w_old}, {pow}) = {}",
            v.mod_power_of_2_add(w, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_add_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, pow) in natural_vector_natural_vector_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_power_of_2_add(&{w}, {pow}) = {}",
            (&v).mod_power_of_2_add(&w, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_add_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w, pow) in natural_vector_natural_vector_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_power_of_2_add_assign(&w, pow);
        println!("v := {v_old}; v.mod_power_of_2_add_assign(&{w}, {pow}); v = {v}");
    }
}

fn benchmark_natural_vector_mod_power_of_2_add_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_power_of_2_add(NaturalVector, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_vector_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "NaturalVector.mod_power_of_2_add(NaturalVector, u64)",
                &mut |(v, w, pow)| {
                    no_out!(v.mod_power_of_2_add(w, pow));
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add(&NaturalVector, u64)",
                &mut |(v, w, pow)| {
                    no_out!(v.mod_power_of_2_add(&w, pow));
                },
            ),
            (
                "(&NaturalVector).mod_power_of_2_add(NaturalVector, u64)",
                &mut |(v, w, pow)| {
                    no_out!((&v).mod_power_of_2_add(w, pow));
                },
            ),
            (
                "(&NaturalVector).mod_power_of_2_add(&NaturalVector, u64)",
                &mut |(v, w, pow)| {
                    no_out!((&v).mod_power_of_2_add(&w, pow));
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_assign(NaturalVector, u64)",
                &mut |(mut v, w, pow)| {
                    v.mod_power_of_2_add_assign(w, pow);
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_assign(&NaturalVector, u64)",
                &mut |(mut v, w, pow)| {
                    v.mod_power_of_2_add_assign(&w, pow);
                },
            ),
        ],
    );
}
