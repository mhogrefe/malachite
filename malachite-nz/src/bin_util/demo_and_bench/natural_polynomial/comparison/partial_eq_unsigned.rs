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
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_polynomial_partial_eq_unsigned);
    register_unsigned_demos!(runner, demo_unsigned_partial_eq_natural_polynomial);

    register_unsigned_benches!(
        runner,
        benchmark_natural_polynomial_partial_eq_unsigned_algorithms
    );
}

fn demo_natural_polynomial_partial_eq_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalPolynomial: PartialEq<T>,
{
    for (p, c) in natural_polynomial_unsigned_pair_gen::<T>()
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

fn demo_unsigned_partial_eq_natural_polynomial<
    T: PartialEq<NaturalPolynomial> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c) in natural_polynomial_unsigned_pair_gen::<T>()
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
fn benchmark_natural_polynomial_partial_eq_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalPolynomial: PartialEq<T>,
    Natural: From<T>,
{
    run_benchmark(
        &format!("NaturalPolynomial == {}", T::NAME),
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, c)| no_out!(p == c)),
            ("converting the value to a polynomial", &mut |(p, c)| {
                no_out!(&p == &NaturalPolynomial::from(Natural::from(c)));
            }),
        ],
    );
}
