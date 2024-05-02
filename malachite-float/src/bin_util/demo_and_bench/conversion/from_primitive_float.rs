// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    pair_primitive_float_bit_u64_max_bucketer, primitive_float_bucketer,
    triple_1_2_primitive_float_bit_u64_max_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_unsigned_pair_gen_var_4,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::conversion::from_primitive_float::alt_precision;
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::{
    primitive_float_unsigned_rounding_mode_triple_gen_var_3,
    primitive_float_unsigned_rounding_mode_triple_gen_var_4,
};
use malachite_float::{ComparableFloat, Float};
use rug::float::Round;
use rug::ops::AssignRound;
use rug::Assign;
use std::cmp::{max, Ordering};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_float_from_primitive_float);
    register_primitive_float_demos!(runner, demo_float_from_primitive_float_debug);
    register_primitive_float_demos!(runner, demo_float_from_primitive_float_prec);
    register_primitive_float_demos!(runner, demo_float_from_primitive_float_prec_debug);
    register_primitive_float_demos!(runner, demo_float_from_primitive_float_prec_round);
    register_primitive_float_demos!(runner, demo_float_from_primitive_float_prec_round_debug);

    register_primitive_float_benches!(
        runner,
        benchmark_float_from_primitive_float_library_comparison
    );
    register_primitive_float_benches!(
        runner,
        benchmark_float_from_primitive_float_prec_library_comparison
    );
    register_primitive_float_benches!(
        runner,
        benchmark_float_from_primitive_float_prec_round_library_comparison
    );
}

fn demo_float_from_primitive_float<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!("Float::from({}) = {}", NiceFloat(x), Float::from(x));
    }
}

fn demo_float_from_primitive_float_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "Float::from({}) = {:#x}",
            NiceFloat(x),
            ComparableFloat(Float::from(x))
        );
    }
}

fn demo_float_from_primitive_float_prec<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec) in primitive_float_unsigned_pair_gen_var_4::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_primitive_float_prec({}, {}) = {:?}",
            NiceFloat(x),
            prec,
            Float::from_primitive_float_prec(x, prec)
        );
    }
}

fn demo_float_from_primitive_float_prec_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec) in primitive_float_unsigned_pair_gen_var_4::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let (x_out, o) = Float::from_primitive_float_prec(x, prec);
        println!(
            "Float::from_primitive_float_prec({}, {}) = ({:#x}, {:?})",
            NiceFloat(x),
            prec,
            ComparableFloat(x_out),
            o
        );
    }
}

fn demo_float_from_primitive_float_prec_round<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T>,
{
    for (x, prec, rm) in primitive_float_unsigned_rounding_mode_triple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_primitive_float_prec_round({}, {}, {}) = {:?}",
            NiceFloat(x),
            prec,
            rm,
            Float::from_primitive_float_prec_round(x, prec, rm)
        );
    }
}

fn demo_float_from_primitive_float_prec_round_debug<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T>,
{
    for (x, prec, rm) in primitive_float_unsigned_rounding_mode_triple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (x_out, o) = Float::from_primitive_float_prec_round(x, prec, rm);
        println!(
            "Float::from_primitive_float_prec_round({}, {}, {}) = ({:#x}, {:?})",
            NiceFloat(x),
            prec,
            rm,
            ComparableFloat(x_out),
            o
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_float_from_primitive_float_library_comparison<T: PrimitiveFloat>(
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
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [
            ("Malachite", &mut |x| no_out!(Float::from(x))),
            ("rug", &mut |x| {
                no_out!(rug::Float::with_val(
                    max(1, u32::exact_from(alt_precision(x))),
                    x
                ))
            }),
        ],
    );
}

fn benchmark_float_from_primitive_float_prec_library_comparison<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T>,
    rug::Float: Assign<T>,
{
    run_benchmark(
        &format!("Float::from_primitive_float_prec({}, u64)", T::NAME),
        BenchmarkType::LibraryComparison,
        primitive_float_unsigned_pair_gen_var_4::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_primitive_float_bit_u64_max_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(x, prec)| {
                no_out!(Float::from_primitive_float_prec(x, prec))
            }),
            ("rug", &mut |(x, prec)| {
                no_out!(rug::Float::with_val(max(1, u32::exact_from(prec)), x))
            }),
        ],
    );
}

fn benchmark_float_from_primitive_float_prec_round_library_comparison<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T>,
    rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
{
    run_benchmark(
        &format!(
            "Float::from_primitive_float_prec_round({}, u64, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::LibraryComparison,
        primitive_float_unsigned_rounding_mode_triple_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_primitive_float_bit_u64_max_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(x, prec, rm)| {
                no_out!(Float::from_primitive_float_prec_round(x, prec, rm))
            }),
            ("rug", &mut |(x, prec, rm)| {
                no_out!(rug::Float::with_val_round(
                    max(1, u32::exact_from(prec)),
                    x,
                    rug_round_try_from_rounding_mode(rm).unwrap()
                ))
            }),
        ],
    );
}
