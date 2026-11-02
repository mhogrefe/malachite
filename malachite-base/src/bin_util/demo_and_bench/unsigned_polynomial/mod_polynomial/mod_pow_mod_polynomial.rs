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
use malachite_base::test_util::unsigned_polynomial::mod_polynomial::mod_pow_mod_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_pow_mod_polynomial);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_pow_mod_polynomial_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_pow_mod_polynomial_assign
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_pow_mod_polynomial_algorithms
    );
}

fn demo_unsigned_polynomial_mod_pow_mod_polynomial<T: PrimitiveUnsigned>(
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
        println!(
            "({a_old}).mod_pow_mod_polynomial({exp}, {f_old}, {m}) = {}",
            a.mod_pow_mod_polynomial(exp, f, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_pow_mod_polynomial_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, exp, f, m) in
        unsigned_polynomial_unsigned_unsigned_polynomial_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "(&({a})).mod_pow_mod_polynomial({exp}, &({f}), {m}) = {}",
            (&a).mod_pow_mod_polynomial(exp, &f, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_pow_mod_polynomial_assign<T: PrimitiveUnsigned>(
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
        a.mod_pow_mod_polynomial_assign(exp, f, m);
        println!("a := {a_old}; a.mod_pow_mod_polynomial_assign({exp}, {f_old}, {m}); a = {a}");
    }
}

fn benchmark_unsigned_polynomial_mod_pow_mod_polynomial_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_pow_mod_polynomial(u64, UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_polynomial_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .map(|(a, exp, f, m)| (a, f, (exp, m))),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("a", "f"),
        &mut [
            ("default", &mut |(a, f, (exp, m))| {
                no_out!((&a).mod_pow_mod_polynomial(exp, &f, m));
            }),
            ("naive", &mut |(a, f, (exp, m))| {
                no_out!(mod_pow_mod_polynomial_naive(&a, exp, &f, m));
            }),
        ],
    );
}
