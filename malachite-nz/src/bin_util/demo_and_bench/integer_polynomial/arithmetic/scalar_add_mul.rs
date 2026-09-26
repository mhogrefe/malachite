// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::triple_1_2_vec_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::arithmetic::scalar_add_mul::integers_add_mul_scalar_assign;
use malachite_nz::test_util::generators::integer_vec_integer_vec_integer_triple_gen;
use malachite_nz::test_util::integer_polynomial::arithmetic::scalar_add_mul::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integers_add_mul_scalar_assign);

    register_bench!(runner, benchmark_integers_add_mul_scalar_assign_algorithms);
}

fn demo_integers_add_mul_scalar_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys, c) in integer_vec_integer_vec_integer_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        integers_add_mul_scalar_assign(&mut xs, &ys, &c);
        println!(
            "xs := {xs_old:?}; integers_add_mul_scalar_assign(&mut xs, {ys:?}, {c}); xs = {xs:?}"
        );
    }
}

fn benchmark_integers_add_mul_scalar_assign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "integers_add_mul_scalar_assign(&mut Vec<Integer>, &[Integer], &Integer)",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("default", &mut |(mut xs, ys, c)| {
                integers_add_mul_scalar_assign(&mut xs, &ys, &c);
            }),
            ("naive", &mut |(xs, ys, c)| {
                no_out!(integers_add_mul_scalar_naive(&xs, &ys, &c));
            }),
        ],
    );
}
