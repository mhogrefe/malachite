// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, triple_1_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_unsigned_pair_gen_var_16, unsigned_vec_unsigned_pair_gen_var_20,
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_2,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::shr_round::{
    limbs_shr_exact, limbs_shr_round, limbs_shr_round_nearest, limbs_shr_round_up,
    limbs_vec_shr_exact_in_place, limbs_vec_shr_round_in_place,
    limbs_vec_shr_round_nearest_in_place, limbs_vec_shr_round_up_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_signed_rounding_mode_triple_gen_var_2, natural_unsigned_rounding_mode_triple_gen_var_1,
};
use std::ops::Shl;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_shr_round_up);
    register_demo!(runner, demo_limbs_shr_round_nearest);
    register_demo!(runner, demo_limbs_shr_exact);
    register_demo!(runner, demo_limbs_shr_round);
    register_demo!(runner, demo_limbs_vec_shr_round_up_in_place);
    register_demo!(runner, demo_limbs_vec_shr_round_nearest_in_place);
    register_demo!(runner, demo_limbs_vec_shr_exact_in_place);
    register_demo!(runner, demo_limbs_vec_shr_round_in_place);
    register_unsigned_demos!(runner, demo_natural_shr_round_assign_unsigned);
    register_unsigned_demos!(runner, demo_natural_shr_round_unsigned);
    register_unsigned_demos!(runner, demo_natural_shr_round_unsigned_ref);
    register_signed_demos!(runner, demo_natural_shr_round_assign_signed);
    register_signed_demos!(runner, demo_natural_shr_round_signed);
    register_signed_demos!(runner, demo_natural_shr_round_signed_ref);

    register_bench!(runner, benchmark_limbs_shr_round_up);
    register_bench!(runner, benchmark_limbs_shr_round_nearest);
    register_bench!(runner, benchmark_limbs_shr_exact);
    register_bench!(runner, benchmark_limbs_shr_round);
    register_bench!(runner, benchmark_limbs_vec_shr_round_up_in_place);
    register_bench!(runner, benchmark_limbs_vec_shr_round_nearest_in_place);
    register_bench!(runner, benchmark_limbs_vec_shr_exact_in_place);
    register_bench!(runner, benchmark_limbs_vec_shr_round_in_place);
    register_unsigned_benches!(runner, benchmark_natural_shr_round_assign_unsigned);
    register_unsigned_benches!(
        runner,
        benchmark_natural_shr_round_unsigned_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_natural_shr_round_assign_signed);
    register_signed_benches!(
        runner,
        benchmark_natural_shr_round_signed_evaluation_strategy
    );
}

fn demo_limbs_shr_round_up(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, bits) in unsigned_vec_unsigned_pair_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_shr_round_up({:?}, {}) = {:?}",
            xs,
            bits,
            limbs_shr_round_up(&xs, bits)
        );
    }
}

fn demo_limbs_shr_round_nearest(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, bits) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_shr_round_nearest({:?}, {}) = {:?}",
            xs,
            bits,
            limbs_shr_round_nearest(&xs, bits)
        );
    }
}

fn demo_limbs_shr_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, bits) in unsigned_vec_unsigned_pair_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_shr_exact({:?}, {}) = {:?}",
            xs,
            bits,
            limbs_shr_exact(&xs, bits)
        );
    }
}

fn demo_limbs_shr_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, bits, rm) in unsigned_vec_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_shr_round({:?}, {}, {}) = {:?}",
            xs,
            bits,
            rm,
            limbs_shr_round(&xs, bits, rm)
        );
    }
}

