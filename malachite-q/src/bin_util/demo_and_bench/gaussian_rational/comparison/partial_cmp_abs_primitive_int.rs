// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::bench::bucketers::pair_1_gaussian_rational_bit_bucketer;
use malachite_q::test_util::generators::{
    gaussian_rational_signed_pair_gen, gaussian_rational_unsigned_pair_gen,
};
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_gaussian_rational_partial_cmp_abs_unsigned);
    register_signed_demos!(runner, demo_gaussian_rational_partial_cmp_abs_signed);
    register_unsigned_demos!(runner, demo_unsigned_partial_cmp_abs_gaussian_rational);
    register_signed_demos!(runner, demo_signed_partial_cmp_abs_gaussian_rational);

    register_unsigned_benches!(
        runner,
        benchmark_gaussian_rational_partial_cmp_abs_unsigned_algorithms
    );
    register_signed_benches!(
        runner,
        benchmark_gaussian_rational_partial_cmp_abs_signed_algorithms
    );
}

fn demo_gaussian_rational_partial_cmp_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: PartialOrdAbs<T>,
{
    for (x, u) in gaussian_rational_unsigned_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        match x.partial_cmp_abs(&u) {
            Some(Less) => println!("|{x}| < |{u}|"),
            Some(Equal) => println!("|{x}| = |{u}|"),
            Some(Greater) => println!("|{x}| > |{u}|"),
            None => println!("|{x}| and |{u}| are incomparable"),
        }
    }
}

fn demo_gaussian_rational_partial_cmp_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: PartialOrdAbs<T>,
{
    for (x, i) in gaussian_rational_signed_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        match x.partial_cmp_abs(&i) {
            Some(Less) => println!("|{x}| < |{i}|"),
            Some(Equal) => println!("|{x}| = |{i}|"),
            Some(Greater) => println!("|{x}| > |{i}|"),
            None => println!("|{x}| and |{i}| are incomparable"),
        }
    }
}

fn demo_unsigned_partial_cmp_abs_gaussian_rational<
    T: PartialOrdAbs<GaussianRational> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, u) in gaussian_rational_unsigned_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        match u.partial_cmp_abs(&x) {
            Some(Less) => println!("|{u}| < |{x}|"),
            Some(Equal) => println!("|{u}| = |{x}|"),
            Some(Greater) => println!("|{u}| > |{x}|"),
            None => println!("|{u}| and |{x}| are incomparable"),
        }
    }
}

fn demo_signed_partial_cmp_abs_gaussian_rational<
    T: PartialOrdAbs<GaussianRational> + PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, i) in gaussian_rational_signed_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        match i.partial_cmp_abs(&x) {
            Some(Less) => println!("|{i}| < |{x}|"),
            Some(Equal) => println!("|{i}| = |{x}|"),
            Some(Greater) => println!("|{i}| > |{x}|"),
            None => println!("|{i}| and |{x}| are incomparable"),
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_partial_cmp_abs_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianRational: PartialOrdAbs<T>,
    Rational: From<T>,
{
    run_benchmark(
        &format!("GaussianRational.partial_cmp_abs(&{})", T::NAME),
        BenchmarkType::Algorithms,
        gaussian_rational_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y))),
            ("using abs_squared", &mut |(x, y)| {
                no_out!(
                    (&x).abs_squared()
                        .partial_cmp(&Rational::from(y).abs_squared())
                );
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_partial_cmp_abs_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianRational: PartialOrdAbs<T>,
    Rational: From<T>,
{
    run_benchmark(
        &format!("GaussianRational.partial_cmp_abs(&{})", T::NAME),
        BenchmarkType::Algorithms,
        gaussian_rational_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y))),
            ("using abs_squared", &mut |(x, y)| {
                no_out!(
                    (&x).abs_squared()
                        .partial_cmp(&Rational::from(y).abs_squared())
                );
            }),
        ],
    );
}
