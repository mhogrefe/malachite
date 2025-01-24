// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::arithmetic::shr::shr_naive;
use malachite_float::test_util::bench::bucketers::{
    pair_1_float_complexity_bucketer, pair_2_pair_1_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_signed_pair_gen_var_2, float_signed_pair_gen_var_2_rm, float_signed_pair_gen_var_3,
    float_unsigned_pair_gen_var_2, float_unsigned_pair_gen_var_2_rm, float_unsigned_pair_gen_var_3,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::ops::{Shr, ShrAssign};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_float_shr_assign_unsigned);
    register_unsigned_demos!(runner, demo_float_shr_assign_unsigned_debug);
    register_signed_demos!(runner, demo_float_shr_assign_signed);
    register_signed_demos!(runner, demo_float_shr_assign_signed_debug);
    register_unsigned_demos!(runner, demo_float_shr_unsigned);
    register_unsigned_demos!(runner, demo_float_shr_unsigned_debug);
    register_unsigned_demos!(runner, demo_float_shr_unsigned_extreme);
    register_unsigned_demos!(runner, demo_float_shr_unsigned_extreme_debug);
    register_signed_demos!(runner, demo_float_shr_signed);
    register_signed_demos!(runner, demo_float_shr_signed_debug);
    register_signed_demos!(runner, demo_float_shr_signed_extreme);
    register_signed_demos!(runner, demo_float_shr_signed_extreme_debug);
    register_unsigned_demos!(runner, demo_float_shr_unsigned_ref);
    register_unsigned_demos!(runner, demo_float_shr_unsigned_ref_debug);
    register_signed_demos!(runner, demo_float_shr_signed_ref);
    register_signed_demos!(runner, demo_float_shr_signed_ref_debug);

    register_unsigned_benches!(runner, benchmark_float_shr_assign_unsigned);
    register_signed_benches!(runner, benchmark_float_shr_assign_signed);
    register_unsigned_benches!(runner, benchmark_float_shr_unsigned_evaluation_strategy);
    register_signed_benches!(runner, benchmark_float_shr_signed_evaluation_strategy);
    register_unsigned_benches!(runner, benchmark_float_shr_unsigned_algorithms);
    register_signed_benches!(runner, benchmark_float_shr_signed_algorithms);

    register_bench!(runner, benchmark_float_shr_assign_u32_library_comparison);
    register_bench!(runner, benchmark_float_shr_u32_library_comparison);
    register_bench!(runner, benchmark_float_shr_assign_i32_library_comparison);
    register_bench!(runner, benchmark_float_shr_i32_library_comparison);
}

fn demo_float_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShrAssign<T>,
{
    for (mut n, u) in float_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n >>= u;
        println!("x := {n_old}; x >>= {u}; x = {n}");
    }
}

fn demo_float_shr_assign_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShrAssign<T>,
{
    for (mut n, u) in float_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n >>= u;
        println!(
            "x := {:#x}; x >>= {u}; x = {:#x}",
            ComparableFloat(n_old),
            ComparableFloat(n)
        );
    }
}

fn demo_float_shr_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: ShrAssign<T>,
{
    for (mut n, i) in float_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n >>= i;
        println!("x := {n_old}; x >>= {i}; x = {n}");
    }
}

fn demo_float_shr_assign_signed_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShrAssign<T>,
{
    for (mut n, i) in float_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n >>= i;
        println!(
            "x := {:#x}; x >>= {i}; x = {:#x}",
            ComparableFloat(n_old),
            ComparableFloat(n)
        );
    }
}

fn demo_float_shr_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: Shr<T, Output = Float>,
{
    for (n, u) in float_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

fn demo_float_shr_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: Shr<T, Output = Float>,
{
    for (n, u) in float_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{:#x} >> {} = {:#x}",
            ComparableFloat(n_old),
            u,
            ComparableFloat(n >> u)
        );
    }
}

fn demo_float_shr_unsigned_extreme<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: Shr<T, Output = Float>,
{
    for (n, u) in float_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

fn demo_float_shr_unsigned_extreme_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: Shr<T, Output = Float>,
{
    for (n, u) in float_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{:#x} >> {} = {:#x}",
            ComparableFloat(n_old),
            u,
            ComparableFloat(n >> u)
        );
    }
}

fn demo_float_shr_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: Shr<T, Output = Float>,
{
    for (n, i) in float_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, i, n >> i);
    }
}

