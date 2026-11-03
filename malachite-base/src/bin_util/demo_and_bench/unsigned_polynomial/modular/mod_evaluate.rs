// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::modular::mod_mul::mod_mul_precompute_shoup;
use malachite_base::polynomial::modular::traits::ModEvaluate;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate::*;
use malachite_base::unsigned_polynomial::modular::mod_evaluate::{
    mod_evaluate_horner, mod_evaluate_shoup, mod_evaluate_shoup_lazy,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_evaluate);
    register_demo!(runner, demo_unsigned_polynomial_mod_evaluate_ref);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_evaluate_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_evaluate_algorithms
    );
}

fn demo_unsigned_polynomial_mod_evaluate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_evaluate({x}, {m}) = {}",
            p.mod_evaluate(x, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_evaluate_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_evaluate({x}, {m}) = {}",
            (&p).mod_evaluate(x, m)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_evaluate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_evaluate(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            (
                "UnsignedPolynomial<u64>.mod_evaluate(u64, u64)",
                &mut |(p, x, m)| no_out!(p.mod_evaluate(x, m)),
            ),
            (
                "(&UnsignedPolynomial<u64>).mod_evaluate(u64, u64)",
                &mut |(p, x, m)| no_out!((&p).mod_evaluate(x, m)),
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_evaluate_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    // Every algorithm applies to a nonempty polynomial and a modulus at most this, including the
    // lazy Shoup loop.
    let lazy_max = u64::MAX / 3;
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_evaluate(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>()
            .get(gm, config)
            .filter(move |(p, _, m)| !p.coefficients_asc().is_empty() && *m <= lazy_max),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, x, m)| {
                no_out!(p.mod_evaluate(x, m));
            }),
            ("Horner", &mut |(p, x, m)| {
                no_out!(mod_evaluate_horner(p.coefficients_asc(), x, m));
            }),
            ("Shoup", &mut |(p, x, m)| {
                let x_precomp = mod_mul_precompute_shoup(x, m);
                no_out!(mod_evaluate_shoup(p.coefficients_asc(), x, x_precomp, m));
            }),
            ("lazy Shoup", &mut |(p, x, m)| {
                let x_precomp = mod_mul_precompute_shoup(x, m);
                no_out!(mod_evaluate_shoup_lazy(
                    p.coefficients_asc(),
                    x,
                    x_precomp,
                    m,
                ));
            }),
            ("naive", &mut |(p, x, m)| {
                no_out!(mod_evaluate_naive(&p, x, m));
            }),
        ],
    );
}
