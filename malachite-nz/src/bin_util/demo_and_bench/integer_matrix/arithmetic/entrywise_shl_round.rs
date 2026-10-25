// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shr;
use malachite_base::num::arithmetic::traits::{EntrywiseShlRound, EntrywiseShlRoundAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::bench::bucketers::triple_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_signed_rounding_mode_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_signed_demos!(runner, demo_integer_matrix_entrywise_shl_round_signed);
    register_signed_demos!(runner, demo_integer_matrix_entrywise_shl_round_signed_ref);
    register_signed_demos!(
        runner,
        demo_integer_matrix_entrywise_shl_round_assign_signed
    );
    register_signed_benches!(
        runner,
        benchmark_integer_matrix_entrywise_shl_round_signed_evaluation_strategy
    );
}

fn demo_integer_matrix_entrywise_shl_round_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    IntegerMatrix: Shr<T, Output = IntegerMatrix> + EntrywiseShlRound<T, Output = IntegerMatrix>,
{
    for (v, bits, rm) in integer_matrix_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{}.entrywise_shl_round({}, {}) = {}",
            v_old,
            bits,
            rm,
            v.entrywise_shl_round(bits, rm)
        );
    }
}

fn demo_integer_matrix_entrywise_shl_round_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    IntegerMatrix: Shr<T, Output = IntegerMatrix>,
    for<'a> &'a IntegerMatrix: EntrywiseShlRound<T, Output = IntegerMatrix>,
{
    for (v, bits, rm) in integer_matrix_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).entrywise_shl_round({}, {}) = {}",
            v,
            bits,
            rm,
            (&v).entrywise_shl_round(bits, rm)
        );
    }
}

fn demo_integer_matrix_entrywise_shl_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    IntegerMatrix: Shr<T, Output = IntegerMatrix> + EntrywiseShlRoundAssign<T>,
{
    for (mut v, bits, rm) in integer_matrix_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.entrywise_shl_round_assign(bits, rm);
        println!("v := {v_old}; v.entrywise_shl_round_assign({bits}, {rm}); v = {v}");
    }
}

fn benchmark_integer_matrix_entrywise_shl_round_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    IntegerMatrix: Shr<T, Output = IntegerMatrix>
        + EntrywiseShlRound<T, Output = IntegerMatrix>
        + EntrywiseShlRoundAssign<T>,
    for<'a> &'a IntegerMatrix: EntrywiseShlRound<T, Output = IntegerMatrix>,
{
    run_benchmark(
        &format!(
            "IntegerMatrix.entrywise_shl_round({}, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        integer_matrix_signed_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_matrix_bit_bucketer("v"),
        &mut [
            (
                "IntegerMatrix.entrywise_shl_round(T, RoundingMode)",
                &mut |(v, bits, rm)| {
                    no_out!(v.entrywise_shl_round(bits, rm));
                },
            ),
            (
                "(&IntegerMatrix).entrywise_shl_round(T, RoundingMode)",
                &mut |(v, bits, rm)| {
                    no_out!((&v).entrywise_shl_round(bits, rm));
                },
            ),
            (
                "IntegerMatrix.entrywise_shl_round_assign(T, RoundingMode)",
                &mut |(mut v, bits, rm)| {
                    v.entrywise_shl_round_assign(bits, rm);
                },
            ),
        ],
    );
}
