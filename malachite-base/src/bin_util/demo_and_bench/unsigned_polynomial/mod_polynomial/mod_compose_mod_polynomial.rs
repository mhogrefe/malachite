// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_compose_mod_polynomial);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_compose_mod_polynomial_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_compose_mod_polynomial_assign
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_compose_mod_polynomial_algorithms
    );
}

fn demo_unsigned_polynomial_mod_compose_mod_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, m) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let f_old = f.clone();
        let g_old = g.clone();
        let h_old = h.clone();
        println!(
            "({f_old}).mod_compose_mod_polynomial({g_old}, {h_old}, {m}) = {}",
            f.mod_compose_mod_polynomial(g, h, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_compose_mod_polynomial_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (f, g, h, m) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({f})).mod_compose_mod_polynomial(&({g}), &({h}), {m}) = {}",
            (&f).mod_compose_mod_polynomial(&g, &h, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_compose_mod_polynomial_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut f, g, h, m) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let f_old = f.clone();
        f.mod_compose_mod_polynomial_assign(&g, &h, m);
        println!("f := {f_old}; f.mod_compose_mod_polynomial_assign(&({g}), &({h}), {m}); f = {f}");
    }
}

fn benchmark_unsigned_polynomial_mod_compose_mod_polynomial_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_compose_mod_polynomial(UnsignedPolynomial<{}>, Unsigned\
            Polynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_triple_unsigned_quadruple_gen_var_3::<T>()
            .get(gm, config)
            .map(|(f, g, h, m)| (f, h, (g, m))),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("f", "h"),
        &mut [
            ("default", &mut |(f, h, (g, m))| {
                no_out!((&f).mod_compose_mod_polynomial(&g, &h, m));
            }),
            ("Horner", &mut |(f, h, (g, m))| {
                no_out!(mod_compose_mod_polynomial_horner(&f, &g, &h, m));
            }),
            ("Brent-Kung", &mut |(f, h, (g, m))| {
                no_out!(mod_compose_mod_polynomial_brent_kung(&f, &g, &h, m));
            }),
            ("naive", &mut |(f, h, (g, m))| {
                no_out!(mod_compose_mod_polynomial_naive(&f, &g, &h, m));
            }),
        ],
    );
}
