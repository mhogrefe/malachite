// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModIntegral, ModIntegralAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_integral::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_integral);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_integral_ref);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_integral_assign);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_integral_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_integral_algorithms
    );
}

fn demo_unsigned_polynomial_mod_integral<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, _, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .filter(|(p, _, m)| mod_integral_is_defined(p, *m))
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).mod_integral({m}) = {}", p.mod_integral(m));
    }
}

fn demo_unsigned_polynomial_mod_integral_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, _, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .filter(|(p, _, m)| mod_integral_is_defined(p, *m))
        .take(limit)
    {
        println!("(&({p})).mod_integral({m}) = {}", (&p).mod_integral(m));
    }
}

fn demo_unsigned_polynomial_mod_integral_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, _, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .filter(|(p, _, m)| mod_integral_is_defined(p, *m))
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_integral_assign(m);
        println!("p := {p_old}; p.mod_integral_assign({m}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_integral_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial.mod_integral(T)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>()
            .get(gm, config)
            .filter(|(p, _, m)| mod_integral_is_defined(p, *m)),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("p.mod_integral(m)", &mut |(p, _, m)| {
                no_out!(p.mod_integral(m));
            }),
            ("(&p).mod_integral(m)", &mut |(p, _, m)| {
                no_out!((&p).mod_integral(m));
            }),
            ("p.mod_integral_assign(m)", &mut |(mut p, _, m)| {
                p.mod_integral_assign(m);
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_integral_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial.mod_integral(T)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>()
            .get(gm, config)
            .filter(|(p, _, m)| mod_integral_is_defined(p, *m)),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, _, m)| no_out!((&p).mod_integral(m))),
            ("naive", &mut |(p, _, m)| {
                no_out!(mod_integral_naive(&p, m));
            }),
        ],
    );
}
