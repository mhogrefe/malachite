// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::unsigned_vec_natural_vector_pair_gen_var_1;
use malachite_nz::test_util::integer_vector::arithmetic::multi_crt::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_multi_balanced_crt);
    register_demo!(
        runner,
        demo_integer_vector_multi_balanced_crt_unrestricted_moduli
    );

    register_bench!(
        runner,
        benchmark_integer_vector_multi_balanced_crt_algorithms
    );
}

// Each modulus's residues of the vector's elements.
fn residues_of(moduli: &[Limb], v: &NaturalVector) -> Vec<UnsignedVector<Limb>> {
    moduli.iter().map(|&m| v % m).collect()
}

fn print_line(moduli: &[Limb], residues: &[UnsignedVector<Limb>], result: Option<IntegerVector>) {
    println!(
        "multi_balanced_crt({moduli:?}, [{}]) = {}",
        residues.iter().join(", "),
        result.map_or_else(|| "None".to_string(), |v| format!("Some({v})"))
    );
}

fn demo_integer_vector_multi_balanced_crt(gm: GenMode, config: &GenConfig, limit: usize) {
    for (moduli, v) in unsigned_vec_natural_vector_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let residues = residues_of(&moduli, &v);
        print_line(
            &moduli,
            &residues,
            IntegerVector::multi_balanced_crt(&moduli, &residues),
        );
    }
}

// Any nonempty list of moduli, with residues of 0, which are reduced modulo every nonzero modulus.
// Most of these lists are unusable.
fn demo_integer_vector_multi_balanced_crt_unrestricted_moduli(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for moduli in unsigned_vec_gen::<Limb>()
        .get(gm, config)
        .filter(|moduli| !moduli.is_empty())
        .take(limit)
    {
        let residues = vec![UnsignedVector::zero(2); moduli.len()];
        print_line(
            &moduli,
            &residues,
            IntegerVector::multi_balanced_crt(&moduli, &residues),
        );
    }
}

fn benchmark_integer_vector_multi_balanced_crt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector::multi_balanced_crt(&[Limb], &[UnsignedVector<Limb>])",
        BenchmarkType::Algorithms,
        unsigned_vec_natural_vector_pair_gen_var_1()
            .get(gm, config)
            .map(|(moduli, v)| {
                let residues = residues_of(&moduli, &v);
                (moduli, residues)
            }),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("moduli"),
        &mut [
            ("default", &mut |(moduli, residues)| {
                no_out!(IntegerVector::multi_balanced_crt(&moduli, &residues));
            }),
            ("one element at a time", &mut |(moduli, residues)| {
                no_out!(integer_vector_multi_balanced_crt_naive(&moduli, &residues));
            }),
        ],
    );
}
