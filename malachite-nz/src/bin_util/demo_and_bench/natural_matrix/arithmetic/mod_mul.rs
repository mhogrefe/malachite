// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModMul, ModMulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_natural_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_mul);
    register_demo!(runner, demo_natural_matrix_mod_mul_ref);
    register_demo!(runner, demo_natural_matrix_mod_mul_assign);

    register_bench!(runner, benchmark_natural_matrix_mod_mul_algorithms);
}

fn demo_natural_matrix_mod_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, c, m) in natural_matrix_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.mod_mul({c}, {m}) = {}", a.mod_mul(&c, &m));
    }
}

fn demo_natural_matrix_mod_mul_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, c, m) in natural_matrix_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).mod_mul(&{c}, &{m}) = {}", (&a).mod_mul(&c, &m));
    }
}

fn demo_natural_matrix_mod_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, c, m) in natural_matrix_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_mul_assign(&c, &m);
        println!("a := {a_old}; a.mod_mul_assign({c}, {m}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, c, m)| {
                no_out!(a.mod_mul(c, m));
            }),
            ("element by element, no precomputation", &mut |(a, c, m)| {
                no_out!(
                    a.into_elements()
                        .into_iter()
                        .map(|x| x.mod_mul(&c, &m))
                        .collect::<Vec<Natural>>()
                );
            }),
            ("multiply, then reduce", &mut |(a, c, m)| {
                no_out!((a * c) % m);
            }),
        ],
    );
}
