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
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::unsigned_vec_integer_vector_pair_gen_var_1;
use malachite_nz::test_util::integer_vector::arithmetic::multi_mod::integer_vector_multi_mod_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_multi_mod);

    register_bench!(runner, benchmark_integer_vector_multi_mod_algorithms);
}

fn demo_integer_vector_multi_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (moduli, v) in unsigned_vec_integer_vector_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{v}.multi_mod({moduli:?}) = [{}]",
            v.multi_mod(&moduli).iter().join(", ")
        );
    }
}

fn benchmark_integer_vector_multi_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.multi_mod(&[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_integer_vector_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("moduli"),
        &mut [
            ("default", &mut |(moduli, v)| no_out!(v.multi_mod(&moduli))),
            ("one modulus at a time", &mut |(moduli, v)| {
                no_out!(integer_vector_multi_mod_naive(&v, &moduli));
            }),
        ],
    );
}