fn demo_float_shr_signed_debug<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: Shr<T, Output = Float>,
{
    for (n, i) in float_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{:#x} >> {} = {:#x}",
            ComparableFloat(n_old),
            i,
            ComparableFloat(n >> i)
        );
    }
}

fn demo_float_shr_signed_extreme<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: Shr<T, Output = Float>,
{
    for (n, i) in float_signed_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, i, n >> i);
    }
}

fn demo_float_shr_signed_extreme_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: Shr<T, Output = Float>,
{
    for (n, i) in float_signed_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{:#x} >> {} = {:#x}",
            ComparableFloat(n_old),
            i,
            ComparableFloat(n >> i)
        );
    }
}

fn demo_float_shr_unsigned_ref<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a> &'a Float: Shr<T, Output = Float>,
{
    for (n, u) in float_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

fn demo_float_shr_unsigned_ref_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: Shr<T, Output = Float>,
{
    for (n, u) in float_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "&{:#x} >> {} = {:#x}",
            ComparableFloatRef(&n),
            u,
            ComparableFloatRef(&(&n >> u))
        );
    }
}

fn demo_float_shr_signed_ref<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a> &'a Float: Shr<T, Output = Float>,
{
    for (n, i) in float_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{} >> {} = {}", n, i, &n >> i);
    }
}

fn demo_float_shr_signed_ref_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: Shr<T, Output = Float>,
{
    for (n, i) in float_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "&{:#x} >> {} = {:#x}",
            ComparableFloatRef(&n),
            i,
            ComparableFloatRef(&(&n >> i))
        );
    }
}

fn benchmark_float_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: ShrAssign<T>,
{
    run_benchmark(
        &format!("Float >>= {}", T::NAME),
        BenchmarkType::Single,
        float_unsigned_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u)| n >>= u)],
    );
}

fn benchmark_float_shr_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: ShrAssign<T>,
{
    run_benchmark(
        &format!("Float >>= {}", T::NAME),
        BenchmarkType::Single,
        float_signed_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, i)| n >>= i)],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_shr_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: Shr<T, Output = Float>,
    for<'a> &'a Float: Shr<T, Output = Float>,
{
    run_benchmark(
        &format!("Float >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("n"),
        &mut [
            (&format!("Float >> {}", T::NAME), &mut |(x, y)| {
                no_out!(x >> y)
            }),
            (&format!("&Float >> {}", T::NAME), &mut |(x, y)| {
                no_out!(&x >> y)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_shr_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: Shr<T, Output = Float>,
    for<'a> &'a Float: Shr<T, Output = Float>,
{
    run_benchmark(
        &format!("Float >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_signed_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("n"),
        &mut [
            (&format!("Float >> {}", T::NAME), &mut |(x, y)| {
                no_out!(x >> y)
            }),
            (&format!("&Float >> {}", T::NAME), &mut |(x, y)| {
                no_out!(&x >> y)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_shr_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: Shr<T, Output = Float>,
    Rational: Shr<T, Output = Rational>,
    for<'a> &'a Float: Shr<T, Output = Float>,
{
    run_benchmark(
        &format!("Float >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("n"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x >> y)),
            ("naive", &mut |(x, y)| no_out!(shr_naive(x, y))),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_shr_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: Shr<T, Output = Float>,
    Rational: Shr<T, Output = Rational>,
    for<'a> &'a Float: Shr<T, Output = Float>,
{
    run_benchmark(
        &format!("Float >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_signed_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("n"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x >> y)),
            ("naive", &mut |(x, y)| no_out!(shr_naive(x, y))),
        ],
    );
}

fn benchmark_float_shr_assign_u32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float >>= u32",
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_var_2_rm::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (mut x, y))| x >>= y),
            ("rug", &mut |((mut x, y), _)| x >>= y),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_float_shr_u32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float >> u32",
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_var_2_rm::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x >> y)),
            ("rug", &mut |((x, y), _)| no_out!(x >> y)),
        ],
    );
}

fn benchmark_float_shr_assign_i32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float >>= i32",
        BenchmarkType::LibraryComparison,
        float_signed_pair_gen_var_2_rm::<i32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (mut x, y))| x >>= y),
            ("rug", &mut |((mut x, y), _)| x >>= y),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_float_shr_i32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float >> i32",
        BenchmarkType::LibraryComparison,
        float_signed_pair_gen_var_2_rm::<i32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x >> y)),
            ("rug", &mut |((x, y), _)| no_out!(x >> y)),
        ],
    );
}
