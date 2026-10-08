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
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_unsigned_vector_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_integer_vector_partial_eq_unsigned_vector);
    register_unsigned_demos!(runner, demo_unsigned_vector_partial_eq_integer_vector);

    register_unsigned_benches!(
        runner,
        benchmark_integer_vector_partial_eq_unsigned_vector_algorithms
    );
}

fn demo_integer_vector_partial_eq_unsigned_vector<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialEq<T>,
{
    for (v, w) in integer_vector_unsigned_vector_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if v == w {
            println!("{v} = {w}");
        } else {
            println!("{v} ≠ {w}");
        }
    }
}

fn demo_unsigned_vector_partial_eq_integer_vector<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialEq<T>,
{
    for (v, w) in integer_vector_unsigned_vector_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if w == v {
            println!("{w} = {v}");
        } else {
            println!("{w} ≠ {v}");
        }
    }
}

// The comparison's result is what is being timed, and the converting arm compares with an owned
// vector on purpose.
#[allow(clippy::cmp_owned, clippy::no_effect, clippy::op_ref, unused_must_use)]
fn benchmark_integer_vector_partial_eq_unsigned_vector_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T> + PartialEq<T>,
{
    run_benchmark(
        &format!("IntegerVector == UnsignedVector<{}>", T::NAME),
        BenchmarkType::Algorithms,
        integer_vector_unsigned_vector_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, w)| no_out!(v == w)),
            ("converting the UnsignedVector", &mut |(v, w)| {
                no_out!(&v == &IntegerVector::from(w));
            }),
        ],
    );
}
