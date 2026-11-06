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
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_vec_max_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::mod_polynomial::mod_compose_mod_polynomial_many::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_polynomial_mod_compose_mod_polynomial_many
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_compose_mod_polynomial_many_ref_ref
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_compose_mod_polynomial_many_algorithms
    );
}

fn polynomials_to_string(ps: &[NaturalPolynomial]) -> String {
    let strings: Vec<String> = ps.iter().map(ToString::to_string).collect();
    format!("[{}]", strings.join(", "))
}

fn demo_natural_polynomial_mod_compose_mod_polynomial_many(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (fs, g, h, m) in
        natural_polynomial_vec_natural_polynomial_pair_natural_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let g_old = g.clone();
        let h_old = h.clone();
        let m_old = m.clone();
        println!(
            "{}.mod_compose_mod_polynomial({g_old}, {h_old}, {m_old}) = {}",
            polynomials_to_string(&fs),
            polynomials_to_string(&fs.as_slice().mod_compose_mod_polynomial(g, h, m))
        );
    }
}

fn demo_natural_polynomial_mod_compose_mod_polynomial_many_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (fs, g, h, m) in
        natural_polynomial_vec_natural_polynomial_pair_natural_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "{}.mod_compose_mod_polynomial(&({g}), &({h}), {m}) = {}",
            polynomials_to_string(&fs),
            polynomials_to_string(&fs.as_slice().mod_compose_mod_polynomial(&g, &h, &m))
        );
    }
}

fn benchmark_natural_polynomial_mod_compose_mod_polynomial_many_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "[NaturalPolynomial].mod_compose_mod_polynomial(&NaturalPolynomial, &NaturalPolynomial, &N\
        atural)",
        BenchmarkType::Algorithms,
        natural_polynomial_vec_natural_polynomial_pair_natural_quadruple_gen_var_1()
            .get(gm, config)
            .map(|(fs, g, h, m)| (fs, h, (g, m))),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_vec_max_bit_bucketer("fs", "h"),
        &mut [
            ("default", &mut |(fs, h, (g, m))| {
                no_out!(fs.as_slice().mod_compose_mod_polynomial(&g, &h, &m));
            }),
            ("one at a time", &mut |(fs, h, (g, m))| {
                no_out!(mod_compose_mod_polynomial_many_one_at_a_time(
                    &fs, &g, &h, &m
                ));
            }),
            ("Brent-Kung", &mut |(fs, h, (g, m))| {
                no_out!(mod_compose_mod_polynomial_many_brent_kung(&fs, &g, &h, &m));
            }),
            ("naive", &mut |(fs, h, (g, m))| {
                no_out!(mod_compose_mod_polynomial_many_naive(&fs, &g, &h, &m));
            }),
        ],
    );
}
