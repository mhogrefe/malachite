// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModSub, ModSubAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_matrix_natural_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_sub);
    register_demo!(runner, demo_natural_matrix_mod_sub_ref_ref);
    register_demo!(runner, demo_natural_matrix_mod_sub_assign);
    register_bench!(runner, benchmark_natural_matrix_mod_sub_evaluation_strategy);
}

fn demo_natural_matrix_mod_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in natural_matrix_natural_matrix_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (a_old, b_old) = (a.clone(), b.clone());
        println!("{a_old}.mod_sub({b_old}, {m}) = {}", a.mod_sub(b, &m));
    }
}

fn demo_natural_matrix_mod_sub_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in natural_matrix_natural_matrix_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).mod_sub(&{b}, {m}) = {}", (&a).mod_sub(&b, &m));
    }
}

fn demo_natural_matrix_mod_sub_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, m) in natural_matrix_natural_matrix_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_sub_assign(&b, &m);
        println!("a := {a_old}; a.mod_sub_assign(&{b}, {m}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_sub(NaturalMatrix, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_matrix_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_sub(NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_sub(b, m));
                },
            ),
            (
                "NaturalMatrix.mod_sub(&NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_sub(&b, m));
                },
            ),
            (
                "(&NaturalMatrix).mod_sub(NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_sub(b, m));
                },
            ),
            (
                "(&NaturalMatrix).mod_sub(&NaturalMatrix, Natural)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_sub(&b, m));
                },
            ),
            (
                "NaturalMatrix.mod_sub_assign(NaturalMatrix, Natural)",
                &mut |(mut a, b, m)| {
                    a.mod_sub_assign(b, m);
                },
            ),
            (
                "NaturalMatrix.mod_sub_assign(&NaturalMatrix, Natural)",
                &mut |(mut a, b, m)| {
                    a.mod_sub_assign(&b, m);
                },
            ),
        ],
    );
}
