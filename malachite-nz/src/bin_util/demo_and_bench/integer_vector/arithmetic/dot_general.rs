// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::triple_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_vector::arithmetic::dot_general::vec_dot_general;
use malachite_nz::test_util::generators::integer_vec_integer_vec_integer_triple_gen_var_1;
use malachite_nz::test_util::integer_vector::arithmetic::dot_general::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_vec_dot_general);
    register_bench!(runner, benchmark_vec_dot_general_algorithms);
}

// Each generated case is printed with and without the initial value, and with every combination of
// `subtract` and `reverse`.
fn demo_vec_dot_general(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, initial) in integer_vec_integer_vec_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        for initial in [None, Some(&initial)] {
            for subtract in [false, true] {
                for reverse in [false, true] {
                    println!(
                        "vec_dot_general({:?}, {}, {:?}, {:?}, {}) = {}",
                        initial,
                        subtract,
                        xs,
                        ys,
                        reverse,
                        vec_dot_general(initial, subtract, &xs, &ys, reverse)
                    );
                }
            }
        }
    }
}

fn benchmark_vec_dot_general_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "vec_dot_general(Some(&Integer), false, &[Integer], &[Integer], false)",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_integer_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(xs, ys, initial)| {
                no_out!(vec_dot_general(Some(&initial), false, &xs, &ys, false));
            }),
            ("naive", &mut |(xs, ys, initial)| {
                no_out!(vec_dot_general_naive(
                    Some(&initial),
                    false,
                    &xs,
                    &ys,
                    false
                ));
            }),
        ],
    );
}
