// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_unsigned_polynomial_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_rational_polynomial_partial_eq_unsigned_polynomial
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_partial_eq_rational_polynomial
    );

    register_unsigned_benches!(
        runner,
        benchmark_rational_polynomial_partial_eq_unsigned_polynomial_algorithms
    );
}

fn demo_rational_polynomial_partial_eq_unsigned_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialEq<T>,
{
    for (p, q) in rational_polynomial_unsigned_polynomial_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if p == q {
            println!("{p} = {q}");
        } else {
            println!("{p} ≠ {q}");
        }
    }
}

fn demo_unsigned_polynomial_partial_eq_rational_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialEq<T>,
{
    for (p, q) in rational_polynomial_unsigned_polynomial_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if q == p {
            println!("{q} = {p}");
        } else {
            println!("{q} ≠ {p}");
        }
    }
}

// The comparison's result is what is being timed, and the converting arm compares with an owned
// polynomial on purpose.
#[allow(clippy::cmp_owned, clippy::no_effect, clippy::op_ref, unused_must_use)]
fn benchmark_rational_polynomial_partial_eq_unsigned_polynomial_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T> + PartialEq<T>,
{
    run_benchmark(
        &format!("RationalPolynomial == UnsignedPolynomial<{}>", T::NAME),
        BenchmarkType::Algorithms,
        rational_polynomial_unsigned_polynomial_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, q)| no_out!(p == q)),
            ("converting the UnsignedPolynomial", &mut |(p, q)| {
                no_out!(&p == &RationalPolynomial::from(q));
            }),
        ],
    );
}
