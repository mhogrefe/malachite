// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{EntrywiseShrRound, EntrywiseShrRoundAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_rounding_mode_triple_gen_var_1;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_unsigned_matrix_entrywise_shr_round);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_matrix_entrywise_shr_round_ref);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_matrix_entrywise_shr_round_assign);

    register_unsigned_unsigned_benches!(
        runner,
        benchmark_unsigned_matrix_entrywise_shr_round_evaluation_strategy
    );
}

fn demo_unsigned_matrix_entrywise_shr_round<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedMatrix<T>: EntrywiseShrRound<U, Output = UnsignedMatrix<T>>,
{
    for (v, bits, rm) in unsigned_matrix_unsigned_rounding_mode_triple_gen_var_1::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.entrywise_shr_round({bits}, {rm}) = {}",
            v.entrywise_shr_round(bits, rm)
        );
    }
}

fn demo_unsigned_matrix_entrywise_shr_round_ref<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a UnsignedMatrix<T>: EntrywiseShrRound<U, Output = UnsignedMatrix<T>>,
{
    for (v, bits, rm) in unsigned_matrix_unsigned_rounding_mode_triple_gen_var_1::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).entrywise_shr_round({bits}, {rm}) = {}",
            (&v).entrywise_shr_round(bits, rm)
        );
    }
}

fn demo_unsigned_matrix_entrywise_shr_round_assign<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedMatrix<T>: EntrywiseShrRoundAssign<U>,
{
    for (mut v, bits, rm) in unsigned_matrix_unsigned_rounding_mode_triple_gen_var_1::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.entrywise_shr_round_assign(bits, rm);
        println!("v := {v_old}; v.entrywise_shr_round_assign({bits}, {rm}); v = {v}");
    }
}

fn benchmark_unsigned_matrix_entrywise_shr_round_evaluation_strategy<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    UnsignedMatrix<T>:
        EntrywiseShrRound<U, Output = UnsignedMatrix<T>> + EntrywiseShrRoundAssign<U>,
    for<'a> &'a UnsignedMatrix<T>: EntrywiseShrRound<U, Output = UnsignedMatrix<T>>,
{
    run_benchmark(
        &format!(
            "UnsignedMatrix<{}>.entrywise_shr_round({}, RoundingMode)",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_rounding_mode_triple_gen_var_1::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("v"),
        &mut [
            (
                "UnsignedMatrix.entrywise_shr_round(U, RoundingMode)",
                &mut |(v, bits, rm)| {
                    no_out!(v.entrywise_shr_round(bits, rm));
                },
            ),
            (
                "(&UnsignedMatrix).entrywise_shr_round(U, RoundingMode)",
                &mut |(v, bits, rm)| {
                    no_out!((&v).entrywise_shr_round(bits, rm));
                },
            ),
            (
                "UnsignedMatrix.entrywise_shr_round_assign(U, RoundingMode)",
                &mut |(mut v, bits, rm)| {
                    v.entrywise_shr_round_assign(bits, rm);
                },
            ),
        ],
    );
}
