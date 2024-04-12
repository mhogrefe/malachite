// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ShlRound, ShlRoundAssign, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_2_unsigned_abs_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_signed_rounding_mode_triple_gen_var_4, unsigned_signed_rounding_mode_triple_gen_var_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_signed_demos!(runner, demo_shl_round_unsigned_signed);
    register_signed_signed_demos!(runner, demo_shl_round_signed_signed);
    register_unsigned_signed_demos!(runner, demo_shl_round_assign_unsigned_signed);
    register_signed_signed_demos!(runner, demo_shl_round_assign_signed_signed);

    register_unsigned_signed_benches!(runner, benchmark_shl_round_unsigned_signed);
    register_signed_signed_benches!(runner, benchmark_shl_round_signed_signed);
    register_unsigned_signed_benches!(runner, benchmark_shl_round_assign_unsigned_signed);
    register_signed_signed_benches!(runner, benchmark_shl_round_assign_signed_signed);
}

fn demo_shl_round_unsigned_signed<
    T: PrimitiveUnsigned + ShlRound<U, Output = T>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i, rm) in unsigned_signed_rounding_mode_triple_gen_var_2::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.shl_round({}, {}) = {:?}", n, i, rm, n.shl_round(i, rm));
    }
}

fn demo_shl_round_signed_signed<
    T: PrimitiveSigned + ShlRound<U, Output = T>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i, rm) in signed_signed_rounding_mode_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).shl_round({}, {}) = {:?}",
            n,
            i,
            rm,
            n.shl_round(i, rm)
        );
    }
}

fn demo_shl_round_assign_unsigned_signed<
    T: PrimitiveUnsigned + ShlRoundAssign<U>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, i, rm) in unsigned_signed_rounding_mode_triple_gen_var_2::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        let o = n.shl_round_assign(i, rm);
        println!("x := {old_n}; x.shl_round_assign({i}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_shl_round_assign_signed_signed<
    T: PrimitiveSigned + ShlRoundAssign<U>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, i, rm) in signed_signed_rounding_mode_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        let o = n.shl_round_assign(i, rm);
        println!("x := {old_n}; x.shl_round_assign({i}, {rm}) = {o:?}; x = {n}");
    }
}

fn benchmark_shl_round_unsigned_signed<
    T: PrimitiveUnsigned + ShlRound<U, Output = T>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<<U as UnsignedAbs>::Output>,
{
    run_benchmark(
        &format!("{}.shl_round({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_signed_rounding_mode_triple_gen_var_2::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_unsigned_abs_bucketer("i"),
        &mut [("Malachite", &mut |(n, i, rm)| no_out!(n.shl_round(i, rm)))],
    );
}

fn benchmark_shl_round_signed_signed<
    T: PrimitiveSigned + ShlRound<U, Output = T>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<<U as UnsignedAbs>::Output>,
{
    run_benchmark(
        &format!("{}.shl_round({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_signed_rounding_mode_triple_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_unsigned_abs_bucketer("i"),
        &mut [("Malachite", &mut |(n, i, rm)| no_out!(n.shl_round(i, rm)))],
    );
}

fn benchmark_shl_round_assign_unsigned_signed<
    T: PrimitiveUnsigned + ShlRoundAssign<U>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<<U as UnsignedAbs>::Output>,
{
    run_benchmark(
        &format!("{}.shl_round_assign({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_signed_rounding_mode_triple_gen_var_2::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_unsigned_abs_bucketer("i"),
        &mut [("Malachite", &mut |(mut n, i, rm)| {
            no_out!(n.shl_round_assign(i, rm))
        })],
    );
}

fn benchmark_shl_round_assign_signed_signed<
    T: PrimitiveSigned + ShlRoundAssign<U>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<<U as UnsignedAbs>::Output>,
{
    run_benchmark(
        &format!("{}.shl_round_assign({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_signed_rounding_mode_triple_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_unsigned_abs_bucketer("i"),
        &mut [("Malachite", &mut |(mut n, i, rm)| {
            no_out!(n.shl_round_assign(i, rm))
        })],
    );
}
