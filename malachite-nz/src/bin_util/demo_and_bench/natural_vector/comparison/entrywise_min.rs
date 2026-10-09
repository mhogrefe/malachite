// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{EntrywiseMin, EntrywiseMinAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_natural_vector_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_entrywise_min);
    register_demo!(runner, demo_natural_vector_entrywise_min_ref_ref);
    register_demo!(runner, demo_natural_vector_entrywise_min_assign);

    register_bench!(
        runner,
        benchmark_natural_vector_entrywise_min_evaluation_strategy
    );
}

fn demo_natural_vector_entrywise_min(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in natural_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{v_old}.entrywise_min({w_old}) = {}", v.entrywise_min(w));
    }
}

fn demo_natural_vector_entrywise_min_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in natural_vector_pair_gen_var_1().get(gm, config).take(limit) {
        println!("(&{v}).entrywise_min(&{w}) = {}", (&v).entrywise_min(&w));
    }
}

fn demo_natural_vector_entrywise_min_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w) in natural_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let v_old = v.clone();
        v.entrywise_min_assign(&w);
        println!("v := {v_old}; v.entrywise_min_assign(&{w}); v = {v}");
    }
}

fn benchmark_natural_vector_entrywise_min_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.entrywise_min(NaturalVector)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_vector_max_bit_bucketer("v", "w"),
        &mut [
            (
                "NaturalVector.entrywise_min(NaturalVector)",
                &mut |(v, w)| {
                    no_out!(v.entrywise_min(w));
                },
            ),
            (
                "NaturalVector.entrywise_min(&NaturalVector)",
                &mut |(v, w)| {
                    no_out!(v.entrywise_min(&w));
                },
            ),
            (
                "(&NaturalVector).entrywise_min(NaturalVector)",
                &mut |(v, w)| {
                    no_out!((&v).entrywise_min(w));
                },
            ),
            (
                "(&NaturalVector).entrywise_min(&NaturalVector)",
                &mut |(v, w)| {
                    no_out!((&v).entrywise_min(&w));
                },
            ),
            (
                "NaturalVector.entrywise_min_assign(NaturalVector)",
                &mut |(mut v, w)| {
                    v.entrywise_min_assign(w);
                },
            ),
            (
                "NaturalVector.entrywise_min_assign(&NaturalVector)",
                &mut |(mut v, w)| {
                    v.entrywise_min_assign(&w);
                },
            ),
        ],
    );
}
