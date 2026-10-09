// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_51;
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::bench::bucketers::natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_standard_basis_index);
    register_demo!(
        runner,
        demo_natural_vector_standard_basis_index_basis_vectors
    );
    register_bench!(runner, benchmark_natural_vector_standard_basis_index);
}

fn demo_natural_vector_standard_basis_index(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in natural_vector_gen().get(gm, config).take(limit) {
        println!(
            "{v}.standard_basis_index() = {:?}",
            v.standard_basis_index()
        );
    }
}

// Most generated vectors are not standard basis vectors, so this demo builds them.
fn demo_natural_vector_standard_basis_index_basis_vectors(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (index, dimension) in unsigned_pair_gen_var_51().get(gm, config).take(limit) {
        let v = NaturalVector::standard_basis_vector(dimension, index);
        println!(
            "{v}.standard_basis_index() = {:?}",
            v.standard_basis_index()
        );
    }
}

fn benchmark_natural_vector_standard_basis_index(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.standard_basis_index()",
        BenchmarkType::Single,
        natural_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |v| {
            no_out!(v.standard_basis_index());
        })],
    );
}
