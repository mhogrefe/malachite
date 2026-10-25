// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::HermiteNormalForm;
use malachite_base::num::basic::traits::One;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_is_in_hermite_normal_form);
    register_demo!(
        runner,
        demo_integer_matrix_is_in_hermite_normal_form_of_form
    );
    register_bench!(runner, benchmark_integer_matrix_is_in_hermite_normal_form);
}

fn demo_integer_matrix_is_in_hermite_normal_form(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!(
            "is_in_hermite_normal_form({a}) = {}",
            a.is_in_hermite_normal_form()
        );
    }
}

// Hermite normal forms, which are in Hermite normal form, and the forms with one element changed,
// which may not be.
fn demo_integer_matrix_is_in_hermite_normal_form_of_form(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let mut h = a.hermite_normal_form();
        println!(
            "is_in_hermite_normal_form({h}) = {}",
            h.is_in_hermite_normal_form()
        );
        if !h.is_empty() {
            let i = usize::try_from(h.height() - 1).unwrap();
            let j = usize::try_from(h.width() - 1).unwrap();
            h[(i, j)] += Integer::ONE;
            println!(
                "is_in_hermite_normal_form({h}) = {}",
                h.is_in_hermite_normal_form()
            );
        }
    }
}

fn benchmark_integer_matrix_is_in_hermite_normal_form(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.is_in_hermite_normal_form()",
        BenchmarkType::Single,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [("Malachite", &mut |a| {
            no_out!(a.is_in_hermite_normal_form());
        })],
    );
}
