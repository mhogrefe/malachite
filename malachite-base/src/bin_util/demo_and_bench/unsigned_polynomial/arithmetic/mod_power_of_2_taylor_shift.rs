// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModPowerOf2TaylorShift, ModPowerOf2TaylorShiftAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_10,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_11,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_taylor_shift::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_taylor_shift::{
    mod_power_of_2_taylor_shift_divide_and_conquer, mod_power_of_2_taylor_shift_horner,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_taylor_shift);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_taylor_shift_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_taylor_shift_assign
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_taylor_shift_long
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_taylor_shift_small_shift
    );

    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_taylor_shift_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_taylor_shift_algorithms
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_taylor_shift_algorithms_long
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_taylor_shift<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_taylor_shift({c}, {pow}) = {}",
            p.mod_power_of_2_taylor_shift(c, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_taylor_shift_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_taylor_shift({c}, {pow}) = {}",
            (&p).mod_power_of_2_taylor_shift(c, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_taylor_shift_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, c, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_taylor_shift_assign(c, pow);
        println!("p := {p_old}; p.mod_power_of_2_taylor_shift_assign({c}, {pow}); p = {p}");
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_taylor_shift_long<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_10::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).mod_power_of_2_taylor_shift({c}, {pow}) = {}",
            (&p).mod_power_of_2_taylor_shift(c, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_taylor_shift_small_shift<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_11::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).mod_power_of_2_taylor_shift({c}, {pow}) = {}",
            (&p).mod_power_of_2_taylor_shift(c, pow)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_taylor_shift_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_taylor_shift({}, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            (
                "p.mod_power_of_2_taylor_shift(c, pow)",
                &mut |(p, c, pow)| {
                    no_out!(p.mod_power_of_2_taylor_shift(c, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_taylor_shift(c, pow)",
                &mut |(p, c, pow)| {
                    no_out!((&p).mod_power_of_2_taylor_shift(c, pow));
                },
            ),
            (
                "p.mod_power_of_2_taylor_shift_assign(c, pow)",
                &mut |(mut p, c, pow)| {
                    p.mod_power_of_2_taylor_shift_assign(c, pow);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_taylor_shift_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_taylor_shift({}, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, c, pow)| {
                no_out!((&p).mod_power_of_2_taylor_shift(c, pow));
            }),
            ("Horner", &mut |(p, c, pow)| {
                let mut xs = p.coefficients_asc().to_vec();
                mod_power_of_2_taylor_shift_horner(&mut xs, c, pow);
            }),
            ("divide and conquer", &mut |(p, c, pow)| {
                let mut xs = p.coefficients_asc().to_vec();
                mod_power_of_2_taylor_shift_divide_and_conquer(&mut xs, c, pow);
            }),
            ("naive", &mut |(p, c, pow)| {
                no_out!(mod_power_of_2_taylor_shift_naive(&p, c, pow));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_taylor_shift_algorithms_long<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_taylor_shift({}, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_10::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, c, pow)| {
                no_out!((&p).mod_power_of_2_taylor_shift(c, pow));
            }),
            ("naive", &mut |(p, c, pow)| {
                no_out!(mod_power_of_2_taylor_shift_naive(&p, c, pow));
            }),
        ],
    );
}
