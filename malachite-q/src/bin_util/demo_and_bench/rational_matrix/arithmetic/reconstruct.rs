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
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_8;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;
use malachite_q::test_util::rational_matrix::arithmetic::reconstruct::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_reconstruct);
    register_demo!(runner, demo_rational_matrix_reconstruct_ref);
    register_demo!(runner, demo_rational_matrix_reconstruct_round_trip);
    register_bench!(
        runner,
        benchmark_rational_matrix_reconstruct_evaluation_strategy
    );
}

fn format_result(x: Option<RationalMatrix>) -> String {
    x.map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
}

fn demo_rational_matrix_reconstruct(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "RationalMatrix::reconstruct({a_old}, {m}) = {}",
            format_result(RationalMatrix::reconstruct(a, &m))
        );
    }
}

fn demo_rational_matrix_reconstruct_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "RationalMatrix::reconstruct_ref(&{a}, &{m}) = {}",
            format_result(RationalMatrix::reconstruct_ref(&a, &m))
        );
    }
}

// The residues of a generated rational matrix modulo a modulus large enough to recover it.
fn round_trip_inputs(a: &RationalMatrix) -> (NaturalMatrix, Natural) {
    let m = rational_matrix_reconstruction_modulus(a);
    (rational_matrix_residues(a, &m), m)
}

fn demo_rational_matrix_reconstruct_round_trip(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen().get(gm, config).take(limit) {
        let (residues, m) = round_trip_inputs(&a);
        println!(
            "RationalMatrix::reconstruct({residues}, {m}) = {}",
            format_result(RationalMatrix::reconstruct_ref(&residues, &m))
        );
    }
}

fn benchmark_rational_matrix_reconstruct_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix::reconstruct(NaturalMatrix, &Natural)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen()
            .get(gm, config)
            .map(|a| round_trip_inputs(&a)),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "RationalMatrix::reconstruct(NaturalMatrix, &Natural)",
                &mut |(a, m)| {
                    no_out!(RationalMatrix::reconstruct(a, &m));
                },
            ),
            (
                "RationalMatrix::reconstruct_ref(&NaturalMatrix, &Natural)",
                &mut |(a, m)| {
                    no_out!(RationalMatrix::reconstruct_ref(&a, &m));
                },
            ),
        ],
    );
}
