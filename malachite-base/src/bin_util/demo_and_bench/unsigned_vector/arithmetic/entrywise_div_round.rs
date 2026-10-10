// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{EntrywiseDivRound, EntrywiseDivRoundAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_rounding_mode_triple_gen_var_2;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_entrywise_div_round);
    register_unsigned_demos!(runner, demo_unsigned_vector_entrywise_div_round_ref);
    register_unsigned_demos!(runner, demo_unsigned_vector_entrywise_div_round_assign);

    register_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_entrywise_div_round_evaluation_strategy
    );
}

fn demo_unsigned_vector_entrywise_div_round<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, c, rm) in unsigned_vector_unsigned_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.entrywise_div_round({c}, {rm}) = {}",
            v.entrywise_div_round(c, rm)
        );
    }
}

fn demo_unsigned_vector_entrywise_div_round_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, c, rm) in unsigned_vector_unsigned_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).entrywise_div_round({c}, {rm}) = {}",
            (&v).entrywise_div_round(c, rm)
        );
    }
}

fn demo_unsigned_vector_entrywise_div_round_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut v, c, rm) in unsigned_vector_unsigned_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.entrywise_div_round_assign(c, rm);
        println!("v := {v_old}; v.entrywise_div_round_assign({c}, {rm}); v = {v}");
    }
}

fn benchmark_unsigned_vector_entrywise_div_round_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.entrywise_div_round({}, RoundingMode)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_rounding_mode_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            (
                "UnsignedVector.entrywise_div_round(T, RoundingMode)",
                &mut |(v, c, rm)| {
                    no_out!(v.entrywise_div_round(c, rm));
                },
            ),
            (
                "(&UnsignedVector).entrywise_div_round(T, RoundingMode)",
                &mut |(v, c, rm)| {
                    no_out!((&v).entrywise_div_round(c, rm));
                },
            ),
            (
                "UnsignedVector.entrywise_div_round_assign(T, RoundingMode)",
                &mut |(mut v, c, rm)| {
                    v.entrywise_div_round_assign(c, rm);
                },
            ),
        ],
    );
}
