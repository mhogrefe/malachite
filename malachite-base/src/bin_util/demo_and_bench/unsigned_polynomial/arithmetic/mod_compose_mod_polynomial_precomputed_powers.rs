// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_compose_mod_polynomial_precomputed_powers
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_compose_mod_polynomial_precomputed_powers_ref_ref
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_compose_mod_polynomial_powers_algorithms
    );
}

fn demo_unsigned_polynomial_mod_compose_mod_polynomial_precomputed_powers<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, m) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let data =
            UnsignedPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
        let f_old = f.clone();
        let h_old = h.clone();
        println!(
            "({f_old}).mod_compose_mod_polynomial_precomputed_powers({h_old}, {m}, &data) = {}, \
            where data is from g = {g}",
            f.mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_compose_mod_polynomial_precomputed_powers_ref_ref<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, m) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let data =
            UnsignedPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
        println!(
            "(&({f})).mod_compose_mod_polynomial_precomputed_powers(&({h}), {m}, &data) = {}, \
            where data is from g = {g}",
            (&f).mod_compose_mod_polynomial_precomputed_powers(&h, m, &data)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_compose_mod_polynomial_powers_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_compose_mod_polynomial_precomputed_powers(UnsignedPolynomi\
            al<{}>, Unsigned\
            Polynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_triple_unsigned_quadruple_gen_var_3::<T>()
            .get(gm, config)
            .map(|(f, g, h, m)| {
                let data = UnsignedPolynomial::precompute_mod_compose_mod_polynomial_powers_data(
                    &g, &h, &m,
                );
                (f, h, (g, m, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("f", "h"),
        &mut [
            ("precomputed", &mut |(f, h, (_, m, data))| {
                no_out!((&f).mod_compose_mod_polynomial_precomputed_powers(&h, m, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(f, h, (g, m, _))| {
                    let data =
                        UnsignedPolynomial::precompute_mod_compose_mod_polynomial_powers_data(
                            &g, &h, &m,
                        );
                    no_out!((&f).mod_compose_mod_polynomial_precomputed_powers(&h, m, &data));
                },
            ),
            ("not precomputed", &mut |(f, h, (g, m, _))| {
                no_out!((&f).mod_compose_mod_polynomial(&g, &h, m));
            }),
        ],
    );
}
