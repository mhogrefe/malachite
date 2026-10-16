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
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_matrix_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_matrix_partial_eq_unsigned_matrix);
    register_unsigned_demos!(runner, demo_unsigned_matrix_partial_eq_natural_matrix);

    register_unsigned_benches!(
        runner,
        benchmark_natural_matrix_partial_eq_unsigned_matrix_algorithms
    );
}

fn demo_natural_matrix_partial_eq_unsigned_matrix<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: PartialEq<T>,
{
    for (m, n) in natural_matrix_unsigned_matrix_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if m == n {
            println!("{m} = {n}");
        } else {
            println!("{m} ≠ {n}");
        }
    }
}

fn demo_unsigned_matrix_partial_eq_natural_matrix<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: PartialEq<T>,
{
    for (m, n) in natural_matrix_unsigned_matrix_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if n == m {
            println!("{n} = {m}");
        } else {
            println!("{n} ≠ {m}");
        }
    }
}

// The comparison's result is what is being timed, and the converting arm compares with an owned
// vector on purpose.
#[allow(clippy::cmp_owned, clippy::no_effect, clippy::op_ref, unused_must_use)]
fn benchmark_natural_matrix_partial_eq_unsigned_matrix_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T> + PartialEq<T>,
{
    run_benchmark(
        &format!("NaturalMatrix == UnsignedMatrix<{}>", T::NAME),
        BenchmarkType::Algorithms,
        natural_matrix_unsigned_matrix_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("m"),
        &mut [
            ("default", &mut |(m, n)| no_out!(m == n)),
            ("converting the UnsignedMatrix", &mut |(m, n)| {
                no_out!(&m == &NaturalMatrix::from(n));
            }),
        ],
    );
}
