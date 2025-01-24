// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{
    triple_2_bucketer, triple_2_unsigned_abs_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_signed_rounding_mode_triple_gen_var_3, signed_unsigned_rounding_mode_triple_gen_var_2,
    unsigned_signed_rounding_mode_triple_gen_var_1,
    unsigned_unsigned_rounding_mode_triple_gen_var_4,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_shr_round_unsigned_unsigned);
    register_unsigned_signed_demos!(runner, demo_shr_round_unsigned_signed);
    register_signed_unsigned_demos!(runner, demo_shr_round_signed_unsigned);
    register_signed_signed_demos!(runner, demo_shr_round_signed_signed);
    register_unsigned_unsigned_demos!(runner, demo_shr_round_assign_unsigned_unsigned);
    register_unsigned_signed_demos!(runner, demo_shr_round_assign_unsigned_signed);
    register_signed_unsigned_demos!(runner, demo_shr_round_assign_signed_unsigned);
    register_signed_signed_demos!(runner, demo_shr_round_assign_signed_signed);

    register_unsigned_unsigned_benches!(runner, benchmark_shr_round_unsigned_unsigned);
    register_unsigned_signed_benches!(runner, benchmark_shr_round_unsigned_signed);
    register_signed_unsigned_benches!(runner, benchmark_shr_round_signed_unsigned);
    register_signed_signed_benches!(runner, benchmark_shr_round_signed_signed);
    register_unsigned_unsigned_benches!(runner, benchmark_shr_round_assign_unsigned_unsigned);
    register_unsigned_signed_benches!(runner, benchmark_shr_round_assign_unsigned_signed);
    register_signed_unsigned_benches!(runner, benchmark_shr_round_assign_signed_unsigned);
    register_signed_signed_benches!(runner, benchmark_shr_round_assign_signed_signed);
}

fn demo_shr_round_unsigned_unsigned<
    T: PrimitiveUnsigned + ShrRound<U, Output = T>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.shr_round({}, {}) = {:?}", n, u, rm, n.shr_round(u, rm));
    }
}

fn demo_shr_round_unsigned_signed<
    T: PrimitiveUnsigned + ShrRound<U, Output = T>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i, rm) in unsigned_signed_rounding_mode_triple_gen_var_1::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.shr_round({}, {}) = {:?}", n, i, rm, n.shr_round(i, rm));
    }
}

fn demo_shr_round_signed_unsigned<
    T: PrimitiveSigned + ShrRound<U, Output = T>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, rm) in signed_unsigned_rounding_mode_triple_gen_var_2::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).shr_round({}, {}) = {:?}",
            n,
            u,
            rm,
            n.shr_round(u, rm)
        );
    }
}

fn demo_shr_round_signed_signed<
    T: PrimitiveSigned + ShrRound<U, Output = T>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i, rm) in signed_signed_rounding_mode_triple_gen_var_3::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).shr_round({}, {}) = {:?}",
            n,
            i,
            rm,
            n.shr_round(i, rm)
        );
    }
}

fn demo_shr_round_assign_unsigned_unsigned<
    T: PrimitiveUnsigned + ShrRoundAssign<U>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        let o = n.shr_round_assign(u, rm);
        println!("x := {old_n}; x.shr_round_assign({u}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_shr_round_assign_unsigned_signed<
    T: PrimitiveUnsigned + ShrRoundAssign<U>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, i, rm) in unsigned_signed_rounding_mode_triple_gen_var_1::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        let o = n.shr_round_assign(i, rm);
        println!("x := {old_n}; x.shr_round_assign({i}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_shr_round_assign_signed_unsigned<
    T: PrimitiveSigned + ShrRoundAssign<U>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, rm) in signed_unsigned_rounding_mode_triple_gen_var_2::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        let o = n.shr_round_assign(u, rm);
        println!("x := {old_n}; x.shr_round_assign({u}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_shr_round_assign_signed_signed<
    T: PrimitiveSigned + ShrRoundAssign<U>,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, i, rm) in signed_signed_rounding_mode_triple_gen_var_3::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        let o = n.shr_round_assign(i, rm);
        println!("x := {old_n}; x.shr_round_assign({i}, {rm}) = {o:?}; x = {n}");
    }
}

fn benchmark_shr_round_unsigned_unsigned<
    T: PrimitiveUnsigned + ShrRound<U, Output = T>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<U>,
{
    run_benchmark(
        &format!("{}.shr_round({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_unsigned_rounding_mode_triple_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("u"),
        &mut [("Malachite", &mut |(n, u, rm)| no_out!(n.shr_round(u, rm)))],
    );
}

fn benchmark_shr_round_unsigned_signed<
    T: PrimitiveUnsigned + ShrRound<U, Output = T>,
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
        &format!("{}.shr_round({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_signed_rounding_mode_triple_gen_var_1::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_unsigned_abs_bucketer("i"),
        &mut [("Malachite", &mut |(n, i, rm)| no_out!(n.shr_round(i, rm)))],
    );
}

fn benchmark_shr_round_signed_unsigned<
    T: PrimitiveSigned + ShrRound<U, Output = T>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<U>,
{
    run_benchmark(
        &format!("{}.shr_round({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_unsigned_rounding_mode_triple_gen_var_2::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("u"),
        &mut [("Malachite", &mut |(n, u, rm)| no_out!(n.shr_round(u, rm)))],
    );
}

fn benchmark_shr_round_signed_signed<
    T: PrimitiveSigned + ShrRound<U, Output = T>,
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
        &format!("{}.shr_round({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_signed_rounding_mode_triple_gen_var_3::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_unsigned_abs_bucketer("i"),
        &mut [("Malachite", &mut |(n, i, rm)| no_out!(n.shr_round(i, rm)))],
    );
}

fn benchmark_shr_round_assign_unsigned_unsigned<
    T: PrimitiveUnsigned + ShrRoundAssign<U>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<U>,
{
    run_benchmark(
        &format!("{}.shr_round_assign({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_unsigned_rounding_mode_triple_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("u"),
        &mut [("Malachite", &mut |(mut n, u, rm)| {
            no_out!(n.shr_round_assign(u, rm))
        })],
    );
}

fn benchmark_shr_round_assign_unsigned_signed<
    T: PrimitiveUnsigned + ShrRoundAssign<U>,
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
        &format!("{}.shr_round_assign({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_signed_rounding_mode_triple_gen_var_1::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_unsigned_abs_bucketer("i"),
        &mut [("Malachite", &mut |(mut n, i, rm)| {
            no_out!(n.shr_round_assign(i, rm))
        })],
    );
}

fn benchmark_shr_round_assign_signed_unsigned<
    T: PrimitiveSigned + ShrRoundAssign<U>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<U>,
{
    run_benchmark(
        &format!("{}.shr_round_assign({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_unsigned_rounding_mode_triple_gen_var_2::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("u"),
        &mut [("Malachite", &mut |(mut n, u, rm)| {
            no_out!(n.shr_round_assign(u, rm))
        })],
    );
}

fn benchmark_shr_round_assign_signed_signed<
    T: PrimitiveSigned + ShrRoundAssign<U>,
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
        &format!("{}.shr_round_assign({}, RoundingMode)", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_signed_rounding_mode_triple_gen_var_3::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_unsigned_abs_bucketer("i"),
        &mut [("Malachite", &mut |(mut n, i, rm)| {
            no_out!(n.shr_round_assign(i, rm))
        })],
    );
}
