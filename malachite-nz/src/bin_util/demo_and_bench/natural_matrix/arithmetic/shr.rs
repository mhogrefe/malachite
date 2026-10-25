// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::EntrywiseShrRound;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_matrix_signed_pair_gen_var_1, natural_matrix_unsigned_pair_gen_var_5,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_matrix_shr_unsigned);
    register_unsigned_demos!(runner, demo_natural_matrix_shr_unsigned_ref);
    register_unsigned_demos!(runner, demo_natural_matrix_shr_assign_unsigned);
    register_signed_demos!(runner, demo_natural_matrix_shr_signed);
    register_signed_demos!(runner, demo_natural_matrix_shr_signed_ref);
    register_signed_demos!(runner, demo_natural_matrix_shr_assign_signed);
    register_unsigned_benches!(
        runner,
        benchmark_natural_matrix_shr_unsigned_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_natural_matrix_shr_unsigned_algorithms);
    register_signed_benches!(
        runner,
        benchmark_natural_matrix_shr_signed_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_natural_matrix_shr_signed_algorithms);
}

fn demo_natural_matrix_shr_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalMatrix: Shr<T, Output = NaturalMatrix>,
{
    for (v, bits) in natural_matrix_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} >> {bits} = {}", v >> bits);
    }
}

fn demo_natural_matrix_shr_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a NaturalMatrix: Shr<T, Output = NaturalMatrix>,
{
    for (v, bits) in natural_matrix_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} >> {bits} = {}", &v >> bits);
    }
}

fn demo_natural_matrix_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalMatrix: ShrAssign<T>,
{
    for (mut v, bits) in natural_matrix_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v >>= bits;
        println!("v := {v_old}; v >>= {bits}; v = {v}");
    }
}

fn benchmark_natural_matrix_shr_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalMatrix: Shr<T, Output = NaturalMatrix> + ShrAssign<T>,
    for<'a> &'a NaturalMatrix: Shr<T, Output = NaturalMatrix>,
{
    run_benchmark(
        &format!("NaturalMatrix >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_matrix_unsigned_pair_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("v"),
        &mut [
            ("NaturalMatrix >> T", &mut |(v, bits)| {
                no_out!(v >> bits);
            }),
            ("&NaturalMatrix >> T", &mut |(v, bits)| {
                no_out!(&v >> bits);
            }),
            ("NaturalMatrix >>= T", &mut |(mut v, bits)| {
                v >>= bits;
            }),
        ],
    );
}

fn benchmark_natural_matrix_shr_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalMatrix: Shr<T, Output = NaturalMatrix> + EntrywiseShrRound<T, Output = NaturalMatrix>,
{
    run_benchmark(
        &format!("NaturalMatrix >> {}", T::NAME),
        BenchmarkType::Algorithms,
        natural_matrix_unsigned_pair_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, bits)| {
                no_out!(v >> bits);
            }),
            ("using entrywise_shr_round with Floor", &mut |(v, bits)| {
                no_out!(v.entrywise_shr_round(bits, Floor));
            }),
        ],
    );
}

fn demo_natural_matrix_shr_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    NaturalMatrix: Shr<T, Output = NaturalMatrix>,
{
    for (v, bits) in natural_matrix_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} >> {bits} = {}", v >> bits);
    }
}

fn demo_natural_matrix_shr_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a NaturalMatrix: Shr<T, Output = NaturalMatrix>,
{
    for (v, bits) in natural_matrix_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} >> {bits} = {}", &v >> bits);
    }
}

fn demo_natural_matrix_shr_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalMatrix: ShrAssign<T>,
{
    for (mut v, bits) in natural_matrix_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v >>= bits;
        println!("v := {v_old}; v >>= {bits}; v = {v}");
    }
}

fn benchmark_natural_matrix_shr_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalMatrix: Shr<T, Output = NaturalMatrix> + ShrAssign<T>,
    for<'a> &'a NaturalMatrix: Shr<T, Output = NaturalMatrix>,
{
    run_benchmark(
        &format!("NaturalMatrix >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_matrix_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("v"),
        &mut [
            ("NaturalMatrix >> T", &mut |(v, bits)| {
                no_out!(v >> bits);
            }),
            ("&NaturalMatrix >> T", &mut |(v, bits)| {
                no_out!(&v >> bits);
            }),
            ("NaturalMatrix >>= T", &mut |(mut v, bits)| {
                v >>= bits;
            }),
        ],
    );
}

fn benchmark_natural_matrix_shr_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalMatrix: Shr<T, Output = NaturalMatrix> + EntrywiseShrRound<T, Output = NaturalMatrix>,
{
    run_benchmark(
        &format!("NaturalMatrix >> {}", T::NAME),
        BenchmarkType::Algorithms,
        natural_matrix_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, bits)| {
                no_out!(v >> bits);
            }),
            ("using entrywise_shr_round with Floor", &mut |(v, bits)| {
                no_out!(v.entrywise_shr_round(bits, Floor));
            }),
        ],
    );
}
