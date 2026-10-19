// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::matrix::Matrix;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::unsigned_vec_integer_matrix_pair_gen_var_1;
use malachite_nz::test_util::integer_matrix::arithmetic::multi_crt::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_multi_balanced_crt);
    register_demo!(
        runner,
        demo_integer_matrix_multi_balanced_crt_unrestricted_moduli
    );

    register_bench!(
        runner,
        benchmark_integer_matrix_multi_balanced_crt_algorithms
    );
}

fn print_line(moduli: &[Limb], residues: &[UnsignedMatrix<Limb>], result: Option<IntegerMatrix>) {
    println!(
        "multi_balanced_crt({moduli:?}, [{}]) = {}",
        residues.iter().join(", "),
        result.map_or_else(|| "None".to_string(), |a| format!("Some({a})"))
    );
}

fn demo_integer_matrix_multi_balanced_crt(gm: GenMode, config: &GenConfig, limit: usize) {
    for (moduli, a) in unsigned_vec_integer_matrix_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let residues = a.multi_mod(&moduli);
        print_line(
            &moduli,
            &residues,
            IntegerMatrix::multi_balanced_crt(&moduli, &residues),
        );
    }
}

// Any nonempty list of moduli, with residues of 0, which are reduced modulo every nonzero modulus.
// Most of these lists are unusable.
fn demo_integer_matrix_multi_balanced_crt_unrestricted_moduli(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for moduli in unsigned_vec_gen::<Limb>()
        .get(gm, config)
        .filter(|moduli| !moduli.is_empty())
        .take(limit)
    {
        let residues = vec![UnsignedMatrix::zero(2, 2); moduli.len()];
        print_line(
            &moduli,
            &residues,
            IntegerMatrix::multi_balanced_crt(&moduli, &residues),
        );
    }
}

fn benchmark_integer_matrix_multi_balanced_crt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix::multi_balanced_crt(&[Limb], &[UnsignedMatrix<Limb>])",
        BenchmarkType::Algorithms,
        unsigned_vec_integer_matrix_pair_gen_var_1()
            .get(gm, config)
            .map(|(moduli, a)| {
                let residues = a.multi_mod(&moduli);
                (moduli, residues)
            }),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("moduli"),
        &mut [
            ("default", &mut |(moduli, residues)| {
                no_out!(IntegerMatrix::multi_balanced_crt(&moduli, &residues));
            }),
            ("one element at a time", &mut |(moduli, residues)| {
                no_out!(integer_matrix_multi_balanced_crt_naive(&moduli, &residues));
            }),
        ],
    );
}
