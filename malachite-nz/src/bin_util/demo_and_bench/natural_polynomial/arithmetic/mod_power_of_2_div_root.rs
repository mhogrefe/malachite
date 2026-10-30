// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{
    ModPowerOf2DivRoot, ModPowerOf2DivRootAssign, ModPowerOf2Evaluate,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_div_root::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_div_root);
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_div_root_ref_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_div_root_assign
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_div_root_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_div_root_algorithms
    );
}

fn demo_natural_polynomial_mod_power_of_2_div_root(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let c_old = c.clone();
        let (q, r) = p.mod_power_of_2_div_root(c, pow);
        println!("({p_old}).mod_power_of_2_div_root({c_old}, {pow}) = ({q}, {r})");
    }
}

fn demo_natural_polynomial_mod_power_of_2_div_root_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (q, r) = (&p).mod_power_of_2_div_root(&c, pow);
        println!("(&({p})).mod_power_of_2_div_root(&({c}), {pow}) = ({q}, {r})");
    }
}

fn demo_natural_polynomial_mod_power_of_2_div_root_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, c, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let c_old = c.clone();
        let r = p.mod_power_of_2_div_root_assign(c, pow);
        println!("p := {p_old}; p.mod_power_of_2_div_root_assign({c_old}, {pow}) = {r}; p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_div_root_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_div_root(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("p.mod_power_of_2_div_root(c, pow)", &mut |(p, c, pow)| {
                no_out!(p.mod_power_of_2_div_root(c, pow));
            }),
            ("p.mod_power_of_2_div_root(&c, pow)", &mut |(p, c, pow)| {
                no_out!(p.mod_power_of_2_div_root(&c, pow));
            }),
            ("(&p).mod_power_of_2_div_root(c, pow)", &mut |(
                p,
                c,
                pow,
            )| {
                no_out!((&p).mod_power_of_2_div_root(c, pow));
            }),
            (
                "(&p).mod_power_of_2_div_root(&c, pow)",
                &mut |(p, c, pow)| {
                    no_out!((&p).mod_power_of_2_div_root(&c, pow));
                },
            ),
            (
                "p.mod_power_of_2_div_root_assign(c, pow)",
                &mut |(mut p, c, pow)| {
                    no_out!(p.mod_power_of_2_div_root_assign(c, pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_div_root_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_div_root(Natural, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, c, pow)| {
                no_out!((&p).mod_power_of_2_div_root(&c, pow));
            }),
            ("naive", &mut |(p, c, pow)| {
                no_out!(mod_power_of_2_div_root_naive(&p, &c, pow));
            }),
            ("evaluation only", &mut |(p, c, pow)| {
                no_out!((&p).mod_power_of_2_evaluate(&c, pow));
            }),
        ],
    );
}
