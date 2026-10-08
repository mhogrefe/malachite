// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModNeg, ModNegAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_neg);
    register_demo!(runner, demo_natural_vector_mod_neg_ref);
    register_demo!(runner, demo_natural_vector_mod_neg_assign);
    register_bench!(runner, benchmark_natural_vector_mod_neg_evaluation_strategy);
}

fn demo_natural_vector_mod_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in natural_vector_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("({v_old}).mod_neg({m}) = {}", v.mod_neg(&m));
    }
}

fn demo_natural_vector_mod_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in natural_vector_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).mod_neg(&{m}) = {}", (&v).mod_neg(&m));
    }
}

fn demo_natural_vector_mod_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, m) in natural_vector_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_neg_assign(&m);
        println!("v := {v_old}; v.mod_neg_assign(&{m}); v = {v}");
    }
}

fn benchmark_natural_vector_mod_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_neg(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector.mod_neg(Natural)", &mut |(v, m)| {
                no_out!(v.mod_neg(m));
            }),
            ("NaturalVector.mod_neg(&Natural)", &mut |(v, m)| {
                no_out!(v.mod_neg(&m));
            }),
            ("(&NaturalVector).mod_neg(Natural)", &mut |(v, m)| {
                no_out!((&v).mod_neg(m));
            }),
            ("(&NaturalVector).mod_neg(&Natural)", &mut |(v, m)| {
                no_out!((&v).mod_neg(&m));
            }),
            (
                "NaturalVector.mod_neg_assign(Natural)",
                &mut |(mut v, m)| {
                    v.mod_neg_assign(m);
                },
            ),
            (
                "NaturalVector.mod_neg_assign(&Natural)",
                &mut |(mut v, m)| {
                    v.mod_neg_assign(&m);
                },
            ),
        ],
    );
}
