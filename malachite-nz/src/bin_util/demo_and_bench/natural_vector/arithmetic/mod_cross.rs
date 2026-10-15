// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModCross, ModCrossAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_3_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_vector_natural_triple_gen_var_3;
use malachite_nz::test_util::natural_vector::arithmetic::mod_cross::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_cross);
    register_demo!(runner, demo_natural_vector_mod_cross_ref);
    register_demo!(runner, demo_natural_vector_mod_cross_assign);
    register_bench!(
        runner,
        benchmark_natural_vector_mod_cross_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_vector_mod_cross_algorithms);
}

fn demo_natural_vector_mod_cross(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, m) in natural_vector_natural_vector_natural_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{v_old}.mod_cross({w_old}, {m}) = {}", v.mod_cross(w, &m));
    }
}

fn demo_natural_vector_mod_cross_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, m) in natural_vector_natural_vector_natural_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).mod_cross(&{w}, {m}) = {}", (&v).mod_cross(&w, &m));
    }
}

fn demo_natural_vector_mod_cross_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w, m) in natural_vector_natural_vector_natural_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_cross_assign(&w, &m);
        println!("v := {v_old}; v.mod_cross_assign(&{w}, {m}); v = {v}");
    }
}

fn benchmark_natural_vector_mod_cross_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_cross(NaturalVector, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_vector_natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            ("v.mod_cross(w, &m)", &mut |(v, w, m)| {
                no_out!(v.mod_cross(w, &m));
            }),
            ("v.mod_cross(&w, &m)", &mut |(v, w, m)| {
                no_out!(v.mod_cross(&w, &m));
            }),
            ("(&v).mod_cross(w, &m)", &mut |(v, w, m)| {
                no_out!((&v).mod_cross(w, &m));
            }),
            ("(&v).mod_cross(&w, &m)", &mut |(v, w, m)| {
                no_out!((&v).mod_cross(&w, &m));
            }),
            ("v.mod_cross_assign(w, &m)", &mut |(mut v, w, m)| {
                v.mod_cross_assign(w, &m);
            }),
            ("v.mod_cross_assign(&w, &m)", &mut |(mut v, w, m)| {
                v.mod_cross_assign(&w, &m);
            }),
        ],
    );
}

fn benchmark_natural_vector_mod_cross_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_cross(NaturalVector, Natural)",
        BenchmarkType::Algorithms,
        natural_vector_natural_vector_natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            ("default", &mut |(v, w, m)| {
                no_out!((&v).mod_cross(&w, &m));
            }),
            ("naive", &mut |(v, w, m)| {
                no_out!(natural_vector_mod_cross_naive(&v, &w, &m));
            }),
        ],
    );
}
