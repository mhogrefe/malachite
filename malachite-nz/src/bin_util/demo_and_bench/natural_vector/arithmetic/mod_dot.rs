// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModDot;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_vector_natural_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_dot);
    register_demo!(runner, demo_natural_vector_mod_dot_ref_ref);
    register_bench!(runner, benchmark_natural_vector_mod_dot_evaluation_strategy);
}

fn demo_natural_vector_mod_dot(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, m) in natural_vector_natural_vector_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        let w_old = w.clone();
        println!(
            "{v_old}.mod_dot({w_old}, {m}) = {}",
            v.mod_dot(w, m.clone())
        );
    }
}

fn demo_natural_vector_mod_dot_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, m) in natural_vector_natural_vector_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_dot(&{w}, {m}) = {}",
            (&v).mod_dot(&w, m.clone())
        );
    }
}

fn benchmark_natural_vector_mod_dot_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_dot(NaturalVector, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_vector_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "NaturalVector.mod_dot(NaturalVector, Natural)",
                &mut |(v, w, m)| {
                    no_out!(v.mod_dot(w, m));
                },
            ),
            (
                "NaturalVector.mod_dot(&NaturalVector, Natural)",
                &mut |(v, w, m)| {
                    no_out!(v.mod_dot(&w, m));
                },
            ),
            (
                "(&NaturalVector).mod_dot(NaturalVector, Natural)",
                &mut |(v, w, m)| {
                    no_out!((&v).mod_dot(w, m));
                },
            ),
            (
                "(&NaturalVector).mod_dot(&NaturalVector, Natural)",
                &mut |(v, w, m)| {
                    no_out!((&v).mod_dot(&w, m));
                },
            ),
        ],
    );
}
