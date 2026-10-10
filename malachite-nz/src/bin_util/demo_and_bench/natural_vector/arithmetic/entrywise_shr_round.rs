// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shl;
use malachite_base::num::arithmetic::traits::{EntrywiseShrRound, EntrywiseShrRoundAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_vector_signed_rounding_mode_triple_gen_var_2,
    natural_vector_unsigned_rounding_mode_triple_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_vector_entrywise_shr_round_unsigned);
    register_unsigned_demos!(runner, demo_natural_vector_entrywise_shr_round_unsigned_ref);
    register_unsigned_demos!(
        runner,
        demo_natural_vector_entrywise_shr_round_assign_unsigned
    );
    register_signed_demos!(runner, demo_natural_vector_entrywise_shr_round_signed);
    register_signed_demos!(runner, demo_natural_vector_entrywise_shr_round_signed_ref);
    register_signed_demos!(
        runner,
        demo_natural_vector_entrywise_shr_round_assign_signed
    );
    register_unsigned_benches!(
        runner,
        benchmark_natural_vector_entrywise_shr_round_unsigned_evaluation_strategy
    );
    register_signed_benches!(
        runner,
        benchmark_natural_vector_entrywise_shr_round_signed_evaluation_strategy
    );
}

fn demo_natural_vector_entrywise_shr_round_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: Shl<T, Output = NaturalVector> + EntrywiseShrRound<T, Output = NaturalVector>,
{
    for (v, bits, rm) in natural_vector_unsigned_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{}.entrywise_shr_round({}, {}) = {}",
            v_old,
            bits,
            rm,
            v.entrywise_shr_round(bits, rm)
        );
    }
}

fn demo_natural_vector_entrywise_shr_round_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: Shl<T, Output = NaturalVector>,
    for<'a> &'a NaturalVector: EntrywiseShrRound<T, Output = NaturalVector>,
{
    for (v, bits, rm) in natural_vector_unsigned_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).entrywise_shr_round({}, {}) = {}",
            v,
            bits,
            rm,
            (&v).entrywise_shr_round(bits, rm)
        );
    }
}

fn demo_natural_vector_entrywise_shr_round_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: Shl<T, Output = NaturalVector> + EntrywiseShrRoundAssign<T>,
{
    for (mut v, bits, rm) in natural_vector_unsigned_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.entrywise_shr_round_assign(bits, rm);
        println!("v := {v_old}; v.entrywise_shr_round_assign({bits}, {rm}); v = {v}");
    }
}

fn benchmark_natural_vector_entrywise_shr_round_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalVector: Shl<T, Output = NaturalVector>
        + EntrywiseShrRound<T, Output = NaturalVector>
        + EntrywiseShrRoundAssign<T>,
    for<'a> &'a NaturalVector: EntrywiseShrRound<T, Output = NaturalVector>,
{
    run_benchmark(
        &format!(
            "NaturalVector.entrywise_shr_round({}, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        natural_vector_unsigned_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "NaturalVector.entrywise_shr_round(T, RoundingMode)",
                &mut |(v, bits, rm)| {
                    no_out!(v.entrywise_shr_round(bits, rm));
                },
            ),
            (
                "(&NaturalVector).entrywise_shr_round(T, RoundingMode)",
                &mut |(v, bits, rm)| {
                    no_out!((&v).entrywise_shr_round(bits, rm));
                },
            ),
            (
                "NaturalVector.entrywise_shr_round_assign(T, RoundingMode)",
                &mut |(mut v, bits, rm)| {
                    v.entrywise_shr_round_assign(bits, rm);
                },
            ),
        ],
    );
}

fn demo_natural_vector_entrywise_shr_round_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: Shl<T, Output = NaturalVector> + EntrywiseShrRound<T, Output = NaturalVector>,
{
    for (v, bits, rm) in natural_vector_signed_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{}.entrywise_shr_round({}, {}) = {}",
            v_old,
            bits,
            rm,
            v.entrywise_shr_round(bits, rm)
        );
    }
}

fn demo_natural_vector_entrywise_shr_round_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: Shl<T, Output = NaturalVector>,
    for<'a> &'a NaturalVector: EntrywiseShrRound<T, Output = NaturalVector>,
{
    for (v, bits, rm) in natural_vector_signed_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).entrywise_shr_round({}, {}) = {}",
            v,
            bits,
            rm,
            (&v).entrywise_shr_round(bits, rm)
        );
    }
}

fn demo_natural_vector_entrywise_shr_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: Shl<T, Output = NaturalVector> + EntrywiseShrRoundAssign<T>,
{
    for (mut v, bits, rm) in natural_vector_signed_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.entrywise_shr_round_assign(bits, rm);
        println!("v := {v_old}; v.entrywise_shr_round_assign({bits}, {rm}); v = {v}");
    }
}

fn benchmark_natural_vector_entrywise_shr_round_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalVector: Shl<T, Output = NaturalVector>
        + EntrywiseShrRound<T, Output = NaturalVector>
        + EntrywiseShrRoundAssign<T>,
    for<'a> &'a NaturalVector: EntrywiseShrRound<T, Output = NaturalVector>,
{
    run_benchmark(
        &format!(
            "NaturalVector.entrywise_shr_round({}, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        natural_vector_signed_rounding_mode_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "NaturalVector.entrywise_shr_round(T, RoundingMode)",
                &mut |(v, bits, rm)| {
                    no_out!(v.entrywise_shr_round(bits, rm));
                },
            ),
            (
                "(&NaturalVector).entrywise_shr_round(T, RoundingMode)",
                &mut |(v, bits, rm)| {
                    no_out!((&v).entrywise_shr_round(bits, rm));
                },
            ),
            (
                "NaturalVector.entrywise_shr_round_assign(T, RoundingMode)",
                &mut |(mut v, bits, rm)| {
                    v.entrywise_shr_round_assign(bits, rm);
                },
            ),
        ],
    );
}
