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
    EvaluateGeometricMod, EvaluateManyMod, EvaluateMod, EvaluateModPowerOf2,
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
    evaluate_mod_horner, evaluate_mod_shoup, evaluate_mod_shoup_lazy,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_evaluate_mod_power_of_2);
    register_demo!(runner, demo_unsigned_polynomial_evaluate_mod_power_of_2_ref);

    register_bench!(
        runner,
        benchmark_unsigned_polynomial_evaluate_mod_power_of_2_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_evaluate_mod_power_of_2_algorithms
    );
    register_demo!(runner, demo_unsigned_polynomial_evaluate_mod);
    register_demo!(runner, demo_unsigned_polynomial_evaluate_mod_ref);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_evaluate_mod_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_evaluate_mod_algorithms
    );
    register_demo!(runner, demo_unsigned_polynomial_evaluate_many_mod);
    register_demo!(runner, demo_unsigned_polynomial_evaluate_geometric_mod);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_evaluate_many_mod_algorithms
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_evaluate_geometric_mod_algorithms
    );
}

fn demo_unsigned_polynomial_evaluate_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).evaluate_mod_power_of_2({x}, {pow}) = {}",
            p.evaluate_mod_power_of_2(x, pow)
        );
    }
}

fn demo_unsigned_polynomial_evaluate_mod_power_of_2_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_mod_power_of_2({x}, {pow}) = {}",
            (&p).evaluate_mod_power_of_2(x, pow)
        );
    }
}

fn benchmark_unsigned_polynomial_evaluate_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.evaluate_mod_power_of_2(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            (
                "UnsignedPolynomial<u64>.evaluate_mod_power_of_2(u64, u64)",
                &mut |(p, x, pow)| no_out!(p.evaluate_mod_power_of_2(x, pow)),
            ),
            (
                "(&UnsignedPolynomial<u64>).evaluate_mod_power_of_2(u64, u64)",
                &mut |(p, x, pow)| no_out!((&p).evaluate_mod_power_of_2(x, pow)),
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_evaluate_mod_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.evaluate_mod_power_of_2(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, x, pow)| {
                no_out!(p.evaluate_mod_power_of_2(x, pow));
            }),
            ("naive", &mut |(p, x, pow)| {
                no_out!(evaluate_mod_power_of_2_naive(&p, x, pow));
            }),
        ],
    );
}

fn demo_unsigned_polynomial_evaluate_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).evaluate_mod({x}, {m}) = {}",
            p.evaluate_mod(x, m)
        );
    }
}

fn demo_unsigned_polynomial_evaluate_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_mod({x}, {m}) = {}",
            (&p).evaluate_mod(x, m)
        );
    }
}

fn benchmark_unsigned_polynomial_evaluate_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.evaluate_mod(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            (
                "UnsignedPolynomial<u64>.evaluate_mod(u64, u64)",
                &mut |(p, x, m)| no_out!(p.evaluate_mod(x, m)),
            ),
            (
                "(&UnsignedPolynomial<u64>).evaluate_mod(u64, u64)",
                &mut |(p, x, m)| no_out!((&p).evaluate_mod(x, m)),
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_evaluate_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    // Every algorithm applies to a nonempty polynomial and a modulus at most this, including the
    // lazy Shoup loop.
    let lazy_max = u64::MAX / 3;
    run_benchmark(
        "UnsignedPolynomial<u64>.evaluate_mod(u64, u64)",
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
                no_out!(p.evaluate_mod(x, m));
            }),
            ("Horner", &mut |(p, x, m)| {
                no_out!(evaluate_mod_horner(p.coefficients_asc(), x, m));
            }),
            ("Shoup", &mut |(p, x, m)| {
                let x_precomp = mod_mul_precompute_shoup(x, m);
                no_out!(evaluate_mod_shoup(p.coefficients_asc(), x, x_precomp, m));
            }),
            ("lazy Shoup", &mut |(p, x, m)| {
                let x_precomp = mod_mul_precompute_shoup(x, m);
                no_out!(evaluate_mod_shoup_lazy(
                    p.coefficients_asc(),
                    x,
                    x_precomp,
                    m,
                ));
            }),
            ("naive", &mut |(p, x, m)| {
                no_out!(evaluate_mod_naive(&p, x, m));
            }),
        ],
    );
}

fn demo_unsigned_polynomial_evaluate_many_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, xs, m) in unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_many_mod(&{xs:?}, {m}) = {:?}",
            (&p).evaluate_many_mod(&xs, m)
        );
    }
}

fn demo_unsigned_polynomial_evaluate_geometric_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, k, m) in unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_geometric_mod({q}, {k}, {m}) = {:?}",
            (&p).evaluate_geometric_mod(q, k, m)
        );
    }
}

fn benchmark_unsigned_polynomial_evaluate_many_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedPolynomial<u64>).evaluate_many_mod(&[u64], u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, xs, m)| {
                no_out!((&p).evaluate_many_mod(&xs, m));
            }),
            ("one at a time", &mut |(p, xs, m)| {
                no_out!(
                    xs.iter()
                        .map(|&x| (&p).evaluate_mod(x, m))
                        .collect::<Vec<_>>()
                );
            }),
            ("naive", &mut |(p, xs, m)| {
                no_out!(evaluate_many_mod_naive(&p, &xs, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_evaluate_geometric_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedPolynomial<u64>).evaluate_geometric_mod(u64, u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, q, k, m)| {
                no_out!((&p).evaluate_geometric_mod(q, k, m));
            }),
            ("one at a time", &mut |(p, q, k, m)| {
                no_out!(
                    (0..k)
                        .map(|j| (&p).evaluate_mod(q.mod_pow(j, m), m))
                        .collect::<Vec<_>>()
                );
            }),
            ("naive", &mut |(p, q, k, m)| {
                no_out!(evaluate_geometric_mod_naive(&p, q, k, m));
            }),
        ],
    );
}
