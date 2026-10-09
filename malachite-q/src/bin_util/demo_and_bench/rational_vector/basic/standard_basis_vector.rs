// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_51;
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_q::rational_vector::RationalVector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_standard_basis_vector);
    register_bench!(runner, benchmark_rational_vector_standard_basis_vector);
}

fn demo_rational_vector_standard_basis_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for (index, dimension) in unsigned_pair_gen_var_51().get(gm, config).take(limit) {
        println!(
            "RationalVector::standard_basis_vector({dimension}, {index}) = {}",
            RationalVector::standard_basis_vector(dimension, index)
        );
    }
}

fn benchmark_rational_vector_standard_basis_vector(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector::standard_basis_vector(u64, u64)",
        BenchmarkType::Single,
        unsigned_pair_gen_var_51().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("dimension"),
        &mut [("Malachite", &mut |(index, dimension)| {
            no_out!(RationalVector::standard_basis_vector(dimension, index));
        })],
    );
}
