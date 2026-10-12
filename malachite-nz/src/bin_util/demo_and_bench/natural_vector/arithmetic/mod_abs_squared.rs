// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModAbsSquared;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_abs_squared);
    register_demo!(runner, demo_natural_vector_mod_abs_squared_ref);
    register_bench!(
        runner,
        benchmark_natural_vector_mod_abs_squared_evaluation_strategy
    );
}

fn demo_natural_vector_mod_abs_squared(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in natural_vector_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        let m_old = m.clone();
        println!(
            "{v_old}.mod_abs_squared({m_old}) = {}",
            v.mod_abs_squared(m)
        );
    }
}

fn demo_natural_vector_mod_abs_squared_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in natural_vector_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).mod_abs_squared({m}) = {}", (&v).mod_abs_squared(&m));
    }
}

fn benchmark_natural_vector_mod_abs_squared_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_abs_squared(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector.mod_abs_squared(Natural)", &mut |(v, m)| {
                no_out!(v.mod_abs_squared(m));
            }),
            ("NaturalVector.mod_abs_squared(&Natural)", &mut |(v, m)| {
                no_out!(v.mod_abs_squared(&m));
            }),
            (
                "(&NaturalVector).mod_abs_squared(Natural)",
                &mut |(v, m)| {
                    no_out!((&v).mod_abs_squared(m));
                },
            ),
            (
                "(&NaturalVector).mod_abs_squared(&Natural)",
                &mut |(v, m)| {
                    no_out!((&v).mod_abs_squared(&m));
                },
            ),
        ],
    );
}
