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
use malachite_nz::test_util::bench::bucketers::triple_1_integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_integer_rounding_mode_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_entrywise_div_round);
    register_demo!(runner, demo_integer_vector_entrywise_div_round_ref);
    register_demo!(runner, demo_integer_vector_entrywise_div_round_assign);

    register_bench!(
        runner,
        benchmark_integer_vector_entrywise_div_round_evaluation_strategy
    );
}

fn demo_integer_vector_entrywise_div_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, rm) in integer_vector_integer_rounding_mode_triple_gen_var_1()
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

fn demo_integer_vector_entrywise_div_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, rm) in integer_vector_integer_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).entrywise_div_round(&{c}, {rm}) = {}",
            (&v).entrywise_div_round(&c, rm)
        );
    }
}

fn demo_integer_vector_entrywise_div_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, c, rm) in integer_vector_integer_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.entrywise_div_round_assign(&c, rm);
        println!("v := {v_old}; v.entrywise_div_round_assign({c}, {rm}); v = {v}");
    }
}

fn benchmark_integer_vector_entrywise_div_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.entrywise_div_round(Integer, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        integer_vector_integer_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_vector_bit_bucketer("v"),
        &mut [
            (
                "IntegerVector.entrywise_div_round(Integer, RoundingMode)",
                &mut |(v, c, rm)| {
                    no_out!(v.entrywise_div_round(c, rm));
                },
            ),
            (
                "IntegerVector.entrywise_div_round(&Integer, RoundingMode)",
                &mut |(v, c, rm)| {
                    no_out!(v.entrywise_div_round(&c, rm));
                },
            ),
            (
                "(&IntegerVector).entrywise_div_round(Integer, RoundingMode)",
                &mut |(v, c, rm)| {
                    no_out!((&v).entrywise_div_round(c, rm));
                },
            ),
            (
                "(&IntegerVector).entrywise_div_round(&Integer, RoundingMode)",
                &mut |(v, c, rm)| {
                    no_out!((&v).entrywise_div_round(&c, rm));
                },
            ),
            (
                "IntegerVector.entrywise_div_round_assign(Integer, RoundingMode)",
                &mut |(mut v, c, rm)| {
                    v.entrywise_div_round_assign(c, rm);
                },
            ),
            (
                "IntegerVector.entrywise_div_round_assign(&Integer, RoundingMode)",
                &mut |(mut v, c, rm)| {
                    v.entrywise_div_round_assign(&c, rm);
                },
            ),
        ],
    );
}
