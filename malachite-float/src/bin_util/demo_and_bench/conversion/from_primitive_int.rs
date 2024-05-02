// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::{
    pair_primitive_int_bit_u64_max_bucketer, signed_bit_bucketer,
    triple_1_2_primitive_int_bit_u64_max_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_gen, signed_unsigned_pair_gen_var_20, unsigned_gen, unsigned_pair_gen_var_32,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::{
    signed_unsigned_rounding_mode_triple_gen_var_3, signed_unsigned_rounding_mode_triple_gen_var_4,
    unsigned_unsigned_rounding_mode_triple_gen_var_5,
    unsigned_unsigned_rounding_mode_triple_gen_var_6,
};
use malachite_float::{ComparableFloat, Float};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use rug::float::Round;
use rug::ops::AssignRound;
use rug::Assign;
use std::cmp::{max, Ordering};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_float_from_unsigned);
    register_unsigned_demos!(runner, demo_float_from_unsigned_debug);
    register_unsigned_demos!(runner, demo_float_from_unsigned_prec);
    register_unsigned_demos!(runner, demo_float_from_unsigned_prec_debug);
    register_unsigned_demos!(runner, demo_float_from_unsigned_prec_round);
    register_unsigned_demos!(runner, demo_float_from_unsigned_prec_round_debug);

    register_signed_demos!(runner, demo_float_from_signed);
    register_signed_demos!(runner, demo_float_from_signed_debug);
    register_signed_demos!(runner, demo_float_from_signed_prec);
    register_signed_demos!(runner, demo_float_from_signed_prec_debug);
    register_signed_demos!(runner, demo_float_from_signed_prec_round);
    register_signed_demos!(runner, demo_float_from_signed_prec_round_debug);

    register_unsigned_benches!(runner, benchmark_float_from_unsigned_library_comparison);
    register_unsigned_benches!(
        runner,
        benchmark_float_from_unsigned_prec_library_comparison
    );
    register_unsigned_benches!(
        runner,
        benchmark_float_from_unsigned_prec_round_library_comparison
    );

    register_signed_benches!(runner, benchmark_float_from_signed_library_comparison);
    register_signed_benches!(runner, benchmark_float_from_signed_prec_library_comparison);
    register_signed_benches!(
        runner,
        benchmark_float_from_signed_prec_round_library_comparison
    );
}

fn demo_float_from_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T>,
{
    for n in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("Float::from({}) = {}", n, Float::from(n));
    }
}

fn demo_float_from_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T>,
{
    for n in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!(
            "Float::from({:#x}) = {:#x}",
            n,
            ComparableFloat(Float::from(n))
        );
    }
}

fn demo_float_from_unsigned_prec<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for (n, p) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_unsigned_prec({}, {}) = {:?}",
            n,
            p,
            Float::from_unsigned_prec(n, p)
        );
    }
}

fn demo_float_from_unsigned_prec_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for (n, p) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_unsigned_prec(n, p);
        println!(
            "Float::from_unsigned_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_from_unsigned_prec_round<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for (n, p, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_unsigned_prec_round({}, {}, {:?}) = {:?}",
            n,
            p,
            rm,
            Float::from_unsigned_prec_round(n, p, rm)
        );
    }
}

fn demo_float_from_unsigned_prec_round_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for (n, p, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_unsigned_prec_round(n, p, rm);
        println!(
            "Float::from_unsigned_prec_round({}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_from_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T>,
{
    for n in signed_gen::<T>().get(gm, config).take(limit) {
        println!("Float::from({}) = {}", n, Float::from(n));
    }
}

fn demo_float_from_signed_debug<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T>,
{
    for n in signed_gen::<T>().get(gm, config).take(limit) {
        println!(
            "Float::from({:#x}) = {:#x}",
            n,
            ComparableFloat(Float::from(n))
        );
    }
}

fn demo_float_from_signed_prec<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: From<T>,
{
    for (n, p) in signed_unsigned_pair_gen_var_20::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_signed_prec({}, {}) = {:?}",
            n,
            p,
            Float::from_signed_prec(n, p)
        );
    }
}

