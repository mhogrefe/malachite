// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::mod_mul::mod_mul_precompute_shoup;
use malachite_base::num::arithmetic::traits::ModPow;
use malachite_base::polynomial::{
    ModEvaluate, ModEvaluateGeometric, ModEvaluateMany, ModPowerOf2Evaluate,
};
use malachite_base::test_util::bench::bucketers::{
    quadruple_1_unsigned_polynomial_len_bucketer, triple_1_unsigned_polynomial_len_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1,
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::evaluate::*;
use malachite_base::unsigned_polynomial::arithmetic::evaluate::{
    mod_evaluate_horner, mod_evaluate_shoup, mod_evaluate_shoup_lazy,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_power_of_2_evaluate);
    register_demo!(runner, demo_unsigned_polynomial_mod_power_of_2_evaluate_ref);

    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_evaluate_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_evaluate_algorithms
    );
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
    register_demo!(runner, demo_unsigned_polynomial_mod_evaluate_many);
    register_demo!(runner, demo_unsigned_polynomial_mod_evaluate_geometric);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_evaluate_many_algorithms
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_evaluate_geometric_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_evaluate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_evaluate({x}, {pow}) = {}",
            p.mod_power_of_2_evaluate(x, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_evaluate_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_evaluate({x}, {pow}) = {}",
            (&p).mod_power_of_2_evaluate(x, pow)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_evaluate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_power_of_2_evaluate(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            (
                "UnsignedPolynomial<u64>.mod_power_of_2_evaluate(u64, u64)",
                &mut |(p, x, pow)| no_out!(p.mod_power_of_2_evaluate(x, pow)),
            ),
            (
                "(&UnsignedPolynomial<u64>).mod_power_of_2_evaluate(u64, u64)",
                &mut |(p, x, pow)| no_out!((&p).mod_power_of_2_evaluate(x, pow)),
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_evaluate_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_power_of_2_evaluate(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, x, pow)| {
                no_out!(p.mod_power_of_2_evaluate(x, pow));
            }),
            ("naive", &mut |(p, x, pow)| {
                no_out!(mod_power_of_2_evaluate_naive(&p, x, pow));
            }),
        ],
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

fn demo_unsigned_polynomial_mod_evaluate_many(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, xs, m) in unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_evaluate_many(&{xs:?}, {m}) = {:?}",
            (&p).mod_evaluate_many(&xs, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_evaluate_geometric(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, k, m) in unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_evaluate_geometric({q}, {k}, {m}) = {:?}",
            (&p).mod_evaluate_geometric(q, k, m)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_evaluate_many_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedPolynomial<u64>).mod_evaluate_many(&[u64], u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, xs, m)| {
                no_out!((&p).mod_evaluate_many(&xs, m));
            }),
            ("one at a time", &mut |(p, xs, m)| {
                no_out!(
                    xs.iter()
                        .map(|&x| (&p).mod_evaluate(x, m))
                        .collect::<Vec<_>>()
                );
            }),
            ("naive", &mut |(p, xs, m)| {
                no_out!(mod_evaluate_many_naive(&p, &xs, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_evaluate_geometric_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedPolynomial<u64>).mod_evaluate_geometric(u64, u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, q, k, m)| {
                no_out!((&p).mod_evaluate_geometric(q, k, m));
            }),
            ("one at a time", &mut |(p, q, k, m)| {
                no_out!(
                    (0..k)
                        .map(|j| (&p).mod_evaluate(q.mod_pow(j, m), m))
                        .collect::<Vec<_>>()
                );
            }),
            ("naive", &mut |(p, q, k, m)| {
                no_out!(mod_evaluate_geometric_naive(&p, q, k, m));
            }),
        ],
    );
}
