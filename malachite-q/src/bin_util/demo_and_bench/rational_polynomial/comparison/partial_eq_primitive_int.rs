// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_polynomial_signed_pair_gen, rational_polynomial_unsigned_pair_gen,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_rational_polynomial_partial_eq_unsigned);
    register_signed_demos!(runner, demo_rational_polynomial_partial_eq_signed);
    register_unsigned_demos!(runner, demo_unsigned_partial_eq_rational_polynomial);
    register_signed_demos!(runner, demo_signed_partial_eq_rational_polynomial);

    register_unsigned_benches!(
        runner,
        benchmark_rational_polynomial_partial_eq_unsigned_algorithms
    );
    register_signed_benches!(
        runner,
        benchmark_rational_polynomial_partial_eq_signed_algorithms
    );
}

fn demo_rational_polynomial_partial_eq_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalPolynomial: PartialEq<T>,
{
    for (p, c) in rational_polynomial_unsigned_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if p == c {
            println!("{p} = {c}");
        } else {
            println!("{p} ≠ {c}");
        }
    }
}

fn demo_rational_polynomial_partial_eq_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalPolynomial: PartialEq<T>,
{
    for (p, c) in rational_polynomial_signed_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if p == c {
            println!("{p} = {c}");
        } else {
            println!("{p} ≠ {c}");
        }
    }
}

fn demo_unsigned_partial_eq_rational_polynomial<
    T: PartialEq<RationalPolynomial> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c) in rational_polynomial_unsigned_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if c == p {
            println!("{c} = {p}");
        } else {
            println!("{c} ≠ {p}");
        }
    }
}

fn demo_signed_partial_eq_rational_polynomial<
    T: PartialEq<RationalPolynomial> + PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c) in rational_polynomial_signed_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if c == p {
            println!("{c} = {p}");
        } else {
            println!("{c} ≠ {p}");
        }
    }
}

// The comparison's result is what is being timed, and the converting arm compares with an owned
// polynomial on purpose.
#[allow(clippy::cmp_owned, clippy::no_effect, clippy::op_ref, unused_must_use)]
fn benchmark_rational_polynomial_partial_eq_primitive_int_algorithms<T: PrimitiveInt>(
    pairs: Box<dyn Iterator<Item = (RationalPolynomial, T)>>,
    gm: GenMode,
    limit: usize,
    file_name: &str,
) where
    RationalPolynomial: PartialEq<T>,
    Rational: From<T>,
{
    run_benchmark(
        &format!("RationalPolynomial == {}", T::NAME),
        BenchmarkType::Algorithms,
        pairs,
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, c)| no_out!(p == c)),
            ("converting the value to a polynomial", &mut |(p, c)| {
                no_out!(&p == &RationalPolynomial::from(Rational::from(c)));
            }),
        ],
    );
}

fn benchmark_rational_polynomial_partial_eq_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalPolynomial: PartialEq<T>,
    Rational: From<T>,
{
    benchmark_rational_polynomial_partial_eq_primitive_int_algorithms(
        rational_polynomial_unsigned_pair_gen::<T>().get(gm, config),
        gm,
        limit,
        file_name,
    );
}

fn benchmark_rational_polynomial_partial_eq_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalPolynomial: PartialEq<T>,
    Rational: From<T>,
{
    benchmark_rational_polynomial_partial_eq_primitive_int_algorithms(
        rational_polynomial_signed_pair_gen::<T>().get(gm, config),
        gm,
        limit,
        file_name,
    );
}
