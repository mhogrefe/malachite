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
        demo_unsigned_polynomial_mod_mul_mod_polynomial_precomputed
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_mul_mod_polynomial_precomputed_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_mul_mod_polynomial_precomputed_assign
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_mul_mod_polynomial_precomputed_algorithms
    );
}

fn demo_unsigned_polynomial_mod_mul_mod_polynomial_precomputed<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, f, m) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        let f_old = f.clone();
        let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &m);
        println!(
            "({a_old}).mod_mul_mod_polynomial_precomputed({b_old}, {f_old}, {m}, &data) = {}",
            a.mod_mul_mod_polynomial_precomputed(b, f, m, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_mul_mod_polynomial_precomputed_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, f, m) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &m);
        println!(
            "(&({a})).mod_mul_mod_polynomial_precomputed(&({b}), &({f}), {m}, &data) = {}",
            (&a).mod_mul_mod_polynomial_precomputed(&b, &f, m, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_mul_mod_polynomial_precomputed_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, b, f, m) in unsigned_polynomial_triple_unsigned_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        let f_old = f.clone();
        let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &m);
        a.mod_mul_mod_polynomial_precomputed_assign(b, f, m, &data);
        println!(
            "a := {a_old}; a.mod_mul_mod_polynomial_precomputed_assign({b_old}, {f_old}, {m}, &dat\
            a); a = {a}"
        );
    }
}

fn benchmark_unsigned_polynomial_mod_mul_mod_polynomial_precomputed_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_mul_mod_polynomial_precomputed(UnsignedPolynomial<{}>, Uns\
            ignedPolynomial<{}>, {}, &data)",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_triple_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .map(|(a, b, f, m)| {
                let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &m);
                (a, b, (f, m, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("a", "b"),
        &mut [
            ("precomputed", &mut |(a, b, (f, m, data))| {
                no_out!((&a).mod_mul_mod_polynomial_precomputed(&b, &f, m, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(a, b, (f, m, _))| {
                    let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &m);
                    no_out!((&a).mod_mul_mod_polynomial_precomputed(&b, &f, m, &data));
                },
            ),
            ("not precomputed", &mut |(a, b, (f, m, _))| {
                no_out!((&a).mod_mul_mod_polynomial(&b, &f, m));
            }),
        ],
    );
}
