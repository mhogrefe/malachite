// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_nz::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::natural_polynomial::mod_polynomial::mod_power_of_2_compose_mod_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_ref_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_assign
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_compose_mod_polynomial_algorithms
    );
}

fn demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, pow) in natural_polynomial_triple_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let f_old = f.clone();
        let g_old = g.clone();
        let h_old = h.clone();
        println!(
            "({f_old}).mod_power_of_2_compose_mod_polynomial({g_old}, {h_old}, {pow}) = {}",
            f.mod_power_of_2_compose_mod_polynomial(g, h, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, pow) in natural_polynomial_triple_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({f})).mod_power_of_2_compose_mod_polynomial(&({g}), &({h}), {pow}) = {}",
            (&f).mod_power_of_2_compose_mod_polynomial(&g, &h, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut f, g, h, pow) in natural_polynomial_triple_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let f_old = f.clone();
        f.mod_power_of_2_compose_mod_polynomial_assign(&g, &h, pow);
        println!(
            "f := {f_old}; f.mod_power_of_2_compose_mod_polynomial_assign(&({g}), &({h}), \
            {pow}); f = {f}"
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_compose_mod_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_compose_mod_polynomial(NaturalPolynomial, \
        NaturalPolynomial, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_triple_unsigned_quadruple_gen_var_2()
            .get(gm, config)
            .map(|(f, g, h, pow)| (f, h, (g, pow))),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("f", "h"),
        &mut [
            ("default", &mut |(f, h, (g, pow))| {
                no_out!((&f).mod_power_of_2_compose_mod_polynomial(&g, &h, pow));
            }),
            ("Horner", &mut |(f, h, (g, pow))| {
                no_out!(mod_power_of_2_compose_mod_polynomial_horner(
                    &f, &g, &h, pow
                ));
            }),
            ("Brent-Kung", &mut |(f, h, (g, pow))| {
                no_out!(mod_power_of_2_compose_mod_polynomial_brent_kung(
                    &f, &g, &h, pow
                ));
            }),
            ("naive", &mut |(f, h, (g, pow))| {
                no_out!(mod_power_of_2_compose_mod_polynomial_naive(&f, &g, &h, pow));
            }),
        ],
    );
}
