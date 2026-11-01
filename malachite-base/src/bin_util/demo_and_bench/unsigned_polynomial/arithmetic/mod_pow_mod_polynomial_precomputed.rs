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
        demo_unsigned_polynomial_mod_pow_mod_polynomial_precomputed
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_pow_mod_polynomial_precomputed_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_pow_mod_polynomial_precomputed_assign
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_pow_mod_polynomial_precomputed_algorithms
    );
}

fn demo_unsigned_polynomial_mod_pow_mod_polynomial_precomputed<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, exp, f, m) in
        unsigned_polynomial_unsigned_unsigned_polynomial_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        let a_old = a.clone();
        let f_old = f.clone();
        let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &m);
        println!(
            "({a_old}).mod_pow_mod_polynomial_precomputed({exp}, {f_old}, {m}, &data) = {}",
            a.mod_pow_mod_polynomial_precomputed(exp, f, m, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_pow_mod_polynomial_precomputed_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, exp, f, m) in
        unsigned_polynomial_unsigned_unsigned_polynomial_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &m);
        println!(
            "(&({a})).mod_pow_mod_polynomial_precomputed({exp}, &({f}), {m}, &data) = {}",
            (&a).mod_pow_mod_polynomial_precomputed(exp, &f, m, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_pow_mod_polynomial_precomputed_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, exp, f, m) in
        unsigned_polynomial_unsigned_unsigned_polynomial_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        let a_old = a.clone();
        let f_old = f.clone();
        let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &m);
        a.mod_pow_mod_polynomial_precomputed_assign(exp, f, m, &data);
        println!(
            "a := {a_old}; a.mod_pow_mod_polynomial_precomputed_assign({exp}, {f_old}, {m}, &data)\
            ; a = {a}"
        );
    }
}

fn benchmark_unsigned_polynomial_mod_pow_mod_polynomial_precomputed_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_pow_mod_polynomial_precomputed(u64, UnsignedPolynomial<{}>\
            , {}, &data)",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_polynomial_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .map(|(a, exp, f, m)| {
                let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &m);
                (a, f, (exp, m, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("a", "f"),
        &mut [
            ("precomputed", &mut |(a, f, (exp, m, data))| {
                no_out!((&a).mod_pow_mod_polynomial_precomputed(exp, &f, m, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(a, f, (exp, m, _))| {
                    let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &m);
                    no_out!((&a).mod_pow_mod_polynomial_precomputed(exp, &f, m, &data));
                },
            ),
            ("not precomputed", &mut |(a, f, (exp, m, _))| {
                no_out!((&a).mod_pow_mod_polynomial(exp, &f, m));
            }),
        ],
    );
}
