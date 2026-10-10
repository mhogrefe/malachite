// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{EntrywiseDivRound, EntrywiseDivRoundAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_rounding_mode_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_entrywise_div_round);
    register_demo!(runner, demo_natural_vector_entrywise_div_round_ref);
    register_demo!(runner, demo_natural_vector_entrywise_div_round_assign);

    register_bench!(
        runner,
        benchmark_natural_vector_entrywise_div_round_evaluation_strategy
    );
}

fn demo_natural_vector_entrywise_div_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, rm) in natural_vector_natural_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.entrywise_div_round({c}, {rm}) = {}",
            v.entrywise_div_round(&c, rm)
        );
    }
}

fn demo_natural_vector_entrywise_div_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, rm) in natural_vector_natural_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).entrywise_div_round(&{c}, {rm}) = {}",
            (&v).entrywise_div_round(&c, rm)
        );
    }
}

fn demo_natural_vector_entrywise_div_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, c, rm) in natural_vector_natural_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.entrywise_div_round_assign(&c, rm);
        println!("v := {v_old}; v.entrywise_div_round_assign({c}, {rm}); v = {v}");
    }
}

fn benchmark_natural_vector_entrywise_div_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.entrywise_div_round(Natural, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "NaturalVector.entrywise_div_round(Natural, RoundingMode)",
                &mut |(v, c, rm)| {
                    no_out!(v.entrywise_div_round(c, rm));
                },
            ),
            (
                "NaturalVector.entrywise_div_round(&Natural, RoundingMode)",
                &mut |(v, c, rm)| {
                    no_out!(v.entrywise_div_round(&c, rm));
                },
            ),
            (
                "(&NaturalVector).entrywise_div_round(Natural, RoundingMode)",
                &mut |(v, c, rm)| {
                    no_out!((&v).entrywise_div_round(c, rm));
                },
            ),
            (
                "(&NaturalVector).entrywise_div_round(&Natural, RoundingMode)",
                &mut |(v, c, rm)| {
                    no_out!((&v).entrywise_div_round(&c, rm));
                },
            ),
            (
                "NaturalVector.entrywise_div_round_assign(Natural, RoundingMode)",
                &mut |(mut v, c, rm)| {
                    v.entrywise_div_round_assign(c, rm);
                },
            ),
            (
                "NaturalVector.entrywise_div_round_assign(&Natural, RoundingMode)",
                &mut |(mut v, c, rm)| {
                    v.entrywise_div_round_assign(&c, rm);
                },
            ),
        ],
    );
}
