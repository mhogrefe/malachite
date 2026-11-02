// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_ref_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_assign
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_algorithms
    );
}

fn demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, pow) in natural_polynomial_triple_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
        let f_old = f.clone();
        let g_old = g.clone();
        let h_old = h.clone();
        println!(
            "({f_old}).mod_power_of_2_compose_mod_polynomial_precomputed({g_old}, {h_old}, \
            {pow}, &data) = {}",
            f.mod_power_of_2_compose_mod_polynomial_precomputed(g, h, pow, &data)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, pow) in natural_polynomial_triple_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
        println!(
            "(&({f})).mod_power_of_2_compose_mod_polynomial_precomputed(&({g}), &({h}), {pow}, \
            &data) = {}",
            (&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, pow, &data)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut f, g, h, pow) in natural_polynomial_triple_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
        let f_old = f.clone();
        f.mod_power_of_2_compose_mod_polynomial_precomputed_assign(&g, &h, pow, &data);
        println!(
            "f := {f_old}; f.mod_power_of_2_compose_mod_polynomial_precomputed_assign(&({g}), \
            &({h}), {pow}, &data); f = {f}"
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_compose_mod_polynomial_precomputed(NaturalPolynomial, \
        NaturalPolynomial, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_triple_unsigned_quadruple_gen_var_2()
            .get(gm, config)
            .map(|(f, g, h, pow)| {
                let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
                (f, h, (g, pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("f", "h"),
        &mut [
            ("precomputed", &mut |(f, h, (g, pow, data))| {
                no_out!((&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, pow, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(f, h, (g, pow, _))| {
                    let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
                    no_out!(
                        (&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, pow, &data)
                    );
                },
            ),
            ("not precomputed", &mut |(f, h, (g, pow, _))| {
                no_out!((&f).mod_power_of_2_compose_mod_polynomial(&g, &h, pow));
            }),
        ],
    );
}
