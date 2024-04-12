// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, triple_2_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_unsigned_pair_gen_var_16, unsigned_vec_unsigned_pair_gen_var_33,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_23,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::shr::{
    limbs_shr, limbs_shr_to_out, limbs_slice_shr_in_place, limbs_vec_shr_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_natural_bit_bucketer, pair_2_pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_signed_pair_gen_var_2, natural_signed_pair_gen_var_2_rm,
    natural_unsigned_pair_gen_var_4, natural_unsigned_pair_gen_var_4_rm,
};
use std::ops::{Shr, ShrAssign};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_shr);
    register_demo!(runner, demo_limbs_shr_to_out);
    register_demo!(runner, demo_limbs_slice_shr_in_place);
    register_demo!(runner, demo_limbs_vec_shr_in_place);
    register_unsigned_demos!(runner, demo_natural_shr_assign_unsigned);
    register_unsigned_demos!(runner, demo_natural_shr_unsigned);
    register_unsigned_demos!(runner, demo_natural_shr_unsigned_ref);
    register_signed_demos!(runner, demo_natural_shr_assign_signed);
    register_signed_demos!(runner, demo_natural_shr_signed);
    register_signed_demos!(runner, demo_natural_shr_signed_ref);

    register_bench!(runner, benchmark_limbs_shr);
    register_bench!(runner, benchmark_limbs_shr_to_out);
    register_bench!(runner, benchmark_limbs_slice_shr_in_place);
    register_bench!(runner, benchmark_limbs_vec_shr_in_place);
    register_bench!(runner, benchmark_natural_shr_assign_u32_library_comparison);
    register_bench!(runner, benchmark_natural_shr_u32_library_comparison);
    register_bench!(runner, benchmark_natural_shr_assign_i32_library_comparison);
    register_bench!(runner, benchmark_natural_shr_i32_library_comparison);
    register_unsigned_benches!(runner, benchmark_natural_shr_assign_unsigned);
    register_unsigned_benches!(runner, benchmark_natural_shr_unsigned_evaluation_strategy);
    register_signed_benches!(runner, benchmark_natural_shr_assign_signed);
    register_signed_benches!(runner, benchmark_natural_shr_signed_evaluation_strategy);
}

fn demo_limbs_shr(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, bits) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!("limbs_shr({:?}, {}) = {:?}", xs, bits, limbs_shr(&xs, bits));
    }
}

fn demo_limbs_shr_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (out, xs, bits) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_23::<Limb, Limb>()
        .get(gm, config)
        .take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let carry = limbs_shr_to_out(&mut out, &xs, bits);
        println!(
            "out := {out_old:?}; \
            limbs_shr_to_out(&mut out, {xs:?}, {bits}) = {carry}; out = {out:?}",
        );
    }
}

fn demo_limbs_slice_shr_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, bits) in unsigned_vec_unsigned_pair_gen_var_33::<Limb, Limb>()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry = limbs_slice_shr_in_place(&mut xs, bits);
        println!(
            "xs := {xs_old:?}; limbs_slice_shr_in_place(&mut xs, {bits}) = {carry}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_shr_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, bits) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_vec_shr_in_place(&mut xs, bits);
        println!("xs := {xs_old:?}; limbs_vec_shr_in_place(&mut xs, {bits}); xs = {xs:?}");
    }
}

fn demo_natural_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ShrAssign<T>,
{
    for (mut n, u) in natural_unsigned_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n >>= u;
        println!("x := {n_old}; x >>= {u}; x = {n}");
    }
}

fn demo_natural_shr_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: Shr<T, Output = Natural>,
{
    for (n, u) in natural_unsigned_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

fn demo_natural_shr_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: Shr<T, Output = Natural>,
{
    for (n, u) in natural_unsigned_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

fn demo_natural_shr_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: ShrAssign<T>,
{
    for (mut n, u) in natural_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n >>= u;
        println!("x := {n_old}; x >>= {u}; x = {n}");
    }
}

fn demo_natural_shr_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: Shr<T, Output = Natural>,
{
    for (n, u) in natural_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

fn demo_natural_shr_signed_ref<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a> &'a Natural: Shr<T, Output = Natural>,
{
    for (n, u) in natural_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

fn benchmark_limbs_shr(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_shr(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, bits)| no_out!(limbs_shr(&xs, bits)))],
    );
}

fn benchmark_limbs_shr_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_shr_to_out(&mut [Limb], &[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_23::<Limb, Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, bits)| {
            no_out!(limbs_shr_to_out(&mut out, &xs, bits))
        })],
    );
}

fn benchmark_limbs_slice_shr_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_shr_in_place(&mut [Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_33::<Limb, Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, bits)| {
            no_out!(limbs_slice_shr_in_place(&mut xs, bits))
        })],
    );
}

fn benchmark_limbs_vec_shr_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_shr_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16::<Limb, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, bits)| {
            limbs_vec_shr_in_place(&mut xs, bits)
        })],
    );
}

fn benchmark_natural_shr_assign_u32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural >>= u32",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_4_rm::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (mut x, y))| x >>= y),
            ("rug", &mut |((mut x, y), _)| x >>= y),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_shr_u32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural >> u32",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_4_rm::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x >> y)),
            ("rug", &mut |((x, y), _)| no_out!(x >> y)),
        ],
    );
}

fn benchmark_natural_shr_assign_i32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural >>= u32",
        BenchmarkType::LibraryComparison,
        natural_signed_pair_gen_var_2_rm::<i32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (mut x, y))| x >>= y),
            ("rug", &mut |((mut x, y), _)| x >>= y),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_shr_i32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural >> i32",
        BenchmarkType::LibraryComparison,
        natural_signed_pair_gen_var_2_rm::<i32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x >> y)),
            ("rug", &mut |((x, y), _)| no_out!(x >> y)),
        ],
    );
}

fn benchmark_natural_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ShrAssign<T>,
{
    run_benchmark(
        &format!("Natural >>= {}", T::NAME),
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut x, y)| no_out!(x >>= y))],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_shr_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: Shr<T, Output = Natural>,
    for<'a> &'a Natural: Shr<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            (&format!("Natural >> {}", T::NAME), &mut |(x, y)| {
                no_out!(x >> y)
            }),
            (&format!("&Natural >> {}", T::NAME), &mut |(x, y)| {
                no_out!(&x >> y)
            }),
        ],
    );
}

fn benchmark_natural_shr_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ShrAssign<T>,
{
    run_benchmark(
        &format!("Natural >>= {}", T::NAME),
        BenchmarkType::Single,
        natural_signed_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut x, y)| no_out!(x >>= y))],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_shr_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: Shr<T, Output = Natural>,
    for<'a> &'a Natural: Shr<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_signed_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            (&format!("Natural >> {}", T::NAME), &mut |(x, y)| {
                no_out!(x >> y)
            }),
            (&format!("&Natural >> {}", T::NAME), &mut |(x, y)| {
                no_out!(&x >> y)
            }),
        ],
    );
}
