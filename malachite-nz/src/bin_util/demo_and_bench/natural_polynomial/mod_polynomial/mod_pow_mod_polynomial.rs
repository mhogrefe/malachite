// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::mod_polynomial::mod_pow_mod_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_pow_mod_polynomial);
    register_demo!(
        runner,
        demo_natural_polynomial_mod_pow_mod_polynomial_ref_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_pow_mod_polynomial_assign
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_pow_mod_polynomial_algorithms
    );
}

fn demo_natural_polynomial_mod_pow_mod_polynomial(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp, f, m) in
        natural_polynomial_unsigned_natural_polynomial_natural_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let a_old = a.clone();
        let f_old = f.clone();
        println!(
            "({a_old}).mod_pow_mod_polynomial({exp}, {f_old}, {m}) = {}",
            a.mod_pow_mod_polynomial(exp, f, m.clone())
        );
    }
}

fn demo_natural_polynomial_mod_pow_mod_polynomial_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, exp, f, m) in
        natural_polynomial_unsigned_natural_polynomial_natural_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "(&({a})).mod_pow_mod_polynomial({exp}, &({f}), {m}) = {}",
            (&a).mod_pow_mod_polynomial(exp, &f, &m)
        );
    }
}

fn demo_natural_polynomial_mod_pow_mod_polynomial_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, exp, f, m) in
        natural_polynomial_unsigned_natural_polynomial_natural_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let a_old = a.clone();
        let f_old = f.clone();
        a.mod_pow_mod_polynomial_assign(exp, f, &m);
        println!("a := {a_old}; a.mod_pow_mod_polynomial_assign({exp}, {f_old}, {m}); a = {a}");
    }
}

fn benchmark_natural_polynomial_mod_pow_mod_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_pow_mod_polynomial(u64, NaturalPolynomial, Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_natural_polynomial_natural_quadruple_gen_var_1()
            .get(gm, config)
            .map(|(a, exp, f, m)| (a, f, (exp, m))),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("a", "f"),
        &mut [
            ("default", &mut |(a, f, (exp, m))| {
                no_out!((&a).mod_pow_mod_polynomial(exp, &f, &m));
            }),
            ("naive", &mut |(a, f, (exp, m))| {
                no_out!(mod_pow_mod_polynomial_naive(&a, exp, &f, &m));
            }),
        ],
    );
}