fn demo_float_from_signed_prec_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: From<T>,
{
    for (n, p) in signed_unsigned_pair_gen_var_20::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_signed_prec(n, p);
        println!(
            "Float::from_signed_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_from_signed_prec_round<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: From<T>,
{
    for (n, p, rm) in signed_unsigned_rounding_mode_triple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_signed_prec_round({}, {}, {:?}) = {:?}",
            n,
            p,
            rm,
            Float::from_signed_prec_round(n, p, rm)
        );
    }
}

fn demo_float_from_signed_prec_round_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: From<T>,
{
    for (n, p, rm) in signed_unsigned_rounding_mode_triple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_signed_prec_round(n, p, rm);
        println!(
            "Float::from_signed_prec_round({}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_float_from_unsigned_library_comparison<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T>,
    rug::Float: Assign<T>,
{
    run_benchmark(
        &format!("Float::from({})", T::NAME),
        BenchmarkType::LibraryComparison,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("Malachite", &mut |n| no_out!(Float::from(n))),
            ("rug", &mut |n| {
                no_out!(rug::Float::with_val(
                    max(1, u32::exact_from(n.significant_bits())),
                    n
                ))
            }),
        ],
    );
}

fn benchmark_float_from_unsigned_prec_library_comparison<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T>,
    Natural: From<T>,
    rug::Float: Assign<T>,
{
    run_benchmark(
        &format!("Float::from_unsigned_prec({}, u64)", T::NAME),
        BenchmarkType::LibraryComparison,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_primitive_int_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("Malachite", &mut |(n, prec)| {
                no_out!(Float::from_unsigned_prec(n, prec))
            }),
            ("rug", &mut |(n, prec)| {
                no_out!(rug::Float::with_val(u32::exact_from(prec), n))
            }),
        ],
    );
}

fn benchmark_float_from_unsigned_prec_round_library_comparison<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T>,
    Natural: From<T>,
    rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
{
    run_benchmark(
        &format!(
            "Float::from_unsigned_prec_round({}, u64, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::LibraryComparison,
        unsigned_unsigned_rounding_mode_triple_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_primitive_int_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("Malachite", &mut |(n, prec, rm)| {
                no_out!(Float::from_unsigned_prec_round(n, prec, rm))
            }),
            ("rug", &mut |(n, prec, rm)| {
                no_out!(rug::Float::with_val_round(
                    u32::exact_from(prec),
                    n,
                    rug_round_try_from_rounding_mode(rm).unwrap()
                ))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_from_signed_library_comparison<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T>,
    rug::Float: Assign<T>,
{
    run_benchmark(
        &format!("Float::from({})", T::NAME),
        BenchmarkType::LibraryComparison,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            ("Malachite", &mut |n| no_out!(Float::from(n))),
            ("rug", &mut |n| {
                no_out!(rug::Float::with_val(
                    max(1, u32::exact_from(n.significant_bits())),
                    n
                ))
            }),
        ],
    );
}

fn benchmark_float_from_signed_prec_library_comparison<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T>,
    Integer: From<T>,
    rug::Float: Assign<T>,
{
    run_benchmark(
        &format!("Float::from_signed_prec({}, u64)", T::NAME),
        BenchmarkType::LibraryComparison,
        signed_unsigned_pair_gen_var_20::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_primitive_int_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("Malachite", &mut |(n, prec)| {
                no_out!(Float::from_signed_prec(n, prec))
            }),
            ("rug", &mut |(n, prec)| {
                no_out!(rug::Float::with_val(u32::exact_from(prec), n))
            }),
        ],
    );
}

fn benchmark_float_from_signed_prec_round_library_comparison<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T>,
    Integer: From<T>,
    rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
{
    run_benchmark(
        &format!(
            "Float::from_signed_prec_round({}, u64, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::LibraryComparison,
        signed_unsigned_rounding_mode_triple_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_primitive_int_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("Malachite", &mut |(n, prec, rm)| {
                no_out!(Float::from_signed_prec_round(n, prec, rm))
            }),
            ("rug", &mut |(n, prec, rm)| {
                no_out!(rug::Float::with_val_round(
                    u32::exact_from(prec),
                    n,
                    rug_round_try_from_rounding_mode(rm).unwrap()
                ))
            }),
        ],
    );
}
