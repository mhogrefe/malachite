// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ShlRound, ShlRoundAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_signed_rounding_mode_triple_gen_var_1;
use std::ops::Shr;

pub(crate) fn register(runner: &mut Runner) {
    register_signed_demos!(runner, demo_natural_shl_round_assign);
    register_signed_demos!(runner, demo_natural_shl_round);
    register_signed_demos!(runner, demo_natural_shl_round_ref);

    register_signed_benches!(runner, benchmark_natural_shl_round_assign);
    register_signed_benches!(runner, benchmark_natural_shl_round_evaluation_strategy);
}

fn demo_natural_shl_round_assign<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: ShlRoundAssign<T> + Shr<T, Output = Natural>,
{
    for (mut n, i, rm) in natural_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_round_assign(i, rm);
        println!("x := {n_old}; x.shl_round_assign({i}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_natural_shl_round<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: ShlRound<T, Output = Natural> + Shr<T, Output = Natural>,
{
    for (n, i, rm) in natural_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_round({}, {}) = {:?}",
            n_old,
            i,
            rm,
            n.shl_round(i, rm)
        );
    }
}

fn demo_natural_shl_round_ref<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: Shr<T, Output = Natural>,
    for<'a> &'a Natural: ShlRound<T, Output = Natural>,
{
    for (n, i, rm) in natural_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).shl_round({}, {}) = {:?}",
            n,
            i,
            rm,
            (&n).shl_round(i, rm)
        );
    }
}

fn benchmark_natural_shl_round_assign<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ShlRoundAssign<T> + Shr<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.shl_round_assign({}, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        natural_signed_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.shl_round_assign(y, rm))
        })],
    );
}

fn benchmark_natural_shl_round_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ShlRound<T, Output = Natural> + Shr<T, Output = Natural>,
    for<'a> &'a Natural: ShlRound<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.shl_round({}, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_signed_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [
            (
                &format!("Natural.shl_round({}, RoundingMode)", T::NAME),
                &mut |(x, y, rm)| no_out!(x.shl_round(y, rm)),
            ),
            (
                &format!("(&Natural).shl_round({}, RoundingMode)", T::NAME),
                &mut |(x, y, rm)| no_out!((&x).shl_round(y, rm)),
            ),
        ],
    );
}
