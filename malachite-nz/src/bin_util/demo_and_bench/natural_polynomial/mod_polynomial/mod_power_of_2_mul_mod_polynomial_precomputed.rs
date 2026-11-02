// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomial;
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
        demo_natural_polynomial_mod_power_of_2_mul_mod_polynomial_precomputed
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_mul_mod_polynomial_precomputed_ref_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_mul_mod_polynomial_precomputed_assign
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_mul_mod_polynomial_precomputed_algorithms
    );
}

fn demo_natural_polynomial_mod_power_of_2_mul_mod_polynomial_precomputed(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, f, pow) in natural_polynomial_triple_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        let f_old = f.clone();
        let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, pow);
        println!(
            "({a_old}).mod_power_of_2_mul_mod_polynomial_precomputed({b_old}, {f_old}, {pow}, &dat\
            a) = {}",
            a.mod_power_of_2_mul_mod_polynomial_precomputed(b, f, pow, &data)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_mul_mod_polynomial_precomputed_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, f, pow) in natural_polynomial_triple_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, pow);
        println!(
            "(&({a})).mod_power_of_2_mul_mod_polynomial_precomputed(&({b}), &({f}), {pow}, &data) \
            = {}",
            (&a).mod_power_of_2_mul_mod_polynomial_precomputed(&b, &f, pow, &data)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_mul_mod_polynomial_precomputed_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, b, f, pow) in natural_polynomial_triple_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        let f_old = f.clone();
        let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, pow);
        a.mod_power_of_2_mul_mod_polynomial_precomputed_assign(b, f, pow, &data);
        println!(
            "a := {a_old}; a.mod_power_of_2_mul_mod_polynomial_precomputed_assign({b_old}, {f_old}\
            , {pow}, &data); a = {a}"
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_mul_mod_polynomial_precomputed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_mul_mod_polynomial_precomputed(NaturalPolynomial, Natura\
        lPolynomial, u64, &data)",
        BenchmarkType::Algorithms,
        natural_polynomial_triple_unsigned_quadruple_gen_var_1()
            .get(gm, config)
            .map(|(a, b, f, pow)| {
                let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, pow);
                (a, b, (f, pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("a", "b"),
        &mut [
            ("precomputed", &mut |(a, b, (f, pow, data))| {
                no_out!((&a).mod_power_of_2_mul_mod_polynomial_precomputed(&b, &f, pow, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(a, b, (f, pow, _))| {
                    let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, pow);
                    no_out!((&a).mod_power_of_2_mul_mod_polynomial_precomputed(&b, &f, pow, &data));
                },
            ),
            ("not precomputed", &mut |(a, b, (f, pow, _))| {
                no_out!((&a).mod_power_of_2_mul_mod_polynomial(&b, &f, pow));
            }),
        ],
    );
}
