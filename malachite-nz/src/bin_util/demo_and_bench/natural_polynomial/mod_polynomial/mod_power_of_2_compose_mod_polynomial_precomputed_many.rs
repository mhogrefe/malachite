// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_vec_max_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_many
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_many_ref_ref
    );
    register_bench!(runner, benchmark_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_many_algorithms);
}

fn polynomials_to_string(ps: &[NaturalPolynomial]) -> String {
    let strings: Vec<String> = ps.iter().map(ToString::to_string).collect();
    format!("[{}]", strings.join(", "))
}

fn demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_many(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (fs, g, h, pow) in
        natural_polynomial_vec_natural_polynomial_pair_unsigned_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
        let g_old = g.clone();
        let h_old = h.clone();
        println!(
            "{}.mod_power_of_2_compose_mod_polynomial_precomputed({g_old}, {h_old}, {pow}) = {}",
            polynomials_to_string(&fs),
            polynomials_to_string(
                &fs.as_slice()
                    .mod_power_of_2_compose_mod_polynomial_precomputed(g, h, pow, &data)
            )
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_many_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (fs, g, h, pow) in
        natural_polynomial_vec_natural_polynomial_pair_unsigned_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
        println!(
            "{}.mod_power_of_2_compose_mod_polynomial_precomputed(&({g}), &({h}), {pow}) = {}",
            polynomials_to_string(&fs),
            polynomials_to_string(
                &fs.as_slice()
                    .mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, pow, &data)
            )
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_many_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "[NaturalPolynomial].mod_power_of_2_compose_mod_polynomial_precomputed(&NaturalPolynomial\
        , &NaturalPolynomial, u64, &ModPolynomialData<Natural>)",
        BenchmarkType::Algorithms,
        natural_polynomial_vec_natural_polynomial_pair_unsigned_quadruple_gen_var_1()
            .get(gm, config)
            .map(|(fs, g, h, pow)| {
                let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
                (fs, h, (g, pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_vec_max_bit_bucketer("fs", "h"),
        &mut [
            ("precomputed", &mut |(fs, h, (g, pow, data))| {
                no_out!(
                    fs.as_slice()
                        .mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, pow, &data)
                );
            }),
            ("not precomputed", &mut |(fs, h, (g, pow, _))| {
                no_out!(
                    fs.as_slice()
                        .mod_power_of_2_compose_mod_polynomial(&g, &h, pow)
                );
            }),
            ("one at a time, precomputed", &mut |(
                fs,
                h,
                (g, pow, data),
            )| {
                no_out!(
                    fs.iter()
                        .map(|f| f
                            .mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, pow, &data))
                        .collect::<Vec<_>>()
                );
            }),
        ],
    );
}