fn demo_limbs_vec_shr_round_up_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, bits) in unsigned_vec_unsigned_pair_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let o = limbs_vec_shr_round_up_in_place(&mut xs, bits);
        println!(
            "xs := {xs_old:?}; \
            limbs_vec_shr_round_up_in_place(&mut xs, {bits}) = {o:?}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_shr_round_nearest_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, bits) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let o = limbs_vec_shr_round_nearest_in_place(&mut xs, bits);
        println!(
            "limbs := {xs_old:?}; \
            limbs_vec_shr_round_nearest_in_place(&mut limbs, {bits}) = {o:?}; limbs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_shr_exact_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, bits) in unsigned_vec_unsigned_pair_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let result = limbs_vec_shr_exact_in_place(&mut xs, bits);
        println!(
            "xs := {xs_old:?}; \
            limbs_vec_shr_exact_in_place(&mut xs, {bits}) = {result}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_shr_round_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, bits, rm) in unsigned_vec_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let result = limbs_vec_shr_round_in_place(&mut xs, bits, rm);
        println!(
            "xs := {xs_old:?}; \
            limbs_vec_shr_round_in_place(&mut xs, {bits}, {rm}) = {result:?}; xs = {xs:?}",
        );
    }
}

fn demo_natural_shr_round_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: Shl<T, Output = Natural> + ShrRoundAssign<T>,
{
    for (mut n, u, rm) in natural_unsigned_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shr_round_assign(u, rm);
        println!("x := {n_old}; x.shr_round_assign({u}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_natural_shr_round_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: Shl<T, Output = Natural> + ShrRound<T, Output = Natural>,
{
    for (n, u, rm) in natural_unsigned_rounding_mode_triple_gen_var_1::<T>()
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

fn demo_natural_shr_round_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: Shl<T, Output = Natural>,
    for<'a> &'a Natural: ShrRound<T, Output = Natural>,
{
    for (n, u, rm) in natural_unsigned_rounding_mode_triple_gen_var_1::<T>()
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

fn demo_natural_shr_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: Shl<T, Output = Natural> + ShrRoundAssign<T>,
{
    for (mut n, i, rm) in natural_signed_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shr_round_assign(i, rm);
        println!("x := {n_old}; x.shr_round_assign({i}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_natural_shr_round_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: Shl<T, Output = Natural> + ShrRound<T, Output = Natural>,
{
    for (n, i, rm) in natural_signed_rounding_mode_triple_gen_var_2::<T>()
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

fn demo_natural_shr_round_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: Shl<T, Output = Natural>,
    for<'a> &'a Natural: ShrRound<T, Output = Natural>,
{
    for (n, i, rm) in natural_signed_rounding_mode_triple_gen_var_2::<T>()
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

fn benchmark_limbs_shr_round_up(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_shr_round_up(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, bits)| {
            no_out!(limbs_shr_round_up(&xs, bits))
        })],
    );
}

fn benchmark_limbs_shr_round_nearest(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_shr_round_nearest(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, bits)| {
            no_out!(limbs_shr_round_nearest(&xs, bits))
        })],
    );
}

fn benchmark_limbs_shr_exact(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_shr_exact(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, bits)| {
            no_out!(limbs_shr_exact(&xs, bits))
        })],
    );
}

fn benchmark_limbs_shr_round(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_shr_round(&[Limb], u64, RoundingMode)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, bits, rm)| {
            no_out!(limbs_shr_round(&xs, bits, rm))
        })],
    );
}

fn benchmark_limbs_vec_shr_round_up_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_shr_round_up_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, bits)| {
            no_out!(limbs_vec_shr_round_up_in_place(&mut xs, bits))
        })],
    );
}

fn benchmark_limbs_vec_shr_round_nearest_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_shr_round_nearest_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, bits)| {
            no_out!(limbs_vec_shr_round_nearest_in_place(&mut xs, bits))
        })],
    );
}

fn benchmark_limbs_vec_shr_exact_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_shr_exact_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, bits)| {
            no_out!(limbs_vec_shr_exact_in_place(&mut xs, bits))
        })],
    );
}

fn benchmark_limbs_vec_shr_round_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_shr_round_in_place(&mut Vec<Limb>, u64, RoundingMode)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, bits, rm)| {
            no_out!(limbs_vec_shr_round_in_place(&mut xs, bits, rm))
        })],
    );
}

fn benchmark_natural_shr_round_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: Shl<T, Output = Natural> + ShrRoundAssign<T>,
{
    run_benchmark(
        &format!("Natural.shr_round_assign({}, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        natural_unsigned_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.shr_round_assign(y, rm))
        })],
    );
}

fn benchmark_natural_shr_round_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: Shl<T, Output = Natural> + ShrRound<T, Output = Natural>,
    for<'a> &'a Natural: ShrRound<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.shr_round({}, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [
            (
                &format!("Natural.shr_round({}, RoundingMode)", T::NAME),
                &mut |(x, y, rm)| no_out!(x.shr_round(y, rm)),
            ),
            (
                &format!("(&Natural).shr_round({}, RoundingMode)", T::NAME),
                &mut |(x, y, rm)| no_out!((&x).shr_round(y, rm)),
            ),
        ],
    );
}

fn benchmark_natural_shr_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: Shl<T, Output = Natural> + ShrRoundAssign<T>,
{
    run_benchmark(
        &format!("Natural.shr_round_assign({}, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        natural_signed_rounding_mode_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.shr_round_assign(y, rm))
        })],
    );
}

fn benchmark_natural_shr_round_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: Shl<T, Output = Natural> + ShrRound<T, Output = Natural>,
    for<'a> &'a Natural: ShrRound<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.shr_round({}, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_signed_rounding_mode_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [
            (
                &format!("Natural.shr_round({}, RoundingMode)", T::NAME),
                &mut |(x, y, rm)| no_out!(x.shr_round(y, rm)),
            ),
            (
                &format!("(&Natural).shr_round({}, RoundingMode)", T::NAME),
                &mut |(x, y, rm)| no_out!((&x).shr_round(y, rm)),
            ),
        ],
    );
}
