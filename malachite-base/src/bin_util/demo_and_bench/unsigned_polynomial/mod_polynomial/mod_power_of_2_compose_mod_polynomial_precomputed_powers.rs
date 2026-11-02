// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::mod_polynomial::ModComposeModPolynomialData;
use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_powers
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_powers_ref_ref
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_powers_algorithms
    );
}

fn powers_data<T: PrimitiveUnsigned>(
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    pow: u64,
) -> ModComposeModPolynomialData<T> {
    UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(g, h, &pow)
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_powers<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, pow) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let data = powers_data(&g, &h, pow);
        let f_old = f.clone();
        let h_old = h.clone();
        println!(
            "({f_old}).mod_power_of_2_compose_mod_polynomial_precomputed_powers({h_old}, {pow}, \
            &data) = {}, where data is from g = {g}",
            f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h, pow, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_precomputed_powers_ref_ref<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, pow) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let data = powers_data(&g, &h, pow);
        println!(
            "(&({f})).mod_power_of_2_compose_mod_polynomial_precomputed_powers(&({h}), {pow}, \
            &data) = {}, where data is from g = {g}",
            (&f).mod_power_of_2_compose_mod_polynomial_precomputed_powers(&h, pow, &data)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_powers_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_compose_mod_polynomial_precomputed_powers(Unsig\
            nedPolynomial<{}>, Unsigned\
            Polynomial<{}>, u64)",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_triple_unsigned_quadruple_gen_var_4::<T>()
            .get(gm, config)
            .map(|(f, g, h, pow)| {
                let data = powers_data(&g, &h, pow);
                (f, h, (g, pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("f", "h"),
        &mut [
            ("precomputed", &mut |(f, h, (_, pow, data))| {
                no_out!(
                    (&f).mod_power_of_2_compose_mod_polynomial_precomputed_powers(&h, pow, &data)
                );
            }),
            (
                "precomputed, including precomputation",
                &mut |(f, h, (g, pow, _))| {
                    let data = powers_data(&g, &h, pow);
                    no_out!(
                        (&f).mod_power_of_2_compose_mod_polynomial_precomputed_powers(
                            &h, pow, &data
                        )
                    );
                },
            ),
            ("not precomputed", &mut |(f, h, (g, pow, _))| {
                no_out!((&f).mod_power_of_2_compose_mod_polynomial(&g, &h, pow));
            }),
        ],
    );
}
