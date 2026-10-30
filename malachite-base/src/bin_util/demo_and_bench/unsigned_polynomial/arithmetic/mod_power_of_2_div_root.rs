// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModPowerOf2DivRoot, ModPowerOf2DivRootAssign, ModPowerOf2Evaluate,
};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_div_root::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_div_root);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_div_root_ref);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_div_root_assign
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_div_root_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_div_root_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_div_root<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let (q, r) = p.mod_power_of_2_div_root(c, pow);
        println!("({p_old}).mod_power_of_2_div_root({c}, {pow}) = ({q}, {r})");
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_div_root_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (q, r) = (&p).mod_power_of_2_div_root(c, pow);
        println!("(&({p})).mod_power_of_2_div_root({c}, {pow}) = ({q}, {r})");
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_div_root_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, c, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let r = p.mod_power_of_2_div_root_assign(c, pow);
        println!("p := {p_old}; p.mod_power_of_2_div_root_assign({c}, {pow}) = {r}; p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_div_root_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_div_root({}, {})",
            T::NAME,
            T::NAME,
            "u64"
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("p.mod_power_of_2_div_root(c, pow)", &mut |(p, c, pow)| {
                no_out!(p.mod_power_of_2_div_root(c, pow));
            }),
            ("(&p).mod_power_of_2_div_root(c, pow)", &mut |(
                p,
                c,
                pow,
            )| {
                no_out!((&p).mod_power_of_2_div_root(c, pow));
            }),
            (
                "p.mod_power_of_2_div_root_assign(c, pow)",
                &mut |(mut p, c, pow)| {
                    no_out!(p.mod_power_of_2_div_root_assign(c, pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_div_root_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_div_root({}, {})",
            T::NAME,
            T::NAME,
            "u64"
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, c, pow)| {
                no_out!((&p).mod_power_of_2_div_root(c, pow));
            }),
            ("naive", &mut |(p, c, pow)| {
                no_out!(mod_power_of_2_div_root_naive(&p, c, pow));
            }),
            ("evaluation only", &mut |(p, c, pow)| {
                no_out!((&p).mod_power_of_2_evaluate(c, pow));
            }),
        ],
    );
}
