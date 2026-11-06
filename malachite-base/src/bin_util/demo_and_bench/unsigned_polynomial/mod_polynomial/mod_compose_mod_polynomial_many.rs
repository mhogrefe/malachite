// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::test_util::bench::bucketers::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial_many::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_compose_mod_polynomial_many
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_compose_mod_polynomial_many_ref_ref
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_compose_mod_polynomial_many_algorithms
    );
}

fn polynomials_to_string<T: PrimitiveUnsigned>(ps: &[UnsignedPolynomial<T>]) -> String {
    let strings: Vec<String> = ps.iter().map(ToString::to_string).collect();
    format!("[{}]", strings.join(", "))
}

fn demo_unsigned_polynomial_mod_compose_mod_polynomial_many<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (fs, g, h, m) in
        unsigned_polynomial_vec_unsigned_polynomial_pair_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        let g_old = g.clone();
        let h_old = h.clone();
        println!(
            "{}.mod_compose_mod_polynomial({g_old}, {h_old}, {m}) = {}",
            polynomials_to_string(&fs),
            polynomials_to_string(&fs.as_slice().mod_compose_mod_polynomial(g, h, m))
        );
    }
}

fn demo_unsigned_polynomial_mod_compose_mod_polynomial_many_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (fs, g, h, m) in
        unsigned_polynomial_vec_unsigned_polynomial_pair_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "{}.mod_compose_mod_polynomial(&({g}), &({h}), {m}) = {}",
            polynomials_to_string(&fs),
            polynomials_to_string(&fs.as_slice().mod_compose_mod_polynomial(&g, &h, m))
        );
    }
}

fn benchmark_unsigned_polynomial_mod_compose_mod_polynomial_many_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "[UnsignedPolynomial<{}>].mod_compose_mod_polynomial(&UnsignedPolynomial<{}>, &Unsigne\
            dPolynomial<{}>, \
            {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_vec_unsigned_polynomial_pair_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .map(|(fs, g, h, m)| (fs, h, (g, m))),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_vec_total_len_max_len_bucketer("fs", "h"),
        &mut [
            ("default", &mut |(fs, h, (g, m))| {
                no_out!(fs.as_slice().mod_compose_mod_polynomial(&g, &h, m));
            }),
            ("one at a time", &mut |(fs, h, (g, m))| {
                no_out!(mod_compose_mod_polynomial_many_one_at_a_time(
                    &fs, &g, &h, m
                ));
            }),
            ("Brent-Kung", &mut |(fs, h, (g, m))| {
                no_out!(mod_compose_mod_polynomial_many_brent_kung(&fs, &g, &h, m));
            }),
            ("naive", &mut |(fs, h, (g, m))| {
                no_out!(mod_compose_mod_polynomial_many_naive(&fs, &g, &h, m));
            }),
        ],
    );
}
