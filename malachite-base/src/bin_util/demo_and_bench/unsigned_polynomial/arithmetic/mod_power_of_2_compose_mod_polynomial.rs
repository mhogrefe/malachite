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
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_compose_mod_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_assign
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, pow) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_4::<T>()
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

fn demo_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, pow) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({f})).mod_power_of_2_compose_mod_polynomial(&({g}), &({h}), {pow}) = {}",
            (&f).mod_power_of_2_compose_mod_polynomial(&g, &h, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut f, g, h, pow) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_4::<T>()
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

fn benchmark_unsigned_polynomial_mod_power_of_2_compose_mod_polynomial_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_compose_mod_polynomial(UnsignedPolynomial<{}>, \
            Unsigned\
            Polynomial<{}>, u64)",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_triple_unsigned_quadruple_gen_var_4::<T>()
            .get(gm, config)
            .map(|(f, g, h, pow)| (f, h, (g, pow))),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("f", "h"),
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
