// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::triple_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_signed_rounding_mode_triple_gen_var_2, integer_unsigned_rounding_mode_triple_gen_var_2,
};
use std::ops::Shl;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_integer_shr_round_assign_unsigned);
    register_signed_demos!(runner, demo_integer_shr_round_assign_signed);
    register_unsigned_demos!(runner, demo_integer_shr_round_unsigned);
    register_signed_demos!(runner, demo_integer_shr_round_signed);
    register_unsigned_demos!(runner, demo_integer_shr_round_ref_unsigned);
    register_signed_demos!(runner, demo_integer_shr_round_ref_signed);

    register_unsigned_benches!(runner, benchmark_integer_shr_round_assign_unsigned);
    register_signed_benches!(runner, benchmark_integer_shr_round_assign_signed);
    register_unsigned_benches!(
        runner,
        benchmark_integer_shr_round_evaluation_strategy_unsigned
    );
    register_signed_benches!(
        runner,
        benchmark_integer_shr_round_evaluation_strategy_signed
    );
}

fn demo_integer_shr_round_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: ShrRoundAssign<T> + Shl<T, Output = Integer>,
{
    for (mut n, u, rm) in integer_unsigned_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shr_round_assign(u, rm);
        println!("x := {n_old}; x.shr_round_assign({u}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_integer_shr_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: ShrRoundAssign<T> + Shl<T, Output = Integer>,
{
    for (mut n, i, rm) in integer_signed_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shr_round_assign(i, rm);
        println!("x := {n_old}; x.shr_round_assign({i}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_integer_shr_round_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: ShrRound<T, Output = Integer> + Shl<T, Output = Integer>,
{
    for (n, u, rm) in integer_unsigned_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shr_round({}, {}) = {:?}",
            n_old,
            u,
            rm,
            n.shr_round(u, rm)
        );
    }
}

fn demo_integer_shr_round_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: ShrRound<T, Output = Integer> + Shl<T, Output = Integer>,
{
    for (n, i, rm) in integer_signed_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shr_round({}, {}) = {:?}",
            n_old,
            i,
            rm,
            n.shr_round(i, rm)
        );
    }
}

fn demo_integer_shr_round_ref_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: Shl<T, Output = Integer>,
    for<'a> &'a Integer: ShrRound<T, Output = Integer>,
{
    for (n, u, rm) in integer_unsigned_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).shr_round({}, {}) = {:?}",
            n,
            u,
            rm,
            (&n).shr_round(u, rm)
        );
    }
}

fn demo_integer_shr_round_ref_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: Shl<T, Output = Integer>,
    for<'a> &'a Integer: ShrRound<T, Output = Integer>,
{
    for (n, i, rm) in integer_signed_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).shr_round({}, {}) = {:?}",
            n,
            i,
            rm,
            (&n).shr_round(i, rm)
        );
    }
}

fn benchmark_integer_shr_round_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: ShrRoundAssign<T> + Shl<T, Output = Integer>,
{
    run_benchmark(
        &format!("Integer.shr_round_assign({}, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_rounding_mode_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.shr_round_assign(y, rm))
        })],
    );
}

fn benchmark_integer_shr_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: ShrRoundAssign<T> + Shl<T, Output = Integer>,
{
    run_benchmark(
        &format!("Integer.shr_round_assign({}, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        integer_signed_rounding_mode_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.shr_round_assign(y, rm))
        })],
    );
}

fn benchmark_integer_shr_round_evaluation_strategy_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: ShrRound<T, Output = Integer> + Shl<T, Output = Integer>,
    for<'a> &'a Integer: ShrRound<T, Output = Integer>,
{
    run_benchmark(
        &format!("Integer.shr_round({}, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_rounding_mode_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("n"),
        &mut [
            (
                &format!("Integer.shr_round({}, RoundingMode)", T::NAME),
                &mut |(x, y, rm)| no_out!(x.shr_round(y, rm)),
            ),
            (
                &format!("(&Integer).shr_round({}, RoundingMode)", T::NAME),
                &mut |(x, y, rm)| no_out!((&x).shr_round(y, rm)),
            ),
        ],
    );
}

fn benchmark_integer_shr_round_evaluation_strategy_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: ShrRound<T, Output = Integer> + Shl<T, Output = Integer>,
    for<'a> &'a Integer: ShrRound<T, Output = Integer>,
{
    run_benchmark(
        &format!("Integer.shr_round({}, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        integer_signed_rounding_mode_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("n"),
        &mut [
            (
                &format!("Integer.shr_round({}, RoundingMode)", T::NAME),
                &mut |(x, y, rm)| no_out!(x.shr_round(y, rm)),
            ),
            (
                &format!("(&Integer).shr_round({}, RoundingMode)", T::NAME),
                &mut |(x, y, rm)| no_out!((&x).shr_round(y, rm)),
            ),
        ],
    );
}
