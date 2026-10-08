// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_natural_vector_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_partial_eq_natural_vector);
    register_demo!(runner, demo_natural_vector_partial_eq_integer_vector);

    register_bench!(
        runner,
        benchmark_integer_vector_partial_eq_natural_vector_algorithms
    );
}

fn demo_integer_vector_partial_eq_natural_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in integer_vector_natural_vector_pair_gen()
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

fn demo_natural_vector_partial_eq_integer_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in integer_vector_natural_vector_pair_gen()
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
fn benchmark_integer_vector_partial_eq_natural_vector_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector == NaturalVector",
        BenchmarkType::Algorithms,
        integer_vector_natural_vector_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, w)| no_out!(v == w)),
            ("converting the NaturalVector", &mut |(v, w)| {
                no_out!(&v == &IntegerVector::from(w));
            }),
        ],
    );
}
